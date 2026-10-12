//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ClientCommonPacketListenerImplMixin by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$onServerTick
///   java: @Inject(method = "handlePing", at = @At("RETURN"))
/// target: net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl#handlePing  [exists]
/// Rust: net/minecraft/client/multiplayer/client_common_packet_listener_impl.rs:1204 fn handle_ping(&mut self, packet: &ClientboundPingPacket)
fn skyblocker_on_server_tick(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::multiplayer::client_common_packet_listener_impl::ClientCommonPacketListenerImpl>()) else { return; };
    let Some(packet) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::protocol::common::clientbound_ping_packet::ClientboundPingPacket>(0)) else { return; };
    crate::de::hysky::skyblocker::utils::server_tick_counter::ServerTickCounter::on_server_tick(&packet);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl#handlePing", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_server_tick)));
}
