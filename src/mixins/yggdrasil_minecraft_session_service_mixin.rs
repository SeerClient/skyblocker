//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.YggdrasilMinecraftSessionServiceMixin by fabric2seer.
//! @Mixin(com.mojang.authlib.yggdrasil.YggdrasilMinecraftSessionService)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$dontLogIncorrectEndingByteExceptions
///   java: @WrapWithCondition(method = "unpackTextures", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;error(Ljava/lang/String;Ljava/lang/Throwable;)V", ordinal = 0))
/// target: org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Throwable;)V  [class not in JavaSrc]
/// within: com.mojang.authlib.yggdrasil.YggdrasilMinecraftSessionService#unpackTextures
/// note: ordinal 0: the call is under an `if` in the Java of the method, so the run-time count (per run of the method) may differ from Mixin's bytecode ordinal - check it
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
/// note: `within` com.mojang.authlib.yggdrasil.YggdrasilMinecraftSessionService#unpackTextures: class not in JavaSrc - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn skyblocker_dont_log_incorrect_ending_byte_exceptions(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach(ci.this_mut::<Logger>()) else { return true; };
    // TODO(fabric2seer): `throwable`: this argument is not visible to the injector (see the notes above the handler)
    //   java: return !Utils.isOnHypixel() && throwable instanceof IllegalArgumentException;
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`com.mojang.authlib.yggdrasil.YggdrasilMinecraftSessionService#unpackTextures`, `org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Throwable;)V`).ordinal(0).priority(1000).wrap_with_condition(skyblocker_dont_log_incorrect_ending_byte_exceptions));
}
