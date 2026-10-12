//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MouseHandlerMixin by fabric2seer.
//! @Mixin(net.minecraft.client.MouseHandler)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyExpressionValue skyblocker$gardenMouseLock
///   java: @ModifyExpressionValue(method = "turnPlayer", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/OptionInstance;get()Ljava/lang/Object;", ordinal = 0))
/// target: net.minecraft.client.OptionInstance#get  [exists]
/// within: net.minecraft.client.MouseHandler#turnPlayer
/// Rust: net/minecraft/client/option_instance.rs:624 fn get(&self)
///
/// `ordinal = 0` is `turnPlayer:352`'s `this.minecraft.options.sensitivity().get()`. The port reads that option in
/// the caller and hands its value in as `turn_player`'s `sensitivity` (argument 1, used at `:352`), so the value the
/// expression produces is that argument: the modification is an `@ModifyArg` on it.
fn skyblocker_garden_mouse_lock(v: &f64) -> f64 {
    let mut original = *v;
    if crate::de::hysky::skyblocker::skyblock::garden::lower_sensitivity::LowerSensitivity::is_sensitivity_lowered() {
        return -1.0 / 3.0;
    } else {
        return original;
    }
}

/// @Inject skyblocker$keepCursorGrabbedDuringTransfer
///   java: @Inject(method = "releaseMouse", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.MouseHandler#releaseMouse  [exists]
/// Rust: net/minecraft/client/mouse_handler.rs:508 fn release_mouse(&mut self, screen_width: i32, screen_height: i32)
fn skyblocker_keep_cursor_grabbed_during_transfer(ci: &mut CallbackInfo<'_>) {
    use mcrust::net::minecraft::client::gui::gui::ScreenKind;
    // Keep the cursor hidden through Hypixel's transfer screens so the transfer looks seamless
    let minecraft = compat::minecraft();
    if minecraft.is_window_active() && (minecraft.gui.screen_kind == ScreenKind::LevelLoading || minecraft.gui.screen_kind == ScreenKind::ServerReconfig) && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() {
        // The port's `releaseMouse` answers whether the caller releases the cursor (`grabOrReleaseMouse`); a
        // cancelled one releases nothing.
        ci.set_return_value(false);
    }
}

/// @Inject dontResetMouseInStorageOverlay
///   java: @Inject(method = "releaseMouse", at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/platform/InputConstants;grabOrReleaseMouse(Lcom/mojang/blaze3d/platform/Window;IDD)V", shift = At.Shift.AFTER))
/// target: com.mojang.blaze3d.platform.InputConstants#grabOrReleaseMouse  [exists]
/// within: net.minecraft.client.MouseHandler#releaseMouse
/// Rust: com/mojang/blaze3d/platform/input_constants.rs:711 fn grab_or_release_mouse(g: &crate::lwjgl::glfw::Glfw, handle: *mut std::ffi::c_void, cursor_mode: std::os::raw::c_int, xpos: f64, ypos: f64)
///
/// The port's `releaseMouse` (`:426-433`) answers `true` where Java goes on to `grabOrReleaseMouse(window, NORMAL,
/// xpos, ypos)`, and its caller makes that call with `this.xpos` / `this.ypos` right after. So "after the call, set
/// `xpos` / `ypos` and put the cursor there" is: at RETURN, when the answer is `true`, set the two fields the
/// caller's `grabOrReleaseMouse` then places the cursor at.
fn dont_reset_mouse_in_storage_overlay(ci: &mut CallbackInfo<'_>) {
    if ci.return_value::<bool>().copied() != Some(true) {
        return;
    }
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::mouse_handler::MouseHandlerState>()) else { return; };
    if mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen>, _>(|_| ()).is_some() {
        let position: Option<crate::unresolved::Vector2dc> = crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen::get_previous_mouse_position();
        if let Some(position) = position {
            this.xpos = position.x();
            this.ypos = position.y();
        }
        //   java: GLFW.glfwSetCursorPos(minecraft.getWindow().handle(), xpos, ypos); - the caller's `grabOrReleaseMouse`.
    }
}

/// @Inject skyblocker$resetTransferState
///   java: @Inject(method = "grabMouse", at = @At("HEAD"))
/// target: net.minecraft.client.MouseHandler#grabMouse  [exists]
/// Rust: net/minecraft/client/mouse_handler.rs:490 fn grab_mouse(&mut self, screen_width: i32, screen_height: i32)
fn skyblocker_reset_transfer_state(ci: &mut CallbackInfo<'_>) {
    // The transfer is over once we're back in the game
    crate::de::hysky::skyblocker::utils::render::gui::server_transfer_helper::ServerTransferHelper::set_interrupted(false);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // java: @At(INVOKE OptionInstance.get, ordinal = 0) in turnPlayer - see `skyblocker_garden_mouse_lock`.
    report(hook::modify_arg::<f64>("net.minecraft.client.MouseHandler#turnPlayer", 1, skyblocker_garden_mouse_lock));
    report(hook::register_injector("net.minecraft.client.MouseHandler#releaseMouse", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_keep_cursor_grabbed_during_transfer)));
    // java: @At(INVOKE InputConstants.grabOrReleaseMouse, shift = AFTER) in releaseMouse - see the handler.
    report(hook::register_injector("net.minecraft.client.MouseHandler#releaseMouse", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(dont_reset_mouse_in_storage_overlay)));
    report(hook::register_injector("net.minecraft.client.MouseHandler#grabMouse", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_reset_transfer_state)));
}
