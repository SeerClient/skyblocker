//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.YggdrasilServicesKeyInfoMixin by fabric2seer.
//! @Mixin(com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo)

#[allow(unused_imports)]
use crate::prelude::*;

use mcrust::java::util::base64::Decoder;

/// java: @Shadow @Final private static Logger LOGGER; - `YggdrasilServicesKeyInfo`'s logger.
static LOGGER: mcrust::org::slf4j::logger::Logger = mcrust::org::slf4j::logger::get_logger("YggdrasilServicesKeyInfo");

/// java: @Unique private static final Map<String, String> REPLACEMENT_MAP = Map.of();
static REPLACEMENT_MAP: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(std::collections::HashMap::new);

/// java: @Unique private static final IntList ERRONEUS_SIGNATURE_HASHES = IntLists.synchronize(new IntArrayList());
static ERRONEUS_SIGNATURE_HASHES: Mutex<Vec<i32>> = Mutex::new(Vec::new());

/// `decode.call(decoder, s)` for an argument other than the call's own: `Decoder.decode(String)`'s body
/// (`decode(src.getBytes(ISO_8859_1))`) on the same decoder. SeerClient runs a wrapper's `original` at most once and
/// with the call's own arguments, so the retries call the unhooked `decode(byte[])` the original delegates to.
fn decode_call(decoder: &Decoder, s: &str) -> Result<Vec<u8>, String> {
    let bytes: Vec<u8> = s.encode_utf16().map(|unit| if unit <= 0xff { unit as u8 } else { b'?' }).collect();
    decoder.decode_bytes(&bytes)
}

/// @WrapOperation skyblocker$replaceKnownWrongBase64
///   java: @WrapOperation(method = "validateProperty", at = @At(value = "INVOKE", target = "Ljava/util/Base64$Decoder;decode(Ljava/lang/String;)[B"))
/// target: java.util.Base64$Decoder#decode(Ljava/lang/String;)[B  [exists: java/util/base64.rs, hooked]
/// within: com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty  [hooked]
/// The `IllegalArgumentException` is the `Err` of the port's `decode`.
fn skyblocker_replace_known_wrong_base64(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    let Some(decoder) = ci.this::<Decoder>().copied() else { original.call(ci); return; };
    let Some(signature) = ci.arg::<str>(0).map(str::to_string) else { original.call(ci); return; };
    // try { return decode.call(decoder, signature); }
    original.call(ci);
    if ci.return_value::<Result<Vec<u8>, String>>().is_none_or(|r| r.is_ok()) {
        return;
    }
    // catch (IllegalArgumentException e) { try { return decode.call(decoder, signature.replaceAll("[^A-Za-z0-9+/=]", "")); }
    let cleaned: String = signature.chars().filter(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=')).collect();
    if let Ok(bytes) = decode_call(&decoder, &cleaned) {
        ci.set_return_value::<Result<Vec<u8>, String>>(Ok(bytes));
        return;
    }
    // catch (IllegalArgumentException _)
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        if let Some(replacement) = REPLACEMENT_MAP.get(&signature) {
            // `return decode.call(decoder, REPLACEMENT_MAP.get(signature))` - its own exception propagates as the result.
            ci.set_return_value::<Result<Vec<u8>, String>>(decode_call(&decoder, replacement));
            return;
        }
        let signature_hash_code = compat::string_hash_code(&signature);
        let mut hashes = compat::lock(&ERRONEUS_SIGNATURE_HASHES);
        if !hashes.contains(&signature_hash_code) {
            hashes.push(signature_hash_code);
            let n = (hashes.len() - 1).to_string();
            drop(hashes);
            LOGGER.warn_2("[Skyblocker Base64 Fixer] Failed to decode base64 string No.{}: {}", &n, &signature);
        } else {
            let index = hashes.iter().position(|h| *h == signature_hash_code).map_or(-1, |i| i as i32).to_string();
            drop(hashes);
            LOGGER.warn_1("[Skyblocker Base64 Fixer] Failed to decode the base64 string No.{} again", &index);
        }
    }
    // throw e; - the original's `Err` stays the result.
}

/// @WrapWithCondition skyblocker$dontLogFailedSignatureValidations
///   java: @WrapWithCondition(method = "validateProperty", at = @At(value = "INVOKE", target = "Lorg/slf4j/Logger;error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V"))
/// target: org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V  [exists: org/slf4j/logger.rs]
/// within: com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty  [hooked]
fn skyblocker_dont_log_failed_signature_validations(_ci: &CallbackInfo<'_>) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel());
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty", "java.util.Base64$Decoder#decode(Ljava/lang/String;)[B").priority(1000).wrap_operation(skyblocker_replace_known_wrong_base64));
    report(hook::call_site("com.mojang.authlib.yggdrasil.YggdrasilServicesKeyInfo#validateProperty", "org.slf4j.Logger#error(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V").priority(1000).wrap_with_condition(skyblocker_dont_log_failed_signature_validations));
}
