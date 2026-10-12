//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.jgit.UrlConfigMixin by fabric2seer.
//! @Mixin(org.eclipse.jgit.transport.UrlConfig)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapOperation skyblocker$ignoreUrlRedirects
///   java: @WrapOperation(method = "load", at = @At(value = "INVOKE", target = "Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"))
/// target: java.util.Map#put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;  [class not in JavaSrc]
/// within: org.eclipse.jgit.transport.UrlConfig#load
/// note: class not in JavaSrc: java.util.Map has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
/// note: `within` org.eclipse.jgit.transport.UrlConfig#load: class not in JavaSrc - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn skyblocker_ignore_url_redirects(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    // java: return null; (the redirect is never stored)
    ci.set_return_value::<Option<compat::Object>>(None);
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: java.util.Map has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`org.eclipse.jgit.transport.UrlConfig#load`, `java.util.Map#put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;`).priority(1000).wrap_operation(skyblocker_ignore_url_redirects));
}
