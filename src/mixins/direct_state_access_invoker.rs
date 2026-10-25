//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.DirectStateAccessInvoker by fabric2seer.
//! @Mixin(com.mojang.renderpearl.backend.opengl.DirectStateAccess)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeBufferData -> com.mojang.renderpearl.backend.opengl.DirectStateAccess.bufferData
// @Invoker: native mods call the method directly: `obj.buffer_data(..)` (or `mcrust::com::mojang::renderpearl::backend::opengl::direct_state_access::DirectStateAccess::buffer_data(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
