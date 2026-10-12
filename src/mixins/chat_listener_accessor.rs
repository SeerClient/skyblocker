//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.ChatListenerAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.chat.ChatListener)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeLogSystemMessage -> net.minecraft.client.multiplayer.chat.ChatListener.logSystemMessage
// @Invoker: native mods call the method directly: `obj.log_system_message(..)` (or `mcrust::net::minecraft::client::multiplayer::chat::chat_listener::ChatListener::log_system_message(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
