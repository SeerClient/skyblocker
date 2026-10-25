//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.SkinManagerMixin by fabric2seer.
//! @Mixin(net.minecraft.client.resources.SkinManager$1)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$dontLogInvalidSignatureWarnings
///   java: @WrapWithCondition(method = "lambda$load$0", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: net.minecraft.client.resources.SkinManager$1#lambda$load$0
/// within: the port's `skin_manager.rs` `lambda_load_0`, hooked as `SkinManager$1#lambda$load$0`.
fn skyblocker_dont_log_invalid_signature_warnings(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.resources.SkinManager$1#lambda$load$0", "org.slf4j.Logger#warn(Ljava/lang/String;Ljava/lang/Object;)V").priority(1000).wrap_with_condition(skyblocker_dont_log_invalid_signature_warnings));
}
