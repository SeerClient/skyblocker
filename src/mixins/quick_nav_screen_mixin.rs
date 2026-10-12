//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.QuickNavScreenMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.Screen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$extractUnselectedQuickNavButtons
///   java: @Inject(method = "extractBackground", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/Hud;extractDeferredSubtitles()V"))
/// target: net.minecraft.client.gui.Hud#extractDeferredSubtitles()V  [no Rust counterpart]
/// within: net.minecraft.client.gui.screens.Screen#extractBackground
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
/// note: `within` net.minecraft.client.gui.screens.Screen#extractBackground: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn skyblocker_extract_unselected_quick_nav_buttons(ci: &mut CallbackInfo<'_>) {
}

/// @Inject skyblocker$extractSelectedQuickNavButtons
///   java: @Inject(method = "extractRenderStateWithTooltipAndSubtitles", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V", shift = At.Shift.AFTER))
/// target: net.minecraft.client.gui.screens.Screen#extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V  [no Rust counterpart]
/// within: net.minecraft.client.gui.screens.Screen#extractRenderStateWithTooltipAndSubtitles
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
/// note: `within` net.minecraft.client.gui.screens.Screen#extractRenderStateWithTooltipAndSubtitles: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn skyblocker_extract_selected_quick_nav_buttons(ci: &mut CallbackInfo<'_>) {
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
    // report(hook::call_site(`net.minecraft.client.gui.screens.Screen#extractBackground`, `net.minecraft.client.gui.Hud#extractDeferredSubtitles()V`).priority(1000).inject(Shift::Before, skyblocker_extract_unselected_quick_nav_buttons));
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
    // report(hook::call_site(`net.minecraft.client.gui.screens.Screen#extractRenderStateWithTooltipAndSubtitles`, `net.minecraft.client.gui.screens.Screen#extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V`).priority(1000).inject(Shift::After, skyblocker_extract_selected_quick_nav_buttons));
}
