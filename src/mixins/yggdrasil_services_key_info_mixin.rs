//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.YggdrasilServicesKeyInfoMixin by fabric2seer.
//! @Mixin(com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo)

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): state `REPLACEMENT_MAP` of type `Map<String, String>` - give it a Rust type (`static REPLACEMENT_MAP: LazyLock<Mutex<T>>`)
// TODO(fabric2seer): state `ERRONEUS_SIGNATURE_HASHES` of type `IntList` - give it a Rust type (`static ERRONEUS_SIGNATURE_HASHES: LazyLock<Mutex<T>>`)

/// @WrapOperation skyblocker$replaceKnownWrongBase64
///   java: @WrapOperation(method = "validateProperty", at = @At(value = "INVOKE", target = "Ljava/util/Base64$Decoder;decode(Ljava/lang/String;)[B"))
/// target: java.util.Base64$Decoder#decode(Ljava/lang/String;)[B  [class not in JavaSrc]
/// within: com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty
/// note: class not in JavaSrc: java.util.Base64.Decoder has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
/// note: `within` com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty: class not in JavaSrc - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn skyblocker_replace_known_wrong_base64(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    // java: try {..} catch (IllegalArgumentException e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
    {
        // TODO(fabric2seer): `decoder`: this argument is not visible to the injector (see the notes above the handler)
        //   java: return decode.call(decoder, signature);
    }
    // TODO(fabric2seer): the `catch (IllegalArgumentException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
    //   java: {
    //   java: 			try {
    //   java: 				return decode.call(decoder, signature.replaceAll("[^A-Za-z0-9+/=]", ""));
    //   java: 			} catch (IllegalArgumentException _) {
    //   java: 				if (Utils.isOnSkyblock()) {
    //   java: 					if (REPLACEMENT_MAP.containsKey(signature)) {
    //   java: 						return decode.call(decoder, REPLACEMENT_MAP.get(signature));
    //   java: 					}
    //   java: 					int signatureHashCode = signature.hashCode();
    //   java: 					if (!ERRONEUS_SIGNATURE_HASHES.contains(signatureHashCode)) {
    //   java: 						ERRONEUS_SIGNATURE_HASHES.add(signatureHashCode);
    //   java: 						LOGGER.warn("[Skyblocker Base64 Fixer] Failed to decode base64 string No.{}: {}", ERRONEUS_SIGNATURE_HASHES.size() - 1, signature);
    //   java: 					} else {
    //   java: 						LOGGER.warn("[Skyblocker Base64 Fixer] Failed to decode the base64 string No.{} again", ERRONEUS_SIGNATURE_HASHES.indexOf(signatureHashCode));
    //   java: 					}
    //   java: 				}
    //   java: 			}
    //   java: 			throw e;
    //   java: 		}
}

/// @WrapWithCondition skyblocker$dontLogFailedSignatureValidations
///   java: @WrapWithCondition(method = "validateProperty", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V  [class not in JavaSrc]
/// within: com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty
/// note: class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
/// note: `within` com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty: class not in JavaSrc - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn skyblocker_dont_log_failed_signature_validations(ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: java.util.Base64.Decoder has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty`, `java.util.Base64$Decoder#decode(Ljava/lang/String;)[B`).priority(1000).wrap_operation(skyblocker_replace_known_wrong_base64));
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.slf4j.Logger has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty`, `org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V`).priority(1000).wrap_with_condition(skyblocker_dont_log_failed_signature_validations));
}
