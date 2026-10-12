//! Ported from de.hysky.skyblocker.skyblock.foraging.galatea.SeaLumiesHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::foraging::abstract_block_highlighter::AbstractBlockHighlighter;
use crate::de::hysky::skyblocker::skyblock::foraging::abstract_block_highlighter::AbstractBlockHighlighterBase;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::chunk::chunk_access::ChunkAccess;
use mcrust::net::minecraft::world::level::chunk::level_chunk::LevelChunk;

/// java: public static final SeaLumiesHighlighter INSTANCE = new SeaLumiesHighlighter();
pub static INSTANCE: compat::MainThread<compat::Shared<SeaLumiesHighlighter>> = compat::MainThread::new(|| SeaLumiesHighlighter::new());

/// java: public final class SeaLumiesHighlighter extends AbstractBlockHighlighter
#[derive(Clone)]
pub struct SeaLumiesHighlighter {
    /// java: extends AbstractBlockHighlighter
    pub base: AbstractBlockHighlighterBase,
    /// java: private final BlockPosSet allBlocks = new BlockPosSet();
    pub all_blocks: compat::Shared<BlockPosSet>,
}

impl std::ops::Deref for SeaLumiesHighlighter {
    type Target = AbstractBlockHighlighterBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SeaLumiesHighlighter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SeaLumiesHighlighter {
    /// java: private SeaLumiesHighlighter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = AbstractBlockHighlighterBase::new_in(blocks::blocks().get("SEA_PICKLE"), DyeColor::Cyan, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                all_blocks: BlockPosSet::new(),
            });
            ((__h.clone() as compat::Shared<dyn AbstractBlockHighlighter>), __h)
        });
        __this
    }

    /// java: @Init public static void initClass()
    pub fn init_class() {
        ({ let __r = INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.init()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractBlockHighlighter>)) });
    }

    /// java: public void configCallback()
    pub fn config_callback(__this: &compat::Shared<Self>) {
        BlockPosSet::clear(&compat::read(__this, |__t| __t.highlighted_blocks.clone()));
        let mut world: Option<Arc<ClientLevel>> = mcrust::mcrust::modding::game::level();
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_process()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractBlockHighlighter>)) }) || world.is_none() || !(SeaLumiesHighlighter::is_enabled(__this)) {
            return;
        }
        for pos in BlockPosSet::iterator(&compat::read(__this, |__t| __t.all_blocks.clone())) {
            let mut state: Arc<BlockState> = world.as_ref().unwrap().get_block_state_typed(&pos);
            if ((compat::read(__this, |__t| __t.state_predicate.clone()).unwrap()))(&*state) && SeaLumiesHighlighter::is_enough_pickles(__this, &state) {
                BlockPosSet::add(&compat::read(__this, |__t| __t.highlighted_blocks.clone()), &pos);
            }
        }
    }

    /// java: private boolean isEnoughPickles(BlockState state)
    pub fn is_enough_pickles(__this: &compat::Shared<Self>, state: &BlockState) -> bool {
        // java: state.hasProperty(SeaPickleBlock.PICKLES) && state.getValue(SeaPickleBlock.PICKLES) >= minimumCount
        // (`SeaPickleBlock.PICKLES` is `BlockStateProperties.PICKLES`)
        let __pickles = mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::pickles();
        return state.has_property(&__pickles) && match state.get_value(&__pickles) {
            mcrust::net::minecraft::world::level::block::state::properties::property::Comparable::Integer(count) => count >= SkyblockerConfigManager::get().foraging.moonglade_marsh.sea_lumies_minimum_count,
            other => panic!("SeaPickleBlock.PICKLES is not an integer: {other:?}"),
        };
    }

    /// java: private boolean isEnabled()
    pub fn is_enabled(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().foraging.moonglade_marsh.enable_sea_lumies_highlighter;
    }

}

impl AbstractBlockHighlighter for SeaLumiesHighlighter {
    fn abstract_block_highlighter_base(&self) -> &AbstractBlockHighlighterBase {
        &self.base
    }
    fn abstract_block_highlighter_base_mut(&mut self) -> &mut AbstractBlockHighlighterBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void onBlockUpdate(BlockPos pos, BlockState oldState, BlockState newState) (dispatch: the body for this class)
    fn on_block_update(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>, BlockPos, &Arc<BlockState>, &Arc<BlockState>) {
        SeaLumiesHighlighter::on_block_update_dyn
    }

    /// java: @Override protected void onChunkUnload(ClientLevel world, LevelChunk chunk) (dispatch: the body for this class)
    fn on_chunk_unload(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>, &ClientLevel, &LevelChunk) {
        SeaLumiesHighlighter::on_chunk_unload_dyn
    }

    /// java: @Override protected void onChunkLoad(ClientLevel world, LevelChunk chunk) (dispatch: the body for this class)
    fn on_chunk_load(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>, &ClientLevel, &LevelChunk) {
        SeaLumiesHighlighter::on_chunk_load_dyn
    }

    /// java: @Override protected boolean shouldProcess() (dispatch: the body for this class)
    fn should_process(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>) -> bool {
        SeaLumiesHighlighter::should_process_dyn
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>) {
        SeaLumiesHighlighter::reset_dyn
    }

}

impl SeaLumiesHighlighter {
    /// java: @Override protected void onBlockUpdate(BlockPos pos, BlockState oldState, BlockState newState)
    pub fn on_block_update_impl(__this: &compat::Shared<Self>, mut pos: BlockPos, old_state: &Arc<BlockState>, new_state: &Arc<BlockState>) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_process()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractBlockHighlighter>)) }) {
            return;
        }
        if ((compat::read(__this, |__t| __t.state_predicate.clone()).unwrap()))(&**new_state) {
            BlockPosSet::add(&compat::read(__this, |__t| __t.all_blocks.clone()), &pos);
            if SeaLumiesHighlighter::is_enabled(__this) && SeaLumiesHighlighter::is_enough_pickles(__this, new_state) {
                BlockPosSet::add(&compat::read(__this, |__t| __t.highlighted_blocks.clone()), &pos);
            }
        } else {
            BlockPosSet::remove_block_pos(&compat::read(__this, |__t| __t.all_blocks.clone()), &pos);
            BlockPosSet::remove_block_pos(&compat::read(__this, |__t| __t.highlighted_blocks.clone()), &pos);
        }
    }

    /// SeaLumiesHighlighter.onBlockUpdate as the dispatch of AbstractBlockHighlighter.onBlockUpdate
    pub fn on_block_update_dyn(__this: &compat::Shared<dyn AbstractBlockHighlighter>, __p0: BlockPos, __p1: &Arc<BlockState>, __p2: &Arc<BlockState>) {
        Self::on_block_update_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override protected void onChunkUnload(ClientLevel world, LevelChunk chunk)
    pub fn on_chunk_unload_impl(__this: &compat::Shared<Self>, world: &ClientLevel, chunk: &LevelChunk) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_process()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractBlockHighlighter>)) }) {
            return;
        }
        let mut iterator = BlockPosSet::iterator(&compat::read(__this, |__t| __t.all_blocks.clone()));
        while iterator.has_next() {
            let mut pos: BlockPos = iterator.next().unwrap();
            // java: ChunkAccess holder = world.getChunk(pos); if (holder.equals(chunk)) - the same chunk object
            let mut holder: Option<Arc<LevelChunk>> = world.get_chunk(pos.get_x() >> 4, pos.get_z() >> 4);
            if holder.as_ref().is_some_and(|holder| std::ptr::eq(Arc::as_ptr(holder), chunk)) {
                iterator.remove();
                BlockPosSet::remove_block_pos(&compat::read(__this, |__t| __t.highlighted_blocks.clone()), &pos);
            }
        }
    }

    /// SeaLumiesHighlighter.onChunkUnload as the dispatch of AbstractBlockHighlighter.onChunkUnload
    pub fn on_chunk_unload_dyn(__this: &compat::Shared<dyn AbstractBlockHighlighter>, __p0: &ClientLevel, __p1: &LevelChunk) {
        Self::on_chunk_unload_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override protected void onChunkLoad(ClientLevel world, LevelChunk chunk)
    pub fn on_chunk_load_impl(__this: &compat::Shared<Self>, world: &ClientLevel, chunk: &LevelChunk) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_process()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractBlockHighlighter>)) }) {
            return;
        }
        let __state_predicate = compat::read(__this, |__t| __t.state_predicate.clone()).unwrap();
        chunk.access.find_blocks(&*__state_predicate, &mut |pos, state| {
            BlockPosSet::add(&compat::read(__this, |__t| __t.all_blocks.clone()), &pos);
            if SeaLumiesHighlighter::is_enabled(__this) && SeaLumiesHighlighter::is_enough_pickles(__this, state) {
                BlockPosSet::add(&compat::read(__this, |__t| __t.highlighted_blocks.clone()), &pos);
            }
        });
    }

    /// SeaLumiesHighlighter.onChunkLoad as the dispatch of AbstractBlockHighlighter.onChunkLoad
    pub fn on_chunk_load_dyn(__this: &compat::Shared<dyn AbstractBlockHighlighter>, __p0: &ClientLevel, __p1: &LevelChunk) {
        Self::on_chunk_load_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override protected boolean shouldProcess()
    pub fn should_process_impl(__this: &compat::Shared<Self>) -> bool {
        return Utils::is_in_galatea();
    }

    /// SeaLumiesHighlighter.shouldProcess as the dispatch of AbstractBlockHighlighter.shouldProcess
    pub fn should_process_dyn(__this: &compat::Shared<dyn AbstractBlockHighlighter>) -> bool {
        Self::should_process_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        BlockPosSet::clear(&compat::read(__this, |__t| __t.all_blocks.clone()));
        BlockPosSet::clear(&compat::read(__this, |__t| __t.highlighted_blocks.clone()));
    }

    /// SeaLumiesHighlighter.reset as the dispatch of AbstractBlockHighlighter.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn AbstractBlockHighlighter>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
