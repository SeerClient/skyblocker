//! Ported from de.hysky.skyblocker.skyblock.carnival.ZombieShootout by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::carnival::chivalrous_carnival::ChivalrousCarnival;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::entity::monster::zombie::zombie::Zombie;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final float[] RED = { 1, 0, 0 };
pub static RED: LazyLock<Vec<f32>> = LazyLock::new(|| vec![((1) as f32) as f32, ((0) as f32) as f32, ((0) as f32) as f32]);
/// java: private static final AABB SHOOTING_BOX = AABB.encapsulatingFullBlocks(new BlockPos(-100, 70, 45), new BlockPos(-102, 75, 43));
pub static SHOOTING_BOX: compat::MainThread<AABB> = compat::MainThread::new(|| AABB::encapsulating_full_blocks(&BlockPos::new(-100, 70, 45), &BlockPos::new(-102, 75, 43)));
/// java: private static final BlockPos[] LAMPS = { new BlockPos(-96, 76, 61), new BlockPos(-99, 77, 62), new BlockPos(-102, 75, 62), new BlockPos(-106, 77, 61), new BlockPos(-109, 75, 60), new BlockPos(-112, 7
pub static LAMPS: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| vec![BlockPos::new(-96, 76, 61), BlockPos::new(-99, 77, 62), BlockPos::new(-102, 75, 62), BlockPos::new(-106, 77, 61), BlockPos::new(-109, 75, 60), BlockPos::new(-112, 76, 58), BlockPos::new(-115, 77, 55), BlockPos::new(-117, 76, 52), BlockPos::new(-118, 76, 49), BlockPos::new(-119, 75, 45), BlockPos::new(-119, 77, 42), BlockPos::new(-118, 76, 39)]);

/// java: public class ZombieShootout
#[derive(Clone)]
pub struct ZombieShootout {}

impl ZombieShootout {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { ZombieShootout::extract_rendering(&__a0); }));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if ZombieShootout::is_in_zombie_shootout() && mcrust::mcrust::modding::game::level().is_some() {
            for pos in LAMPS.get_mut().iter() {
                let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(pos);
                let mut block: Arc<Block> = state.get_block();
                if std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("REDSTONE_LAMP"))) && state.has_property(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::lit()) && state.get_value(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::lit()) == mcrust::net::minecraft::world::level::block::state::properties::property::Comparable::Boolean(true) {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box()); __f(collector, pos, &RED, 5.0f32, false) });
                }
            }
        }
    }

    /// java: public static int getZombieGlowColor(Zombie zombie)
    pub fn get_zombie_glow_color(zombie: &Zombie) -> i32 {
        if !(zombie.get_item_by_slot(EquipmentSlot::Chest).is_empty()) {
            let mut item: Arc<Item> = zombie.get_item_by_slot(EquipmentSlot::Chest).get_item();
            // TODO(fabric2seer): a pattern switch expression
            //   java: return switch (item) {
            //   java: 				case Item i when i == Items.DIAMOND_CHESTPLATE -> DisciplineTestHelper.SWORD_TO_COLOR_LOOKUP.get("DIAMOND_SWORD");
            //   java: 				case Item i when i == Items.GOLDEN_CHESTPLATE -> DisciplineTestHelper.SWORD_TO_COLOR_LOOKUP.get("GOLD_SWORD");
            //   java: 				case Item i when i == Items.IRON_CHESTPLATE -> DisciplineTestHelper.SWORD_TO_COLOR_LOOKUP.get("IRON_SWORD");
            //   java: 				case Item i when i == Items.LEATHER_CHESTPLATE -> DisciplineTestHelper.SWORD_TO_COLOR_LOOKUP.get("WOOD_SWORD");
            //   java:
            //   java: 				default -> MobGlow.NO_GLOW;
            //   java: 			};
        }
        return *mob_glow::NO_GLOW;
    }

    /// java: public static boolean isInZombieShootout()
    pub fn is_in_zombie_shootout() -> bool {
        if ChivalrousCarnival::is_in_carnival() && SkyblockerConfigManager::get().helpers.carnival.zombie_shootout_helper && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            let mut pos: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap();
            return SHOOTING_BOX.get_mut().contains((pos.get_x()) as f64, (pos.get_y()) as f64, (pos.get_z()) as f64);
        }
        return false;
    }

}
