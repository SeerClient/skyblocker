//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.discordipc.ConnectionMixin by fabric2seer.
//! @Mixin(meteordevelopment.discordipc.connection.UnixConnection, meteordevelopment.discordipc.connection.WinConnection)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Redirect write
///   java: @Redirect(method = "write", at = @At(value = "INVOKE", target = "Ljava/io/IOException;printStackTrace()V"))
/// target: java.io.IOException#printStackTrace()V, inside UnixConnection#write and WinConnection#write
///
/// `meteordevelopment.discordipc` is a library the mod bundles, ported inside this crate
/// (`src/meteordevelopment/discordipc/connection/{unix,win}_connection.rs`), so the redirect is applied where the two
/// `write_buffer` bodies catch the `IOException`: both call this instead of printing the stack trace.
pub fn write(e: &std::io::Error) {
    crate::meteordevelopment::discordipc::discord_ipc::DiscordIPC::stop();
    crate::de::hysky::skyblocker::utils::discord::discord_rpc_manager::LOGGER.warn(&format!("[Skyblocker] Discord RPC failed to update activity, connection lost\njava.io.IOException: {e}"));
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. Nothing to register: the redirect is
/// compiled into the bundled library's call sites (see `write`).
pub fn register() {}
