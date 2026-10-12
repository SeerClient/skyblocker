//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.MapRendererInvoker by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.MapRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeExtractDecorationRenderState -> net.minecraft.client.renderer.MapRenderer.extractDecorationRenderState
// @Invoker: native mods call the method directly: `obj.extract_decoration_render_state(..)` (or `mcrust::net::minecraft::client::renderer::map_renderer::MapRenderer::extract_decoration_render_state(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
