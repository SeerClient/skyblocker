//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.PlayerTabOverlayMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.PlayerTabOverlay)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$hideLatencyIcon
///   java: @Inject(method = "extractPingIcon", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.gui.components.PlayerTabOverlay#extractPingIcon  [exists]
/// Rust: net/minecraft/client/gui/components/player_tab_overlay.rs:2661 fn extract_ping_icon(gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, slot_width: i32, xo: i32, yo: i32, latency: i32)
fn skyblocker_hide_latency_icon(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        ci.cancel();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.components.PlayerTabOverlay#extractPingIcon", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_hide_latency_icon)));
}
