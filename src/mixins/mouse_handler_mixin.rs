//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MouseHandlerMixin by fabric2seer.
//! @Mixin(net.minecraft.client.MouseHandler)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyExpressionValue skyblocker$gardenMouseLock
///   java: @ModifyExpressionValue(method = "turnPlayer", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/OptionInstance;get()Ljava/lang/Object;", ordinal = 0))
/// target: net.minecraft.client.OptionInstance#get  [exists]
/// within: net.minecraft.client.MouseHandler#turnPlayer
/// Rust: net/minecraft/client/option_instance.rs:624 fn get(&self)
/// note: ordinal 0: the call is in a `?:` in the Java of the method, so the run-time count (per run of the method) may differ from Mixin's bytecode ordinal - check it
fn skyblocker_garden_mouse_lock(v: compat::Object) -> compat::Object {
    let mut original = v;
    if crate::de::hysky::skyblocker::skyblock::garden::lower_sensitivity::LowerSensitivity::is_sensitivity_lowered() {
        return (std::sync::Arc::new(((-1 as f64) / 3.0)) as compat::Object);
    } else {
        return original;
    }
}

/// @Inject skyblocker$keepCursorGrabbedDuringTransfer
///   java: @Inject(method = "releaseMouse", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.MouseHandler#releaseMouse  [exists]
/// Rust: net/minecraft/client/mouse_handler.rs:508 fn release_mouse(&mut self, screen_width: i32, screen_height: i32)
fn skyblocker_keep_cursor_grabbed_during_transfer(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::mouse_handler::MouseHandler>()) else { return; };
    // TODO(fabric2seer): the condition: `.isWindowActive(..)` on a Minecraft
    //   java: this.minecraft.isWindowActive() && (this.minecraft.gui.screen() instanceof LevelLoadingScreen || this.minecraft.gui.screen() instanceof ServerReconfigScreen) && Utils.isOnHypixel()
    if todo!("fabric2seer: the condition (TODO above)") {
        ci.cancel();
    }
}

/// @Inject dontResetMouseInStorageOverlay
///   java: @Inject(method = "releaseMouse", at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/platform/InputConstants;grabOrReleaseMouse(Lcom/mojang/blaze3d/platform/Window;IDD)V", shift = At.Shift.AFTER))
/// target: com.mojang.blaze3d.platform.InputConstants#grabOrReleaseMouse  [exists]
/// within: net.minecraft.client.MouseHandler#releaseMouse
/// Rust: com/mojang/blaze3d/platform/input_constants.rs:711 fn grab_or_release_mouse(g: &crate::lwjgl::glfw::Glfw, handle: *mut std::ffi::c_void, cursor_mode: std::os::raw::c_int, xpos: f64, ypos: f64)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn dont_reset_mouse_in_storage_overlay(ci: &mut CallbackInfo<'_>) {
    if mcrust::mcrust::modding::container_screens::current_mod_container_screen::<compat::ContainerScreenAdapter<crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen>, _>(|_| ()).is_some() {
        let mut position: crate::unresolved::Vector2dc = crate::de::hysky::skyblocker::skyblock::storageoverlay::storage_overlay_screen::StorageOverlayScreen::get_previous_mouse_position().unwrap();
        if true /* java: position != null: never null here */ {
            // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
            //   java: this.xpos = position.x();
            // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
            //   java: this.ypos = position.y();
        }
        // TODO(fabric2seer): static call GLFW.glfwSetCursorPos(..)
        //   java: GLFW.glfwSetCursorPos(minecraft.getWindow().handle(), xpos, ypos);
    }
}

/// @Inject skyblocker$resetTransferState
///   java: @Inject(method = "grabMouse", at = @At("HEAD"))
/// target: net.minecraft.client.MouseHandler#grabMouse  [exists]
/// Rust: net/minecraft/client/mouse_handler.rs:490 fn grab_mouse(&mut self, screen_width: i32, screen_height: i32)
fn skyblocker_reset_transfer_state(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::mouse_handler::MouseHandler>()) else { return; };
    crate::de::hysky::skyblocker::utils::render::gui::server_transfer_helper::ServerTransferHelper::set_interrupted(false);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.MouseHandler#turnPlayer", "net.minecraft.client.OptionInstance#get").ordinal(0).priority(1000).modify_expression_value::<compat::Object>(skyblocker_garden_mouse_lock));
    report(hook::register_injector("net.minecraft.client.MouseHandler#releaseMouse", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_keep_cursor_grabbed_during_transfer)));
    report(hook::call_site("net.minecraft.client.MouseHandler#releaseMouse", "com.mojang.blaze3d.platform.InputConstants#grabOrReleaseMouse").priority(1000).inject(Shift::After, dont_reset_mouse_in_storage_overlay));
    report(hook::register_injector("net.minecraft.client.MouseHandler#grabMouse", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_reset_transfer_state)));
}
