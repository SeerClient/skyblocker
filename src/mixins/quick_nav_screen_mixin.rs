//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.QuickNavScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.Screen)
//!
//! The two handlers are empty here and overridden by `QuickNavMixin` (`AbstractContainerScreen`), so in Java they do
//! something only when the screen is a container screen. The port has no `Screen#extractBackground` /
//! `Screen#extractRenderStateWithTooltipAndSubtitles` bodies (the frame loop draws the backdrop and the strata), so
//! both are registered at the first hooked point a container screen's frame reaches after its background:
//! `AbstractContainerScreen#extractRenderState`'s HEAD ("the three statements every container screen runs once its
//! background is down"), unselected first, then selected - Java's order.
//!
//! The unselected tabs are drawn at `Hud#extractDeferredSubtitles` - Java's own point, which every port
//! `extractBackground` calls between its backdrop gradient and its panel - so they sit under the panel as in Java.
//! The selected tab is drawn at `AbstractContainerScreen#extractContents` HEAD (after the background), which also
//! updates every tab's game widget for the frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// `extractContents`'s frame partial tick (argument 10) - Java's `a`.
fn container_screen_partial_tick(ci: &CallbackInfo<'_>) -> f32 {
    ci.arg::<f32>(10).copied().unwrap_or_default()
}

/// @Inject skyblocker$extractUnselectedQuickNavButtons
///   java: @Inject(method = "extractBackground", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Hud;extractDeferredSubtitles()V"))
/// within: net.minecraft.client.gui.screens.Screen#extractBackground - the port's `extractBackground`s are per-class
/// free functions, so the call itself is the injection point (it is made from nowhere else).
fn skyblocker_extract_unselected_quick_nav_buttons(_ci: &mut CallbackInfo<'_>) {
    // `QuickNavMixin`'s override (`this instanceof AbstractContainerScreen`).
    crate::mixins::quick_nav_mixin::skyblocker_extract_unselected_quick_nav_buttons_backdrop();
}

/// Not a Java injector: the screen-holding half of the tabs' `extractRenderState` (box, `isHovered`), at
/// `Gui#extractRenderState` HEAD - the only hooked point of the frame handed the LIVE `Screen` (the one `init` added
/// the tabs to; the container screen the draw hooks get holds an empty husk - see `quick_nav_mixin::FRAME_SCREEN`).
fn skyblocker_update_quick_nav_buttons(ci: &mut CallbackInfo<'_>) {
    let (mouse_x, mouse_y) = ci.arg_named::<(i32, i32)>("mouse").copied().unwrap_or_default();
    let Some(screen) = ci.arg_named_mut::<mcrust::net::minecraft::client::gui::screens::screen::Screen>("screen") else { return; };
    crate::mixins::quick_nav_mixin::skyblocker_update_quick_nav_buttons_live(screen, mouse_x, mouse_y);
}

/// @Inject skyblocker$extractSelectedQuickNavButtons
///   java: @Inject(method = "extractRenderStateWithTooltipAndSubtitles", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V", shift = At.Shift.AFTER))
/// registered at: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractContents HEAD (after the
/// background; every container screen reaches it once per frame, `InventoryScreen` included).
fn skyblocker_extract_selected_quick_nav_buttons(ci: &mut CallbackInfo<'_>) {
    crate::mixins::quick_nav_mixin::skyblocker_record_partial_tick(container_screen_partial_tick(ci));
    crate::mixins::quick_nav_mixin::skyblocker_extract_selected_quick_nav_buttons();
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // Same priority, so they run in registration order: unselected (behind), then selected (in front).
    report(hook::register_injector("net.minecraft.client.gui.Gui#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_update_quick_nav_buttons)));
    report(hook::register_injector("net.minecraft.client.gui.Hud#extractDeferredSubtitles", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_extract_unselected_quick_nav_buttons)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractContents", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_extract_selected_quick_nav_buttons)));
}
