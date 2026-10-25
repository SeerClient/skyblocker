//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.FishingHookMixin by fabric2seer.
//! @Mixin(net.minecraft.world.entity.projectile.FishingHook)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$dontLogInvalidOwner
///   java: @WrapWithCondition(method = "recreateFromPacket", at = @At(
/// target: org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.world.entity.projectile.FishingHook#recreateFromPacket
fn skyblocker_dont_log_invalid_owner(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // `org.slf4j.Logger` is the game's `org::slf4j::logger::Logger` (its overloads are hooked by descriptor).
    report(hook::call_site("net.minecraft.world.entity.projectile.FishingHook#recreateFromPacket", "org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V").priority(1000).wrap_with_condition(skyblocker_dont_log_invalid_owner));
}
