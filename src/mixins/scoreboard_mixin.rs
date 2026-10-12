//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ScoreboardMixin by fabric2seer.
//! @Mixin(net.minecraft.world.scores.Scoreboard)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$cancelTeamWarning
///   java: @WrapWithCondition(method = "addPlayerTeam", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.world.scores.Scoreboard#addPlayerTeam
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_cancel_team_warning(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.world.scores.Scoreboard#addPlayerTeam`, `org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_cancel_team_warning));
}
