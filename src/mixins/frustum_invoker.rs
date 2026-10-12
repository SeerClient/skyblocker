//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.FrustumInvoker by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.culling.Frustum)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeCubeInFrustum -> net.minecraft.client.renderer.culling.Frustum.cubeInFrustum
// @Invoker: native mods call the method directly: `obj.cube_in_frustum(..)` (or `mcrust::net::minecraft::client::renderer::culling::frustum::Frustum::cube_in_frustum(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
