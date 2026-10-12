//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.DisciplineTestHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::dojo_manager;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::dojo_manager::DojoChallenges;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Map<String, String> SWORD_TO_NAME_LOOKUP = Map.of( "WOOD_SWORD", SkyBlockIcons.UNDEAD + " Wood", "IRON_SWORD", SkyBlockIcons.UNDEAD + " Iron", "GOLD_SWORD", SkyBlockIcons.UNDEAD +
pub static SWORD_TO_NAME_LOOKUP: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("WOOD_SWORD".to_string(), format!("{} Wood", sky_block_icons::UNDEAD)), ("IRON_SWORD".to_string(), format!("{} Iron", sky_block_icons::UNDEAD)), ("GOLD_SWORD".to_string(), format!("{} Gold", sky_block_icons::UNDEAD)), ("DIAMOND_SWORD".to_string(), format!("{} Diamond", sky_block_icons::UNDEAD))]));
/// java: public static final Map<String, Integer> SWORD_TO_COLOR_LOOKUP = Map.of( "WOOD_SWORD", 0xA52A2A, "IRON_SWORD", 0xC0C0C0, "GOLD_SWORD", 0xFFD700, "DIAMOND_SWORD", 0x00FFFF );
pub static SWORD_TO_COLOR_LOOKUP: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| std::collections::HashMap::from([("WOOD_SWORD".to_string(), 0xA52A2A), ("IRON_SWORD".to_string(), 0xC0C0C0), ("GOLD_SWORD".to_string(), 0xFFD700), ("DIAMOND_SWORD".to_string(), 0x00FFFF)]));

/// java: public class DisciplineTestHelper
#[derive(Clone)]
pub struct DisciplineTestHelper {}

impl DisciplineTestHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static boolean shouldGlow(String name)
    pub fn should_glow(name: &str) -> bool {
        if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        let mut held_id: String = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().get_skyblock_id()).unwrap();
        return (SWORD_TO_NAME_LOOKUP.get(&held_id).cloned().as_deref() == Some(&*(name)));
    }

    /// java: protected static int getColor()
    pub fn get_color() -> i32 {
        if compat::get_state(&dojo_manager::CURRENT_CHALLENGE) != DojoChallenges::Discipline || false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return 0;
        }
        let mut held_id: String = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().get_skyblock_id()).unwrap();
        return SWORD_TO_COLOR_LOOKUP.get(&held_id).cloned().unwrap_or_else(|| 0);
    }

}
