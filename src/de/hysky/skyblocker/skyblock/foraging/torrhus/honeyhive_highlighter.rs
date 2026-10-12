//! Ported from de.hysky.skyblocker.skyblock.foraging.torrhus.HoneyhiveHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::foraging::abstract_block_highlighter::AbstractBlockHighlighter;
use crate::de::hysky::skyblocker::skyblock::foraging::abstract_block_highlighter::AbstractBlockHighlighterBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;

/// java: private static final HoneyhiveHighlighter INSTANCE = new HoneyhiveHighlighter();
pub static INSTANCE: compat::MainThread<compat::Shared<HoneyhiveHighlighter>> = compat::MainThread::new(|| HoneyhiveHighlighter::new());

/// java: public final class HoneyhiveHighlighter extends AbstractBlockHighlighter
#[derive(Clone)]
pub struct HoneyhiveHighlighter {
    /// java: extends AbstractBlockHighlighter
    pub base: AbstractBlockHighlighterBase,
}

impl std::ops::Deref for HoneyhiveHighlighter {
    type Target = AbstractBlockHighlighterBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HoneyhiveHighlighter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HoneyhiveHighlighter {
    /// java: private HoneyhiveHighlighter()
    pub fn new() -> compat::Shared<Self> {
        let mut pos_predicate: Arc<dyn Fn(ClientLevel, BlockPos) -> bool> = Arc::new(move |level, pos| !(level.get_block_state_typed(&pos.above()).is_block(&blocks::blocks().get("birch_fence"))));
        // TODO(fabric2seer): static `BeehiveBlock.HONEY_LEVEL` has no Rust counterpart in the port's BeehiveBlock file
        // TODO(fabric2seer): static `BeehiveBlock.MAX_HONEY_LEVELS` has no Rust counterpart in the port's BeehiveBlock file
        // rust: let mut state_predicate: Arc<dyn Fn(Arc<BlockState>) -> bool> = Arc::new(move |state| (state.is_block(&blocks::blocks().get("bee_nest")) && state.base.holder.get_nullable_value(&Block::HONEY_LEVEL).clone().unwrap_or(0) == Block::MAX_HONEY_LEVELS));
        let mut state_predicate: Arc<dyn Fn(Arc<BlockState>) -> bool> = todo!("fabric2seer: the initialiser of `statePredicate` (TODO above)");
        // TODO(fabric2seer): super(..) with 0 argument(s)
        //   java:
        let __this: compat::Shared<Self> = todo!("constructor");
        // TODO(fabric2seer): super statement
        //   java: super(posPredicate, statePredicate, ColorUtils.getFloatComponents(DyeColor.BLUE.getTextColor()));
        __this
    }

    /// java: @Init public static void initClass()
    pub fn init_class() {
        ({ let __r = INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.init()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractBlockHighlighter>)) });
    }

}

impl AbstractBlockHighlighter for HoneyhiveHighlighter {
    fn abstract_block_highlighter_base(&self) -> &AbstractBlockHighlighterBase {
        &self.base
    }
    fn abstract_block_highlighter_base_mut(&mut self) -> &mut AbstractBlockHighlighterBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected boolean shouldProcess() (dispatch: the body for this class)
    fn should_process(&self) -> fn(&compat::Shared<dyn AbstractBlockHighlighter>) -> bool {
        HoneyhiveHighlighter::should_process_dyn
    }

}

impl HoneyhiveHighlighter {
    /// java: @Override protected boolean shouldProcess()
    pub fn should_process_impl(__this: &compat::Shared<Self>) -> bool {
        return (Utils::is_in_torrhus_canyon() && SkyblockerConfigManager::get().foraging.torrhus_canyon.enable_honeyhive_highlighter);
    }

    /// HoneyhiveHighlighter.shouldProcess as the dispatch of AbstractBlockHighlighter.shouldProcess
    pub fn should_process_dyn(__this: &compat::Shared<dyn AbstractBlockHighlighter>) -> bool {
        Self::should_process_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
