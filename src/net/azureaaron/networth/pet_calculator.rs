//! java: public class PetCalculator (net.azureaaron.networth)
//!
//! Only `calculatePetLevel` is ported; `calculate` (the pet's price) needs `ModifierValues` and the price supplier
//! and is reached only through `ItemCalculator`, which is not ported yet (GAPS_Q #1).

use super::item::pet_info::PetInfo;
use super::utils::pet_constants;

/// java: public class PetCalculator
pub struct PetCalculator;

impl PetCalculator {
    /// java: public static IntIntPair calculatePetLevel(PetInfo info) - (left = level, right = max xp)
    pub fn calculate_pet_level(info: &PetInfo) -> (i32, i32) {
        //Legendary pets can't be tier boosted so we don't need to do any bounds checks
        let pet_tier: &str =
            if info.held_item().map(|s| s.as_str()).unwrap_or("") == "PET_ITEM_TIER_BOOST" {
                // PET_RARITIES.indexOf(tier) is -1 for an unknown tier, so the boosted index is 0
                let index = pet_constants::PET_RARITIES
                    .iter()
                    .position(|r| *r == info.tier())
                    .map(|i| i as i32)
                    .unwrap_or(-1);
                pet_constants::PET_RARITIES[(index + 1) as usize]
            } else {
                info.tier()
            };
        let max_pet_level: i32 = *pet_constants::SPECIAL_MAX_LVLS
            .get(info.r#type())
            .unwrap_or(&100);
        // Object2IntMap.getInt answers the default return value, 0, for a missing key
        let pet_offset: i32 = *pet_constants::RARITY_OFFSETS
            .get(if info.r#type() == "BINGO" {
                "COMMON"
            } else {
                pet_tier
            })
            .unwrap_or(&0);
        let pet_levels: &[i32] = &pet_constants::PET_LEVELS
            [pet_offset as usize..(pet_offset + max_pet_level - 1) as usize];

        let mut level: i32 = 1;
        let mut total_xp: i32 = 0;

        for &lvl_xp in pet_levels {
            total_xp = total_xp.wrapping_add(lvl_xp);

            if total_xp as f64 > info.xp() {
                total_xp = total_xp.wrapping_sub(lvl_xp);

                break;
            }

            level += 1;
        }
        let _ = total_xp;

        (
            level.min(max_pet_level),
            pet_levels.iter().fold(0i32, |sum, l| sum.wrapping_add(*l)),
        )
    }
}
