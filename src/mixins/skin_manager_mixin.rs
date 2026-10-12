//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.SkinManagerMixin by fabric2seer.
//! @Mixin(net.minecraft.client.resources.SkinManager$1)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$dontLogInvalidSignatureWarnings
///   java: @WrapWithCondition(method = "lambda$load$0", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: net.minecraft.client.resources.SkinManager$1#lambda$load$0
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
/// note: `within` net.minecraft.client.resources.SkinManager$1#lambda$load$0: class not in JavaSrc (a synthetic lambda of the vanilla method: the port has no method of that name - hook the method that contains the lambda) - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn skyblocker_dont_log_invalid_signature_warnings(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`net.minecraft.client.resources.SkinManager$1#lambda$load$0`, `org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_dont_log_invalid_signature_warnings));
}
