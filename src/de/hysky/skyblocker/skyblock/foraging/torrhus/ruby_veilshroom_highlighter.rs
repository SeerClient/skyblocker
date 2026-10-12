//! Ported from de.hysky.skyblocker.skyblock.foraging.torrhus.RubyVeilshroomHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::foraging::abstract_block_highlighter::AbstractBlockHighlighter;
use crate::de::hysky::skyblocker::skyblock::foraging::abstract_block_highlighter::AbstractBlockHighlighterBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::blocks;

/// java: private static final RubyVeilshroomHighlighter INSTANCE = new RubyVeilshroomHighlighter();
pub static INSTANCE: compat::MainThread<compat::Shared<RubyVeilshroomHighlighter>> = compat::MainThread::new(|| RubyVeilshroomHighlighter::new());

/// java: public final class RubyVeilshroomHighlighter extends AbstractBlockHighlighter
#[derive(Clone)]
pub struct RubyVeilshroomHighlighter {
    /// java: extends AbstractBlockHighlighter
    pub base: AbstractBlockHighlighterBase,
}

impl std::ops::Deref for RubyVeilshroomHighlighter {
    type Target = AbstractBlockHighlighterBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RubyVeilshroomHighlighter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RubyVeilshroomHighlighter {
    /// java: private RubyVeilshroomHighlighter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = AbstractBlockHighlighterBase::new_in(blocks::blocks().get("CRIMSON_FUNGUS"), DyeColor::Purple, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn AbstractBlockHighlighter>), __h)
        });
        __this
    }

    /// java: @Init public static void initClass()
    pub fn init_class() {
        ({ let __r = INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.init()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractBlockHighlighter>)) });
    }

}

impl AbstractBlockHighlighter for RubyVeilshroomHighlighter {
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
        RubyVeilshroomHighlighter::should_process_dyn
    }

}

impl RubyVeilshroomHighlighter {
    /// java: @Override protected boolean shouldProcess()
    pub fn should_process_impl(__this: &compat::Shared<Self>) -> bool {
        return (Utils::is_in_torrhus_canyon() && SkyblockerConfigManager::get().foraging.torrhus_canyon.enable_ruby_veilshroom_highlighter);
    }

    /// RubyVeilshroomHighlighter.shouldProcess as the dispatch of AbstractBlockHighlighter.shouldProcess
    pub fn should_process_dyn(__this: &compat::Shared<dyn AbstractBlockHighlighter>) -> bool {
        Self::should_process_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
