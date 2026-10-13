//! java: public class ItemCalculator (net.azureaaron.networth)
//!
//! `ToDoubleFunction<String> prices` is `&dyn Fn(&str) -> f64`; `Object2ObjectMap<String, SkyblockItemData>` is a
//! `HashMap<String, SkyblockItemData>`. Where the Java hands the price function a `null` id (`ItemConstants.REFORGES`
//! has no entry for the reforge), the port hands it `""` - the one supplier (Skyblocker's
//! `NetworthDataSuppliers.getPrice` -> `ItemUtils.getItemPrice`) answers both with an empty price (`:427`).
//! A Java exception (`Optional.get()` on an empty optional, a `null` map lookup dereferenced) is a panic, which
//! the caller (`NetworthCalculator.getItemNetworth`) catches as the Java does.

use std::collections::HashMap;
use std::sync::LazyLock;

use mcrust::com::mojang::datafixers::util::either::Either;

use super::calculation::{Calculation, Type};
use super::data::modifier_values::{self, ModifierValues};
use super::data::skyblock_item_data::{GearUpgrade, GemstoneSlot, SkyblockItemData};
use super::item::item_metadata::ItemMetadata;
use super::item::skyblock_item_stack::SkyblockItemStack;
use super::networth_result::NetworthResult;
use super::pet_calculator::PetCalculator;
use super::utils::item_constants;
use super::utils::prestige_constants;

/// java: itemId.replaceAll("(?:(?:HOT|BURNING|FIERY|INFERNAL)_)", "") - the pattern `String.replaceAll` compiles
static PRESTIGE_PREFIX_PATTERN: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new("(?:(?:HOT|BURNING|FIERY|INFERNAL)_)").expect("a valid pattern")
});

/// java: `List<T> upgradeCosts` in `upgradeCosts` - each element is either a `List<GearUpgrade>` (a star's costs)
/// or a `GearUpgrade` (a prestige cost), told apart by `instanceof` there.
enum UpgradeCost<'a> {
    List(&'a Vec<GearUpgrade>),
    Single(&'a GearUpgrade),
}

/// java: public class ItemCalculator
pub struct ItemCalculator;

impl ItemCalculator {
    /// java: public static NetworthResult calculate(SkyblockItemStack stack, ToDoubleFunction<String> prices,
    /// Object2ObjectMap<String, SkyblockItemData> skyblockItems)
    pub fn calculate(
        stack: &SkyblockItemStack,
        prices: &dyn Fn(&str) -> f64,
        skyblock_items: &HashMap<String, SkyblockItemData>,
    ) -> NetworthResult {
        ItemCalculator::calculate_4(stack, prices, skyblock_items, &modifier_values::DEFAULT)
    }

    /// java: public static NetworthResult calculate(SkyblockItemStack stack, ToDoubleFunction<String> prices,
    /// Object2ObjectMap<String, SkyblockItemData> skyblockItems, ModifierValues modifiers)
    ///
    /// Calculates the networth for a skyblock item. Automatically handles pets in their item form.
    pub fn calculate_4(
        stack: &SkyblockItemStack,
        prices: &dyn Fn(&str) -> f64,
        skyblock_items: &HashMap<String, SkyblockItemData>,
        modifiers: &ModifierValues,
    ) -> NetworthResult {
        let mut item_id: String = stack.id().to_string();
        let skyblock_item_data: Option<&SkyblockItemData> = skyblock_items.get(&item_id);
        let meta: &dyn ItemMetadata = stack.metadata();
        let mut calculations: Vec<Calculation> = Vec::new();

        //Pet Items
        if item_id == "PET" {
            return PetCalculator::calculate(meta.pet_info(), prices, modifiers);
        }

        //Runes
        if (item_id == "RUNE" || item_id == "UNIQUE_RUNE")
            && meta.cosmetics().runes().is_some()
            && !meta.cosmetics().runes().unwrap().is_empty()
        {
            let first = &meta.cosmetics().runes().unwrap()[0];

            item_id = format!("{}_RUNE_{}", first.0.to_uppercase(), first.1);
        }

        //Limited Edition Items
        let limited = meta.limited_edition_info();
        if item_id == "NEW_YEAR_CAKE" && limited.new_year_cake_year().is_some() {
            item_id = format!("{}_{}", item_id, limited.new_year_cake_year().unwrap());
        } else if item_constants::COLOURED_ANNIVERSARY_ITEMS.contains(item_id.as_str())
            && limited.party_hat_colour().is_some()
        {
            item_id = format!("{}_{}", item_id, limited.party_hat_colour().unwrap().to_uppercase());
        } else if item_id == "PARTY_HAT_SLOTH" && limited.party_hat_emoji().is_some() {
            item_id = format!("{}_{}", item_id, limited.party_hat_emoji().unwrap().to_uppercase());
        } else if item_id == "ABICASE" && limited.abicase_model().is_some() {
            item_id = format!("{}_{}", item_id, limited.abicase_model().unwrap().to_uppercase());
        } //default -> Do Nothing

        //Shiny Items (Cosmetic Effect)
        if meta.cosmetics().shiny() {
            item_id = format!("SHINY_{item_id}");
        }

        //We multiply by stack count ahead of time because items with modifiers cannot be stacked.
        let mut price: f64 = prices(&item_id) * stack.count() as f64;
        let mut base: f64 = prices(&item_id) * stack.count() as f64;

        //Shen's price
        if price == 0.0 && meta.auction_bid_info().price().is_some() {
            price = meta.auction_bid_info().price().unwrap() as f64 * 0.85;
            base = meta.auction_bid_info().price().unwrap() as f64 * 0.85;
        }

        //Pickonimbus Durability scaling
        if item_id == "PICKONIMBUS" && meta.drill_info().pickonimbus_durability().is_some() {
            let reduction: f64 = meta.drill_info().pickonimbus_durability().unwrap() as f64
                / item_constants::PICKONIMBUS_DURABILITY as f64;

            price += price * (reduction - 1.0);
            base += price * (reduction - 1.0);
        }

        //God Roll Attributes
        if item_id != "ATTRIBUTE_SHARD" && !meta.attributes().is_empty() {
            let mut sorted_attributes: Vec<String> =
                meta.attributes().iter().map(|(s, _)| s.to_uppercase()).collect();
            sorted_attributes.sort();
            let mut base_id: String = PRESTIGE_PREFIX_PATTERN.replace_all(&item_id, "").into_owned();
            base_id += &format!("-{}", sorted_attributes.join("-"));
            let god_roll_id: String = base_id;
            let god_roll_price: f64 = prices(&god_roll_id);

            if god_roll_price > price {
                price = god_roll_price;
                base = god_roll_price;

                calculations.push(Calculation::of(Type::GodRoll, god_roll_id, god_roll_price));
            }
        }

        //Armour Upgrades like crimson
        //The price is checked for whether its 0, because upgraded pieces are soul bound and do not have a price
        if price == 0.0 && prestige_constants::PRESTIGES.contains_key(item_id.as_str()) {
            for prestige_item_id in prestige_constants::PRESTIGES.get(item_id.as_str()).unwrap().iter() {
                let item_data: Option<&SkyblockItemData> = skyblock_items.get(*prestige_item_id);

                if let Some(item_data) = item_data {
                    if let Some(upgrade_costs) = item_data.upgrade_costs() {
                        let entries: Vec<UpgradeCost> = upgrade_costs.iter().map(UpgradeCost::List).collect();
                        price += ItemCalculator::upgrade_costs(
                            prices,
                            &mut calculations,
                            &entries,
                            Some(*prestige_item_id),
                            modifiers,
                        );
                    }
                    if let Some(prestige) = item_data.prestige() {
                        let entries: Vec<UpgradeCost> = prestige.costs().iter().map(UpgradeCost::Single).collect();
                        price += ItemCalculator::upgrade_costs(
                            prices,
                            &mut calculations,
                            &entries,
                            Some(*prestige_item_id),
                            modifiers,
                        );
                    }
                }
            }
        }

        //Shen's Auction Price Paid
        if meta.auction_bid_info().price().is_some()
            && meta.auction_bid_info().auction().is_some()
            && meta.auction_bid_info().bid().is_some()
        {
            let scaled_price_paid: f64 = meta.auction_bid_info().price().unwrap() as f64
                * modifiers.regular().get_double("shensAuctionPrice");

            if scaled_price_paid > price {
                price = scaled_price_paid;
                calculations.push(Calculation::of(Type::ShenAuction, item_id.clone(), scaled_price_paid));
            }
        }

        //Midas Stuff
        if item_id == "MIDAS_STAFF" || item_id == "MIDAS_SWORD" {
            let max_bid: i64 = if item_id == "MIDAS_SWORD" { 50_000_000 } else { 100_000_000 };

            if let Some(winning_bid) = meta.auction_bid_info().winning_bid() {
                if winning_bid >= max_bid {
                    let max_item: &str = if item_id == "MIDAS_SWORD" { "MIDAS_SWORD_50M" } else { "MIDAS_STAFF_100M" };
                    let calculation = Calculation::of(Type::WinningBid, max_item.to_string(), prices(max_item));

                    price = calculation.price();
                    calculations.push(calculation);
                } else {
                    let calculation = Calculation::of(
                        Type::WinningBid,
                        item_id.clone(),
                        winning_bid as f64 * modifiers.regular().get_double("winningBid"),
                    );
                    price = calculation.price();
                    calculations.push(calculation);
                }
            }
        }

        //Enchanted Books
        if !meta.enchantments().is_empty() && item_id == "ENCHANTED_BOOK" {
            let multi_enchantment_book: bool = meta.enchantments().len() > 1;

            //Single Enchant Book
            if !multi_enchantment_book {
                let first = &meta.enchantments()[0];
                let enchantment_api_name: String = format!("{}_{}", first.0.to_uppercase(), first.1);
                let calculation_price = prices(&format!("ENCHANTMENT_{enchantment_api_name}"));
                let calculation = Calculation::of(Type::Enchantment, enchantment_api_name, calculation_price);

                price = calculation.price();
                calculations.push(calculation);
            } else {
                //Multiple Enchant Book
                let mut enchantments_price: f64 = 0.0;

                for entry in meta.enchantments().iter() {
                    let enchantment_api_name: String = format!("{}_{}", entry.0.to_uppercase(), entry.1);
                    let calculation_price = prices(&format!("ENCHANTMENT_{enchantment_api_name}"))
                        * modifiers.regular().get_double("enchantments");
                    let calculation = Calculation::of(Type::Enchantment, enchantment_api_name, calculation_price);
                    enchantments_price += calculation.price();

                    calculations.push(calculation);
                }

                price = enchantments_price;
            }
        }

        //Skins
        if let Some(skin) = meta.cosmetics().skin() {
            let calculation = Calculation::of(
                Type::Skin,
                skin.clone(),
                prices(skin) * modifiers.regular().get_double("skins"),
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Regular Enchanted Items
        if !meta.enchantments().is_empty() && item_id != "ENCHANTED_BOOK" {
            for entry in meta.enchantments().iter() {
                let enchantment_id: String = entry.0.to_uppercase();
                let mut level: i32 = entry.1;

                //Skip blocked and ignored enchantments
                if item_constants::BLOCKED_ENCHANTMENTS.contains_key(item_id.as_str())
                    && item_constants::BLOCKED_ENCHANTMENTS
                        .get(item_id.as_str())
                        .unwrap()
                        .contains(enchantment_id.as_str())
                {
                    continue;
                }
                if item_constants::IGNORED_ENCHANTMENTS.contains_key(enchantment_id.as_str())
                    && *item_constants::IGNORED_ENCHANTMENTS.get(enchantment_id.as_str()).unwrap() == level
                {
                    continue;
                }

                //Set stacking enchant level to 1 so we can get a price off the bazaar for it
                if item_constants::STACKING_ENCHANTMENTS.contains(enchantment_id.as_str()) {
                    level = 1;
                }

                //Add Silex price
                if enchantment_id == "EFFICIENCY"
                    && level > 5
                    && !item_constants::IGNORE_SILEX.contains(item_id.as_str())
                {
                    let silexes: i32 = level - 5;

                    if silexes > 0 {
                        let calculation = Calculation::of_4(
                            Type::Silex,
                            "SIL_EX".to_string(),
                            prices("SIL_EX") * silexes as f64 * modifiers.regular().get_double("silex"),
                            silexes,
                        );

                        price += calculation.price();
                        calculations.push(calculation);
                    }
                }

                //Add Golden Bounty price
                if enchantment_id == "SCAVENGER" && level == 6 {
                    let calculation = Calculation::of(
                        Type::GoldenBounty,
                        "GOLDEN_BOUNTY".to_string(),
                        prices("GOLDEN_BOUNTY") * modifiers.regular().get_double("goldenBounty"),
                    );

                    price += calculation.price();
                    calculations.push(calculation);
                }

                let enchantment_worth: f64 = modifiers
                    .enchantments()
                    .get_or_default(&enchantment_id, modifiers.regular().get_double("enchantments"));
                let enchantment_price: f64 = if enchantment_id.starts_with("ULTIMATE") {
                    //For some ultimate enchants like Chimera the t5 books prices are way lower (due to nobody selling them) than if u got 16 t1 books
                    let lvl1_books_needed: i32 = 2f64.powf((level - 1) as f64) as i32;
                    let direct_price: f64 = prices(&format!("ENCHANTMENT_{enchantment_id}_{level}"));
                    let price_from_lvl1s: f64 =
                        prices(&format!("ENCHANTMENT_{enchantment_id}_{}", 1)) * lvl1_books_needed as f64;

                    if price_from_lvl1s > direct_price { price_from_lvl1s } else { direct_price }
                } else {
                    prices(&format!("ENCHANTMENT_{enchantment_id}_{level}"))
                };

                let calculation = Calculation::of(
                    Type::Enchantment,
                    format!("{enchantment_id}_{level}"),
                    enchantment_price * enchantment_worth,
                );

                price += calculation.price();
                calculations.push(calculation);
            }
        }

        //Attributes
        if !meta.attributes().is_empty() {
            for entry in meta.attributes().iter() {
                let attribute: String = entry.0.to_uppercase();
                let tier: i32 = entry.1;

                //Better described as the # of level 1 attributes needed to get to the tier
                let shards: i32 = (2f64.powf((tier - 1) as f64) - 1.0) as i32;
                let base_attribute_id: String = format!("ATTRIBUTE_SHARD-{attribute}_");

                //Calculate all possible level 1 attribute combinations to avoid cases where the price is over inflated
                //because there is very few level 1 attribute shards on the ah
                let attribute_prices: [f64; 3] = [
                    prices(&format!("{base_attribute_id}{}", 1)),
                    prices(&format!("{base_attribute_id}{}", 2)) / 2.0,
                    prices(&format!("{base_attribute_id}{}", 3)) / 4.0,
                ];
                let mut base_attribute_price: f64 = attribute_prices
                    .iter()
                    .copied()
                    .filter(|shard_price| *shard_price > 0.0) //For some attributes like dominance and mana regeneration its impossible to get a lvl 1 shard
                    .fold(None, |min: Option<f64>, p| Some(match min { Some(m) => m.min(p), None => p }))
                    .unwrap_or(0.0); //Choose lowest price to further prevent prices being inflated

                //Some items like Hellfire Rods for example are typically start out as Magma Rods because they are way cheaper to
                //craft & merge together to get high attribute levels on, then these Magma Rods are upgraded to Hellfire Rods.
                if item_constants::ATTRIBUTE_BASE_COSTS.contains_key(item_id.as_str())
                    && prices(item_constants::ATTRIBUTE_BASE_COSTS.get(item_id.as_str()).unwrap()) < base_attribute_price
                {
                    base_attribute_price = prices(item_constants::ATTRIBUTE_BASE_COSTS.get(item_id.as_str()).unwrap());
                }

                //Kuudra Helmets are valued differently since they usually aren't as useful and we don't want to "pollute" the values of
                //the other armour prices
                if item_constants::KUUDRA_HELMETS.contains(item_id.as_str())
                    && prices(&format!("KUUDRA_HELMET_{attribute}")) < base_attribute_price
                {
                    base_attribute_price = prices(&format!("KUUDRA_HELMET_{attribute}"));
                } else if item_constants::KUUDRA_OTHERS.contains(item_id.as_str()) {
                    //Same thing as above but for chestplates, leggings, and boots
                    let kuudra_armour_prices: [f64; 3] = [
                        prices(&format!("KUUDRA_CHESTPLATE_{attribute}")),
                        prices(&format!("KUUDRA_LEGGINGS_{attribute}")),
                        prices(&format!("KUUDRA_BOOTS_{attribute}")),
                    ];
                    let mean_armour_price: f64 = kuudra_armour_prices.iter().sum::<f64>() / 3.0;

                    if mean_armour_price > 0.0
                        && (base_attribute_price <= 0.0 || mean_armour_price < base_attribute_price)
                    {
                        base_attribute_price = mean_armour_price;
                    }
                }

                let calculation = Calculation::of(
                    Type::Attribute,
                    format!("{attribute}_{tier}"),
                    base_attribute_price * shards as f64 * modifiers.regular().get_double("attributes"),
                );

                price += calculation.price();
                calculations.push(calculation);
            }
        }

        //Pocket Sack-in-a-Sack
        if let Some(count) = meta.misc_modifiers().pocket_sack_in_a_sacks() {
            let calculation = Calculation::of_4(
                Type::PocketSackInASack,
                "POCKET_SACK_IN_A_SACK".to_string(),
                prices("POCKET_SACK_IN_A_SACK") * count as f64 * modifiers.regular().get_double("pocketSackInASack"),
                count,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Wood Singularity
        if let Some(count) = meta.gear_upgrades().wood_singularities() {
            let calculation = Calculation::of_4(
                Type::WoodSingularity,
                "WOOD_SINGULARITY".to_string(),
                prices("WOOD_SINGULARITY") * count as f64 * modifiers.regular().get_double("woodSingularity"),
                count,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Jalapeno Book
        if let Some(count) = meta.misc_modifiers().jalapeno_books() {
            let calculation = Calculation::of_4(
                Type::JalapenoBook,
                "JALAPENO_BOOK".to_string(),
                prices("JALAPENO_BOOK") * count as f64 * modifiers.regular().get_double("jalapenoBook"),
                count,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Transmission Tuners
        if let Some(count) = meta.misc_modifiers().transmission_tuners() {
            let calculation = Calculation::of_4(
                Type::TransmissionTuner,
                "TRANSMISSION_TUNER".to_string(),
                prices("TRANSMISSION_TUNER") * count as f64 * modifiers.regular().get_double("transmissionTuner"),
                count,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Mana Disintegrators
        if let Some(count) = meta.misc_modifiers().mana_disintegrators() {
            let calculation = Calculation::of_4(
                Type::ManaDisintegrator,
                "MANA_DISINTEGRATOR".to_string(),
                prices("MANA_DISINTEGRATOR") * count as f64 * modifiers.regular().get_double("manaDisintegrator"),
                count,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Pulse Ring
        if meta.accessory_upgrades().thunder_charges().is_some() && item_id == "PULSE_RING" {
            let thunder_bottles_used: i32 = meta.accessory_upgrades().thunder_charges().unwrap().div_euclid(50_000);
            let calculation = Calculation::of_4(
                Type::ThunderInABottle,
                "THUNDER_IN_A_BOTTLE".to_string(),
                prices("THUNDER_IN_A_BOTTLE")
                    * thunder_bottles_used as f64
                    * modifiers.regular().get_double("thunderInABottle"),
                thunder_bottles_used,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Runes (Applied)
        if meta.cosmetics().runes().is_some()
            && !meta.cosmetics().runes().unwrap().is_empty()
            && !item_id.contains("RUNE")
        {
            let rune = &meta.cosmetics().runes().unwrap()[0];
            let rune_item_id: String = format!("{}_RUNE_{}", rune.0.to_uppercase(), rune.1);

            if item_constants::VALID_RUNES.contains(rune_item_id.as_str()) {
                let calculation_price = prices(&rune_item_id) * modifiers.regular().get_double("runes");
                let calculation = Calculation::of(Type::Rune, rune_item_id, calculation_price);

                price += calculation.price();
                calculations.push(calculation);
            }
        }

        //Hot Potato Books
        if let Some(hot_potatoes) = meta.gear_upgrades().hot_potatoes() {
            //Fuming Potato Books
            if hot_potatoes > 10 {
                let fuming_potato_books_used: i32 = hot_potatoes - 10;
                let calculation = Calculation::of_4(
                    Type::FumingPotatoBook,
                    "FUMING_POTATO_BOOK".to_string(),
                    prices("FUMING_POTATO_BOOK")
                        * fuming_potato_books_used as f64
                        * modifiers.regular().get_double("fumingPotatoBook"),
                    fuming_potato_books_used,
                );

                price += calculation.price();
                calculations.push(calculation);
            }

            //Regular Potato Books
            let hot_potato_books_used: i32 = hot_potatoes.min(10);
            let calculation = Calculation::of_4(
                Type::HotPotatoBook,
                "HOT_POTATO_BOOK".to_string(),
                prices("HOT_POTATO_BOOK") * hot_potato_books_used as f64 * modifiers.regular().get_double("hotPotatoBook"),
                hot_potato_books_used,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Dyes
        if let Some(dye) = meta.cosmetics().dye() {
            let calculation = Calculation::of(Type::Dye, dye.clone(), prices(dye) * modifiers.regular().get_double("dye"));

            price += calculation.price();
            calculations.push(calculation);
        }

        //Art of War
        if let Some(count) = meta.gear_upgrades().art_of_wars() {
            let calculation = Calculation::of_4(
                Type::ArtOfWar,
                "THE_ART_OF_WAR".to_string(),
                prices("THE_ART_OF_WAR") * count as f64 * modifiers.regular().get_double("artOfWar"),
                count,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Art of Peace
        if let Some(count) = meta.gear_upgrades().art_of_peaces() {
            let calculation = Calculation::of_4(
                Type::ArtOfPeace,
                "THE_ART_OF_PEACE".to_string(),
                prices("THE_ART_OF_PEACE") * count as f64 * modifiers.regular().get_double("artOfPeace"),
                count,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Farming for Dummies
        if let Some(count) = meta.misc_modifiers().farming4_dummies() {
            let calculation = Calculation::of_4(
                Type::FarmingForDummies,
                "FARMING_FOR_DUMMIES".to_string(),
                prices("FARMING_FOR_DUMMIES") * count as f64 * modifiers.regular().get_double("farming4Dummies"),
                count,
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Enrichments
        if meta.accessory_upgrades().enrichment().is_some() {
            //Since all enrichments are the same bits price, find the cheapest one to avoid inflated prices
            // java: ENRICHMENTS.stream().mapToDouble(prices::applyAsDouble).min().getAsDouble() - the set is never empty
            let enrichment_price: f64 = item_constants::ENRICHMENTS
                .iter()
                .map(|id| prices(id))
                .fold(None, |min: Option<f64>, p| Some(match min { Some(m) => m.min(p), None => p }))
                .expect("NoSuchElementException");
            let calculation = Calculation::of(
                Type::TalismanEnrichment,
                "TALISMAN_ENRICHMENT".to_string(),
                enrichment_price * modifiers.regular().get_double("enrichment"),
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Recombobulators - ignore dungeon mob drop items
        if meta.rarity_upgrades() > 0 && meta.dungeon_upgrades().item_tier().is_none() {
            //SkyHelper has extra filtering, I don't know why it matters but I won't use it for now
            let calculation = Calculation::of_4(
                Type::Recombobulator,
                "RECOMBOBULATOR_3000".to_string(),
                prices("RECOMBOBULATOR_3000")
                    * meta.rarity_upgrades() as f64
                    * modifiers.regular().get_double("recombobulator"),
                meta.rarity_upgrades(),
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Gemstones
        if (!meta.gemstone_slots().is_empty() || !meta.gemstones().borrow().is_empty()) && skyblock_item_data.is_some() {
            let skyblock_item_data: &SkyblockItemData = skyblock_item_data.unwrap();

            if let Some(item_gemstone_slots) = skyblock_item_data.gemstone_slots() {
                let mut unlocked_slots_on_item: Vec<String> = meta.gemstone_slots().clone();
                let mut unlocked_slots: Vec<String> = Vec::new();
                let mut applied_gemstones: Vec<AppliedGemstone> = Vec::new();

                for slot in item_gemstone_slots.iter() {
                    // java: !slot.costs().isEmpty() - the Optional is present
                    if slot.costs().is_some() && !unlocked_slots_on_item.is_empty() {
                        // java: unlockedSlotsOnItem.remove(type) - the first equal element, which is this one
                        if let Some(index) = unlocked_slots_on_item.iter().position(|r#type| r#type.starts_with(slot.r#type())) {
                            unlocked_slots.push(slot.r#type().to_string());
                            unlocked_slots_on_item.remove(index);
                        }
                    }

                    if slot.costs().is_none() {
                        unlocked_slots.push(slot.r#type().to_string());
                    }

                    let key: Option<String> = meta
                        .gemstones()
                        .borrow()
                        .iter()
                        .map(|(k, _)| k)
                        .find(|k| k.starts_with(slot.r#type()) && !k.ends_with("_gem"))
                        .cloned();

                    if let Some(key) = key {
                        let gemstones = meta.gemstones().borrow();
                        let gemstone_type: String = if item_constants::GEMSTONE_GROUP_SLOT_TYPES.contains(slot.r#type()) {
                            // java: meta.gemstones().get(key + "_gem").left().get()
                            let gem_key = format!("{key}_gem");
                            gemstones
                                .iter()
                                .find(|(k, _)| *k == gem_key)
                                .map(|(_, v)| v)
                                .expect("NullPointerException")
                                .left_ref()
                                .expect("NoSuchElementException")
                                .clone()
                        } else {
                            slot.r#type().to_string()
                        };
                        let gemstone: &Either<String, super::item::gemstone::Gemstone> = gemstones
                            .iter()
                            .find(|(k, _)| *k == key)
                            .map(|(_, v)| v)
                            .expect("NullPointerException");
                        let gemstone_quality: String = match gemstone.right_ref() {
                            Some(right) => right.quality().to_string(),
                            None => gemstone.left_ref().expect("NoSuchElementException").clone(),
                        };
                        drop(gemstones);

                        applied_gemstones.push(AppliedGemstone::new(gemstone_type, gemstone_quality, slot.r#type().to_string()));
                        meta.gemstones().borrow_mut().retain(|(k, _)| *k != key);

                        if slot.costs().is_some() && meta.gemstone_slots().is_empty() {
                            unlocked_slots.push(slot.r#type().to_string());
                        }
                    }
                }

                //Unlocked Gemstone Slots
                let is_divans_piece: bool = item_constants::DIVAN_GEAR.contains(item_id.as_str());

                if is_divans_piece || item_constants::KUUDRA_ARMOUR.contains(item_id.as_str()) {
                    let application_value: f64 = if is_divans_piece {
                        modifiers.regular().get_double("gemstoneChambers")
                    } else {
                        modifiers.regular().get_double("gemstoneSlots")
                    };
                    let mut gemstone_slots: Vec<GemstoneSlot> = item_gemstone_slots.clone();

                    for unlocked_slot in unlocked_slots.iter() {
                        let slot_index: Option<usize> = gemstone_slots.iter().position(|s| s.r#type() == unlocked_slot);

                        if let Some(slot_index) = slot_index {
                            let mut total: i32 = 0;

                            for cost in gemstone_slots[slot_index].costs().map(|c| c.as_slice()).unwrap_or(&[]) {
                                match cost.r#type() {
                                    "COINS" => {
                                        total = total.wrapping_add(cost.coins().expect("NoSuchElementException"));
                                    }
                                    "ITEM" => {
                                        // java: int total += double - the compound assignment narrows (d2i)
                                        total = (total as f64
                                            + prices(cost.item_id().expect("NoSuchElementException"))
                                                * cost.amount().expect("NoSuchElementException") as f64)
                                            as i32;
                                    }
                                    _ => {}
                                }
                            }

                            let calculation = Calculation::of(
                                Type::GemstoneSlot,
                                "GEMSTONE_SLOT".to_string(),
                                total as f64 * application_value,
                            );

                            price += calculation.price();
                            calculations.push(calculation);

                            gemstone_slots.remove(slot_index);
                        }
                    }
                }

                //Gemstones
                for gemstone in applied_gemstones.iter() {
                    let gemstone_id: String =
                        format!("{}_{}_GEM", gemstone.quality(), gemstone.gemstone_type()).to_uppercase();
                    let calculation_price = prices(&gemstone_id) * modifiers.regular().get_double("gemstones");
                    let calculation = Calculation::of(Type::Gemstone, gemstone_id, calculation_price);

                    price += calculation.price();
                    calculations.push(calculation);
                }
            }
        }

        //Power Scrolls
        if let Some(power_scroll) = meta.gear_upgrades().power_scroll() {
            let calculation = Calculation::of(
                Type::PowerScroll,
                power_scroll.clone(),
                prices(power_scroll) * modifiers.regular().get_double("powerScroll"),
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //Reforges
        if let Some(reforge) = meta.reforge() {
            let is_accessory: bool = skyblock_item_data.is_some()
                && skyblock_item_data.unwrap().category().is_some()
                && skyblock_item_data.unwrap().category().map(|s| s.as_str()).unwrap_or("") == "ACCESSORY";

            //Accessory reforges do not count anymore
            if !is_accessory {
                // java: prices.applyAsDouble(ItemConstants.REFORGES.get(reforge)) - a `null` id for an unknown reforge (see the header)
                let reforge_item: &str = item_constants::REFORGES.get(reforge.as_str()).copied().unwrap_or("");
                let calculation = Calculation::of(
                    Type::Reforge,
                    reforge.clone(),
                    prices(reforge_item) * modifiers.regular().get_double("reforge"),
                );

                price += calculation.price();
                calculations.push(calculation);
            }
        }

        //Stars
        //In a separate block to prevent leaking the LVs
        {
            let dungeon_item_level: i32 = meta.dungeon_upgrades().item_level().unwrap_or(0);
            let upgrade_level: i32 = meta.upgrade_level();
            let has_upgrade_costs: bool =
                skyblock_item_data.is_some() && skyblock_item_data.unwrap().upgrade_costs().is_some();

            //Master Stars
            if has_upgrade_costs && (dungeon_item_level > 5 || upgrade_level > 5) {
                let stars_used_dungeons: i32 = dungeon_item_level - 5;
                let stars_used_upgrade: i32 = upgrade_level - 5;
                let stars_used: i32 = stars_used_dungeons.max(stars_used_upgrade);
                let upgrade_costs: &Vec<Vec<GearUpgrade>> = skyblock_item_data.unwrap().upgrade_costs().unwrap();

                if upgrade_costs.len() <= 5 {
                    for i in 0..stars_used {
                        // java: MASTER_STARS.get(i) - IndexOutOfBoundsException past the fifth
                        let master_star_id: &str = item_constants::MASTER_STARS[i as usize];
                        let calculation = Calculation::of(
                            Type::MasterStar,
                            master_star_id.to_string(),
                            prices(master_star_id) * modifiers.regular().get_double("masterStars"),
                        );

                        price += calculation.price();
                        calculations.push(calculation);
                    }
                }
            }

            if has_upgrade_costs && (dungeon_item_level > 0 || upgrade_level > 0) {
                let level: i32 = dungeon_item_level.max(upgrade_level);
                let upgrade_costs: &Vec<Vec<GearUpgrade>> = skyblock_item_data.unwrap().upgrade_costs().unwrap();
                let entries: Vec<UpgradeCost> = upgrade_costs[0..((level + 1) as usize).min(upgrade_costs.len())]
                    .iter()
                    .map(UpgradeCost::List)
                    .collect();

                price += ItemCalculator::upgrade_costs(prices, &mut calculations, &entries, None, modifiers);
            }
        }

        //Wither Blade Scrolls
        if let Some(ability_scrolls) = meta.dungeon_upgrades().ability_scrolls() {
            for scroll in ability_scrolls.iter() {
                let calculation = Calculation::of(
                    Type::WitherBladeScroll,
                    scroll.clone(),
                    prices(scroll) * modifiers.regular().get_double("witherScroll"),
                );

                price += calculation.price();
                calculations.push(calculation);
            }
        }

        //Drills
        if meta.drill_info().is_any_drill_part_present() {
            if let Some(fuel_tank) = meta.drill_info().fuel_tank() {
                let calculation = Calculation::of(
                    Type::DrillPart,
                    fuel_tank.to_uppercase(),
                    prices(&fuel_tank.to_uppercase()) * modifiers.regular().get_double("drillPart"),
                );

                price += calculation.price();
                calculations.push(calculation);
            }

            if let Some(engine) = meta.drill_info().engine() {
                let calculation = Calculation::of(
                    Type::DrillPart,
                    engine.to_uppercase(),
                    prices(&engine.to_uppercase()) * modifiers.regular().get_double("drillPart"),
                );

                price += calculation.price();
                calculations.push(calculation);
            }

            if let Some(upgrade_module) = meta.drill_info().upgrade_module() {
                let calculation = Calculation::of(
                    Type::DrillPart,
                    upgrade_module.to_uppercase(),
                    prices(&upgrade_module.to_uppercase()) * modifiers.regular().get_double("drillPart"),
                );

                price += calculation.price();
                calculations.push(calculation);
            }

            //Polarvoid Books
            if meta.drill_info().polarvoids() > 0 {
                let calculation = Calculation::of_4(
                    Type::PolarvoidBook,
                    "POLARVOID_BOOK".to_string(),
                    prices("POLARVOID_BOOK")
                        * meta.drill_info().polarvoids() as f64
                        * modifiers.regular().get_double("polarvoidBook"),
                    meta.drill_info().polarvoids(),
                );

                price += calculation.price();
                calculations.push(calculation);
            }

            if let Some(divan_powder_coating) = meta.drill_info().divan_powder_coating() {
                let calculation = Calculation::of(
                    Type::DivanPowderCoating,
                    "DIVAN_POWDER_COATING".to_string(),
                    prices("DIVAN_POWDER_COATING")
                        * divan_powder_coating as f64
                        * modifiers.regular().get_double("divanPowderCoating"),
                );

                price += calculation.price();
                calculations.push(calculation);
            }
        }

        //Etherwarp Conduit
        if meta.misc_modifiers().ethermerge().is_some() {
            let calculation = Calculation::of(
                Type::EtherwarpConduit,
                "ETHERWARP_CONDUIT".to_string(),
                prices("ETHERWARP_CONDUIT") * modifiers.regular().get_double("etherwarp"),
            );

            price += calculation.price();
            calculations.push(calculation);
        }

        //New Year Cake Bag
        if item_id == "NEW_YEAR_CAKE_BAG" {
            let mut cakes_price: f64 = 0.0;

            for year in meta.cake_bag_cake_years().iter() {
                cakes_price += prices(&format!("NEW_YEAR_CAKE_{year}"));
            }

            let calculation = Calculation::of(Type::NewYearCakes, "NEW_YEAR_CAKES".to_string(), cakes_price);

            price += calculation.price();
            calculations.push(calculation);
        }

        NetworthResult::of(price, base, calculations)
    }

    /// java: private static <T> double upgradeCosts(ToDoubleFunction<String> prices, List<Calculation> calculations,
    /// List<T> upgradeCosts, String prestigeItem, ModifierValues modifiers)
    fn upgrade_costs(
        prices: &dyn Fn(&str) -> f64,
        calculations: &mut Vec<Calculation>,
        upgrade_costs: &[UpgradeCost],
        prestige_item: Option<&str>,
        modifiers: &ModifierValues,
    ) -> f64 {
        let mut price: f64 = 0.0;
        let mut star: i32 = 0;
        let mut gear_calculations: Vec<Calculation> = Vec::new();

        for upgrade in upgrade_costs.iter() {
            star += 1;

            match upgrade {
                UpgradeCost::List(upgrades_list) => {
                    for gear_upgrade in upgrades_list.iter() {
                        let calculation = ItemCalculator::upgrade_cost(prices, gear_upgrade, star, modifiers);
                        gear_calculations.push(calculation.clone());

                        if prestige_item.is_none() {
                            price += calculation.price();
                            calculations.push(calculation);
                        }
                    }
                }
                UpgradeCost::Single(gear_upgrade) => {
                    //Check should never fail if it comes to this branch
                    let calculation = ItemCalculator::upgrade_cost(prices, gear_upgrade, 0, modifiers);
                    gear_calculations.push(calculation.clone());

                    if prestige_item.is_none() {
                        price += calculation.price();
                        calculations.push(calculation);
                    }
                }
            }
        }

        if let Some(prestige_item) = prestige_item
            && !gear_calculations.is_empty()
        {
            let is_prestige: bool = gear_calculations[0].r#type() == Type::Prestige;
            let price_sum: f64 = gear_calculations.iter().map(|c| c.price()).sum(); //Reduce calculations to one single number
            let mut reduced = Calculation::of_4(
                if is_prestige { Type::Prestige } else { Type::Star },
                prestige_item.to_string(),
                price_sum,
                if is_prestige { 1 } else { star },
            );
            let prestige_item_price: f64 = prices(prestige_item);

            if is_prestige && prestige_item_price != 0.0 {
                reduced = reduced.add(prestige_item_price);
            }

            price += reduced.price();
            calculations.push(reduced);
        }

        price
    }

    /// java: private static Calculation upgradeCost(ToDoubleFunction<String> prices, GearUpgrade upgrade, int star, ModifierValues modifiers)
    fn upgrade_cost(
        prices: &dyn Fn(&str) -> f64,
        upgrade: &GearUpgrade,
        star: i32,
        modifiers: &ModifierValues,
    ) -> Calculation {
        let is_essence_upgrade: bool = upgrade.essence_type().is_some();
        let upgrade_price: f64 = if is_essence_upgrade {
            prices(&format!("ESSENCE_{}", upgrade.essence_type().unwrap()))
        } else {
            prices(upgrade.item_id().map(|s| s.as_str()).unwrap_or(""))
        };

        Calculation::of_4(
            if star == 0 { Type::Prestige } else { Type::Star },
            if is_essence_upgrade {
                format!("ESSENCE_{}", upgrade.essence_type().unwrap())
            } else {
                upgrade.item_id().expect("NoSuchElementException").clone()
            },
            upgrade.amount().unwrap_or(0) as f64
                * upgrade_price
                * if upgrade.essence_type().is_some() { modifiers.regular().get_double("essence") } else { 1.0 },
            upgrade.amount().unwrap_or(0),
        )
    }
}

/// java: private record AppliedGemstone(String gemstoneType, String quality, String slotType)
#[derive(Clone, Debug, PartialEq)]
struct AppliedGemstone {
    gemstone_type: String,
    quality: String,
    slot_type: String,
}

impl AppliedGemstone {
    /// java: the canonical record constructor
    fn new(gemstone_type: String, quality: String, slot_type: String) -> Self {
        Self {
            gemstone_type,
            quality,
            slot_type,
        }
    }

    /// java: the record accessor `gemstoneType()`
    fn gemstone_type(&self) -> &str {
        &self.gemstone_type
    }

    /// java: the record accessor `quality()`
    fn quality(&self) -> &str {
        &self.quality
    }

    /// java: the record accessor `slotType()`
    #[allow(dead_code)]
    fn slot_type(&self) -> &str {
        &self.slot_type
    }
}
