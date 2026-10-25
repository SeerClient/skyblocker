//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ScoreboardMixin by fabric2seer.
//! @Mixin(net.minecraft.world.scores.Scoreboard)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$cancelTeamWarning
///   java: @WrapWithCondition(method = "addPlayerTeam", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.world.scores.Scoreboard#addPlayerTeam
fn skyblocker_cancel_team_warning(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // `org.slf4j.Logger` is the game's `org::slf4j::logger::Logger` (its overloads are hooked by descriptor).
    report(hook::call_site("net.minecraft.world.scores.Scoreboard#addPlayerTeam", "org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V").priority(1000).wrap_with_condition(skyblocker_cancel_team_warning));
}
