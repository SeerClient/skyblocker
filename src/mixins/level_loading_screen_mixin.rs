//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.LevelLoadingScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.LevelLoadingScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$hideWorldLoadingScreen
///   java: @Inject(method = { "extractRenderState", "extractBackground" }, at = @At("HEAD"), cancellable = true, require = 2)
/// target: net.minecraft.client.gui.screens.LevelLoadingScreen#extractRenderState  [exists]
/// Rust: net/minecraft/client/gui/screens/level_loading_screen.rs:258 fn extract_render_state(gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, gui_w: i32, gui_h: i32, load_tracker: Option<&crate::net::minecraft::client::multiplayer::level_load_tracker::LevelLoadTracker>, smoothed_progress: f32)
///
/// The port's screen body is a free function; `this.loadTracker` is its `load_tracker` argument (4). `None` there
/// is "no tracker published" - no status view either, which is what Java's test asks.
fn skyblocker_hide_world_loading_screen(ci: &mut CallbackInfo<'_>) {
    //   java: Utils.isOnHypixel() && this.loadTracker.statusView() == null
    let no_status_view: bool = compat::hook_arg::<Option<&mcrust::net::minecraft::client::multiplayer::level_load_tracker::LevelLoadTracker>>(ci, 4).copied().flatten().is_none_or(|tracker| tracker.status_view().is_none());
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() && no_status_view {
        ci.cancel();
    }
}

/// @Inject skyblocker$hideWorldLoadingScreen
///   java: @Inject(method = { "extractRenderState", "extractBackground" }, at = @At("HEAD"), cancellable = true, require = 2)
/// target: net.minecraft.client.gui.screens.LevelLoadingScreen#extractBackground  [exists]
/// Rust: net/minecraft/client/gui/screens/level_loading_screen.rs fn extract_background(gl, gfx, gui_w, gui_h, load_tracker, drawn: &mut bool)
/// - called by the screen pass before it draws the panorama, blur and menu tile; cancelling it draws none of them.
/// `this.loadTracker` is its argument 4, as for `extract_render_state`.
fn skyblocker_hide_world_loading_screen_1(ci: &mut CallbackInfo<'_>) {
    //   java: Utils.isOnHypixel() && this.loadTracker.statusView() == null
    let no_status_view: bool = compat::hook_arg::<Option<&mcrust::net::minecraft::client::multiplayer::level_load_tracker::LevelLoadTracker>>(ci, 4).copied().flatten().is_none_or(|tracker| tracker.status_view().is_none());
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() && no_status_view {
        ci.cancel();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.LevelLoadingScreen#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_hide_world_loading_screen)));
    report(hook::register_injector("net.minecraft.client.gui.screens.LevelLoadingScreen#extractBackground", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_hide_world_loading_screen_1)));
}
