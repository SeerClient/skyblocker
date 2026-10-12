//! Ported from de.hysky.skyblocker.skyblock.ChestValue by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::dungeons_config::DungeonChestProfit;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::crimson::crimson_faction::CrimsonFaction;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra::Kuudra;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra_profile_data;
use crate::de::hysky::skyblocker::skyblock::hunting::attribute::Attribute;
use crate::de::hysky::skyblocker::skyblock::hunting::attributes::Attributes;
use crate::de::hysky::skyblocker::skyblock::hunting::hunting_box_helper;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::networth::networth_calculator::NetworthCalculator;
use crate::de::hysky::skyblocker::utils::regex_list_utils::RegexListUtils;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::string_widget;
use mcrust::net::minecraft::client::gui::components::string_widget::TextOverflow;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Set<String> DUNGEON_CHESTS = Set.of("Wood Chest", "Wood", "Gold Chest", "Gold", "Diamond Chest", "Diamond", "Emerald Chest", "Emerald", "Obsidian Chest", "Obsidian", "Bedrock Ches
pub static DUNGEON_CHESTS: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["Wood Chest".to_string(), "Wood".to_string(), "Gold Chest".to_string(), "Gold".to_string(), "Diamond Chest".to_string(), "Diamond".to_string(), "Emerald Chest".to_string(), "Emerald".to_string(), "Obsidian Chest".to_string(), "Obsidian".to_string(), "Bedrock Chest".to_string(), "Bedrock".to_string()]));
/// java: public static final Pattern DUNGEON_CHEST_COIN_COST_PATTERN = Pattern.compile("^([0-9,]+) Coins$");
pub static DUNGEON_CHEST_COIN_COST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^([0-9,]+) Coins$"));
/// java: private static final Set<String> KUUDRA_CHESTS = Set.of("Free Chest", "Free Chest Chest", "Paid Chest", "Paid Chest Chest");
pub static KUUDRA_CHESTS: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["Free Chest".to_string(), "Free Chest Chest".to_string(), "Paid Chest".to_string(), "Paid Chest Chest".to_string()]));
/// java: private static final Map<String, String> KUUDRA_KEYS = Map.of( "Kuudra Key", "KUUDRA_TIER_KEY", "Hot Kuudra Key", "KUUDRA_HOT_TIER_KEY", "Burning Kuudra Key", "KUUDRA_BURNING_TIER_KEY", "Fiery Kuudra 
pub static KUUDRA_KEYS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("Kuudra Key".to_string(), "KUUDRA_TIER_KEY".to_string()), ("Hot Kuudra Key".to_string(), "KUUDRA_HOT_TIER_KEY".to_string()), ("Burning Kuudra Key".to_string(), "KUUDRA_BURNING_TIER_KEY".to_string()), ("Fiery Kuudra Key".to_string(), "KUUDRA_FIERY_TIER_KEY".to_string()), ("Infernal Kuudra Key".to_string(), "KUUDRA_INFERNAL_TIER_KEY".to_string())]));
/// java: public static final Set<String> WORTHLESS_ITEMS = Set.of( "DUNGEON_DISC_1", "DUNGEON_DISC_2", "DUNGEON_DISC_3", "DUNGEON_DISC_4", "DUNGEON_DISC_5", "MAXOR_THE_FISH", "STORM_THE_FISH", "GOLDOR_THE_FISH
pub static WORTHLESS_ITEMS: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| std::collections::HashSet::from(["DUNGEON_DISC_1".to_string(), "DUNGEON_DISC_2".to_string(), "DUNGEON_DISC_3".to_string(), "DUNGEON_DISC_4".to_string(), "DUNGEON_DISC_5".to_string(), "MAXOR_THE_FISH".to_string(), "STORM_THE_FISH".to_string(), "GOLDOR_THE_FISH".to_string(), "ENCHANTMENT_ULTIMATE_NO_PAIN_NO_GAIN_1".to_string(), "ENCHANTMENT_ULTIMATE_NO_PAIN_NO_GAIN_2".to_string(), "ENCHANTMENT_ULTIMATE_NO_PAIN_NO_GAIN_3".to_string(), "ENCHANTMENT_ULTIMATE_NO_PAIN_NO_GAIN_4".to_string(), "ENCHANTMENT_ULTIMATE_NO_PAIN_NO_GAIN_5".to_string(), "ENCHANTMENT_ULTIMATE_COMBO_1".to_string(), "ENCHANTMENT_ULTIMATE_COMBO_2".to_string(), "ENCHANTMENT_ULTIMATE_COMBO_3".to_string(), "ENCHANTMENT_ULTIMATE_COMBO_4".to_string(), "ENCHANTMENT_ULTIMATE_COMBO_5".to_string(), "ENCHANTMENT_ULTIMATE_BANK_1".to_string(), "ENCHANTMENT_ULTIMATE_BANK_2".to_string(), "ENCHANTMENT_ULTIMATE_BANK_3".to_string(), "ENCHANTMENT_ULTIMATE_BANK_4".to_string(), "ENCHANTMENT_ULTIMATE_BANK_5".to_string(), "ENCHANTMENT_ULTIMATE_JERRY_1".to_string(), "ENCHANTMENT_ULTIMATE_JERRY_2".to_string(), "ENCHANTMENT_ULTIMATE_JERRY_3".to_string(), "ENCHANTMENT_ULTIMATE_JERRY_4".to_string(), "ENCHANTMENT_ULTIMATE_JERRY_5".to_string(), "ENCHANTMENT_FEATHER_FALLING_6".to_string(), "ENCHANTMENT_FEATHER_FALLING_7".to_string(), "ENCHANTMENT_FEATHER_FALLING_8".to_string(), "ENCHANTMENT_FEATHER_FALLING_9".to_string(), "ENCHANTMENT_FEATHER_FALLING_10".to_string(), "ENCHANTMENT_INFINITE_QUIVER_6".to_string(), "ENCHANTMENT_INFINITE_QUIVER_7".to_string(), "ENCHANTMENT_INFINITE_QUIVER_8".to_string(), "ENCHANTMENT_INFINITE_QUIVER_9".to_string(), "ENCHANTMENT_INFINITE_QUIVER_10".to_string()]));
/// java: public static final Pattern ESSENCE_PATTERN = Pattern.compile("(?<type>[A-Za-z]+) Essence(?: x(?<amount>\\d+))?");
pub static ESSENCE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?<type>[A-Za-z]+) Essence(?: x(?<amount>\\d+))?"));
/// java: public static final Pattern SHARD_PATTERN = Pattern.compile("[A-Za-z ]+ Shard(?: x(?<amount>\\d+))?");
pub static SHARD_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("[A-Za-z ]+ Shard(?: x(?<amount>\\d+))?"));
/// java: public static final Pattern KUUDRA_TEETH_PATTERN = Pattern.compile("Kuudra Teeth(?: x(?<amount>\\d+))?");
pub static KUUDRA_TEETH_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Kuudra Teeth(?: x(?<amount>\\d+))?"));
/// java: public static final Pattern HEAVY_PEARL_PATTERN = Pattern.compile("Heavy Pearl(?: x(?<amount>\\d+))?");
pub static HEAVY_PEARL_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Heavy Pearl(?: x(?<amount>\\d+))?"));
/// java: private static final Pattern MINION_PATTERN = Pattern.compile("Minion (I|II|III|IV|V|VI|VII|VIII|IX|X|XI|XII)$");
pub static MINION_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Minion (I|II|III|IV|V|VI|VII|VIII|IX|X|XI|XII)$"));
/// java: public static boolean hideChestNameLabel = false;
pub static HIDE_CHEST_NAME_LABEL: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class ChestValue
#[derive(Clone)]
pub struct ChestValue {}

impl ChestValue {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        screen_events::AFTER_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            compat::set_state(&HIDE_CHEST_NAME_LABEL, false);
            // `screen instanceof ContainerScreen genericContainerScreen`: the screen's kind (the vanilla `ContainerScreen`
            // object itself is not lent to the listener - its menu and accessor reads below stay with the TODOs)
            if Utils::is_on_skyblock() && matches!(compat::minecraft().gui.screen_kind, mcrust::net::minecraft::client::gui::gui::ScreenKind::Container) && !(crate::entry::catharsis_compatibility::is_gui_element_hidden("skyblocker:chestValueButton")) {
                let mut title: MutableComponent = compat::component_of(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(screen.get_title().to_string()));
                let mut title_string: String = title.get_string();
                let mut chest_type: Option<RewardChestType> = (if DUNGEON_CHESTS.contains(&title_string) { Some(RewardChestType::Dungeon) } else { (if KUUDRA_CHESTS.contains(&title_string) { Some(RewardChestType::Kuudra) } else { None }) });
                if let Some(chest_type) = &chest_type {
                    if SkyblockerConfigManager::get().dungeons.dungeon_chest_profit.enable_profit_calculator {
                        // TODO(fabric2seer): ScreenEvents.afterTick(..).register: no SeerClient equivalent yet
                        //   java: ScreenEvents.afterTick(screen).register(_ -> {
                        //   java: 							Component dungeonChestProfit = getRewardChestProfit(genericContainerScreen.getMenu(), chestType);
                        //   java: 							if (dungeonChestProfit != null)
                        //   java: 								addValueToContainer(client, genericContainerScreen, dungeonChestProfit, title);
                        //   java: 						});
                    }
                } else {
                    if SkyblockerConfigManager::get().ui_and_visuals.chest_value.enable_chest_value && !(title_string == "SkyBlock Menu") {
                        let mut screen_type: ScreenType = ChestValue::determine_screen_type(&title_string);
                        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
                        //   java: Screens.getWidgets(screen).add(Button
                        //   java: 							.builder(Component.literal("$"), buttonWidget -> {
                        //   java: 								Screens.getWidgets(screen).remove(buttonWidget);
                        //   java: 								ScreenEvents.afterTick(screen).register(_ -> {
                        //   java: 									Component chestValue = getChestValue(genericContainerScreen.getMenu(), screenType);
                        //   java: 									if (chestValue != null) {
                        //   java: 										addValueToContainer(client, genericContainerScreen, chestValue, title);
                        //   java: 									}
                        //   java: 								});
                        //   java: 							})
                        //   java: 							.bounds(((AbstractContainerScreenAccessor) genericContainerScreen).getX() + ((AbstractContainerScreenAccessor) genericContainerScreen).getImageWidth() - 16, ((AbstractContainerScreenAccessor) genericContainerScreen).getY() + 4, 12, 12)
                        //   java: 							.tooltip(Tooltip.create(getButtonTooltipText(screenType)))
                        //   java: 							.build()
                        //   java: 					);
                    }
                }
            }
        }));
    }

    /// java: private static @Nullable Component getRewardChestProfit(ChestMenu handler, RewardChestType chestType)
    pub fn get_reward_chest_profit(handler: &ChestMenu, mut chest_type: RewardChestType) -> Option<MutableComponent> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut profit: f64 = ((0) as f64) as f64;
            let mut has_incomplete_data: bool = false;
            let mut used_kismet: bool = false;
            let mut slots: Vec<Slot> = handler.menu.slots[(0) as usize..(handler.get_row_count().wrapping_mul(9)) as usize].to_vec();
            if compat::slot_item(&slots[(31) as usize]).is_empty() || compat::slot_item(&slots[(50) as usize]).is_empty() {
                return None;
            }
            for slot in slots.iter() {
                let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
                if stack.is_empty() {
                    continue;
                }
                let mut name: String = compat::component_of(stack.get_hover_name()).get_string();
                let mut skyblock_api_id: String = stack.get_skyblock_api_id();
                if !skyblock_api_id.is_empty() && !(name.contains("Essence") || name.contains("Shard")) {
                    if !(WORTHLESS_ITEMS.contains(&skyblock_api_id)) {
                        let mut price_data: Option<f64> = ItemUtils::get_item_price_string(Some(skyblock_api_id.clone()));
                        if price_data.is_some() {
                            profit = (profit + (price_data.clone().unwrap() * (stack.get_count() as f64)));
                        } else {
                            has_incomplete_data = true;
                        }
                    }
                    continue;
                }
                if name.contains("Essence") && SkyblockerConfigManager::get().dungeons.dungeon_chest_profit.include_essence {
                    let mut matcher: Option<compat::regex::Matcher> = Some(ESSENCE_PATTERN.matcher(&name));
                    if matcher.as_mut().unwrap().matches() {
                        let mut r#type: String = matcher.as_mut().unwrap().group_named("type").unwrap().to_uppercase();
                        let mut amount: i32 = RegexUtils::parse_optional_int_from_matcher_match_result_string(matcher.as_ref().unwrap(), "amount").unwrap_or(1);
                        let mut price_data: Option<f64> = ItemUtils::get_item_price_string(Some(skyblock_api_id.clone()));
                        if r#type == "CRIMSON" {
                            amount = ((((amount as f32) * ChestValue::compute_crimson_essence_multiplier())) as i32);
                        }
                        if price_data.is_some() {
                            profit = (profit + (price_data.clone().unwrap() * (amount as f64)));
                        } else {
                            has_incomplete_data = true;
                        }
                        continue;
                    }
                }
                if name.contains("Shard") && !(name.contains("Reroll")) {
                    let mut matcher: Option<compat::regex::Matcher> = Some(SHARD_PATTERN.matcher(&name));
                    if matcher.as_mut().unwrap().matches() {
                        let mut shards: i32 = RegexUtils::parse_optional_int_from_matcher_match_result_string(matcher.as_ref().unwrap(), "amount").unwrap_or(1);
                        let Some(mut attribute) = Attributes::get_attribute_from_item_name(&stack) else {
                            logger().warn(&format!("[Skyblocker Profit Calculator] Encountered unknown shard {}", name));
                            continue;
                        };
                        let mut shard_api_id: String = attribute.api_id();
                        let mut price_data: Option<f64> = ItemUtils::get_item_price_string(Some(shard_api_id.clone()));
                        if price_data.is_some() {
                            profit = (profit + (price_data.clone().unwrap() * (shards as f64)));
                        } else {
                            has_incomplete_data = true;
                        }
                        continue;
                    }
                }
                if name.contains("Open Reward Chest") {
                    // TODO(fabric2seer): static `ChatFormatting.stripFormatting` has no Rust counterpart in the port
                    match chest_type {
                        RewardChestType::Dungeon => {
                            // TODO(fabric2seer): static `ChatFormatting.stripFormatting` has no Rust counterpart in the port
                            // rust: let mut matcher: Option<compat::regex::Matcher> = Some(RegexListUtils::match_in_list_list_function_pattern(&stack.skyblocker_get_lore_strings(), Arc::new(move |__a0| ChatFormatting::strip_formatting(__a0.clone())), &DUNGEON_CHEST_COIN_COST_PATTERN));
                            let mut matcher: compat::regex::Matcher = todo!("fabric2seer: the initialiser of `matcher` (TODO above)");
                            if false /* java: matcher == null: never null here */ {
                                continue;
                            }
                            let mut found_string: String = compat::regex::replace_all(&matcher.group(1).unwrap(), "\\D", "");
                            // TODO(fabric2seer): the condition: static call NumberUtils.isCreatable(..)
                            //   java: !NumberUtils.isCreatable(foundString)
                            if todo!("fabric2seer: the condition (TODO above)") {
                                continue;
                            }
                            profit = (profit - (found_string.parse::<i32>().expect("NumberFormatException") as f64));
                        }
                        RewardChestType::Kuudra => {
                            let Some(mut key) = ItemUtils::get_lore_line_if(&stack, Arc::new(move |__a0| KUUDRA_KEYS.contains_key(&__a0))) else {
                                continue;
                            };
                            let mut key_cost: (f64, bool) = ChestValue::compute_kuudra_key_price(&key);
                            profit = (profit - key_cost.0);
                            // TODO(fabric2seer): Map.Entry.rightBoolean
                            //   java: hasIncompleteData |= !keyCost.rightBoolean();
                        }
                        _ => {}
                    }
                    continue;
                }
                if name.contains("Reroll Chest") {
                    // TODO(fabric2seer): static call StringUtils.isBlank(..)
                    //   java: usedKismet = !StringUtils.isBlank(ItemUtils.getLoreLineContains(stack, "You already rerolled a chest!"));
                }
            }
            if SkyblockerConfigManager::get().dungeons.dungeon_chest_profit.include_kismet && used_kismet {
                let mut kismet_price_data: Option<f64> = ItemUtils::get_item_price_string(Some("KISMET_FEATHER".to_string()));
                if kismet_price_data.is_some() {
                    profit = (profit - kismet_price_data.clone().unwrap());
                } else {
                    has_incomplete_data = true;
                }
            }
            return Some(ChestValue::get_profit_text(((profit) as i64), has_incomplete_data));
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Profit Calculator] Failed to calculate reward chest profit for {}! ", compat::jstring(&chest_type)));
            }
        }
        return None;
    }

    /// java: public static float computeCrimsonEssenceMultiplier()
    pub fn compute_crimson_essence_multiplier() -> f32 {
        let mut kuudra_pet: PetInfo = Kuudra::get_kuudra_profile_data().kuudra_pet();
        let mut percent_bonus: f32 = (match kuudra_pet.tier() {
            SkyblockItemRarity::Common => 10.0f32,
            SkyblockItemRarity::Uncommon | SkyblockItemRarity::Rare => 15.0f32,
            SkyblockItemRarity::Epic | SkyblockItemRarity::Legendary => 20.0f32,
            _ => 10.0f32,
        } * ((kuudra_pet.level() as f32) / 100.0f32));
        return ((percent_bonus / 100.0f32) + 1.0f32);
    }

    /// java: public static DoubleBooleanPair computeKuudraKeyPrice(String kuudraKeyName)
    pub fn compute_kuudra_key_price(kuudra_key_name: &str) -> (f64, bool) {
        let Some(mut kuudra_key_id) = KUUDRA_KEYS.get(kuudra_key_name).cloned() else {
            panic!("IllegalArgumentException: {}", format!("Expected a Kuudra Key variant but got: {}", kuudra_key_name).to_string());
        };
        let mut base_cost: i32 = Kuudra::get_kuudra_profile_data().kuudra_key_prices().get(&kuudra_key_id).cloned().unwrap_or_else(|| kuudra_profile_data::EMPTY.kuudra_key_prices().get(&kuudra_key_id).cloned().unwrap());
        let mut ingredient: String = match Kuudra::get_kuudra_profile_data().faction() {
            CrimsonFaction::Mage => "ENCHANTED_MYCELIUM".to_string(),
            CrimsonFaction::Barbarian => "ENCHANTED_RED_SAND".to_string(),
            _ => unreachable!("switch expression: no case matched"),
        };
        let mut ingredient_amount: i32 = match kuudra_key_id.as_str() {
            "KUUDRA_TIER_KEY" => 2,
            "KUUDRA_HOT_TIER_KEY" => 4,
            "KUUDRA_BURNING_TIER_KEY" => 16,
            "KUUDRA_FIERY_TIER_KEY" => 40,
            "KUUDRA_INFERNAL_TIER_KEY" => 80,
            _ => {
                panic!("IllegalArgumentException: {}", format!("Expected a Kuudra Key variant but got: {}", kuudra_key_name).to_string());
            }
        };
        let mut price: f64 = ((0) as f64) as f64;
        let mut has_complete_data: bool = true;
        let mut ingredient_price_data: Option<f64> = ItemUtils::get_item_price_string(Some(ingredient.clone()));
        let mut nether_star_price_data: Option<f64> = ItemUtils::get_item_price_string(Some("CORRUPTED_NETHER_STAR".to_string()));
        price = (price + (base_cost as f64));
        if ingredient_price_data.is_some() {
            price = (price + (ingredient_price_data.clone().unwrap() * (ingredient_amount as f64)));
        } else {
            has_complete_data = false;
        }
        if nether_star_price_data.is_some() {
            price = (price + (nether_star_price_data.clone().unwrap() * (2 as f64)));
        } else {
            has_complete_data = false;
        }
        return (price, has_complete_data);
    }

    /// java: private static @Nullable Component getChestValue(ChestMenu handler, ScreenType screenType)
    pub fn get_chest_value(handler: &ChestMenu, mut screen_type: ScreenType) -> Option<MutableComponent> {
        // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            let mut value: f64 = ((0) as f64) as f64;
            let mut has_incomplete_data: bool = false;
            let mut slots: Vec<Slot> = match screen_type {
                ScreenType::Minion => ChestValue::get_minion_slots(handler),
                ScreenType::Sack => handler.menu.slots[(10) as usize..((handler.get_row_count().wrapping_mul(9)).wrapping_sub(10)) as usize].to_vec(),
                ScreenType::Stash => handler.menu.slots[(0) as usize..((handler.get_row_count().wrapping_sub(1)).wrapping_mul(9)) as usize].to_vec(),
                ScreenType::Other | ScreenType::HuntingBox => handler.menu.slots[(0) as usize..(handler.get_row_count().wrapping_mul(9)) as usize].to_vec(),
                _ => unreachable!("switch expression: no case matched"),
            };
            for slot in slots.iter() {
                let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = compat::slot_item(slot);
                if stack.is_empty() {
                    continue;
                }
                // java: String coinsLine;
                //   if (screenType == ScreenType.MINION && slot.index == 28 && stack.is(Items.HOPPER) && (coinsLine = ItemUtils.getLoreLineIf(stack, s -> s.contains("Held Coins:"))) != null)
                let coins_line: Option<String> = if screen_type == ScreenType::Minion && slot.index == 28 && stack.is_item(&items::items().get("hopper")) {
                    ItemUtils::get_lore_line_if(&stack, Arc::new(move |s: String| s.contains("Held Coins:")))
                } else {
                    None
                };
                if let Some(coins_line) = coins_line {
                    let mut source: String = (compat::regex::split(&coins_line, ":"))[(1) as usize].clone();
                    // java: try { value += NumberFormat.getNumberInstance(java.util.Locale.US).parse(source.trim()).doubleValue(); }
                    //   catch (ParseException _) { LOGGER.warn("[Skyblocker] Failed to parse `{}`", source); }
                    match number_instance_us_parse(&compat::trim(&source)) {
                        Some(parsed) => value += parsed,
                        None => logger().warn(&format!("[Skyblocker] Failed to parse `{}`", source)),
                    }
                    continue;
                }
                let mut id: String = match screen_type {
                    ScreenType::HuntingBox => 'yield_2: {
                        let mut attribute: Option<Attribute> = Attributes::get_attribute_from_item_name(&stack);
                        break 'yield_2 (if attribute.is_some() { attribute.as_ref().unwrap().api_id() } else { "".to_string() });
                    }
                    _ => stack.get_skyblock_api_id(),
                };
                let mut count: i32 = match screen_type {
                    ScreenType::Sack => 'yield_3: {
                        let mut lines: Vec<String> = stack.skyblocker_get_lore_strings();
                        break 'yield_3 ItemUtils::get_item_count_in_sack(&stack, &lines).unwrap_or(0);
                    }
                    ScreenType::Stash => ItemUtils::get_item_count_in_stash(&stack).unwrap_or(0),
                    ScreenType::HuntingBox => ItemUtils::get_item_count_in_hunting_box(&stack).unwrap_or(0),
                    ScreenType::Other | ScreenType::Minion => stack.get_count(),
                    _ => unreachable!("switch expression: no case matched"),
                };
                if count == 0 {
                    continue;
                }
                if !id.is_empty() {
                    let mut price_data: Option<f64> = ItemUtils::get_item_price_string(Some(id.clone()));
                    if price_data.is_none() {
                        has_incomplete_data = true;
                    }
                    // TODO(fabric2seer): `.price(..)` on a NetworthResult
                    //   java: value += screenType == ScreenType.HUNTING_BOX ? priceData.orElse(0) * count : NetworthCalculator.getItemNetworth(stack, count).price();
                }
            }
            return Some(ChestValue::get_value_text(((value) as i64), has_incomplete_data));
        }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker Value Calculator] Failed to calculate dungeon chest value! ", e);
        //   java: 		}
        return None;
    }

    /// java: private static List<Slot> getMinionSlots(ChestMenu handler)
    pub fn get_minion_slots(handler: &ChestMenu) -> Vec<Slot> {
        return handler.menu.slots[(0) as usize..(handler.get_row_count().wrapping_mul(9)) as usize].to_vec().into_iter().filter(|slot| {
            let mut x: i32 = slot.index.wrapping_rem(9);
            let mut y: i32 = slot.index.wrapping_div(9);
            return (x > 2 && x < 8 && y > 1 && y < 5 || slot.index == 28);
        }).collect::<Vec<_>>();
    }

    /// java: static Component getProfitText(long profit, boolean hasIncompleteData)
    pub fn get_profit_text(mut profit: i64, mut has_incomplete_data: bool) -> MutableComponent {
        return (component::literal(&format!("{}{} Coins", (if profit > (0 as i64) { " +" } else { " " }), formatters::INTEGER_NUMBERS.get_mut().format((profit) as f64)))).with_style_format(ChestValue::get_profit_color(has_incomplete_data, profit));
    }

    /// java: static ChatFormatting getProfitColor(boolean hasIncompleteData, long profit)
    pub fn get_profit_color(mut has_incomplete_data: bool, mut profit: i64) -> ChatFormatting {
        let config = &SkyblockerConfigManager::get().dungeons.dungeon_chest_profit;
        if has_incomplete_data {
            return config.incomplete_color;
        }
        if profit.wrapping_abs() < (config.neutral_threshold as i64) {
            return config.neutral_color;
        }
        if profit > (0 as i64) {
            return config.profit_color;
        }
        return config.loss_color;
    }

    /// java: static Component getValueText(long value, boolean hasIncompleteData)
    pub fn get_value_text(mut value: i64, mut has_incomplete_data: bool) -> MutableComponent {
        let config = &SkyblockerConfigManager::get().ui_and_visuals.chest_value;
        return (component::literal(&format!(" {} Coins", formatters::INTEGER_NUMBERS.get_mut().format((value) as f64)))).with_style_format((if has_incomplete_data { config.incomplete_color } else { config.color }));
    }

    /// java: private static void addValueToContainer(Minecraft client, ContainerScreen genericContainerScreen, Component chestValue, Component title)
    pub fn add_value_to_container(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, generic_container_screen: &ContainerScreen, chest_value: &MutableComponent, title: &MutableComponent) {
        compat::set_state(&HIDE_CHEST_NAME_LABEL, true);
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Screens.getWidgets(genericContainerScreen).removeIf(ChestValueTextWidget.class::isInstance);
        let mut inner_width: i32 = generic_container_screen.container.image_width.wrapping_sub(8);
        let mut x: i32 = generic_container_screen.container.left_pos;
        let mut y: i32 = generic_container_screen.container.top_pos;
        let mut max_chest_value_width: i32 = ((inner_width.wrapping_div(2)).wrapping_sub(2)).max(inner_width.wrapping_sub(compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(title.clone()))));
        let mut chest_value_width: i32 = (compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(chest_value.clone()))).min(max_chest_value_width);
        let mut chest_value_widget: ChestValueTextWidget = ChestValueTextWidget::new(chest_value_width, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT, chest_value.clone(), compat::font());
        // TODO(fabric2seer): `width` takes 2 argument(s) in the port (`max_width: i32, text_width: i32`), Java passes 0
        // rust: chest_value_widget.set_position(((x.wrapping_add(inner_width)).wrapping_sub(string_widget::width())).wrapping_add(4), y.wrapping_add(6));
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Screens.getWidgets(genericContainerScreen).add(chestValueWidget);
        let mut chest_title_widget: ChestValueTextWidget = ChestValueTextWidget::new((inner_width.wrapping_sub(chest_value_width)).wrapping_sub(2), mcrust::net::minecraft::client::gui::font::LINE_HEIGHT, title.copy().with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color(0x404040)), compat::font());
        chest_title_widget.set_position(x.wrapping_add(8), y.wrapping_add(6));
        // TODO(fabric2seer): Fabric API `Screens.getWidgets` has no SeerClient equivalent
        //   java: Screens.getWidgets(genericContainerScreen).add(chestTitleWidget);
    }

    /// java: private static ScreenType determineScreenType(String rawTitleString)
    pub fn determine_screen_type(raw_title_string: &str) -> ScreenType {
        if raw_title_string.to_lowercase().ends_with("sack") {
            return ScreenType::Sack;
        }
        if MINION_PATTERN.matcher(&compat::trim(&raw_title_string)).find() {
            return ScreenType::Minion;
        }
        if compat::equals_ignore_case(&"View Stash", raw_title_string) {
            return ScreenType::Stash;
        }
        if hunting_box_helper::HUNTING_BOX_TITLE_PATTERN.matcher(raw_title_string).matches() {
            return ScreenType::HuntingBox;
        }
        return ScreenType::Other;
    }

    /// java: private static Component getButtonTooltipText(ScreenType screenType)
    pub fn get_button_tooltip_text(mut screen_type: ScreenType) -> MutableComponent {
        return match screen_type {
            ScreenType::Minion => component::translatable("skyblocker.containerValue.minionValue.@Tooltip"),
            ScreenType::Other => component::translatable("skyblocker.containerValue.chestValue.@Tooltip"),
            ScreenType::Stash => component::translatable("skyblocker.containerValue.stashValue.@Tooltip"),
            ScreenType::Sack => component::translatable("skyblocker.containerValue.sackValue.@Tooltip"),
            ScreenType::HuntingBox => component::translatable("skyblocker.containerValue.huntingBoxValue.@Tooltip"),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

}

/// java: private static class ChestValueTextWidget extends StringWidget
pub struct ChestValueTextWidget {
    /// java: extends net.minecraft.client.gui.components.StringWidget
    pub base: AbstractWidget,
}

impl std::ops::Deref for ChestValueTextWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ChestValueTextWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ChestValueTextWidget {
    /// java: private ChestValueTextWidget(int width, int height, Component message, Font textRenderer)
    pub fn new(mut width: i32, mut height: i32, mut message: MutableComponent, mut text_renderer: Font) -> Self {
        let mut this = Self {
            base: compat::string_widget(0, 0, width, height, message.copy().with_style(&mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_shadow_color(0))),
        };
        // TODO(fabric2seer): `width` takes 2 argument(s) in the port (`max_width: i32, text_width: i32`), Java passes 0
        // rust: this.set_max_width(string_widget::width(), TextOverflow::Scrolling);
        this
    }

}

/// java: private enum ScreenType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScreenType {
    Minion,
    Sack,
    Stash,
    HuntingBox,
    Other,
}

impl ScreenType {
    pub const VALUES: [ScreenType; 5] = [ScreenType::Minion, ScreenType::Sack, ScreenType::Stash, ScreenType::HuntingBox, ScreenType::Other];

    /// java: values()
    pub fn values() -> Vec<ScreenType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ScreenType::Minion => "MINION",
            ScreenType::Sack => "SACK",
            ScreenType::Stash => "STASH",
            ScreenType::HuntingBox => "HUNTING_BOX",
            ScreenType::Other => "OTHER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ScreenType {
        match name {
            "MINION" => ScreenType::Minion,
            "SACK" => ScreenType::Sack,
            "STASH" => ScreenType::Stash,
            "HUNTING_BOX" => ScreenType::HuntingBox,
            "OTHER" => ScreenType::Other,
            _ => panic!("IllegalArgumentException: No enum constant ScreenType.{name}"),
        }
    }

}

impl std::fmt::Display for ScreenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: private enum RewardChestType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RewardChestType {
    Dungeon,
    Kuudra,
}

impl RewardChestType {
    pub const VALUES: [RewardChestType; 2] = [RewardChestType::Dungeon, RewardChestType::Kuudra];

    /// java: values()
    pub fn values() -> Vec<RewardChestType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            RewardChestType::Dungeon => "DUNGEON",
            RewardChestType::Kuudra => "KUUDRA",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> RewardChestType {
        match name {
            "DUNGEON" => RewardChestType::Dungeon,
            "KUUDRA" => RewardChestType::Kuudra,
            _ => panic!("IllegalArgumentException: No enum constant RewardChestType.{name}"),
        }
    }

}

impl std::fmt::Display for RewardChestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: `NumberFormat.getNumberInstance(Locale.US).parse(text).doubleValue()` - `DecimalFormat("#,##0.###")`'s
/// `parse(String)`: the longest prefix of `text` that is a number - an optional `-`, digits with `,` grouping
/// separators (a trailing `,` is not taken), at most one `.` (no `,` after it), and an `E` exponent; `NaN` and the
/// infinity sign as the symbols. `None` is the `ParseException` (nothing at the start parsed).
/// rust: port gap - `DecimalFormat.parse` / `NumberFormat.getNumberInstance` (java/text has `new` + `format`); the
/// JDK body is reproduced here for this one caller (GAPS_batchI.md)
fn number_instance_us_parse(text: &str) -> Option<f64> {
    let chars: Vec<char> = text.chars().collect();
    if text.starts_with("NaN") {
        return Some(f64::NAN);
    }
    let mut i: usize = 0;
    let negative = chars.first() == Some(&'-');
    if negative {
        i = 1;
    }
    if chars.get(i) == Some(&'∞') {
        return Some(if negative { f64::NEG_INFINITY } else { f64::INFINITY });
    }
    let mut number = String::new();
    let mut saw_digit = false;
    let mut saw_decimal = false;
    let mut exponent: i32 = 0;
    while i < chars.len() {
        let c = chars[i];
        if let Some(digit) = c.to_digit(10) {
            number.push(char::from_digit(digit, 10).unwrap_or('0'));
            saw_digit = true;
            i += 1;
        } else if c == '.' {
            if saw_decimal {
                break;
            }
            saw_decimal = true;
            number.push('.');
            i += 1;
        } else if c == ',' {
            // a grouping separator: skipped (and not taken at all when no digit follows - nothing reads the position)
            if saw_decimal {
                break;
            }
            i += 1;
        } else if c == 'E' && saw_digit {
            // `subparse` of the exponent: an optional `-`, then digits; without digits the `E` is not taken
            let mut j = i + 1;
            let exponent_negative = chars.get(j) == Some(&'-');
            if exponent_negative {
                j += 1;
            }
            let start = j;
            while j < chars.len() && chars[j].is_ascii_digit() {
                j += 1;
            }
            if j == start {
                break;
            }
            let digits: String = chars[start..j].iter().collect();
            exponent = digits.parse::<i32>().unwrap_or(i32::MAX);
            if exponent_negative {
                exponent = -exponent;
            }
            break;
        } else {
            break;
        }
    }
    if !saw_digit {
        return None;
    }
    let mut parsed: f64 = number.parse::<f64>().ok()?;
    if exponent != 0 {
        parsed *= 10f64.powi(exponent);
    }
    Some(if negative { -parsed } else { parsed })
}
