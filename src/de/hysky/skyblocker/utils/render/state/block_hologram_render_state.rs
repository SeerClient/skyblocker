//! Ported from de.hysky.skyblocker.utils.render.state.BlockHologramRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;

/// java: public record BlockHologramRenderState(BlockPos pos, BlockState state, float alpha)
#[derive(Clone, Debug, PartialEq)]
pub struct BlockHologramRenderState {
    /// java: BlockPos pos
    pub pos: BlockPos,
    /// java: BlockState state
    pub state: Arc<BlockState>,
    /// java: float alpha
    pub alpha: f32,
}

impl BlockHologramRenderState {
    /// java: the canonical record constructor
    pub fn new(mut pos: BlockPos, mut state: Arc<BlockState>, mut alpha: f32) -> Self {
        Self { pos, state, alpha }
    }

    /// java: the record accessor `pos()`
    pub fn pos(&self) -> BlockPos {
        self.pos.clone()
    }

    /// java: the record accessor `state()`
    pub fn state(&self) -> Arc<BlockState> {
        self.state.clone()
    }

    /// java: the record accessor `alpha()`
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

}
