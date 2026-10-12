//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.WindowMixin by fabric2seer.
//! @Mixin(com.mojang.blaze3d.platform.Window)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$onScaleFactorChange
///   java: @Inject(method = "setGuiScale", at = @At("TAIL"))
/// target: com.mojang.blaze3d.platform.Window#setGuiScale  [exists]
/// Rust: com/mojang/blaze3d/platform/window.rs:660 fn set_gui_scale(&mut self, gui_scale: i32)
fn skyblocker_on_scale_factor_change(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::com::mojang::blaze3d::platform::window::WindowState>()) else { return; };
    crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars::FancyStatusBars::update_positions(crate::de::hysky::skyblocker::skyblock::fancybars::fancy_status_bars::INSTANCE.get_mut(), false);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("com.mojang.blaze3d.platform.Window#setGuiScale", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_scale_factor_change)));
}
