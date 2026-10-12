//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.DirectStateAccessInvoker by fabric2seer.
//! @Mixin(com.mojang.blaze3d.opengl.DirectStateAccess)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeBufferData -> com.mojang.blaze3d.opengl.DirectStateAccess.bufferData
// @Invoker: native mods call the method directly: `obj.buffer_data(..)` (or `mcrust::com::mojang::blaze3d::opengl::direct_state_access::DirectStateAccess::buffer_data(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
