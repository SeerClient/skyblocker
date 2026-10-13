//! Ported from de.hysky.skyblocker.skyblock.carnival.CatchAFish by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::carnival::chivalrous_carnival::ChivalrousCarnival;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow;
use crate::de::hysky::skyblocker::skyblock::item::head_textures;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final AABB AREA = AABB.encapsulatingFullBlocks(new BlockPos(-69, 65, 25), new BlockPos(-87, 84, 52));
pub static AREA: compat::MainThread<AABB> = compat::MainThread::new(|| AABB::encapsulating_full_blocks(&BlockPos::new(-69, 65, 25), &BlockPos::new(-87, 84, 52)));
//   java: private static final int YELLOW = Color.YELLOW.getRGB();
/// java: private static final int YELLOW = Color.YELLOW.getRGB();
pub static YELLOW: LazyLock<i32> = LazyLock::new(|| mcrust::java::awt::color::yellow().get_rgb());

/// java: public class CatchAFish
#[derive(Clone)]
pub struct CatchAFish {}

impl CatchAFish {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int getFishGlowColor(ArmorStand armorStand)
    pub fn get_fish_glow_color(armor_stand: &ArmorStand) -> i32 {
        let stack = armor_stand.get_item_by_slot(EquipmentSlot::Head);
        if !stack.is_empty() && stack.is_item(&items::items().get("player_head")) && ItemUtils::get_head_texture(stack) == head_textures::CARNIVAL_YELLOW_FISH {
            return *YELLOW;
        }
        return *mob_glow::NO_GLOW;
    }

    /// java: public static boolean isInCatchAFish()
    pub fn is_in_catch_a_fish() -> bool {
        if ChivalrousCarnival::is_in_carnival() && SkyblockerConfigManager::get().helpers.carnival.catch_a_fish_helper && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            let mut pos: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap();
            return AREA.get_mut().contains((pos.get_x()) as f64, (pos.get_y()) as f64, (pos.get_z()) as f64);
        }
        return false;
    }

}
