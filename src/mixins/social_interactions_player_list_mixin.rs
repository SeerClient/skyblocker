//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.SocialInteractionsPlayerListMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.social.SocialInteractionsPlayerList)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$hideInvalidPlayers
///   java: @WrapWithCondition(method = "addOnlinePlayers", at = @At(value = "INVOKE", target = "Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"))
/// target: java.util.Map#put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;  [class not in JavaSrc]
/// within: net.minecraft.client.gui.screens.social.SocialInteractionsPlayerList#addOnlinePlayers
/// note: class not in JavaSrc: java.util.Map has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_hide_invalid_players(ci: &CallbackInfo<'_>) -> bool {
    // TODO(fabric2seer): `entry`: this argument is not visible to the injector (see the notes above the handler)
    //   java: return !(Utils.isOnSkyblock() && !((PlayerEntry) entry).getPlayerName().matches("[A-Za-z0-9_]+"));
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: java.util.Map has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.client.gui.screens.social.SocialInteractionsPlayerList#addOnlinePlayers`, `java.util.Map#put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`).priority(1000).wrap_with_condition(skyblocker_hide_invalid_players));
}
