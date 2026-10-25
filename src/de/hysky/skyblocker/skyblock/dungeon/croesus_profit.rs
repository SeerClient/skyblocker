//! Ported from de.hysky.skyblocker.skyblock.dungeon.CroesusProfit by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::chest_value;
use crate::de::hysky::skyblocker::skyblock::chest_value::ChestValue;
use crate::de::hysky::skyblocker::skyblock::hunting::attributes::Attributes;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final CroesusProfit INSTANCE = new CroesusProfit();
pub static INSTANCE: compat::MainThread<compat::Shared<CroesusProfit>> = compat::MainThread::new(|| CroesusProfit::new());
/// java: private static final Pattern DUNGEON_CHEST_PATTERN = Pattern.compile("^(?:Wood|Gold|Diamond|Emerald|Obsidian|Bedrock)$");
pub static DUNGEON_CHEST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?:Wood|Gold|Diamond|Emerald|Obsidian|Bedrock)$"));
/// java: private static final Pattern KUUDRA_CHEST_PATTERN = Pattern.compile("^(?:Free|Paid) Chest$");
pub static KUUDRA_CHEST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?:Free|Paid) Chest$"));
/// java: private static final Map<String, String> DUNGEON_DROPS_NAME_TO_API_ID = Map.ofEntries( Map.entry("Enchanted Book (Ultimate Jerry I)", "ENCHANTMENT_ULTIMATE_JERRY_1"), // ultimate books start Map.entry
pub static DUNGEON_DROPS_NAME_TO_API_ID: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("Enchanted Book (Ultimate Jerry I)".to_string(), "ENCHANTMENT_ULTIMATE_JERRY_1".to_string()), ("Enchanted Book (Ultimate Jerry II)".to_string(), "ENCHANTMENT_ULTIMATE_JERRY_2".to_string()), ("Enchanted Book (Ultimate Jerry III)".to_string(), "ENCHANTMENT_ULTIMATE_JERRY_3".to_string()), ("Enchanted Book (Bank I)".to_string(), "ENCHANTMENT_ULTIMATE_BANK_1".to_string()), ("Enchanted Book (Bank II)".to_string(), "ENCHANTMENT_ULTIMATE_BANK_2".to_string()), ("Enchanted Book (Bank III)".to_string(), "ENCHANTMENT_ULTIMATE_BANK_3".to_string()), ("Enchanted Book (Combo I)".to_string(), "ENCHANTMENT_ULTIMATE_COMBO_1".to_string()), ("Enchanted Book (Combo II)".to_string(), "ENCHANTMENT_ULTIMATE_COMBO_2".to_string()), ("Enchanted Book (No Pain No Gain I)".to_string(), "ENCHANTMENT_ULTIMATE_NO_PAIN_NO_GAIN_1".to_string()), ("Enchanted Book (No Pain No Gain II)".to_string(), "ENCHANTMENT_ULTIMATE_NO_PAIN_NO_GAIN_2".to_string()), ("Enchanted Book (Ultimate Wise I)".to_string(), "ENCHANTMENT_ULTIMATE_WISE_1".to_string()), ("Enchanted Book (Ultimate Wise II)".to_string(), "ENCHANTMENT_ULTIMATE_WISE_2".to_string()), ("Enchanted Book (Wisdom I)".to_string(), "ENCHANTMENT_ULTIMATE_WISDOM_1".to_string()), ("Enchanted Book (Wisdom II)".to_string(), "ENCHANTMENT_ULTIMATE_WISDOM_2".to_string()), ("Enchanted Book (Last Stand I)".to_string(), "ENCHANTMENT_ULTIMATE_LAST_STAND_1".to_string()), ("Enchanted Book (Last Stand II)".to_string(), "ENCHANTMENT_ULTIMATE_LAST_STAND_2".to_string()), ("Enchanted Book (Rend I)".to_string(), "ENCHANTMENT_ULTIMATE_REND_1".to_string()), ("Enchanted Book (Rend II)".to_string(), "ENCHANTMENT_ULTIMATE_REND_2".to_string()), ("Enchanted Book (Legion I)".to_string(), "ENCHANTMENT_ULTIMATE_LEGION_1".to_string()), ("Enchanted Book (Swarm I)".to_string(), "ENCHANTMENT_ULTIMATE_SWARM_1".to_string()), ("Enchanted Book (One For All I)".to_string(), "ENCHANTMENT_ULTIMATE_ONE_FOR_ALL_1".to_string()), ("Enchanted Book (Soul Eater I)".to_string(), "ENCHANTMENT_ULTIMATE_SOUL_EATER_1".to_string()), ("Enchanted Book (Infinite Quiver VI)".to_string(), "ENCHANTMENT_INFINITE_QUIVER_6".to_string()), ("Enchanted Book (Infinite Quiver VII)".to_string(), "ENCHANTMENT_INFINITE_QUIVER_7".to_string()), ("Enchanted Book (Feather Falling VI)".to_string(), "ENCHANTMENT_FEATHER_FALLING_6".to_string()), ("Enchanted Book (Feather Falling VII)".to_string(), "ENCHANTMENT_FEATHER_FALLING_7".to_string()), ("Enchanted Book (Rejuvenate I)".to_string(), "ENCHANTMENT_REJUVENATE_1".to_string()), ("Enchanted Book (Rejuvenate II)".to_string(), "ENCHANTMENT_REJUVENATE_2".to_string()), ("Enchanted Book (Rejuvenate III)".to_string(), "ENCHANTMENT_REJUVENATE_3".to_string()), ("Enchanted Book (Overload I)".to_string(), "ENCHANTMENT_OVERLOAD_1".to_string()), ("Enchanted Book (Lethality VI)".to_string(), "ENCHANTMENT_LETHALITY_6".to_string()), ("Enchanted Book (Thunderlord VII)".to_string(), "ENCHANTMENT_THUNDERLORD_7".to_string()), ("Hot Potato Book".to_string(), "HOT_POTATO_BOOK".to_string()), ("Fuming Potato Book".to_string(), "FUMING_POTATO_BOOK".to_string()), ("Recombobulator 3000".to_string(), "RECOMBOBULATOR_3000".to_string()), ("Necromancer's Brooch".to_string(), "NECROMANCER_BROOCH".to_string()), ("Bonzo's Staff".to_string(), "BONZO_STAFF".to_string()), ("Master Skull - Tier 1".to_string(), "MASTER_SKULL_TIER_1".to_string()), ("Bonzo's Mask".to_string(), "BONZO_MASK".to_string()), ("Balloon Snake".to_string(), "BALLOON_SNAKE".to_string()), ("Red Nose".to_string(), "RED_NOSE".to_string()), ("Red Scarf".to_string(), "RED_SCARF".to_string()), ("Adaptive Blade".to_string(), "STONE_BLADE".to_string()), ("Master Skull - Tier 2".to_string(), "MASTER_SKULL_TIER_2".to_string()), ("Adaptive Belt".to_string(), "ADAPTIVE_BELT".to_string()), ("Scarf's Studies".to_string(), "SCARF_STUDIES".to_string()), ("First Master Star".to_string(), "FIRST_MASTER_STAR".to_string()), ("Adaptive Helmet".to_string(), "ADAPTIVE_HELMET".to_string()), ("Adaptive Chestplate".to_string(), "ADAPTIVE_CHESTPLATE".to_string()), ("Adaptive Leggings".to_string(), "ADAPTIVE_LEGGINGS".to_string()), ("Adaptive Boots".to_string(), "ADAPTIVE_BOOTS".to_string()), ("Master Skull - Tier 3".to_string(), "MASTER_SKULL_TIER_3".to_string()), ("Suspicious Vial".to_string(), "SUSPICIOUS_VIAL".to_string()), ("Spirit Sword".to_string(), "SPIRIT_SWORD".to_string()), ("Spirit Shortbow".to_string(), "ITEM_SPIRIT_BOW".to_string()), ("Spirit Boots".to_string(), "THORNS_BOOTS".to_string()), ("Spirit Legendary".to_string(), "LVL_1_LEGENDARY_SPIRIT".to_string()), ("Spirit Epic".to_string(), "LVL_1_EPIC_SPIRIT".to_string()), ("Spirit Mask".to_string(), "SPIRIT_MASK".to_string()), ("Second Master Star".to_string(), "SECOND_MASTER_STAR".to_string()), ("Spirit Wing".to_string(), "SPIRIT_WING".to_string()), ("Spirit Bone".to_string(), "SPIRIT_BONE".to_string()), ("Spirit Stone".to_string(), "SPIRIT_DECOY".to_string()), ("Shadow Fury".to_string(), "SHADOW_FURY".to_string()), ("Last Breath".to_string(), "LAST_BREATH".to_string()), ("Third Master Star".to_string(), "THIRD_MASTER_STAR".to_string()), ("Warped Stone".to_string(), "AOTE_STONE".to_string()), ("Livid Dagger".to_string(), "LIVID_DAGGER".to_string()), ("Shadow Assassin Helmet".to_string(), "SHADOW_ASSASSIN_HELMET".to_string()), ("Shadow Assassin Chestplate".to_string(), "SHADOW_ASSASSIN_CHESTPLATE".to_string()), ("Shadow Assassin Leggings".to_string(), "SHADOW_ASSASSIN_LEGGINGS".to_string()), ("Shadow Assassin Boots".to_string(), "SHADOW_ASSASSIN_BOOTS".to_string()), ("Shadow Assassin Cloak".to_string(), "SHADOW_ASSASSIN_CLOAK".to_string()), ("Master Skull - Tier 4".to_string(), "MASTER_SKULL_TIER_4".to_string()), ("Dark Orb".to_string(), "DARK_ORB".to_string()), ("Precursor Eye".to_string(), "PRECURSOR_EYE".to_string()), ("Giant's Sword".to_string(), "GIANTS_SWORD".to_string()), ("Necromancer Lord Helmet".to_string(), "NECROMANCER_LORD_HELMET".to_string()), ("Necromancer Lord Chestplate".to_string(), "NECROMANCER_LORD_CHESTPLATE".to_string()), ("Necromancer Lord Leggings".to_string(), "NECROMANCER_LORD_LEGGINGS".to_string()), ("Necromancer Lord Boots".to_string(), "NECROMANCER_LORD_BOOTS".to_string()), ("Fourth Master Star".to_string(), "FOURTH_MASTER_STAR".to_string()), ("Summoning Ring".to_string(), "SUMMONING_RING".to_string()), ("Fel Skull".to_string(), "FEL_SKULL".to_string()), ("Necromancer Sword".to_string(), "NECROMANCER_SWORD".to_string()), ("Soulweaver Gloves".to_string(), "SOULWEAVER_GLOVES".to_string()), ("Sadan's Brooch".to_string(), "SADAN_BROOCH".to_string()), ("Giant Tooth".to_string(), "GIANT_TOOTH".to_string()), ("Precursor Gear".to_string(), "PRECURSOR_GEAR".to_string()), ("Necron Dye".to_string(), "DYE_NECRON".to_string()), ("Storm the Fish".to_string(), "STORM_THE_FISH".to_string()), ("Maxor the Fish".to_string(), "MAXOR_THE_FISH".to_string()), ("Goldor the Fish".to_string(), "GOLDOR_THE_FISH".to_string()), ("Dark Claymore".to_string(), "DARK_CLAYMORE".to_string()), ("Necron's Handle".to_string(), "NECRON_HANDLE".to_string()), ("Master Skull - Tier 5".to_string(), "MASTER_SKULL_TIER_5".to_string()), ("Shadow Warp".to_string(), "SHADOW_WARP_SCROLL".to_string()), ("Wither Shield".to_string(), "WITHER_SHIELD_SCROLL".to_string()), ("Implosion".to_string(), "IMPLOSION_SCROLL".to_string()), ("Fifth Master Star".to_string(), "FIFTH_MASTER_STAR".to_string()), ("Auto Recombobulator".to_string(), "AUTO_RECOMBOBULATOR".to_string()), ("Wither Helmet".to_string(), "WITHER_HELMET".to_string()), ("Wither Chestplate".to_string(), "WITHER_CHESTPLATE".to_string()), ("Wither Leggings".to_string(), "WITHER_LEGGINGS".to_string()), ("Wither Boots".to_string(), "WITHER_BOOTS".to_string()), ("Wither Catalyst".to_string(), "WITHER_CATALYST".to_string()), ("Wither Cloak Sword".to_string(), "WITHER_CLOAK".to_string()), ("Wither Blood".to_string(), "WITHER_BLOOD".to_string()), ("Shiny Wither Helmet".to_string(), "SHINY_WITHER_HELMET".to_string()), ("Shiny Wither Chestplate".to_string(), "SHINY_WITHER_CHESTPLATE".to_string()), ("Shiny Wither Leggings".to_string(), "SHINY_WITHER_LEGGINGS".to_string()), ("Shiny Wither Boots".to_string(), "SHINY_WITHER_BOOTS".to_string()), ("Shiny Necron's Handle".to_string(), "SHINY_NECRON_HANDLE".to_string()), ("Dungeon Disc".to_string(), "DUNGEON_DISC_1".to_string()), ("Clown Disc".to_string(), "DUNGEON_DISC_2".to_string()), ("Watcher Disc".to_string(), "DUNGEON_DISC_3".to_string()), ("Old Disc".to_string(), "DUNGEON_DISC_4".to_string()), ("Necron Disc".to_string(), "DUNGEON_DISC_5".to_string()), ("Scarf Shard".to_string(), "SHARD_SCARF".to_string()), ("Thorn Shard".to_string(), "SHARD_THORN".to_string()), ("Wither Shard".to_string(), "SHARD_WITHER".to_string()), ("Apex Dragon Shard".to_string(), "SHARD_APEX_DRAGON".to_string()), ("Power Dragon Shard".to_string(), "SHARD_POWER_DRAGON".to_string())]));
/// java: private static final Map<String, String> KUUDRA_DROPS_NAME_TO_API_ID = Map.ofEntries( // Armour Map.entry("Crimson Helmet", "CRIMSON_HELMET"), Map.entry("Crimson Chestplate", "CRIMSON_CHESTPLATE"), Ma
pub static KUUDRA_DROPS_NAME_TO_API_ID: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("Crimson Helmet".to_string(), "CRIMSON_HELMET".to_string()), ("Crimson Chestplate".to_string(), "CRIMSON_CHESTPLATE".to_string()), ("Crimson Leggings".to_string(), "CRIMSON_LEGGINGS".to_string()), ("Crimson Boots".to_string(), "CRIMSON_BOOTS".to_string()), ("Aurora Helmet".to_string(), "AURORA_HELMET".to_string()), ("Aurora Chestplate".to_string(), "AURORA_CHESTPLATE".to_string()), ("Aurora Leggings".to_string(), "AURORA_LEGGINGS".to_string()), ("Aurora Boots".to_string(), "AURORA_BOOTS".to_string()), ("Terror Helmet".to_string(), "TERROR_HELMET".to_string()), ("Terror Chestplate".to_string(), "TERROR_CHESTPLATE".to_string()), ("Terror Leggings".to_string(), "TERROR_LEGGINGS".to_string()), ("Terror Boots".to_string(), "TERROR_BOOTS".to_string()), ("Hollow Helmet".to_string(), "HOLLOW_HELMET".to_string()), ("Hollow Chestplate".to_string(), "HOLLOW_CHESTPLATE".to_string()), ("Hollow Leggings".to_string(), "HOLLOW_LEGGINGS".to_string()), ("Hollow Boots".to_string(), "HOLLOW_BOOTS".to_string()), ("Fervor Helmet".to_string(), "FERVOR_HELMET".to_string()), ("Fervor Chestplate".to_string(), "FERVOR_CHESTPLATE".to_string()), ("Fervor Leggings".to_string(), "FERVOR_LEGGINGS".to_string()), ("Fervor Boots".to_string(), "FERVOR_BOOTS".to_string()), ("Molten Necklace".to_string(), "MOLTEN_NECKLACE".to_string()), ("Molten Cloak".to_string(), "MOLTEN_CLOAK".to_string()), ("Molten Belt".to_string(), "MOLTEN_BELT".to_string()), ("Molten Bracelet".to_string(), "MOLTEN_BRACELET".to_string()), ("Aurora Staff".to_string(), "RUNIC_STAFF".to_string()), ("Hollow Wand".to_string(), "HOLLOW_WAND".to_string()), ("Tormentor".to_string(), "TORMENTOR".to_string()), ("Hellstorm Wand".to_string(), "HELLSTORM_STAFF".to_string()), ("Enrager".to_string(), "ENRAGER".to_string()), ("Enchanted Book (Fatal Tempo I)".to_string(), "ENCHANTMENT_ULTIMATE_FATAL_TEMPO_1".to_string()), ("Enchanted Book (Inferno I)".to_string(), "ENCHANTMENT_ULTIMATE_INFERNO_1".to_string()), ("Enchanted Book (Strong Vitality I)".to_string(), "ENCHANTMENT_STRONG_MANA_1".to_string()), ("Enchanted Book (Strong Vitality II)".to_string(), "ENCHANTMENT_STRONG_MANA_2".to_string()), ("Enchanted Book (Strong Vitality III)".to_string(), "ENCHANTMENT_STRONG_MANA_3".to_string()), ("Enchanted Book (Strong Vitality IV)".to_string(), "ENCHANTMENT_STRONG_MANA_4".to_string()), ("Enchanted Book (Strong Vitality V)".to_string(), "ENCHANTMENT_STRONG_MANA_5".to_string()), ("Enchanted Book (Vivacious Vitality I)".to_string(), "ENCHANTMENT_FEROCIOUS_MANA_1".to_string()), ("Enchanted Book (Vivacious Vitality II)".to_string(), "ENCHANTMENT_FEROCIOUS_MANA_2".to_string()), ("Enchanted Book (Vivacious Vitality III)".to_string(), "ENCHANTMENT_FEROCIOUS_MANA_3".to_string()), ("Enchanted Book (Vivacious Vitality IV)".to_string(), "ENCHANTMENT_FEROCIOUS_MANA_4".to_string()), ("Enchanted Book (Vivacious Vitality V)".to_string(), "ENCHANTMENT_FEROCIOUS_MANA_5".to_string()), ("Enchanted Book (Hardened Vitality I)".to_string(), "ENCHANTMENT_HARDENED_MANA_1".to_string()), ("Enchanted Book (Hardened Vitality II)".to_string(), "ENCHANTMENT_HARDENED_MANA_2".to_string()), ("Enchanted Book (Hardened Vitality III)".to_string(), "ENCHANTMENT_HARDENED_MANA_3".to_string()), ("Enchanted Book (Hardened Vitality IV)".to_string(), "ENCHANTMENT_HARDENED_MANA_4".to_string()), ("Enchanted Book (Hardened Vitality V)".to_string(), "ENCHANTMENT_HARDENED_MANA_5".to_string()), ("Enchanted Book (Vampiric Vitality I)".to_string(), "ENCHANTMENT_MANA_VAMPIRE_1".to_string()), ("Enchanted Book (Vampiric Vitality II)".to_string(), "ENCHANTMENT_MANA_VAMPIRE_2".to_string()), ("Enchanted Book (Vampiric Vitality III)".to_string(), "ENCHANTMENT_MANA_VAMPIRE_3".to_string()), ("Enchanted Book (Vampiric Vitality IV)".to_string(), "ENCHANTMENT_MANA_VAMPIRE_4".to_string()), ("Enchanted Book (Vampiric Vitality V)".to_string(), "ENCHANTMENT_MANA_VAMPIRE_5".to_string()), ("Dusty Travel Scroll to the Kuudra Skull".to_string(), "NETHER_FORTRESS_BOSS_TRAVEL_SCROLL".to_string()), ("Mandraa".to_string(), "MANDRAA".to_string()), ("Kuudra Mandible".to_string(), "KUUDRA_MANDIBLE".to_string()), ("Kuudra Tentacle".to_string(), "KUUDRA_TENTACLE".to_string()), ("Burning Kuudra Core".to_string(), "BURNING_KUUDRA_CORE".to_string()), ("Wheel of Fate".to_string(), "WHEEL_OF_FATE".to_string()), ("Ananke Feather".to_string(), "ANANKE_FEATHER".to_string()), ("Tentacle Dye".to_string(), "TENTACLE_DYE".to_string())]));

/// java: public class CroesusProfit extends SimpleContainerSolver implements TooltipAdder
#[derive(Clone)]
pub struct CroesusProfit {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
}

impl std::ops::Deref for CroesusProfit {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CroesusProfit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CroesusProfit {
    /// java: private CroesusProfit()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("(?:(?:Master )?Catacombs - Flo.*)|(?:Kuudra - .*)".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

    /// java: @SuppressWarnings("deprecation") private DoubleBooleanPair getChestValue(ItemStack chest)
    pub fn get_chest_value(__this: &compat::Shared<Self>, chest: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> (f64, bool) {
        let mut chest_value: f64 = ((0) as f64) as f64;
        let mut chest_price: f64 = ((0) as f64) as f64;
        let mut has_incomplete_data: bool = false;
        let mut processing_contents: bool = false;
        for line in ItemUtils::get_lore(chest) {
            let mut line_string: String = chat_formatting::strip_formatting(&line.get_string());
            if true && line_string.contains("Contents") {
                processing_contents = true;
                continue;
            } else if true && line_string.is_empty() {
                processing_contents = false;
            } else if true && line_string.ends_with("Coins") {
                if !processing_contents {
                    let mut chest_cost: String = compat::regex::replace_all(&(line_string.replace(",", "")), "[^\\d.]", "");
                    if !is_creatable(&chest_cost) {
                        continue;
                    }
                    chest_price = chest_cost.trim().parse::<f64>().expect("NumberFormatException");
                }
            } else if true && line_string.ends_with("Kuudra Key") {
                if !processing_contents {
                    let mut trimmed: String = compat::trim(&line_string);
                    chest_price = ChestValue::compute_kuudra_key_price(&trimmed).0;
                }
            } else {
            }
            if processing_contents {
                if true && line_string.contains("Essence") && SkyblockerConfigManager::get().dungeons.dungeon_chest_profit.include_essence {
                    let mut matcher: compat::regex::Matcher = chest_value::ESSENCE_PATTERN.matcher(&line_string);
                    if matcher.matches() {
                        let mut r#type: String = matcher.group_named("type").unwrap().to_uppercase();
                        let mut amount: i32 = RegexUtils::parse_optional_int_from_matcher_match_result_string(&matcher, "amount").unwrap_or(1);
                        if r#type == "CRIMSON" {
                            amount = amount.wrapping_mul((ChestValue::compute_crimson_essence_multiplier()) as i32);
                        }
                        let mut price_data: Option<f64> = CroesusProfit::get_item_price(__this, &format!("ESSENCE_{}", r#type));
                        if price_data.is_some() {
                            chest_value = (chest_value + (price_data.clone().unwrap() * (amount as f64)));
                        } else {
                            has_incomplete_data = true;
                        }
                    }
                } else if true && line_string.contains("Shard") {
                    let mut matcher: compat::regex::Matcher = chest_value::SHARD_PATTERN.matcher(&line_string);
                    if matcher.matches() {
                        let mut shards: i32 = RegexUtils::parse_optional_int_from_matcher_match_result_string(&matcher, "amount").unwrap_or(1);
                        let Some(mut attribute) = Attributes::get_attribute_from_item_name_string(line_string.clone()) else {
                            continue;
                        };
                        let mut price_data: Option<f64> = CroesusProfit::get_item_price(__this, &attribute.api_id());
                        if price_data.is_some() {
                            chest_value = (chest_value + (price_data.clone().unwrap() * (shards as f64)));
                        } else {
                            has_incomplete_data = true;
                        }
                    }
                } else if true && line_string.contains("Kuudra Teeth") {
                    let mut matcher: compat::regex::Matcher = chest_value::KUUDRA_TEETH_PATTERN.matcher(&line_string);
                    if matcher.matches() {
                        let mut amount: i32 = RegexUtils::parse_optional_int_from_matcher_match_result_string(&matcher, "amount").unwrap_or(1);
                        let mut price_data: Option<f64> = CroesusProfit::get_item_price(__this, "KUUDRA_TEETH");
                        if price_data.is_some() {
                            chest_value = (chest_value + (price_data.clone().unwrap() * (amount as f64)));
                        } else {
                            has_incomplete_data = true;
                        }
                    }
                } else if true && line_string.contains("Heavy Pearl") {
                    let mut matcher: compat::regex::Matcher = chest_value::HEAVY_PEARL_PATTERN.matcher(&line_string);
                    if matcher.matches() {
                        let mut amount: i32 = RegexUtils::parse_optional_int_from_matcher_match_result_string(&matcher, "amount").unwrap_or(1);
                        let mut price_data: Option<f64> = CroesusProfit::get_item_price(__this, "HEAVY_PEARL");
                        if price_data.is_some() {
                            chest_value = (chest_value + (price_data.clone().unwrap() * (amount as f64)));
                        } else {
                            has_incomplete_data = true;
                        }
                    }
                } else if true && line_string == "[Lvl 1] Spirit" {
                    let mut color: Option<TextColor> = line.get_style().get_color();
                    if color.is_some() && color.clone().unwrap() == TextColor::from_legacy_format(ChatFormatting::DarkPurple).unwrap() {
                        let mut price_data: Option<f64> = CroesusProfit::get_item_price(__this, "Spirit Epic");
                        if price_data.is_some() {
                            chest_value = (chest_value + price_data.clone().unwrap());
                        } else {
                            has_incomplete_data = true;
                        }
                    } else {
                        let mut price_data: Option<f64> = CroesusProfit::get_item_price(__this, "Spirit Legendary");
                        if price_data.is_some() {
                            chest_value = (chest_value + price_data.clone().unwrap());
                        } else {
                            has_incomplete_data = true;
                        }
                    }
                } else {
                    let mut adjusted: String = compat::trim(&(line_string.replace("✪", "")));
                    let mut price_data: Option<f64> = CroesusProfit::get_item_price(__this, &adjusted);
                    if price_data.is_some() {
                        chest_value = (chest_value + price_data.clone().unwrap());
                    } else {
                        has_incomplete_data = true;
                    }
                }
            }
        }
        return ((chest_value - chest_price), has_incomplete_data);
    }

    /// java: private OptionalDouble getItemPrice(String itemName)
    pub fn get_item_price(__this: &compat::Shared<Self>, item_name: &str) -> Option<f64> {
        let mut dungeon_api_id: Option<String> = DUNGEON_DROPS_NAME_TO_API_ID.get(item_name).cloned();
        let mut kuudra_api_id: Option<String> = KUUDRA_DROPS_NAME_TO_API_ID.get(item_name).cloned();
        let mut api_id_to_use: String = (if dungeon_api_id.is_some() { dungeon_api_id.clone().unwrap() } else { (if kuudra_api_id.is_some() { kuudra_api_id.clone() } else { Some(item_name.to_string()) }).unwrap() });
        if chest_value::WORTHLESS_ITEMS.contains(&api_id_to_use) {
            return None;
        }
        return ItemUtils::get_item_price_string(Some(api_id_to_use.clone()));
    }

}

impl SimpleContainerSolver for CroesusProfit {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for CroesusProfit {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase {
        &self.base.base
    }
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase {
        &mut self.base.base
    }

    /// java: @Override public Pattern getTitlePattern() (dispatch: the body for this class)
    fn get_title_pattern(&self) -> fn(&compat::Shared<dyn RegexContainerMatcher>) -> Option<compat::regex::Pattern> {
        <dyn SimpleContainerSolver>::get_title_pattern_from::<Self>
    }

}

impl ContainerMatcher for CroesusProfit {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        CroesusProfit::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for CroesusProfit {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        CroesusProfit::get_colors_dyn
    }

}

impl Resettable for CroesusProfit {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl TooltipAdder for CroesusProfit {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        CroesusProfit::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        CroesusProfit::get_priority_dyn
    }

}

impl CroesusProfit {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().dungeons.dungeon_chest_profit.croesus_profit;
    }

    /// CroesusProfit.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        let mut best_chest: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        let mut second_best_chest: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        let mut best_value: f64 = ((0) as f64) as f64;
        let mut second_best_value: f64 = ((0) as f64) as f64;
        let mut dungeon_key_price_data: f64 = (CroesusProfit::get_item_price(__this, "DUNGEON_CHEST_KEY").unwrap_or(((0) as f64) as f64) * (2 as f64));
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Some(entry.1.clone());
            let mut name: String = compat::component_of(stack.as_ref().unwrap().get_hover_name()).get_string();
            if DUNGEON_CHEST_PATTERN.matcher(&name).matches() {
                let mut value: f64 = CroesusProfit::get_chest_value(__this, stack.as_ref().unwrap()).0;
                if value <= (0 as f64) {
                    continue;
                }
                if value > best_value {
                    second_best_chest = best_chest.clone();
                    second_best_value = best_value;
                    best_chest = stack.clone();
                    best_value = value;
                } else if value > second_best_value {
                    second_best_chest = stack.clone();
                    second_best_value = value;
                }
            } else if KUUDRA_CHEST_PATTERN.matcher(&name).matches() {
                let mut value: f64 = CroesusProfit::get_chest_value(__this, stack.as_ref().unwrap()).0;
                if value <= (0 as f64) {
                    continue;
                }
                if value > best_value {
                    best_chest = stack.clone();
                    best_value = value;
                }
            }
        }
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Some(entry.1.clone());
            if let Some(stack) = &stack {
                if best_chest.as_ref().is_some_and(|b| stack.is_same_instance(b)) {
                    highlights.push(ColorHighlight::green(entry.0));
                } else if second_best_chest.as_ref().is_some_and(|b| stack.is_same_instance(b)) && second_best_value > dungeon_key_price_data {
                    highlights.push(ColorHighlight::yellow(entry.0));
                }
            }
        }
        return highlights;
    }

    /// CroesusProfit.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        if focused_slot.is_none() || !(compat::slot_has_item(focused_slot.as_ref().unwrap())) {
            return;
        }
        let focused_item = compat::slot_item(focused_slot.as_ref().unwrap());
        if !(focused_item.is_item(&items::items().get("player_head"))) {
            return;
        }
        let mut value_data: (f64, bool) = CroesusProfit::get_chest_value(__this, &focused_item);
        let mut value_string: MutableComponent = component::literal(&formatters::INTEGER_NUMBERS.get_mut().format(value_data.0));
        if value_data.1 {
            value_string = value_string.append_str(" ").append(component::translatable("skyblocker.dungeons.croesusHelper.incompletePriceData"));
        }
        lines.push((constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.dungeons.croesusHelper.chestValue", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(value_string.clone())])));
    }

    /// CroesusProfit.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public int getPriority()
    pub fn get_priority_impl(__this: &compat::Shared<Self>) -> i32 {
        return 16;
    }

    /// CroesusProfit.getPriority as the dispatch of TooltipAdder.getPriority
    pub fn get_priority_dyn(__this: &compat::Shared<dyn TooltipAdder>) -> i32 {
        Self::get_priority_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// commons-lang3 `NumberUtils.isCreatable(str)`, statement for statement (private: batch K helper).
fn is_creatable(str: &str) -> bool {
    if str.is_empty() {
        return false;
    }
    let chars: Vec<char> = str.chars().collect();
    let mut sz = chars.len();
    let mut has_exp = false;
    let mut has_dec_point = false;
    let mut allow_signs = false;
    let mut found_digit = false;
    let start = if chars[0] == '-' || chars[0] == '+' { 1 } else { 0 };
    if sz > start + 1 && chars[start] == '0' && !str.contains('.') {
        if chars[start + 1] == 'x' || chars[start + 1] == 'X' {
            let i = start + 2;
            if i == sz {
                return false;
            }
            return chars[i..].iter().all(|c| c.is_ascii_hexdigit());
        }
        if chars[start + 1].is_numeric() {
            return chars[start + 1..].iter().all(|c| ('0'..='7').contains(c));
        }
    }
    sz -= 1;
    let mut i = start;
    while i < sz || (i < sz + 1 && allow_signs && !found_digit) {
        let c = chars[i];
        if c.is_ascii_digit() {
            found_digit = true;
            allow_signs = false;
        } else if c == '.' {
            if has_dec_point || has_exp {
                return false;
            }
            has_dec_point = true;
        } else if c == 'e' || c == 'E' {
            if has_exp || !found_digit {
                return false;
            }
            has_exp = true;
            allow_signs = true;
        } else if c == '+' || c == '-' {
            if !allow_signs {
                return false;
            }
            allow_signs = false;
            found_digit = false;
        } else {
            return false;
        }
        i += 1;
    }
    if i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            return true;
        }
        if c == 'e' || c == 'E' {
            return false;
        }
        if c == '.' {
            if has_dec_point || has_exp {
                return false;
            }
            return found_digit;
        }
        if !allow_signs && (c == 'd' || c == 'D' || c == 'f' || c == 'F') {
            return found_digit;
        }
        if c == 'l' || c == 'L' {
            return found_digit && !has_exp && !has_dec_point;
        }
        return false;
    }
    !allow_signs && found_digit
}
