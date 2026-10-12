//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.FishingHookMixin by fabric2seer.
//! @Mixin(net.minecraft.world.entity.projectile.FishingHook)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$dontLogInvalidOwner
///   java: @WrapWithCondition(method = "recreateFromPacket", at = @At(
/// target: org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.world.entity.projectile.FishingHook#recreateFromPacket
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_dont_log_invalid_owner(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.world.entity.projectile.FishingHook#recreateFromPacket`, `org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_dont_log_invalid_owner));
}
