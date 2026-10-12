//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.Screen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$releaseCursorWhileUnfocused
///   java: @Inject(method = "tick", at = @At("HEAD"))
/// target: net.minecraft.client.gui.screens.Screen#tick  [exists]
/// Rust: net/minecraft/client/gui/screens/screen.rs:5977 fn tick(&mut self, _screen: &mut Screen, _ctx: &mut ScreenContext<'_>)
fn skyblocker_release_cursor_while_unfocused(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::screens::screen::Screen>()) else { return; };
    // TODO(fabric2seer): the condition: `.isWindowActive(..)` on a Minecraft
    //   java: ((Object) this instanceof ServerReconfigScreen || (Object) this instanceof LevelLoadingScreen) && Utils.isOnHypixel() && !this.minecraft.isWindowActive()
    if todo!("fabric2seer: the condition (TODO above)") {
        crate::de::hysky::skyblocker::utils::render::gui::server_transfer_helper::ServerTransferHelper::set_interrupted(true);
        // TODO(fabric2seer): the condition: field `mouseHandler` of a Minecraft
        //   java: this.minecraft.mouseHandler.isMouseGrabbed(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): field `mouseHandler` of a Minecraft
            //   java: this.minecraft.mouseHandler.releaseMouse();
        }
    }
}

/// @Inject skyblocker$hideReconfiguringScreen
///   java: @Inject(method = "extractRenderState", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.gui.screens.Screen#extractRenderState  [no Rust counterpart]
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
fn skyblocker_hide_reconfiguring_screen(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): the condition: `this` (the hooked object) is not available in this callback
    //   java: (Object) this instanceof ServerReconfigScreen && Utils.isOnHypixel() && !ServerTransferHelper.isInterrupted()
    if todo!("fabric2seer: the condition (TODO above)") {
        ci.cancel();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.Screen#tick", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_release_cursor_while_unfocused)));
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
    // report(hook::register_injector(`net.minecraft.client.gui.screens.Screen#extractRenderState`, InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_hide_reconfiguring_screen)));
}
