//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.BlockEntityRenderStateAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.blockentity.state.BlockEntityRenderState)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor setBlockState -> net.minecraft.client.renderer.blockentity.state.BlockEntityRenderState.blockState
// TODO(fabric2seer): `BlockState` - assign `obj.block_state` with its Rust type

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
