//! Ported from net.azureaaron.hmapi.network.HypixelNetworkingImpl, with HypixelCustomPayloadCodecs and
//! PacketCodecUtils (the decode half the library's codec mixins install).

use crate::net::azureaaron::hmapi::events::hypixel_packet_events;
use crate::net::azureaaron::hmapi::network::hypixel_networking::REGISTERED_EVENTS;
use crate::net::azureaaron::hmapi::network::packet::c2s::hypixel_c2s_packet::HypixelC2SPacket;
use crate::net::azureaaron::hmapi::network::packet::c2s::register_c2s_packet::RegisterC2SPacket;
use crate::net::azureaaron::hmapi::network::packet::s2c::error_s2c_packet::ErrorS2CPacket;
use crate::net::azureaaron::hmapi::network::packet::s2c::hello_s2c_packet::HelloS2CPacket;
use crate::net::azureaaron::hmapi::network::packet::s2c::hypixel_s2c_packet::HypixelS2CPacket;
use crate::net::azureaaron::hmapi::network::packet::v1::s2c::location_update_s2c_packet::LocationUpdateS2CPacket;
use crate::net::azureaaron::hmapi::network::packet::v1::s2c::player_info_s2c_packet::PlayerInfoS2CPacket;
use crate::net::azureaaron::hmapi::network::packet::v2::s2c::party_info_s2c_packet::PartyInfoS2CPacket;
use crate::net::azureaaron::hmapi::utils::packet_send_result::PacketSendResult;
use crate::prelude::*;
use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBufReader;

/// java: private static final long COOLDOWN = 1000L;
const COOLDOWN: i64 = 1000;
/// java: private static final Object2LongMap<CustomPacketPayload.Type<?>> COOLDOWNS = Object2LongMaps.synchronize(new Object2LongOpenHashMap<>());
static COOLDOWNS: LazyLock<Mutex<std::collections::HashMap<&'static str, i64>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));

/// java: public class HypixelNetworkingImpl
pub struct HypixelNetworkingImpl;

impl HypixelNetworkingImpl {
    /// java: static <T extends HypixelC2SPacket> PacketSendResult sendPacket(T payload, boolean bypassCooldown)
    pub fn send_packet(payload: HypixelC2SPacket, bypass_cooldown: bool) -> PacketSendResult {
        let r#type = payload.r#type();
        let mut cooldowns = COOLDOWNS.lock().unwrap_or_else(|p| p.into_inner());
        let last: i64 = *cooldowns.entry(r#type).or_insert(0);
        if (compat::millis() + COOLDOWN > last) || bypass_cooldown {
            // java: Objects.requireNonNull(CLIENT.getConnection(), "Cannot send packet while not in game!").send(new ServerboundCustomPayloadPacket(payload));
            let bytes = payload.encode().unwrap_or_else(|e| panic!("EncoderException: {e}"));
            if let Err(e) = networking::send(r#type, bytes) {
                panic!("NullPointerException: Cannot send packet while not in game! ({e})");
            }
            cooldowns.insert(r#type, compat::millis());
            return PacketSendResult::success();
        }
        PacketSendResult::on_cooldown(cooldowns.get(r#type).copied().unwrap_or(0) - compat::millis())
    }

    /// java: private static void sendInitialEventRegistrations()
    fn send_initial_event_registrations() {
        let registered: Vec<(String, i32)> = REGISTERED_EVENTS.lock().unwrap_or_else(|p| p.into_inner()).iter().map(|(id, version)| (id.to_string(), *version)).collect();
        if !registered.is_empty() {
            let _ = HypixelNetworkingImpl::send_packet(HypixelC2SPacket::Register(RegisterC2SPacket::new(1, registered)), true);
        }
    }

    /// java: public static <T extends HypixelS2CPacket> void handlePayload(T payload)
    pub fn handle_payload(payload: HypixelS2CPacket) {
        // java: CLIENT.execute(() -> { switch (payload) {..} });
        crate::compat::execute(move || match &payload {
            HypixelS2CPacket::PartyInfo(_) => hypixel_packet_events::PARTY_INFO.get().invoke(&payload),
            HypixelS2CPacket::PlayerInfo(_) => hypixel_packet_events::PLAYER_INFO.get().invoke(&payload),
            HypixelS2CPacket::Hello(_) => {
                HypixelNetworkingImpl::send_initial_event_registrations();
                hypixel_packet_events::HELLO.get().invoke(&payload);
            }
            HypixelS2CPacket::LocationUpdate(_) => hypixel_packet_events::LOCATION_UPDATE.get().invoke(&payload),
            //Error cases
            HypixelS2CPacket::Error(error) if error.id == PartyInfoS2CPacket::ID => hypixel_packet_events::PARTY_INFO.get().invoke(&payload),
            HypixelS2CPacket::Error(error) if error.id == PlayerInfoS2CPacket::ID => hypixel_packet_events::PLAYER_INFO.get().invoke(&payload),
            HypixelS2CPacket::Error(error) if error.id == HelloS2CPacket::ID => hypixel_packet_events::HELLO.get().invoke(&payload),
            HypixelS2CPacket::Error(error) if error.id == LocationUpdateS2CPacket::ID => hypixel_packet_events::LOCATION_UPDATE.get().invoke(&payload),
            //When the packet encountered was unknown, likely due to encountering an unimplemented version of a packet
            HypixelS2CPacket::Nop => logger().warn("[HM API] Recevied an unknown or unexpected packet!"),
            _ => {}
        });
    }

    /// The library's codec and listener mixins: each Hypixel channel is a mod channel whose payload is decoded by
    /// `HypixelCustomPayloadCodecs.S2C_PACKET_CODEC`'s codec for it and handed to `handlePayload` (where Fabric's
    /// `ClientCommonPacketListenerImplMixin` cancels the vanilla handling). The C2S channels are registered for `send`.
    pub fn register_channels() {
        for id in [PartyInfoS2CPacket::ID, PlayerInfoS2CPacket::ID, HelloS2CPacket::ID, LocationUpdateS2CPacket::ID] {
            let result = networking::register_global_receiver(id, move |payload: &networking::ModPayload, _listener| {
                let packet = HypixelNetworkingImpl::decode_s2c(id, &payload.data);
                HypixelNetworkingImpl::handle_payload(packet);
            });
            if let Err(e) = result {
                logger().error(&format!("[HM API] Failed to register the {} channel: {}", id, e));
            }
        }
        if let Err(e) = networking::register_channel(RegisterC2SPacket::ID) {
            logger().error(&format!("[HM API] Failed to register the {} channel: {}", RegisterC2SPacket::ID, e));
        }
    }

    /// `HypixelCustomPayloadCodecs.S2C_PACKET_CODEC`'s entry for each channel: `dispatchHypixel` (a versioned packet) or
    /// `dispatchSafely` (the hello packet), with the channel's `ErrorS2CPacket` codec.
    fn decode_s2c(id: &'static str, data: &[u8]) -> HypixelS2CPacket {
        let mut buf = FriendlyByteBufReader::new(data);
        match id {
            HelloS2CPacket::ID => HypixelNetworkingImpl::dispatch_safely(&mut buf, id, |b| HelloS2CPacket::decode(b).map(HypixelS2CPacket::Hello)),
            PartyInfoS2CPacket::ID => HypixelNetworkingImpl::dispatch_hypixel(&mut buf, id, |version, b| match version {
                2 => Some(PartyInfoS2CPacket::decode(b).map(HypixelS2CPacket::PartyInfo)),
                _ => None,
            }),
            PlayerInfoS2CPacket::ID => HypixelNetworkingImpl::dispatch_hypixel(&mut buf, id, |version, b| match version {
                1 => Some(PlayerInfoS2CPacket::decode(b).map(HypixelS2CPacket::PlayerInfo)),
                _ => None,
            }),
            LocationUpdateS2CPacket::ID => HypixelNetworkingImpl::dispatch_hypixel(&mut buf, id, |version, b| match version {
                1 => Some(LocationUpdateS2CPacket::decode(b).map(HypixelS2CPacket::LocationUpdate)),
                _ => None,
            }),
            _ => HypixelS2CPacket::Nop,
        }
    }

    /// java: PacketCodecUtils.dispatchHypixel(primaryPacketCodecs, errorCodec).decode(buf): a success flag, then (on success)
    /// the version and that version's codec - any bytes it leaves are skipped (`readAllBytes`) - or (on failure) the
    /// error. A version with no codec is NOP. (A decode failure is the codec's exception: the payload is dropped, NOP.)
    fn dispatch_hypixel(
        buf: &mut FriendlyByteBufReader<'_>,
        id: &'static str,
        primary: impl FnOnce(i32, &mut FriendlyByteBufReader<'_>) -> Option<Result<HypixelS2CPacket, String>>,
    ) -> HypixelS2CPacket {
        let decoded: Result<HypixelS2CPacket, String> = (|| {
            let success = buf.read_boolean()?;
            let version = if success { buf.read_var_int()? } else { -1 };
            if success {
                match primary(version, buf) {
                    Some(packet) => {
                        let packet = packet?;
                        // readAllBytes(buf, true)
                        let rest = buf.readable_bytes();
                        buf.skip_bytes(rest)?;
                        Ok(packet)
                    }
                    None => Ok(HypixelS2CPacket::Nop),
                }
            } else {
                ErrorS2CPacket::decode(id, buf).map(HypixelS2CPacket::Error)
            }
        })();
        decoded.unwrap_or_else(|e| {
            logger().error(&format!("[HM API] Failed to decode a {} packet: {}", id, e));
            HypixelS2CPacket::Nop
        })
    }

    /// java: PacketCodecUtils.dispatchSafely(packetCodec, errorCodec).decode(buf): as `dispatchHypixel` with no version,
    /// and every failure logged and answered NOP.
    fn dispatch_safely(
        buf: &mut FriendlyByteBufReader<'_>,
        id: &'static str,
        packet_codec: impl FnOnce(&mut FriendlyByteBufReader<'_>) -> Result<HypixelS2CPacket, String>,
    ) -> HypixelS2CPacket {
        let decoded: Result<HypixelS2CPacket, String> = (|| {
            if buf.read_boolean()? {
                let packet = packet_codec(buf)?;
                // readAllBytes(buf, false)
                let rest = buf.readable_bytes();
                buf.skip_bytes(rest)?;
                Ok(packet)
            } else {
                ErrorS2CPacket::decode(id, buf).map(HypixelS2CPacket::Error)
            }
        })();
        decoded.unwrap_or_else(|e| {
            logger().error(&format!("[HM API] Encountered an unexpected exception while decoding a packet! {}", e));
            HypixelS2CPacket::Nop
        })
    }
}
