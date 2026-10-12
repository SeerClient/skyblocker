//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.discordipc.ConnectionMixin by fabric2seer.
//! @Mixin(meteordevelopment.discordipc.connection.UnixConnection, meteordevelopment.discordipc.connection.WinConnection)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Redirect write
///   java: @Redirect(method = "write", at = @At(value = "INVOKE", target = "Ljava/io/IOException;printStackTrace()V"))
/// target: java.io.IOException#printStackTrace()V  [class not in JavaSrc]
/// within: meteordevelopment.discordipc.connection.UnixConnection#write
/// note: class not in JavaSrc: java.io.IOException has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
/// note: `within` meteordevelopment.discordipc.connection.UnixConnection#write: class not in JavaSrc - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn write(ci: &mut CallbackInfo<'_>) {
    // `IOException instance` (the redirected call's receiver) is not read by the handler.
    // TODO(fabric2seer): static call DiscordIPC.stop(..)
    //   java: DiscordIPC.stop();
    logger().warn(&format!("[Skyblocker] Discord RPC failed to update activity, connection lost"));
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: java.io.IOException has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::call_site(`meteordevelopment.discordipc.connection.UnixConnection#write`, `java.io.IOException#printStackTrace()V`).priority(1000).redirect(write));
}
