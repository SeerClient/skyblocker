//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.jgit.UrlConfigMixin by fabric2seer.
//! @Mixin(org.eclipse.jgit.transport.UrlConfig)
//!
//! JGit is a library Skyblocker bundles; this port's `NEURepoManager` (utils/neu_repo_manager.rs) replaces it with an
//! HTTP download of the fixed `REMOTE_REPO_URL`, which applies no `url.<base>.insteadOf` rewriting - the redirects
//! this mixin drops are never stored, by construction. The wrapper is still registered on JGit's call site, exactly
//! as Fabric applies it; a target with no hooked body registers and never fires (MODDING.md section 8).

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapOperation skyblocker$ignoreUrlRedirects
///   java: @WrapOperation(method = "load", at = @At(value = "INVOKE", target = "Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"))
/// within: org.eclipse.jgit.transport.UrlConfig#load
fn skyblocker_ignore_url_redirects(ci: &mut CallbackInfo<'_>, _original: &mut Original<'_>) {
    // java: return null; (the redirect is never stored - `original` is not called)
    ci.set_return_value::<Option<compat::Object>>(None);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("org.eclipse.jgit.transport.UrlConfig#load", "java.util.Map#put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;").priority(1000).wrap_operation(skyblocker_ignore_url_redirects));
}
