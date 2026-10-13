//! Ported from net.azureaaron.hmapi.HMApi (the library's ClientModInitializer) and its three mixins.

use crate::net::azureaaron::hmapi::events::hypixel_packet_events;
use crate::net::azureaaron::hmapi::network::hypixel_networking_impl::HypixelNetworkingImpl;
use crate::net::azureaaron::hmapi::network::packet::s2c::hypixel_s2c_packet::HypixelS2CPacket;
use crate::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

/// java: private static final boolean DEBUG_ENABLED = Boolean.parseBoolean(System.getProperty("hmapi.debug", "false")) || FabricLoader.getInstance().isDevelopmentEnvironment();
/// (system properties are environment variables here; the client is never a development environment)
fn debug_enabled() -> bool {
    std::env::var("hmapi.debug").unwrap_or_else(|_| "false".to_string()).eq_ignore_ascii_case("true")
}

/// java: private static boolean sendPacketsInChat;
static SEND_PACKETS_IN_CHAT: AtomicBool = AtomicBool::new(false);

/// java: public class HMApi implements ClientModInitializer
pub struct HMApi;

impl HMApi {
    /// The library's three mixins (`ClientboundCustomPayloadPacketMixin` / `ServerboundCustomPayloadPacketMixin` wrap
    /// the custom-payload codecs so the Hypixel channels decode, `ClientCommonPacketListenerImplMixin` hands the
    /// decoded packets to `HypixelNetworkingImpl.handlePayload`): in the port a mod's channel is decoded and handed to
    /// it through `networking::register_global_receiver` - registered before the codec tables are built, as a mixin
    /// applies before the classes load.
    pub fn register_mixins() {
        HypixelNetworkingImpl::register_channels();
    }

    /// java: @Override public void onInitializeClient()
    pub fn on_initialize_client() {
        if debug_enabled() {
            // java: ClientCommandRegistrationCallback.EVENT.register(HMApi::registerCommands);
            // rust: port gap - the `hmapi` debug command tree (sendPacket partyInfo2 / playerInfo / register4LocationUpdates,
            // toggleSendPacketsInChat) needs nested literals, which the port's command dispatcher does not take (the
            // mod's GAPS: command trees)
            hypixel_packet_events::PARTY_INFO.get_mut().register(Arc::new(HMApi::log_packet));
            hypixel_packet_events::PLAYER_INFO.get_mut().register(Arc::new(HMApi::log_packet));
            hypixel_packet_events::HELLO.get_mut().register(Arc::new(HMApi::log_packet));
            hypixel_packet_events::LOCATION_UPDATE.get_mut().register(Arc::new(HMApi::log_packet));
        }
    }

    /// java: private static void logPacket(HypixelS2CPacket packet)
    fn log_packet(packet: &HypixelS2CPacket) {
        logger().info(&format!("[HM API] Received Packet: {:?}", packet));
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && SEND_PACKETS_IN_CHAT.load(Ordering::Relaxed) {
            script_host::send_feedback(&format!("{:?}", packet));
        }
    }
}
