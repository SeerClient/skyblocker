//! Ported from de.hysky.skyblocker.events.WorldEvents by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::events::Event;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;

/// java: public static final Event<BlockStateUpdate> BLOCK_STATE_UPDATE = EventFactory.createArrayBacked(BlockStateUpdate.class, callbacks -> (pos, oldState, newState) -> { for (BlockStateUpdate callback : cal
pub static BLOCK_STATE_UPDATE: compat::MainThread<Event<dyn Fn(BlockPos, Arc<BlockState>, Arc<BlockState>)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `BLOCK_STATE_UPDATE` (`BLOCK_STATE_UPDATE.invoker()`): the factory's lambda over the listeners
pub fn block_state_update_invoker() -> BlockStateUpdate {
    let callbacks: Vec<BlockStateUpdate> = BLOCK_STATE_UPDATE.listeners();
    Arc::new(move |pos, old_state, new_state| {
        for callback in callbacks.iter() {
            (callback)(pos.clone(), old_state.clone(), new_state.clone());
        }
    })
}

/// java: public class WorldEvents
#[derive(Clone)]
pub struct WorldEvents {}

impl WorldEvents {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface BlockStateUpdate
/// a functional interface: `onBlockStateUpdate(BlockPos pos, BlockState oldState, BlockState newState)` - a lambda / method reference is this closure type
pub type BlockStateUpdate = Arc<dyn Fn(BlockPos, Arc<BlockState>, Arc<BlockState>)>;
