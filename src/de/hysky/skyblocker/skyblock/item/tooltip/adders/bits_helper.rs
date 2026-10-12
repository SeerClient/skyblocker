//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.adders.BitsHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::util;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern BITS_PATTERN = Pattern.compile("Cost (?<amount>[\\d,]+) Bits");
pub static BITS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Cost (?<amount>[\\d,]+) Bits"));
/// java: private static final Pattern CATEGORY_PATTERN = Pattern.compile("Click to browse!");
pub static CATEGORY_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Click to browse!"));
/// java: @Language("RegExp") private static final String TITLE_PATTERN = ".*(?:Community Shop|Bits Shop).*";
pub const TITLE_PATTERN: &str = ".*(?:Community Shop|Bits Shop).*";
/// java: private static final String LOGS_PREFIX = "[Skyblocker Bits Helper] ";
pub const LOGS_PREFIX: &str = "[Skyblocker Bits Helper] ";
/// java: private static final List<String> GOOD_SELLING_ITEMS = List.of( "KAT_FLOWER", "KAT_BOUQUET", "HEAT_CORE", "ULTIMATE_CARROT_CANDY_UPGRADE", "TALISMAN_ENRICHMENT_SWAPPER", "GOD_POTION_2", "KISMET_FEATHE
pub static GOOD_SELLING_ITEMS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["KAT_FLOWER".to_string(), "KAT_BOUQUET".to_string(), "HEAT_CORE".to_string(), "ULTIMATE_CARROT_CANDY_UPGRADE".to_string(), "TALISMAN_ENRICHMENT_SWAPPER".to_string(), "GOD_POTION_2".to_string(), "KISMET_FEATHER".to_string(), "MATRIARCH_PARFUM".to_string(), "AUTOPET_RULES_2".to_string()]);
/// java: private static final Object2IntMap<String> CAT_KAT = Util.make(new Object2IntOpenHashMap<>(), map -> { map.put("KAT_FLOWER", 500); map.put("KAT_BOUQUET", 2500); });
pub static CAT_KAT: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| util::make_with(std::collections::HashMap::new(), |map| {
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("KAT_FLOWER", 500);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("KAT_BOUQUET", 2500);
}));
/// java: private static final Object2IntMap<String> CAT_UPGRADE_COMPONENTS = Util.make(new Object2IntOpenHashMap<>(), map -> { map.put("HEAT_CORE", 3000); map.put("HYPER_CATALYST_UPGRADE", 300); map.put("ULTIM
pub static CAT_UPGRADE_COMPONENTS: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| util::make_with(std::collections::HashMap::new(), |map| {
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("HEAT_CORE", 3000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("HYPER_CATALYST_UPGRADE", 300);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ULTIMATE_CARROT_CANDY_UPGRADE", 8000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("COLOSSAL_EXP_BOTTLE_UPGRADE", 1200);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("JUMBO_BACKPACK_UPGRADE", 4000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("MINION_STORAGE_EXPANDER", 1500);
}));
/// java: private static final Object2IntMap<String> CAT_SACKS = Util.make(new Object2IntOpenHashMap<>(), map -> { map.put("POCKET_SACK_IN_A_SACK", 8000); map.put("LARGE_DUNGEON_SACK", 14000); map.put("RUNE_SAC
pub static CAT_SACKS: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| util::make_with(std::collections::HashMap::new(), |map| {
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("POCKET_SACK_IN_A_SACK", 8000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("LARGE_DUNGEON_SACK", 14000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("RUNE_SACK", 14000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("FLOWER_SACK", 14000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("DWARVEN_MINES_SACK", 14000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("CRYSTAL_HOLLOWS_SACK", 14000);
}));
/// java: private static final Object2IntMap<String> CAT_ABIPHONE = Util.make(new Object2IntOpenHashMap<>(), map -> { map.put("TRIO_CONTACTS_ADDON", 6450); map.put("ABICASE_SUMSUNG_1", 15000); map.put("ABICASE_
pub static CAT_ABIPHONE: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| util::make_with(std::collections::HashMap::new(), |map| {
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TRIO_CONTACTS_ADDON", 6450);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ABICASE_SUMSUNG_1", 15000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ABICASE_SUMSUNG_2", 25000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ABICASE_REZAR", 26000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ABICASE_BLUE_AQUA", 17000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ABICASE_BLUE_BLUE", 17000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ABICASE_BLUE_GREEN", 17000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ABICASE_BLUE_RED", 17000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ABICASE_BLUE_YELLOW", 17000);
}));
/// java: private static final Object2IntMap<String> CAT_DYES = Util.make(new Object2IntOpenHashMap<>(), map -> { map.put("DYE_PURE_WHITE", 250000); map.put("DYE_PURE_BLACK", 250000); });
pub static CAT_DYES: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| util::make_with(std::collections::HashMap::new(), |map| {
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("DYE_PURE_WHITE", 250000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("DYE_PURE_BLACK", 250000);
}));
/// java: private static final Object2IntMap<String> CAT_ENCHANTS = Util.make(new Object2IntOpenHashMap<>(), map -> { map.put("ENCHANTMENT_HECATOMB_1", 6000); map.put("ENCHANTMENT_EXPERTISE_1", 4000); map.put("
pub static CAT_ENCHANTS: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| util::make_with(std::collections::HashMap::new(), |map| {
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ENCHANTMENT_HECATOMB_1", 6000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ENCHANTMENT_EXPERTISE_1", 4000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ENCHANTMENT_COMPACT_1", 4000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ENCHANTMENT_CULTIVATING_1", 4000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ENCHANTMENT_CHAMPION_1", 4000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("ENCHANTMENT_TOXOPHILITE_1", 4000);
}));
/// java: private static final Object2IntMap<String> CAT_ENRICHMENTS = Util.make(new Object2IntOpenHashMap<>(), map -> { map.put("TALISMAN_ENRICHMENT_SWAPPER", 200); map.put("TALISMAN_ENRICHMENT_WALK_SPEED", 50
pub static CAT_ENRICHMENTS: LazyLock<std::collections::HashMap<String, i32>> = LazyLock::new(|| util::make_with(std::collections::HashMap::new(), |map| {
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_SWAPPER", 200);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_WALK_SPEED", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_INTELLIGENCE", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_CRITICAL_DAMAGE", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_CRITICAL_CHANCE", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_STRENGTH", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_DEFENSE", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_HEALTH", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_MAGIC_FIND", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_FEROCITY", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_SEA_CREATURE_CHANCE", 5000);
    // TODO(fabric2seer): `.put(..)` on a T
    //   java: map.put("TALISMAN_ENRICHMENT_ATTACK_SPEED", 5000);
}));
/// java: private static final Map<String, Map<String, Integer>> CATEGORIES = Map.of( "Kat Items", CAT_KAT, "Upgrade Components", CAT_UPGRADE_COMPONENTS, "Sacks", CAT_SACKS, "Abiphone Supershop", CAT_ABIPHONE, 
pub static CATEGORIES: LazyLock<std::collections::HashMap<String, std::collections::HashMap<String, i32>>> = LazyLock::new(|| std::collections::HashMap::from([("Kat Items".to_string(), CAT_KAT.clone()), ("Upgrade Components".to_string(), CAT_UPGRADE_COMPONENTS.clone()), ("Sacks".to_string(), CAT_SACKS.clone()), ("Abiphone Supershop".to_string(), CAT_ABIPHONE.clone()), ("Dyes".to_string(), CAT_DYES.clone()), ("Stacking Enchants".to_string(), CAT_ENCHANTS.clone()), ("Enrichments".to_string(), CAT_ENRICHMENTS.clone())]));
/// java: public static final BitsHelper INSTANCE = new BitsHelper();
pub static INSTANCE: compat::MainThread<compat::Shared<BitsHelper>> = compat::MainThread::new(|| BitsHelper::new());

/// java: public class BitsHelper extends SimpleContainerSolver implements TooltipAdder
#[derive(Clone)]
pub struct BitsHelper {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private final Map<String, ObjectLongImmutablePair<String>> categoryOutput = new HashMap<>();
    pub category_output: std::collections::HashMap<String, (String, i64)>,
    /// java: private int bestSlotIndexSelling = -1;
    pub best_slot_index_selling: i32,
    /// java: private int bestSlotIndexAll = -1;
    pub best_slot_index_all: i32,
    /// java: private boolean megamindInLogs = false;
    pub megamind_in_logs: bool,
}

impl std::ops::Deref for BitsHelper {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BitsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BitsHelper {
    /// java: private BitsHelper()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in(TITLE_PATTERN.to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                category_output: std::collections::HashMap::new(),
                best_slot_index_selling: -1,
                best_slot_index_all: -1,
                megamind_in_logs: false,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        __this
    }

    /// java: private Int2ObjectMap<ItemStack> getSlots()
    pub fn get_slots(__this: &compat::Shared<Self>) -> Option<std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>> {
        // client.gui.screen() instanceof AbstractContainerScreen<?> screen; its getMenu() is the player's open menu
        if compat::container_screen_open() {
            let mut slots: std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = compat::with_open_menu(|handler, containers| {
                let mut slots: std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack> = std::collections::HashMap::new();
                for i in 0..(handler.slots.len() as i32) {
                    // slots.put(i, handler.getSlot(i).getItem()); - the same stack (`duplicate` keeps its identity)
                    slots.insert(i, handler.get_slot(i).get_item(containers).duplicate());
                }
                slots
            }).unwrap_or_default();
            if !(compat::read(__this, |__t| __t.megamind_in_logs)) {
                logger().info(&format!("{}\t\t\tNo slots?\n⠀⣞⢽⢪⢣⢣⢣⢫⡺⡵⣝⡮⣗⢷⢽⢽⢽⣮⡷⡽⣜⣜⢮⢺⣜⢷⢽⢝⡽⣝\n⠸⡸⠜⠕⠕⠁⢁⢇⢏⢽⢺⣪⡳⡝⣎⣏⢯⢞⡿⣟⣷⣳⢯⡷⣽⢽⢯⣳⣫⠇\n⠀⠀⢀⢀⢄⢬⢪⡪⡎⣆⡈⠚⠜⠕⠇⠗⠝⢕⢯⢫⣞⣯⣿⣻⡽⣏⢗⣗⠏⠀\n⠀⠪⡪⡪⣪⢪⢺⢸⢢⢓⢆⢤⢀⠀⠀⠀⠀⠈⢊⢞⡾⣿⡯⣏⢮⠷⠁⠀⠀\n⠀⠀⠀⠈⠊⠆⡃⠕⢕⢇⢇⢇⢇⢇⢏⢎⢎⢆⢄⠀⢑⣽⣿⢝⠲⠉⠀⠀⠀⠀\n⠀⠀⠀⠀⠀⡿⠂⠠⠀⡇⢇⠕⢈⣀⠀⠁⠡⠣⡣⡫⣂⣿⠯⢪⠰⠂⠀⠀⠀⠀\n⠀⠀⠀⠀⡦⡙⡂⢀⢤⢣⠣⡈⣾⡃⠠⠄⠀⡄⢱⣌⣶⢏⢊⠂⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⢝⡲⣜⡮⡏⢎⢌⢂⠙⠢⠐⢀⢘⢵⣽⣿⡿⠁⠁⠀⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⠨⣺⡺⡕⡕⡱⡑⡆⡕⡅⡕⡜⡼⢽⡻⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⣼⣳⣫⣾⣵⣗⡵⡱⡡⢣⢑⢕⢜⢕⡝⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⣴⣿⣾⣿⣿⣿⡿⡽⡑⢌⠪⡢⡣⣣⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⡟⡾⣿⢿⢿⢵⣽⣾⣼⣘⢸⢸⣞⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⠁⠇⠡⠩⡫⢿⣝⡻⡮⣒⢽⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀", LOGS_PREFIX));
                { let __v = true; compat::write(__this, |__t| __t.megamind_in_logs = __v); }
            }
            return Some(slots);
        }
        return None;
    }

    /// java: private BestItemsResult calculateBestItems(Int2ObjectMap<ItemStack> slots)
    pub fn calculate_best_items(__this: &compat::Shared<Self>, slots: &std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> BestItemsResult {
        let mut best_coins_per_bit_selling: i64 = 0;
        let mut best_coins_per_bit_all: i64 = 0;
        let mut best_slot_index_selling: i32 = -1;
        let mut best_slot_index_all: i32 = -1;
        if false /* java: slots == null: never null here */ || slots.is_empty() {
            return BestItemsResult::new(best_slot_index_selling, best_slot_index_all, best_coins_per_bit_selling, best_coins_per_bit_all);
        }
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Some(entry.1.clone());
            if stack.is_none() || stack.as_ref().unwrap().is_empty() {
                continue;
            }
            if CATEGORY_PATTERN.matcher(&ItemUtils::get_concatenated_lore(stack.as_ref().unwrap())).find() {
                let mut category_results: (String, i64) = BitsHelper::calculate_best_in_category(__this, stack.as_ref().unwrap());
                let mut item_id: String = category_results.0.clone();
                let mut coins_per_bit: i64 = category_results.1;
                if GOOD_SELLING_ITEMS.iter().any(|e| *e == item_id.clone()) && coins_per_bit > best_coins_per_bit_selling {
                    best_coins_per_bit_selling = coins_per_bit;
                    best_slot_index_selling = entry.0;
                }
                if coins_per_bit > best_coins_per_bit_all {
                    best_coins_per_bit_all = coins_per_bit;
                    best_slot_index_all = entry.0;
                }
            }
        }
        for entry in slots.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Some(entry.1.clone());
            if stack.is_none() || stack.as_ref().unwrap().is_empty() {
                continue;
            }
            let mut item_id: String = stack.as_ref().unwrap().get_skyblock_api_id();
            let mut lore: String = ItemUtils::get_concatenated_lore(stack.as_ref().unwrap());
            let mut bits_matcher: compat::regex::Matcher = BITS_PATTERN.matcher(&lore);
            if !bits_matcher.find() {
                continue;
            }
            let mut bits_cost: i64 = (bits_matcher.group_named("amount").unwrap().replace(",", "")).trim().parse::<i64>().expect("NumberFormatException");
            let mut item_cost: f64 = (ItemUtils::get_item_price(stack.as_ref().unwrap()).unwrap_or(((0) as f64) as f64) * (stack.as_ref().unwrap().get_count() as f64));
            if item_cost == (0 as f64) || bits_cost == (0 as i64) {
                continue;
            }
            let mut coins_per_bit: i64 = compat::java_round_f64((item_cost / (bits_cost as f64)));
            if GOOD_SELLING_ITEMS.iter().any(|e| *e == item_id.clone()) && coins_per_bit > best_coins_per_bit_selling {
                best_coins_per_bit_selling = coins_per_bit;
                best_slot_index_selling = entry.0;
            }
            if coins_per_bit > best_coins_per_bit_all {
                best_coins_per_bit_all = coins_per_bit;
                best_slot_index_all = entry.0;
            }
        }
        return BestItemsResult::new(best_slot_index_selling, best_slot_index_all, best_coins_per_bit_selling, best_coins_per_bit_all);
    }

    /// java: private ObjectLongImmutablePair<String> calculateBestInCategory(ItemStack stack)
    pub fn calculate_best_in_category(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> (String, i64) {
        let mut best_coins_per_bit_selling: i64 = 0;
        let mut best_coins_per_bit_all: i64 = 0;
        let mut cat_name: String = compat::component_of(stack.get_hover_name()).get_string();
        let __h0 = cat_name.clone();
        let __h1 = ("".to_string(), 0);
        __this.lock().borrow_mut().category_output.insert(__h0, __h1);
        let __h0 = format!("{}GREEN", cat_name);
        let __h1 = ("".to_string(), 0);
        __this.lock().borrow_mut().category_output.insert(__h0, __h1);
        let mut category_results: std::collections::HashMap<String, i64> = BitsHelper::process_category(__this, stack);
        // TODO(fabric2seer): Map.object2LongEntrySet(..)
        //   java: for (Map.Entry<String, Long> categoryEntry : categoryResults.object2LongEntrySet()) {
        //   java: 			String itemID = categoryEntry.getKey();
        //   java: 			long coinsPerBit = categoryEntry.getValue();
        //   java:
        //   java: 			if (GOOD_SELLING_ITEMS.contains(itemID) && (coinsPerBit > bestCoinsPerBitSelling)) {
        //   java: 				categoryOutput.put(catName + "GREEN", ObjectLongImmutablePair.of(itemID, coinsPerBit));
        //   java: 				bestCoinsPerBitSelling = coinsPerBit;
        //   java: 			}
        //   java:
        //   java: 			if (coinsPerBit > bestCoinsPerBitAll) {
        //   java: 				categoryOutput.put(catName, ObjectLongImmutablePair.of(itemID, coinsPerBit));
        //   java: 				bestCoinsPerBitAll = coinsPerBit;
        //   java: 			}
        //   java: 		}
        return __this.lock().borrow().category_output.get(&cat_name).cloned().unwrap();
    }

    /// java: private Object2LongMap<String> processCategory(ItemStack stack)
    pub fn process_category(__this: &compat::Shared<Self>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> std::collections::HashMap<String, i64> {
        let mut category_name: String = compat::component_of(stack.get_hover_name()).get_string();
        if CATEGORIES.contains_key(&category_name) {
            let mut results: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
            let mut category: std::collections::HashMap<String, i32> = CATEGORIES.get(&category_name).cloned().unwrap();
            for entry in category.iter().map(|(k, v)| (k.clone(), v.clone())) {
                let mut item_id: String = entry.0.clone();
                let mut item_bits_price: i32 = entry.1;
                let mut item_cost: f64 = ItemUtils::get_item_price_string(Some(item_id.clone())).unwrap_or(((0) as f64) as f64);
                let mut coins_per_bit: i64 = compat::java_round_f64((item_cost / (item_bits_price as f64)));
                results.insert(item_id.clone(), coins_per_bit);
            }
            return results;
        } else if category_name.contains("Fuel Blocks") {
            let mut item_id: String = "INFERNO_FUEL_BLOCK".to_string();
            let mut item_bits_price: Vec<i32> = vec![75, 3600];
            let mut item_cost: f64 = ItemUtils::get_item_price_string(Some(item_id.clone())).unwrap_or(((0) as f64) as f64);
            let mut coins_per_bit: i64 = (((item_cost / (item_bits_price[(0) as usize] as f64)).max(((item_cost * (64 as f64)) / (item_bits_price[(1) as usize] as f64)))) as i64);
            let mut fuel_block_result: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
            fuel_block_result.insert(item_id.clone(), coins_per_bit);
            return fuel_block_result;
        } else {
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.warn(LOGS_PREFIX + "Can't recognize category {} from bits shop! Consider reporting this to our discord!", categoryName);
        }
        return std::collections::HashMap::new();
    }

}

impl SimpleContainerSolver for BitsHelper {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for BitsHelper {
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

impl ContainerMatcher for BitsHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        BitsHelper::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for BitsHelper {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        BitsHelper::get_colors_dyn
    }

}

impl Resettable for BitsHelper {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override default void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_from::<Self>
    }

}

impl TooltipAdder for BitsHelper {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        BitsHelper::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        BitsHelper::get_priority_dyn
    }

}

impl BitsHelper {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.enable_bits_tooltip;
    }

    /// BitsHelper.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        let mut best_items_result: BestItemsResult = BitsHelper::calculate_best_items(__this, slots);
        { let __v = best_items_result.best_slot_index_selling; compat::write(__this, |__t| __t.best_slot_index_selling = __v); }
        { let __v = best_items_result.best_slot_index_all; compat::write(__this, |__t| __t.best_slot_index_all = __v); }
        if compat::read(__this, |__t| __t.best_slot_index_selling) != -1 && compat::read(__this, |__t| __t.best_slot_index_selling) == compat::read(__this, |__t| __t.best_slot_index_all) {
            highlights.push(ColorHighlight::green(compat::read(__this, |__t| __t.best_slot_index_selling)));
        } else {
            if compat::read(__this, |__t| __t.best_slot_index_selling) != -1 {
                highlights.push(ColorHighlight::green(compat::read(__this, |__t| __t.best_slot_index_selling)));
            }
            if compat::read(__this, |__t| __t.best_slot_index_all) != -1 {
                highlights.push(ColorHighlight::yellow(compat::read(__this, |__t| __t.best_slot_index_all)));
            }
        }
        return highlights;
    }

    /// BitsHelper.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let mut lore: String = ItemUtils::concatenate_lore(lines);
        let mut bits_matcher: compat::regex::Matcher = BITS_PATTERN.matcher(&lore);
        let mut coins_per_bit: i64 = 0;
        let mut item_id: String = "".to_string();
        let mut is_green: bool = false;
        let Some(mut focused_slot) = focused_slot.clone() else {
            return;
        };
        if !(CATEGORY_PATTERN.matcher(&lore).find()) {
            if !bits_matcher.find() {
                return;
            }
            let mut bits_cost: i64 = (bits_matcher.group_named("amount").unwrap().replace(",", "")).trim().parse::<i64>().expect("NumberFormatException");
            let mut item_cost: f64 = (ItemUtils::get_item_price(stack).unwrap_or(((0) as f64) as f64) * (stack.get_count() as f64));
            if item_cost == (0 as f64) {
                return;
            }
            coins_per_bit = compat::java_round_f64((item_cost / (bits_cost as f64)));
        } else {
            let mut outer_key: String = compat::component_of(stack.get_hover_name()).get_string();
            let mut inner_map: Option<(String, i64)> = None;
            let mut inner_map_green: (String, i64);
            let mut best_items_result: BestItemsResult = BitsHelper::calculate_best_items(__this, &BitsHelper::get_slots(__this).unwrap());
            { let __v = best_items_result.best_slot_index_selling; compat::write(__this, |__t| __t.best_slot_index_selling = __v); }
            { let __v = best_items_result.best_slot_index_all; compat::write(__this, |__t| __t.best_slot_index_all = __v); }
            if compat::read(__this, |__t| __t.best_slot_index_selling) != -1 && compat::read(__this, |__t| __t.best_slot_index_selling) == focused_slot.get_container_slot() {
                is_green = true;
            }
            inner_map = __this.lock().borrow().category_output.get(&outer_key).cloned();
            inner_map_green = __this.lock().borrow().category_output.get(&format!("{}GREEN", outer_key)).cloned().unwrap();
            if let Some(inner_map) = &inner_map {
                if is_green {
                    item_id = inner_map_green.0.clone();
                    coins_per_bit = inner_map_green.1;
                } else {
                    item_id = inner_map.0.clone();
                    coins_per_bit = inner_map.1;
                }
            }
        }
        let mut flexible_item_stack: Option<compat::Shared<FlexibleItemStack>> = ItemRepository::get_item_stack(&item_id);
        let mut found_item_stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (if flexible_item_stack.is_none() { None } else { Some(FlexibleItemStack::get_stack_or_throw(flexible_item_stack.as_ref().unwrap())) });
        if item_id.is_empty() {
            lines.push(component::empty().append(component::literal("Bits Cost: ").with_style_format(ChatFormatting::Aqua)).append((component::literal(&format!("{} Coins per bit", formatters::INTEGER_NUMBERS.get_mut().format((coins_per_bit) as f64)))).with_style_format(ChatFormatting::DarkAqua)));
        } else if found_item_stack.is_some() && is_green {
            lines.push(component::empty().append(component::literal("Bits Cost: ").with_style_format(ChatFormatting::Aqua)).append((component::literal(&format!("{} Coins per bit", formatters::INTEGER_NUMBERS.get_mut().format((coins_per_bit) as f64)))).with_style_format(ChatFormatting::DarkAqua)));
            lines.push((component::literal(&format!("From {}", compat::component_of(found_item_stack.as_ref().unwrap().get_hover_name()).get_string()))).with_style_format(ChatFormatting::Green));
        } else if found_item_stack.is_some() {
            lines.push(component::empty().append(component::literal("Bits Cost: ").with_style_format(ChatFormatting::Aqua)).append((component::literal(&format!("{} Coins per bit", formatters::INTEGER_NUMBERS.get_mut().format((coins_per_bit) as f64)))).with_style_format(ChatFormatting::DarkAqua)));
            lines.push((component::literal(&format!("From {}", compat::component_of(found_item_stack.as_ref().unwrap().get_hover_name()).get_string()))).with_style_format(ChatFormatting::DarkAqua));
        } else {
            if !(compat::component_of(stack.get_hover_name()).get_string() == "Stacking Enchants") {
                // TODO(fabric2seer): a log call with a non-literal format
                //   java: LOGGER.warn(LOGS_PREFIX + "ItemStack not found for {}", itemID);
            }
            lines.push(component::empty().append(component::literal("Bits Cost: ").with_style_format(ChatFormatting::Aqua)).append((component::literal(&format!("{} Coins per bit", formatters::INTEGER_NUMBERS.get_mut().format((coins_per_bit) as f64)))).with_style_format(ChatFormatting::DarkAqua)));
            lines.push((component::literal(&format!("From {}", item_id))).with_style_format(ChatFormatting::DarkAqua));
        }
    }

    /// BitsHelper.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public int getPriority()
    pub fn get_priority_impl(__this: &compat::Shared<Self>) -> i32 {
        return 0;
    }

    /// BitsHelper.getPriority as the dispatch of TooltipAdder.getPriority
    pub fn get_priority_dyn(__this: &compat::Shared<dyn TooltipAdder>) -> i32 {
        Self::get_priority_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private record BestItemsResult(int bestSlotIndexSelling, int bestSlotIndexAll, long bestCoinsPerBitSelling, long bestCoinsPerBitAll)
#[derive(Clone, Debug, PartialEq)]
pub struct BestItemsResult {
    /// java: int bestSlotIndexSelling
    pub best_slot_index_selling: i32,
    /// java: int bestSlotIndexAll
    pub best_slot_index_all: i32,
    /// java: long bestCoinsPerBitSelling
    pub best_coins_per_bit_selling: i64,
    /// java: long bestCoinsPerBitAll
    pub best_coins_per_bit_all: i64,
}

impl BestItemsResult {
    /// java: the canonical record constructor
    pub fn new(mut best_slot_index_selling: i32, mut best_slot_index_all: i32, mut best_coins_per_bit_selling: i64, mut best_coins_per_bit_all: i64) -> Self {
        Self { best_slot_index_selling, best_slot_index_all, best_coins_per_bit_selling, best_coins_per_bit_all }
    }

    /// java: the record accessor `bestSlotIndexSelling()`
    pub fn best_slot_index_selling(&self) -> i32 {
        self.best_slot_index_selling
    }

    /// java: the record accessor `bestSlotIndexAll()`
    pub fn best_slot_index_all(&self) -> i32 {
        self.best_slot_index_all
    }

    /// java: the record accessor `bestCoinsPerBitSelling()`
    pub fn best_coins_per_bit_selling(&self) -> i64 {
        self.best_coins_per_bit_selling
    }

    /// java: the record accessor `bestCoinsPerBitAll()`
    pub fn best_coins_per_bit_all(&self) -> i64 {
        self.best_coins_per_bit_all
    }

}
