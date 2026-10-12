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
//! rust: port gap - there is no hookable point between the backdrop gradient and the container panel
//! (`Screen.extractBackground:387`'s `extractDeferredSubtitles` sits inside `super.extractBackground`; each port
//! `X#extractBackground` draws the gradient and the panel in one body through unhooked helpers), so the unselected
//! tabs are drawn over the panel instead of under it (GAPS_M #6).

#[allow(unused_imports)]
use crate::prelude::*;

/// The hooked `AbstractContainerScreen#extractRenderState(gl, gfx, state, menu, containers, mouse_x, mouse_y, .., cooldown_partial_tick, ..)`:
/// `this` (argument 2), `mouseX`, `mouseY` (5, 6) and the frame's partial tick (11) - Java's `a`.
fn container_screen_args<'a>(ci: &'a mut CallbackInfo<'_>) -> Option<(&'a mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, i32, i32, f32)> {
    let mouse_x: i32 = ci.arg::<i32>(5).copied().unwrap_or_default();
    let mouse_y: i32 = ci.arg::<i32>(6).copied().unwrap_or_default();
    let a: f32 = ci.arg::<f32>(11).copied().unwrap_or_default();
    let this = ci.arg_mut::<mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen>(2)?;
    Some((this, mouse_x, mouse_y, a))
}

/// @Inject skyblocker$extractUnselectedQuickNavButtons
///   java: @Inject(method = "extractBackground", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Hud;extractDeferredSubtitles()V"))
/// target: net.minecraft.client.gui.Hud#extractDeferredSubtitles()V  [no Rust counterpart]
/// within: net.minecraft.client.gui.screens.Screen#extractBackground
/// registered at: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractRenderState HEAD (see the file header)
fn skyblocker_extract_unselected_quick_nav_buttons(ci: &mut CallbackInfo<'_>) {
    // `QuickNavMixin`'s override (`this instanceof AbstractContainerScreen`, which the target guarantees).
    let Some((this, mouse_x, mouse_y, a)) = container_screen_args(ci) else { return; };
    crate::mixins::quick_nav_mixin::skyblocker_extract_unselected_quick_nav_buttons(this, mouse_x, mouse_y, a);
}

/// @Inject skyblocker$extractSelectedQuickNavButtons
///   java: @Inject(method = "extractRenderStateWithTooltipAndSubtitles", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V", shift = At.Shift.AFTER))
/// target: net.minecraft.client.gui.screens.Screen#extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V  [no Rust counterpart]
/// within: net.minecraft.client.gui.screens.Screen#extractRenderStateWithTooltipAndSubtitles
/// registered at: net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractRenderState HEAD (see the file header)
fn skyblocker_extract_selected_quick_nav_buttons(ci: &mut CallbackInfo<'_>) {
    let Some((this, mouse_x, mouse_y, a)) = container_screen_args(ci) else { return; };
    crate::mixins::quick_nav_mixin::skyblocker_extract_selected_quick_nav_buttons(this, mouse_x, mouse_y, a);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // Same priority, so they run in registration order: unselected (behind), then selected (in front).
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_extract_unselected_quick_nav_buttons)));
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.AbstractContainerScreen#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_extract_selected_quick_nav_buttons)));
}
