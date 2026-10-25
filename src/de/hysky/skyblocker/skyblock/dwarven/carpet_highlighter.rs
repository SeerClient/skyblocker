//! Ported from de.hysky.skyblocker.skyblock.dwarven.CarpetHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::boxes::Boxes;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public static final CarpetHighlighter INSTANCE = new CarpetHighlighter();
pub static INSTANCE: LazyLock<compat::Shared<CarpetHighlighter>> = LazyLock::new(|| CarpetHighlighter::new());
/// java: private static final Vec3 CARPET_BOUNDING_BOX = Boxes.getLengthVec(CarpetBlock.SHAPE.bounds());
pub static CARPET_BOUNDING_BOX: compat::MainThread<Vec3> = compat::MainThread::new(|| Boxes::get_length_vec(&mcrust::net::minecraft::world::level::block::block::column(16.0, 0.0, 1.0).bounds().unwrap()) /* `CarpetBlock.SHAPE` (private in the port) is `Block.column(16.0, 0.0, 1.0)` (:17) */);
/// java: private static final int SEARCH_RADIUS = 15;
pub const SEARCH_RADIUS: i32 = 15;
/// java: private static final int TICK_INTERVAL = 15;
pub const TICK_INTERVAL: i32 = 15;
/// java: private static final BlockPosSet CARPET_LOCATIONS = new BlockPosSet();
pub static CARPET_LOCATIONS: compat::MainThread<compat::Shared<BlockPosSet>> = compat::MainThread::new(|| BlockPosSet::new());
/// java: private static float[] colorComponents;
pub static COLOR_COMPONENTS: LazyLock<Mutex<Vec<f32>>> = LazyLock::new(|| Mutex::new(Default::default()));
/// java: private static boolean isLocationValid = false;
pub static IS_LOCATION_VALID: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public final class CarpetHighlighter implements Renderable, Resettable
#[derive(Clone)]
pub struct CarpetHighlighter {}

impl CarpetHighlighter {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
        CarpetHighlighter::config_callback(&INSTANCE, &SkyblockerConfigManager::get().mining.dwarven_mines.carpet_highlight_color);
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { ({ let __f = compat::dispatch(&INSTANCE, |__t| __t.extract_rendering()); __f(&compat::view(&INSTANCE, |a| a as compat::Shared<dyn Renderable>), &__a0) }); }));
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |__a0| { CarpetHighlighter::on_location_change(&INSTANCE, __a0); }));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { CarpetHighlighter::tick(&INSTANCE); }), TICK_INTERVAL) });
        // java: ClientPlayConnectionEvents.JOIN.register(INSTANCE); - `Resettable.onPlayReady` is `reset()`
        client_play_connection_events::JOIN.register(Arc::new(|_listener: &mut ClientPacketListener| {
            CarpetHighlighter::reset_impl(&INSTANCE);
        }));
    }

    /// java: public void onLocationChange(Location location)
    pub fn on_location_change(__this: &compat::Shared<Self>, mut location: Location) {
        compat::set_state(&IS_LOCATION_VALID, (location == Location::DwarvenMines));
    }

    /// java: public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        if !(compat::get_state(&IS_LOCATION_VALID)) || !(SkyblockerConfigManager::get().mining.dwarven_mines.enable_carpet_highlighter) || mcrust::mcrust::modding::game::level().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut iterable: mcrust::net::minecraft::core::block_pos::WithinManhattan = BlockPos::within_manhattan(&mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap(), SEARCH_RADIUS);
        for block_pos in iterable {
            if CarpetHighlighter::check_for_carpet(__this, &block_pos) {
                BlockPosSet::add(CARPET_LOCATIONS.get_mut(), &block_pos);
            }
        }
    }

    /// java: private boolean checkForCarpet(BlockPos blockPos)
    pub fn check_for_carpet(__this: &compat::Shared<Self>, block_pos: &BlockPos) -> bool {
        let mut actual_block: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(block_pos);
        if !(actual_block.is_block(mcrust::net::minecraft::world::level::block::blocks::carpet().gray()) || actual_block.is_block(mcrust::net::minecraft::world::level::block::blocks::carpet().light_blue()) || actual_block.is_block(mcrust::net::minecraft::world::level::block::blocks::carpet().light_gray())) {
            return false;
        }
        let mut block_below: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&block_pos.below());
        return block_below.is_block(&blocks::blocks().get("SEA_LANTERN"));
    }

    /// java: public void configCallback(Color color)
    pub fn config_callback(__this: &compat::Shared<Self>, color: &Color) {
        // java: colorComponents = color.getRGBComponents(null);
        compat::set_state(&COLOR_COMPONENTS, color.get_rgb_components(None).to_vec());
    }

}

impl Renderable for CarpetHighlighter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        CarpetHighlighter::extract_rendering_dyn
    }

}

impl Resettable for CarpetHighlighter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        CarpetHighlighter::reset_dyn
    }

}

impl CarpetHighlighter {
    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(compat::get_state(&IS_LOCATION_VALID)) || !(SkyblockerConfigManager::get().mining.dwarven_mines.enable_carpet_highlighter) {
            return;
        }
        for carpet_location in BlockPosSet::iterate_mut(CARPET_LOCATIONS.get_mut()) {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_5()); __f(collector, &vec3::at_lower_corner_of(&carpet_location.immutable()), CARPET_BOUNDING_BOX.get_mut(), &compat::get_state(&COLOR_COMPONENTS), compat::lock(&COLOR_COMPONENTS)[(3) as usize], false) });
        }
    }

    /// CarpetHighlighter.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        compat::set_state(&IS_LOCATION_VALID, false);
        BlockPosSet::clear(CARPET_LOCATIONS.get_mut());
    }

    /// CarpetHighlighter.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
