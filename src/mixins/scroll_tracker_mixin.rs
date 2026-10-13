//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.adaptivetooltips.ScrollTrackerMixin by fabric2seer.
//! @Mixin(dev.isxander.adaptivetooltips.helpers.ScrollTracker)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyExpressionValue skyblocker$includeGridTooltipComponents
///   java: @ModifyExpressionValue(method = "isEqual", at = @At(value = "CONSTANT", args = "intValue=0", ordinal = 4), require = 0)
/// note: @ModifyExpressionValue at @At("CONSTANT"): only a CALL (@At("INVOKE")) has a call-site injector (`hook::call_site`); a local, a field, a constant or `new` has no injection point
fn skyblocker_include_grid_tooltip_components(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): @ModifyExpressionValue is not expressible - the handler is kept for reference
    //   java: @ModifyExpressionValue(method = "isEqual", at = @At(value = "CONSTANT", args = "intValue=0", ordinal = 4), require = 0)
    //   java: 	private static int skyblocker$includeGridTooltipComponents(int original, @Local(name = "c1") ClientTooltipComponent tooltip1, @Local(name = "c2") ClientTooltipComponent tooltip2) {
    //   java: 		if (tooltip1 instanceof GridTooltipComponent grid1 && tooltip2 instanceof GridTooltipComponent grid2) {
    //   java: 			boolean equal = grid1.sequence().equals(grid2.sequence());
    //   java:
    //   java: 			return equal ? 1 : 0;
    //   java: 		}
    //   java:
    //   java: 		return original;
    //   java: 	}
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // `MixinPlugin.shouldApplyMixin` (the mod's `IMixinConfigPlugin`) decides whether this mixin is applied at all.
    if !crate::de::hysky::skyblocker::compatibility::mixin_plugin::MixinPlugin::new().should_apply_mixin("dev.isxander.adaptivetooltips.helpers.ScrollTracker", "de.hysky.skyblocker.mixins.adaptivetooltips.ScrollTrackerMixin") {
        return;
    }
    // TODO(fabric2seer): @ModifyExpressionValue skyblocker$includeGridTooltipComponents: nothing to register - @ModifyExpressionValue at @At("CONSTANT"): only a CALL (@At("INVOKE")) has a call-site injector (`hook::call_site`); a local, a field, a constant or `new` has no injection point
}
