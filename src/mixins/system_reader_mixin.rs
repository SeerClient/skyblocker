//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.jgit.SystemReaderMixin by fabric2seer.
//! @Mixin(org.eclipse.jgit.util.SystemReader$Default)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$blockLoadingSystemGitConfig
///   java: @ModifyReturnValue(method = "getenv", at = @At("RETURN"))
/// target: org.eclipse.jgit.util.SystemReader$Default#getenv  [class not in JavaSrc]
/// note: class not in JavaSrc: org.eclipse.jgit.util.SystemReader.Default has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn skyblocker_block_loading_system_git_config(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): read the return value typed: ci.return_value::<T>() (String)
    // TODO(fabric2seer): unknown name `variable`
    //   java: return variable.equals(Constants.GIT_CONFIG_NOSYSTEM_KEY) ? "FORCE-ENABLE" : original;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: org.eclipse.jgit.util.SystemReader.Default has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::register_injector(`org.eclipse.jgit.util.SystemReader$Default#getenv`, InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_block_loading_system_git_config)));
}
