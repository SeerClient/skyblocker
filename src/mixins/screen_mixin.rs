//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.Screen)

#[allow(unused_imports)]
use crate::prelude::*;

/// `(Object) this instanceof ServerReconfigScreen || (Object) this instanceof LevelLoadingScreen`: the screen being
/// ticked is the open one, and the port names the open screen's class in `gui.screen_kind`. (The hook's `this` is
/// the screen's `ScreenHandler` / state object, not a `Screen`.)
fn is_transfer_screen(minecraft: &mcrust::net::minecraft::client::minecraft::Minecraft) -> bool {
    use mcrust::net::minecraft::client::gui::gui::ScreenKind;
    minecraft.gui.screen_kind == ScreenKind::ServerReconfig || minecraft.gui.screen_kind == ScreenKind::LevelLoading
}

/// @Inject skyblocker$releaseCursorWhileUnfocused
///   java: @Inject(method = "tick", at = @At("HEAD"))
/// target: net.minecraft.client.gui.screens.Screen#tick  [exists]
/// Rust: net/minecraft/client/gui/screens/screen.rs:5977 fn tick(&mut self, _screen: &mut Screen, _ctx: &mut ScreenContext<'_>)
///
/// `ServerReconfigScreen.tick:48-49` and `LevelLoadingScreen.tick:83-84` both open with `super.tick()`, so in Java
/// this HEAD runs first thing in their ticks. The port's two bodies are their own hooked methods
/// (`ServerReconfigScreen#tick` = `tick_screen`, `LevelLoadingScreen#tick` = `tick_progress`) that do not call the
/// `ScreenHandler::tick` default, so the handler is registered at their HEADs too.
fn skyblocker_release_cursor_while_unfocused(_ci: &mut CallbackInfo<'_>) {
    let minecraft = compat::minecraft();
    // If the window isn't focused while a transfer screen is up, give the cursor back so it isn't hidden when you come back
    if is_transfer_screen(minecraft) && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() && !minecraft.is_window_active() {
        crate::de::hysky::skyblocker::utils::render::gui::server_transfer_helper::ServerTransferHelper::set_interrupted(true);
        if minecraft.mouse_handler.is_mouse_grabbed() {
            //   java: this.minecraft.mouseHandler.releaseMouse();
            // The port's `releaseMouse` (26.3 :433-440) answers `true` where Java goes on to
            // `InputConstants.releaseMouse(window, xpos, ypos)` (:438), and leaves that call to its caller (the window
            // handle lives outside it) - the call every game caller makes after it (`gui.rs`).
            let (width, height) = (minecraft.window.get_screen_width(), minecraft.window.get_screen_height());
            if minecraft.mouse_handler.release_mouse(width, height) {
                let handle = mcrust::com::mojang::blaze3d::platform::window::published_handle();
                if !handle.is_null() {
                    mcrust::com::mojang::blaze3d::platform::input_constants::release_mouse(handle, minecraft.mouse_handler.xpos, minecraft.mouse_handler.ypos);
                }
            }
        }
    }
}

/// @Inject skyblocker$hideReconfiguringScreen
///   java: @Inject(method = "extractRenderState", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.gui.screens.Screen#extractRenderState  [exists: abstract_widget.rs
/// `draw_screen_widgets_with`, `Screen.extractRenderState:115-119`'s renderables pass, which draws the
/// `ServerReconfigScreen`'s title and disconnect button]
fn skyblocker_hide_reconfiguring_screen(ci: &mut CallbackInfo<'_>) {
    use mcrust::net::minecraft::client::gui::gui::ScreenKind;
    //   java: (Object) this instanceof ServerReconfigScreen && Utils.isOnHypixel() && !ServerTransferHelper.isInterrupted()
    if compat::minecraft().gui.screen_kind == ScreenKind::ServerReconfig && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() && !crate::de::hysky::skyblocker::utils::render::gui::server_transfer_helper::ServerTransferHelper::is_interrupted() {
        ci.cancel();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.Screen#tick", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_release_cursor_while_unfocused)));
    // `super.tick()` at the top of the two subclasses' own ticks (see the handler).
    report(hook::register_injector("net.minecraft.client.gui.screens.multiplayer.ServerReconfigScreen#tick", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_release_cursor_while_unfocused)));
    // `level_loading_screen::tick`, entered by `tick_arm` while the loading screen is up.
    report(hook::register_injector("net.minecraft.client.gui.screens.LevelLoadingScreen#tick", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_release_cursor_while_unfocused)));
    report(hook::register_injector("net.minecraft.client.gui.screens.Screen#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_hide_reconfiguring_screen)));
}
