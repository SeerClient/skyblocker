//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.BlockEntityRenderStateAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.blockentity.state.BlockEntityRenderState)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor setBlockState -> net.minecraft.client.renderer.blockentity.state.BlockEntityRenderState.blockState
pub fn set_block_state(
    obj: &mut mcrust::net::minecraft::client::renderer::blockentity::state::block_entity_render_state::BlockEntityRenderState,
    value: Arc<mcrust::net::minecraft::world::level::block::state::block_state::BlockState>,
) {
    obj.set_block_state(value);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
