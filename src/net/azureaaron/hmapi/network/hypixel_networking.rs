//! Ported from net.azureaaron.hmapi.network.HypixelNetworking.

use crate::net::azureaaron::hmapi::network::hypixel_networking_impl::HypixelNetworkingImpl;
use crate::net::azureaaron::hmapi::network::packet::c2s::hypixel_c2s_packet::HypixelC2SPacket;
use crate::net::azureaaron::hmapi::network::packet::c2s::party_info_c2s_packet::PartyInfoC2SPacket;
use crate::net::azureaaron::hmapi::network::packet::c2s::player_info_c2s_packet::PlayerInfoC2SPacket;
use crate::net::azureaaron::hmapi::network::packet::c2s::register_c2s_packet::RegisterC2SPacket;
use crate::net::azureaaron::hmapi::network::packet::v1::s2c::location_update_s2c_packet::LocationUpdateS2CPacket;
use crate::net::azureaaron::hmapi::utils::packet_send_result::PacketSendResult;
use crate::net::azureaaron::hmapi::utils::utils::Utils;
use std::sync::{LazyLock, Mutex};

/// java: private static final Object2ObjectMap<CustomPacketPayload.Type<HypixelS2CPacket>, IntList> VALID_EVENTS = .. {LocationUpdateS2CPacket.ID: [1]}
static VALID_EVENTS: LazyLock<Vec<(&'static str, Vec<i32>)>> = LazyLock::new(|| vec![(LocationUpdateS2CPacket::ID, vec![1])]);
/// java: static final Object2IntMap<CustomPacketPayload.Type<HypixelS2CPacket>> REGISTERED_EVENTS = new Object2IntOpenHashMap<>();
pub(crate) static REGISTERED_EVENTS: LazyLock<Mutex<Vec<(&'static str, i32)>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// java: public class HypixelNetworking
pub struct HypixelNetworking;

impl HypixelNetworking {
    /// java: public static PacketSendResult sendPartyInfoC2SPacket(int version)
    pub fn send_party_info_c2s_packet(version: i32) -> PacketSendResult {
        Utils::require_is_on_hypixel();
        HypixelNetworkingImpl::send_packet(HypixelC2SPacket::PartyInfo(PartyInfoC2SPacket::new(Utils::require_in_range(version, 2, 2))), false)
    }

    /// java: public static PacketSendResult sendPlayerInfoC2SPacket(int version)
    pub fn send_player_info_c2s_packet(version: i32) -> PacketSendResult {
        Utils::require_is_on_hypixel();
        HypixelNetworkingImpl::send_packet(HypixelC2SPacket::PlayerInfo(PlayerInfoC2SPacket::new(Utils::require_in_range(version, 1, 1))), false)
    }

    /// java: public static void registerToEvents(Object2IntMap<CustomPacketPayload.Type<HypixelS2CPacket>> requestedEvents)
    pub fn register_to_events(requested_events: &[(&'static str, i32)]) {
        let mut new_event_registrations: Vec<(&'static str, i32)> = Vec::new();
        let mut registered = REGISTERED_EVENTS.lock().unwrap_or_else(|p| p.into_inner());
        for (id, version) in requested_events.iter().copied() {
            //Require that the requested event is actually an event and that the version is supported
            if !VALID_EVENTS.iter().any(|(valid, versions)| *valid == id && versions.contains(&version)) {
                panic!("UnsupportedOperationException: {} is either not a valid event or is an unsupported version of an event!", id);
            }
            //If the event doesn't have any registrations or if the requested version is higher than the one currently registered (if any)
            match registered.iter().find(|(registered_id, _)| *registered_id == id) {
                Some((_, registered_version)) if version <= *registered_version => {}
                _ => new_event_registrations.push((id, version)),
            }
        }
        if !new_event_registrations.is_empty() {
            //Merge maps because the server stores the map of packets that we send as the ones the player wants
            for (id, version) in new_event_registrations {
                match registered.iter_mut().find(|(registered_id, _)| *registered_id == id) {
                    Some(entry) => entry.1 = version,
                    None => registered.push((id, version)),
                }
            }
            //Only register when we're on Hypixel to allow for events to be registered at mod init
            if Utils::is_on_hypixel() {
                let packets_to_register_for: Vec<(String, i32)> = registered.iter().map(|(id, version)| (id.to_string(), *version)).collect();
                drop(registered);
                let _ = HypixelNetworkingImpl::send_packet(HypixelC2SPacket::Register(RegisterC2SPacket::new(1, packets_to_register_for)), true);
            }
        }
    }
}
