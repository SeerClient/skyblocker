//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.SwiftnessTestHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;

/// java: private static BlockPos lastBlock;
pub static LAST_BLOCK: compat::MainThread<Option<BlockPos>> = compat::MainThread::new(|| None);

/// java: public class SwiftnessTestHelper
#[derive(Clone)]
pub struct SwiftnessTestHelper {}

impl SwiftnessTestHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void reset()
    pub fn reset() {
        *LAST_BLOCK.get_mut() = None;
    }

    /// java: protected static void onBlockUpdate(BlockPos pos, BlockState state)
    pub fn on_block_update(pos: &BlockPos, state: &Arc<BlockState>) {
        if state.is_block(mcrust::net::minecraft::world::level::block::blocks::wool().lime()) {
            *LAST_BLOCK.get_mut() = Some(pos.immutable());
        }
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if LAST_BLOCK.get_mut().is_none() {
            return;
        }
        ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, LAST_BLOCK.get_mut().as_ref().unwrap(), &vec![0.0f32, 1.0f32, 0.0f32], 0.5f32, true) });
    }

}
