//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.adaptivetooltips.ScrollTrackerMixin by fabric2seer.
//! @Mixin(dev.isxander.adaptivetooltips.helpers.ScrollTracker)
//!
//! A compatibility mixin into another mod (AdaptiveTooltips), applied only when that mod is loaded
//! (`MixinPlugin.shouldApplyMixin` -> `ADAPTIVE_TOOLTIPS_LOADED`, which asks SeerClient's loader). AdaptiveTooltips has
//! no SeerClient port today, so on a normal install `register` returns before registering, exactly as Fabric skips the
//! mixin when the mod is absent.

#[allow(unused_imports)]
use crate::prelude::*;

use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent;

/// `tooltip instanceof GridTooltipComponent grid`: a grid line is the mod's `GridTooltipComponentLine`.
fn as_grid(
    line: &dyn ClientTooltipComponent,
) -> Option<compat::Shared<crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponent>> {
    line.as_any()
        .and_then(|any| any.downcast_ref::<crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponentLine>())
        .map(|line| line.0.clone())
}

/// @ModifyExpressionValue skyblocker$includeGridTooltipComponents
///   java: @ModifyExpressionValue(method = "isEqual", at = @At(value = "CONSTANT", args = "intValue=0", ordinal = 4), require = 0)
///   java: private static int skyblocker$includeGridTooltipComponents(int original, @Local(name = "c1") ClientTooltipComponent tooltip1, @Local(name = "c2") ClientTooltipComponent tooltip2)
///
/// The handler, over the constant and the two `@Local`s: two grid components are equal (1) when their sequences
/// are, otherwise the constant stands.
pub fn skyblocker_include_grid_tooltip_components(original: i32, tooltip1: &dyn ClientTooltipComponent, tooltip2: &dyn ClientTooltipComponent) -> i32 {
    if let (Some(grid1), Some(grid2)) = (as_grid(tooltip1), as_grid(tooltip2)) {
        let equal: bool = compat::read(&grid1, |g| g.sequence.clone()) == compat::read(&grid2, |g| g.sequence.clone());
        return if equal { 1 } else { 0 };
    }
    original
}

/// The constant is the `0` `isEqual` answers for a component pair it does not know how to compare (its fourth
/// `0`), i.e. "these two lists differ". A constant has no injection point in SeerClient (MODDING.md section 8, "Call
/// sites"), so it is applied at the method's RETURN: when `isEqual(List<ClientTooltipComponent> a, b)` answered
/// "different", the comparison is re-asked with the grid pairs answered by the handler - the lists are equal when
/// they have the same length and every position holds two equal components, comparing grid pairs by sequence and
/// any other pair by identity (the one comparison a mod can make of an arbitrary component).
fn is_equal_return(ci: &mut CallbackInfo<'_>) {
    type Lines = [Box<dyn ClientTooltipComponent>];
    let Some(original) = ci.return_value::<bool>().copied() else { return; };
    if original {
        return;
    }
    let (Some(a), Some(b)) = (compat::hook_arg::<Lines>(ci, 0), compat::hook_arg::<Lines>(ci, 1)) else { return; };
    if a.len() != b.len() || !a.iter().zip(b.iter()).any(|(c1, c2)| as_grid(&**c1).is_some() && as_grid(&**c2).is_some()) {
        return;
    }
    let equal = a.iter().zip(b.iter()).all(|(c1, c2)| {
        let fallback = if std::ptr::addr_eq(&**c1 as *const dyn ClientTooltipComponent, &**c2 as *const dyn ClientTooltipComponent) { 1 } else { 0 };
        skyblocker_include_grid_tooltip_components(fallback, &**c1, &**c2) == 1
    });
    ci.set_return_value(equal);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // `MixinPlugin.shouldApplyMixin` (the mod's `IMixinConfigPlugin`) decides whether this mixin is applied at all.
    if !crate::de::hysky::skyblocker::compatibility::mixin_plugin::MixinPlugin::new().should_apply_mixin("dev.isxander.adaptivetooltips.helpers.ScrollTracker", "de.hysky.skyblocker.mixins.adaptivetooltips.ScrollTrackerMixin") {
        return;
    }
    report(hook::register_injector("dev.isxander.adaptivetooltips.helpers.ScrollTracker#isEqual", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(is_equal_return)));
}
