//! java: public class PetCalculator (net.azureaaron.networth)

use super::calculation::{Calculation, Type};
use super::data::modifier_values::ModifierValues;
use super::item::pet_info::PetInfo;
use super::networth_result::NetworthResult;
use super::utils::pet_constants;

/// java: public class PetCalculator
pub struct PetCalculator;

impl PetCalculator {
    /// java: public static NetworthResult calculate(PetInfo info, ToDoubleFunction<String> prices, ModifierValues modifiers)
    ///
    /// Only call this when you want to calculate the value of a pet when its not in item form.
    pub fn calculate(info: &PetInfo, prices: &dyn Fn(&str) -> f64, modifiers: &ModifierValues) -> NetworthResult {
        if *info == PetInfo::empty() {
            return NetworthResult::empty();
        }

        let lvl1: f64 = prices(&format!("LVL_1_{}_{}", info.tier(), info.r#type()));
        let lvl100: f64 = prices(&format!("LVL_100_{}_{}", info.tier(), info.r#type()));
        let lvl200: f64 = prices(&format!("LVL_200_{}_{}", info.tier(), info.r#type()));

        if lvl1 == 0.0 || (lvl100 == 0.0 && info.r#type() != "GOLDEN_DRAGON") {
            return NetworthResult::empty();
        }

        let mut calculations: Vec<Calculation> = Vec::new();
        let level_info: (i32, i32) = PetCalculator::calculate_pet_level(info);
        let level: i32 = level_info.0;
        let max_xp: i32 = level_info.1;

        let mut price: f64 = if lvl200 != 0.0 { lvl200 } else { lvl100 };

        //Base calculations
        if level < 100 && max_xp != 0 {
            let base_formula: f64 = (lvl100 - lvl1) / max_xp as f64;

            price = base_formula * info.xp() + lvl1;
        }

        if level > 100 && level < 200 {
            //Trim 1st digit off number
            let incremental_level: i32 = level % 10f64.powi((level as f64).log10() as i32) as i32;

            if incremental_level != 1 {
                let base_formula: f64 = (lvl200 - lvl100) / 100.0;

                price = base_formula * incremental_level as f64 + lvl100;
            }
        }

        let base: f64 = price;

        //Pet Skins
        if let Some(skin) = info.skin() {
            let calculation = Calculation::of(
                Type::Skin,
                skin.clone(),
                prices(&format!("PET_SKIN_{skin}")) * modifiers.regular().get_double("skins"),
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Held Item
        if let Some(held_item) = info.held_item() {
            let calculation = Calculation::of(
                Type::PetItem,
                held_item.clone(),
                prices(held_item) * modifiers.regular().get_double("petItem"),
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Pet Candy Price Reduction
        let max_pet_xp_from_candy: i32 = info.candies().wrapping_mul(1_000_000);
        let xp_without_candy: f64 = info.xp() - max_pet_xp_from_candy as f64;

        if info.candies() > 0
            && !pet_constants::BLOCKED_CANDY_REDUCTION_PETS.contains(info.r#type())
            && xp_without_candy < max_xp as f64
        {
            let reduced_value: f64 = price * modifiers.regular().get_double("petCandy");

            price = reduced_value.max(price - if level == 100 { 5_000_000.0 } else { 2_500_000.0 });
        }

        NetworthResult::of(price, base, calculations)
    }

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
