//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.LevelLoadingScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.LevelLoadingScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$hideWorldLoadingScreen
///   java: @Inject(method = { "extractRenderState", "extractBackground" }, at = @At("HEAD"), cancellable = true, require = 2)
/// target: net.minecraft.client.gui.screens.LevelLoadingScreen#extractRenderState  [exists]
/// Rust: net/minecraft/client/gui/screens/level_loading_screen.rs:258 fn extract_render_state(gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, gui_w: i32, gui_h: i32, load_tracker: Option<&crate::net::minecraft::client::multiplayer::level_load_tracker::LevelLoadTracker>, smoothed_progress: f32)
fn skyblocker_hide_world_loading_screen(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): the condition: `this` (the hooked object) is not available in this callback
    //   java: Utils.isOnHypixel() && this.loadTracker.statusView() == null
    if todo!("fabric2seer: the condition (TODO above)") {
        ci.cancel();
    }
}

/// @Inject skyblocker$hideWorldLoadingScreen
///   java: @Inject(method = { "extractRenderState", "extractBackground" }, at = @At("HEAD"), cancellable = true, require = 2)
/// target: net.minecraft.client.gui.screens.LevelLoadingScreen#extractBackground  [no Rust counterpart]
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
fn skyblocker_hide_world_loading_screen_1(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): the condition: `this` (the hooked object) is not available in this callback
    //   java: Utils.isOnHypixel() && this.loadTracker.statusView() == null
    if todo!("fabric2seer: the condition (TODO above)") {
        ci.cancel();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.LevelLoadingScreen#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_hide_world_loading_screen)));
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
    // report(hook::register_injector(`net.minecraft.client.gui.screens.LevelLoadingScreen#extractBackground`, InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_hide_world_loading_screen_1)));
}
