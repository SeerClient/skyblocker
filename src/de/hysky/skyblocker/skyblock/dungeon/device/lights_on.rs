//! Ported from de.hysky.skyblocker.skyblock.dungeon.device.LightsOn by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_boss::DungeonBoss;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::aabb::AABB;

/// java: private static final BlockPos TOP_LEFT = new BlockPos(62, 136, 142);
pub static TOP_LEFT: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(62, 136, 142));
/// java: private static final BlockPos TOP_RIGHT = new BlockPos(58, 136, 142);
pub static TOP_RIGHT: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(58, 136, 142));
/// java: private static final BlockPos MIDDLE_TOP = new BlockPos(60, 135, 142);
pub static MIDDLE_TOP: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(60, 135, 142));
/// java: private static final BlockPos MIDDLE_BOTTOM = new BlockPos(60, 134, 142);
pub static MIDDLE_BOTTOM: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(60, 134, 142));
/// java: private static final BlockPos BOTTOM_LEFT = new BlockPos(62, 133, 142);
pub static BOTTOM_LEFT: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(62, 133, 142));
/// java: private static final BlockPos BOTTOM_RIGHT = new BlockPos(58, 133, 142);
pub static BOTTOM_RIGHT: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(58, 133, 142));
/// java: private static final BlockPos[] LEVERS = { TOP_LEFT, TOP_RIGHT, MIDDLE_TOP, MIDDLE_BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT };
pub static LEVERS: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| vec![TOP_LEFT.get_mut().clone(), TOP_RIGHT.get_mut().clone(), MIDDLE_TOP.get_mut().clone(), MIDDLE_BOTTOM.get_mut().clone(), BOTTOM_LEFT.get_mut().clone(), BOTTOM_RIGHT.get_mut().clone()]);
/// java: private static final float[] RED = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));
/// java: private static final float ALPHA = 0.75f;
pub const ALPHA: f32 = 0.75f32;

/// java: public class LightsOn
#[derive(Clone)]
pub struct LightsOn {}

impl LightsOn {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { LightsOn::extract_rendering(&__a0); }));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(LightsOn::should_process()) {
            return;
        }
        for lever in LEVERS.get_mut().iter() {
            let mut world: Arc<ClientLevel> = mcrust::mcrust::modding::game::level().unwrap();
            let mut state: Arc<BlockState> = world.get_block_state_typed(lever);
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&state.get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("LEVER"))) && state.has_property(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::powered()) && state.get_value(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::powered()) != mcrust::net::minecraft::world::level::block::state::properties::property::Comparable::Boolean(true) {
                let mut r#box: Option<AABB> = RenderHelper::get_block_bounding_box_3(&world, &state, lever);
                if let Some(r#box) = &r#box {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, r#box, &RED, ALPHA, false) });
                }
            }
        }
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (SkyblockerConfigManager::get().dungeons.devices.solve_lights_on && Utils::is_in_dungeons() && DungeonManager::is_in_boss() && DungeonManager::get_boss() == DungeonBoss::Maxor);
    }

}
