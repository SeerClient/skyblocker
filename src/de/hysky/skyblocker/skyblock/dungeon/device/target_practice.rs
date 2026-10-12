//! Ported from de.hysky.skyblocker.skyblock.dungeon.device.TargetPractice by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::world_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_boss::DungeonBoss;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;

/// java: private static final BlockPos PRESSURE_PLATE = new BlockPos(63, 127, 35);
pub static PRESSURE_PLATE: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(63, 127, 35));
/// java: private static final int ACTIVATION_THRESHOLD = 1;
pub const ACTIVATION_THRESHOLD: i32 = 1;
/// java: private static final int UNACTIVATED = 0;
pub const UNACTIVATED: i32 = 0;
/// java: private static final Set<BlockPos> POSSIBLE_TARGETS = Set.of( new BlockPos(68, 130, 50), new BlockPos(66, 130, 50), new BlockPos(64, 130, 50), new BlockPos(68, 128, 50), new BlockPos(66, 128, 50), new
pub static POSSIBLE_TARGETS: compat::MainThread<std::collections::HashSet<BlockPos>> = compat::MainThread::new(|| std::collections::HashSet::from([BlockPos::new(68, 130, 50), BlockPos::new(66, 130, 50), BlockPos::new(64, 130, 50), BlockPos::new(68, 128, 50), BlockPos::new(66, 128, 50), BlockPos::new(64, 128, 50), BlockPos::new(68, 126, 50), BlockPos::new(66, 126, 50), BlockPos::new(64, 126, 50)]));
/// java: private static final List<BlockPos> HIT_TARGETS = new ArrayList<>();
pub static HIT_TARGETS: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| Vec::new());
/// java: private static final float[] RED = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));

/// java: public class TargetPractice
#[derive(Clone)]
pub struct TargetPractice {}

impl TargetPractice {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        world_events::BLOCK_STATE_UPDATE.register(Arc::new(move |__a0, __a1, __a2| { TargetPractice::on_block_state_update(&__a0, &__a1, &__a2); }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { TargetPractice::extract_rendering(&__a0); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            TargetPractice::reset();
        }));
    }

    /// java: private static void onBlockStateUpdate(BlockPos pos, BlockState oldState, BlockState newState)
    pub fn on_block_state_update(pos: &BlockPos, old_state: &Arc<BlockState>, new_state: &Arc<BlockState>) {
        if !(TargetPractice::should_process()) {
            return;
        }
        if pos == PRESSURE_PLATE.get_mut() && std::ptr::addr_eq(std::sync::Arc::as_ptr(&new_state.get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("light_weighted_pressure_plate"))) {
            if new_state.get_value(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::power()) < mcrust::net::minecraft::world::level::block::state::properties::property::Comparable::Integer(ACTIVATION_THRESHOLD) {
                TargetPractice::reset();
                return;
            }
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&old_state.get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("light_weighted_pressure_plate"))) && old_state.get_value(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::power()) == mcrust::net::minecraft::world::level::block::state::properties::property::Comparable::Integer(UNACTIVATED) {
                TargetPractice::reset();
                return;
            }
        }
        if POSSIBLE_TARGETS.get_mut().contains(pos) {
            if std::ptr::addr_eq(std::sync::Arc::as_ptr(&old_state.get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("emerald_block"))) && std::ptr::addr_eq(std::sync::Arc::as_ptr(&new_state.get_block()), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::level::block::blocks::dyed_terracotta().blue())) {
                HIT_TARGETS.get_mut().push(pos.immutable());
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(TargetPractice::should_process()) {
            return;
        }
        for pos in HIT_TARGETS.get_mut().iter() {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, pos, &RED, 0.5f32, false) });
        }
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (SkyblockerConfigManager::get().dungeons.devices.solve_target_practice && Utils::is_in_dungeons() && DungeonManager::is_in_boss() && DungeonManager::get_boss() == DungeonBoss::Maxor);
    }

    /// java: private static void reset()
    pub fn reset() {
        HIT_TARGETS.get_mut().clear();
    }

}
