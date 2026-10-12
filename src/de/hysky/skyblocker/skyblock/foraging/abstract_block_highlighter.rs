//! Ported from de.hysky.skyblocker.skyblock.foraging.AbstractBlockHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::world_events;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::chunk::chunk_access::ChunkAccess;
use mcrust::net::minecraft::world::level::chunk::level_chunk::LevelChunk;
use mcrust::net::minecraft::world::phys::aabb::AABB;

/// java: public abstract class AbstractBlockHighlighter
#[derive(Clone)]
pub struct AbstractBlockHighlighterBase {
    /// java: protected final BlockPosSet highlightedBlocks = new BlockPosSet();
    pub highlighted_blocks: compat::Shared<BlockPosSet>,
    /// java: protected final float[] colour;
    pub colour: Vec<f32>,
    /// java: protected final BiPredicate<ClientLevel, BlockPos> posPredicate;
    pub pos_predicate: Option<Arc<dyn Fn(ClientLevel, BlockPos) -> bool>>,
    /// java: protected final Predicate<BlockState> statePredicate;
    pub state_predicate: Option<Arc<dyn Fn(Arc<BlockState>) -> bool>>,
}

impl AbstractBlockHighlighterBase {
    /// java: protected AbstractBlockHighlighter(Block target, DyeColor colour) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut target: Arc<Block>, mut colour: DyeColor, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractBlockHighlighter>, __R)) -> (compat::Shared<dyn AbstractBlockHighlighter>, __R) {
        let (__this, __r) = Self::new2_in(Arc::new(move |state| state.is_block(&target)), colour, __wrap);
        (__this, __r)
    }

    /// java: protected AbstractBlockHighlighter(Predicate<BlockState> statePredicate, DyeColor colour) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(state_predicate: Arc<dyn Fn(Arc<BlockState>) -> bool>, mut colour: DyeColor, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractBlockHighlighter>, __R)) -> (compat::Shared<dyn AbstractBlockHighlighter>, __R) {
        let (__this, __r) = Self::new_3_in(Arc::new(move |_, _| true), state_predicate.clone(), ColorUtils::get_float_components_dye_color(&colour), __wrap);
        (__this, __r)
    }

    /// java: protected AbstractBlockHighlighter(BiPredicate<ClientLevel, BlockPos> posPredicate, Predicate<BlockState> statePredicate, float[] colour) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_3_in<__R>(pos_predicate: Arc<dyn Fn(ClientLevel, BlockPos) -> bool>, state_predicate: Arc<dyn Fn(Arc<BlockState>) -> bool>, mut colour: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractBlockHighlighter>, __R)) -> (compat::Shared<dyn AbstractBlockHighlighter>, __R) {
        let __init_pos_predicate = Some(pos_predicate.clone());
        let __init_state_predicate = Some(state_predicate.clone());
        let __init_colour = colour.clone();
        let this = Self {
            highlighted_blocks: BlockPosSet::new(),
            colour: __init_colour,
            pos_predicate: __init_pos_predicate,
            state_predicate: __init_state_predicate,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractBlockHighlighter (implemented by AbstractBlockHighlighterBase and every subclass)
pub trait AbstractBlockHighlighter: compat::DynAny {
    fn abstract_block_highlighter_base(&self) -> &AbstractBlockHighlighterBase;
    fn abstract_block_highlighter_base_mut(&mut self) -> &mut AbstractBlockHighlighterBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected void init() (dispatch: the body for this class)
    fn init(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>) {
        <dyn AbstractBlockHighlighter>::init_body
    }

    /// java: protected void onBlockUpdate(BlockPos pos, BlockState oldState, BlockState newState) (dispatch: the body for this class)
    fn on_block_update(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>, BlockPos, &Arc<BlockState>, &Arc<BlockState>) {
        <dyn AbstractBlockHighlighter>::on_block_update_body
    }

    /// java: protected void onChunkLoad(ClientLevel level, LevelChunk chunk) (dispatch: the body for this class)
    fn on_chunk_load(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>, &ClientLevel, &LevelChunk) {
        <dyn AbstractBlockHighlighter>::on_chunk_load_body
    }

    /// java: protected void onChunkUnload(ClientLevel world, LevelChunk chunk) (dispatch: the body for this class)
    fn on_chunk_unload(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>, &ClientLevel, &LevelChunk) {
        <dyn AbstractBlockHighlighter>::on_chunk_unload_body
    }

    /// java: private void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn AbstractBlockHighlighter>::extract_rendering_body
    }

    /// java: public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>) {
        <dyn AbstractBlockHighlighter>::reset_body
    }

    /// java: @SuppressWarnings("BooleanMethodIsAlwaysInverted") protected abstract boolean shouldProcess(); (dispatch: the body for this class)
    fn should_process(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>) -> bool;

}

impl dyn AbstractBlockHighlighter {
    /// java: protected void init()
    pub fn init_body(__this: &compat::Shared<dyn AbstractBlockHighlighter>) {
        // TODO(fabric2seer): ClientChunkEvents.CHUNK_LOAD.register: no SeerClient equivalent yet
        //   java: ClientChunkEvents.CHUNK_LOAD.register(this::onChunkLoad);
        // TODO(fabric2seer): ClientChunkEvents.CHUNK_UNLOAD.register: no SeerClient equivalent yet
        //   java: ClientChunkEvents.CHUNK_UNLOAD.register(this::onChunkUnload);
        let __this_1 = compat::RenderThread::new(__this.clone());
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { ({ let __f = compat::dispatch(__this_1.get(), |__t| __t.extract_rendering()); __f(__this_1.get(), &__a0) }); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            ({ let __f = compat::dispatch(__this, |__t| __t.reset()); __f(__this) });
        }));
        let __this_2 = compat::RenderThread::new(__this.clone());
        world_events::BLOCK_STATE_UPDATE.register(Arc::new(move |__a0, __a1, __a2| { ({ let __f = compat::dispatch(__this_2.get(), |__t| __t.on_block_update()); __f(__this_2.get(), __a0.clone(), &__a1, &__a2) }); }));
    }

    /// java: protected void onBlockUpdate(BlockPos pos, BlockState oldState, BlockState newState)
    pub fn on_block_update_body(__this: &compat::Shared<dyn AbstractBlockHighlighter>, mut pos: BlockPos, old_state: &Arc<BlockState>, new_state: &Arc<BlockState>) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_process()); __f(__this) }) {
            return;
        }
        if ((compat::read(__this, |__t| __t.abstract_block_highlighter_base().pos_predicate.clone()).unwrap()))(mcrust::mcrust::modding::game::level(), pos.clone()) && ((compat::read(__this, |__t| __t.abstract_block_highlighter_base().state_predicate.clone()).unwrap()))(new_state.clone()) {
            BlockPosSet::add(&compat::read(__this, |__t| __t.abstract_block_highlighter_base().highlighted_blocks.clone()), &pos);
        } else {
            BlockPosSet::remove_block_pos(&compat::read(__this, |__t| __t.abstract_block_highlighter_base().highlighted_blocks.clone()), &pos);
        }
    }

    /// java: protected void onChunkLoad(ClientLevel level, LevelChunk chunk)
    pub fn on_chunk_load_body(__this: &compat::Shared<dyn AbstractBlockHighlighter>, level: &ClientLevel, chunk: &LevelChunk) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_process()); __f(__this) }) {
            return;
        }
        let __this_1 = compat::RenderThread::new(__this.clone());
        chunk.access.find_blocks(&compat::read(__this, |__t| __t.abstract_block_highlighter_base().state_predicate.clone()).unwrap(), &mut |pos, _| {
            // TODO(fabric2seer): the mod keeps the game's `ClientLevel` (Java holds a reference to the live object); the port's `ClientLevel` is not a value to copy and the port lends the mod no handle to it
            if ((compat::read(__this_1.get(), |__t| __t.abstract_block_highlighter_base().pos_predicate.clone()).unwrap()))(level.clone(), pos.clone()) {
                BlockPosSet::add(&compat::read(__this_1.get(), |__t| __t.abstract_block_highlighter_base().highlighted_blocks.clone()), &pos);
            }
        });
    }

    /// java: protected void onChunkUnload(ClientLevel world, LevelChunk chunk)
    pub fn on_chunk_unload_body(__this: &compat::Shared<dyn AbstractBlockHighlighter>, world: &ClientLevel, chunk: &LevelChunk) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_process()); __f(__this) }) {
            return;
        }
        let mut iterator = BlockPosSet::iterator(&compat::read(__this, |__t| __t.abstract_block_highlighter_base().highlighted_blocks.clone()));
        // TODO(fabric2seer): the condition: Iterator.hasNext - use a `for` loop / `while let Some(x) = it.next()`
        //   java: iterator.hasNext(..)
        while todo!("fabric2seer: the condition (TODO above)") {
            let mut pos: BlockPos = iterator.next().unwrap();
            // TODO(fabric2seer): `ClientLevel.getChunk` has no Rust method in the port (looked for `get_chunk` in ClientLevel and its supertypes)
            // rust: let mut holder: ChunkAccess = world.get_chunk(&pos);
            let mut holder: ChunkAccess = todo!("fabric2seer: the initialiser of `holder` (TODO above)");
            if std::ptr::eq(&holder, chunk) {
                // TODO(fabric2seer): Stream.remove
                //   java: iterator.remove();
            }
        }
    }

    /// java: private void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_body(__this: &compat::Shared<dyn AbstractBlockHighlighter>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        let client = compat::minecraft();
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_process()); __f(__this) }) || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        for highlight in BlockPosSet::iterate_mut(&compat::read(__this, |__t| __t.abstract_block_highlighter_base().highlighted_blocks.clone())) {
            let mut outline: Option<AABB> = RenderHelper::get_block_bounding_box(&mcrust::mcrust::modding::game::level().unwrap(), &highlight);
            if let Some(outline) = &outline {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, outline, &compat::read(__this, |__t| __t.abstract_block_highlighter_base().colour.clone()), 0.4f32, false) });
            }
        }
    }

    /// java: public void reset()
    pub fn reset_body(__this: &compat::Shared<dyn AbstractBlockHighlighter>) {
        BlockPosSet::clear(&compat::read(__this, |__t| __t.abstract_block_highlighter_base().highlighted_blocks.clone()));
    }

}
