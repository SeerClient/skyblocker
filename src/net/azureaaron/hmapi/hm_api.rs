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
            mcrust::mcrust::modding::events::client_command_registration_callback::EVENT.register(Arc::new(|dispatcher: &mut mcrust::mcrust::modding::events::client_command_registration_callback::ClientCommandDispatcher| HMApi::register_commands(dispatcher)));
            hypixel_packet_events::PARTY_INFO.get_mut().register(Arc::new(HMApi::log_packet));
            hypixel_packet_events::PLAYER_INFO.get_mut().register(Arc::new(HMApi::log_packet));
            hypixel_packet_events::HELLO.get_mut().register(Arc::new(HMApi::log_packet));
            hypixel_packet_events::LOCATION_UPDATE.get_mut().register(Arc::new(HMApi::log_packet));
        }
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    fn register_commands(dispatcher: &mut mcrust::mcrust::modding::events::client_command_registration_callback::ClientCommandDispatcher) {
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
        use mcrust::mcrust::modding::events::client_command_registration_callback::{self as commands, FabricClientCommandSource};
        use crate::net::azureaaron::hmapi::network::hypixel_networking::HypixelNetworking;
        // java: ClientCommands.literal("partyInfo2").executes(_ -> { HypixelNetworking.sendPartyInfoC2SPacket(2); return Command.SINGLE_SUCCESS; })
        let mut party_info2 = commands::literal("partyInfo2");
        party_info2.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            HypixelNetworking::send_party_info_c2s_packet(2);
            Ok(1)
        }));
        // java: ClientCommands.literal("playerInfo").executes(_ -> { HypixelNetworking.sendPlayerInfoC2SPacket(1); return Command.SINGLE_SUCCESS; })
        let mut player_info = commands::literal("playerInfo");
        player_info.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            HypixelNetworking::send_player_info_c2s_packet(1);
            Ok(1)
        }));
        // java: ClientCommands.literal("register4LocationUpdates").executes(_ -> { HypixelNetworking.registerToEvents(Util.make(new Object2IntOpenHashMap<>(), map -> map.put(LocationUpdateS2CPacket.ID, 1))); .. })
        let mut register_location_updates = commands::literal("register4LocationUpdates");
        register_location_updates.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            HypixelNetworking::register_to_events(&[(crate::net::azureaaron::hmapi::network::packet::v1::s2c::location_update_s2c_packet::LocationUpdateS2CPacket::ID, 1)]);
            Ok(1)
        }));
        let mut send_packet = commands::literal("sendPacket");
        send_packet.then(&party_info2);
        send_packet.then(&player_info);
        send_packet.then(&register_location_updates);
        // java: ClientCommands.literal("toggleSendPacketsInChat").executes(_ -> { sendPacketsInChat = true; .. })
        let mut toggle_send_packets_in_chat = commands::literal("toggleSendPacketsInChat");
        toggle_send_packets_in_chat.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            SEND_PACKETS_IN_CHAT.store(true, Ordering::Relaxed);
            Ok(1)
        }));
        let mut hmapi = commands::literal("hmapi");
        hmapi.then(&send_packet);
        hmapi.then(&toggle_send_packets_in_chat);
        dispatcher.dispatcher().register(&hmapi);
    }

    /// java: private static void logPacket(HypixelS2CPacket packet)
    fn log_packet(packet: &HypixelS2CPacket) {
        logger().info(&format!("[HM API] Received Packet: {:?}", packet));
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && SEND_PACKETS_IN_CHAT.load(Ordering::Relaxed) {
            script_host::send_feedback(&format!("{:?}", packet));
        }
    }
}
