//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.YggdrasilMinecraftSessionServiceMixin by fabric2seer.
//! @Mixin(com.mojang.authlib.yggdrasil.YggdrasilMinecraftSessionService)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$dontLogIncorrectEndingByteExceptions
///   java: @WrapWithCondition(method = "unpackTextures", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;error(Ljava/lang/String;Ljava/lang/Throwable;)V", ordinal = 0))
/// target: org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Throwable;)V  [exists: org/slf4j/logger.rs]
/// within: com.mojang.authlib.yggdrasil.YggdrasilMinecraftSessionService#unpackTextures (hooked; its one `error(String, Throwable)` call)
fn skyblocker_dont_log_incorrect_ending_byte_exceptions(ci: &CallbackInfo<'_>) -> bool {
    // `Throwable throwable` is the call's argument 1.
    let throwable_is_illegal_argument_exception = ci
        .arg::<mcrust::java::lang::throwable::Throwable>(1)
        .is_some_and(|t| t.get_class_name() == "java.lang.IllegalArgumentException");
    //   java: return !Utils.isOnHypixel() && throwable instanceof IllegalArgumentException;
    return !crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() && throwable_is_illegal_argument_exception;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("com.mojang.authlib.yggdrasil.YggdrasilMinecraftSessionService#unpackTextures", "org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Throwable;)V").ordinal(0).priority(1000).wrap_with_condition(skyblocker_dont_log_incorrect_ending_byte_exceptions));
}
