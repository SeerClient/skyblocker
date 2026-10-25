//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.jgit.SystemReaderMixin by fabric2seer.
//! @Mixin(org.eclipse.jgit.util.SystemReader$Default)
//!
//! JGit is a library Skyblocker bundles; this port's `NEURepoManager` (utils/neu_repo_manager.rs) replaces it with an
//! HTTP download that reads no git configuration at all, so the system gitconfig this mixin blocks is never loaded -
//! the effect Java's mixin forces holds by construction. The injector is still registered on JGit's method, exactly
//! as Fabric applies it; a target with no hooked body registers and never fires (MODDING.md section 8).

#[allow(unused_imports)]
use crate::prelude::*;

/// org.eclipse.jgit.lib.Constants.GIT_CONFIG_NOSYSTEM_KEY
const GIT_CONFIG_NOSYSTEM_KEY: &str = "GIT_CONFIG_NOSYSTEM";

/// @ModifyReturnValue skyblocker$blockLoadingSystemGitConfig
///   java: @ModifyReturnValue(method = "getenv", at = @At("RETURN"))
/// target: org.eclipse.jgit.util.SystemReader$Default#getenv(Ljava/lang/String;)Ljava/lang/String;
fn skyblocker_block_loading_system_git_config(ci: &mut CallbackInfo<'_>) {
    // `String variable` is `getenv`'s argument 0; `String original` its return value (null is `None`).
    let Some(original) = ci.take_return_value::<Option<String>>() else { return; };
    let variable_is_nosystem = ci.arg::<str>(0).is_some_and(|variable| variable == GIT_CONFIG_NOSYSTEM_KEY);
    //   java: return variable.equals(Constants.GIT_CONFIG_NOSYSTEM_KEY) ? "FORCE-ENABLE" : original;
    ci.set_return_value::<Option<String>>(if variable_is_nosystem { Some("FORCE-ENABLE".to_string()) } else { original });
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("org.eclipse.jgit.util.SystemReader$Default#getenv", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_block_loading_system_git_config)));
}
