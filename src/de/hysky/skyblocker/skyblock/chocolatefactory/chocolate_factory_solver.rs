//! Ported from de.hysky.skyblocker.skyblock.chocolatefactory.ChocolateFactorySolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text::SlotText;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::adders::line_smoothener::LineSmoothener;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolver;
use crate::de::hysky::skyblocker::utils::container::simple_container_solver::SimpleContainerSolverBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;
use crate::de::hysky::skyblocker::utils::formatters;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::regex_utils::RegexUtils;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use crate::de::hysky::skyblocker::utils::time::skyblock_time::SkyblockTime;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::resources::sounds::simple_sound_instance::SimpleSoundInstance;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern CPS_PATTERN = Pattern.compile("([\\d,.]+) Chocolate per second");
pub static CPS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("([\\d,.]+) Chocolate per second"));
/// java: private static final Pattern CPS_INCREASE_PATTERN = Pattern.compile("\\+([\\d,]+) Chocolate per second");
pub static CPS_INCREASE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\+([\\d,]+) Chocolate per second"));
/// java: private static final Pattern COST_PATTERN = Pattern.compile("Cost ([\\d,]+) Chocolate");
pub static COST_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Cost ([\\d,]+) Chocolate"));
/// java: private static final Pattern LEVEL_PATTERN = Pattern.compile("\\[(\\d+)]");
pub static LEVEL_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\[(\\d+)]"));
/// java: private static final Pattern TOTAL_MULTIPLIER_PATTERN = Pattern.compile("Total Multiplier: ([\\d.]+)x");
pub static TOTAL_MULTIPLIER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Total Multiplier: ([\\d.]+)x"));
/// java: private static final Pattern MULTIPLIER_INCREASE_PATTERN = Pattern.compile("\\+([\\d.]+)x Chocolate per second");
pub static MULTIPLIER_INCREASE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\+([\\d.]+)x Chocolate per second"));
/// java: private static final Pattern CHOCOLATE_PATTERN = Pattern.compile("^([\\d,]+) Chocolate$");
pub static CHOCOLATE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^([\\d,]+) Chocolate$"));
/// java: private static final Pattern PRESTIGE_REQUIREMENT_PATTERN = Pattern.compile("Chocolate this Prestige: ([\\d,]+) +Requires (\\S+) Chocolate this Prestige!");
pub static PRESTIGE_REQUIREMENT_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Chocolate this Prestige: ([\\d,]+) +Requires (\\S+) Chocolate this Prestige!"));
/// java: private static final Pattern TIME_TOWER_STATUS_PATTERN = Pattern.compile("Status: (ACTIVE|INACTIVE)");
pub static TIME_TOWER_STATUS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Status: (ACTIVE|INACTIVE)"));
/// java: private static final Pattern TIME_TOWER_MULTIPLIER_PATTERN = Pattern.compile("by \\+([\\d.]+)x for \\dh\\.");
pub static TIME_TOWER_MULTIPLIER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("by \\+([\\d.]+)x for \\dh\\."));
/// java: private static final Pattern AVAILABLE_EGGS_PATTERN = Pattern.compile("Available eggs: (\\d+)");
pub static AVAILABLE_EGGS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Available eggs: (\\d+)"));
/// java: private static final Pattern ROMAN_LEVEL_PATTERN = Pattern.compile("(?:Chocolate Factory|Hand-Baked Chocolate|Time Tower|Rabbit Shrine|Coach Jackrabbit|Rabbit Barn) ?(?<level>\\w+)?");
pub static ROMAN_LEVEL_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(?:Chocolate Factory|Hand-Baked Chocolate|Time Tower|Rabbit Shrine|Coach Jackrabbit|Rabbit Barn) ?(?<level>\\w+)?"));
/// java: private static final Pattern PURCHASED_SLOTS_PATTERN = Pattern.compile("Purchased slots: (\\d+)/(\\d+)");
pub static PURCHASED_SLOTS_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Purchased slots: (\\d+)/(\\d+)"));
/// java: private static final byte STRAY_RABBIT_START = 0;
pub const STRAY_RABBIT_START: i32 = 0; // java: byte (widened to int at every use)
/// java: private static final byte STRAY_RABBIT_END = 26;
pub const STRAY_RABBIT_END: i32 = 26; // java: byte (widened to int at every use)
/// java: private static final byte CHOCOLATE_SLOT = 13;
pub const CHOCOLATE_SLOT: i32 = 13; // java: byte (widened to int at every use)
/// java: private static final byte PRESTIGE_SLOT = 27;
pub const PRESTIGE_SLOT: i32 = 27; // java: byte (widened to int at every use)
/// java: private static final byte RABBITS_START = 28;
pub const RABBITS_START: i32 = 28; // java: byte (widened to int at every use)
/// java: private static final byte RABBITS_END = 34;
pub const RABBITS_END: i32 = 34; // java: byte (widened to int at every use)
/// java: private static final byte RABBIT_BARN_SLOT = 35;
pub const RABBIT_BARN_SLOT: i32 = 35; // java: byte (widened to int at every use)
/// java: private static final byte HAND_BAKED_CHOCOLATE_SLOT = 37;
pub const HAND_BAKED_CHOCOLATE_SLOT: i32 = 37; // java: byte (widened to int at every use)
/// java: private static final byte TIME_TOWER_SLOT = 38;
pub const TIME_TOWER_SLOT: i32 = 38; // java: byte (widened to int at every use)
/// java: private static final byte RABBIT_SHRINE_SLOT = 39;
pub const RABBIT_SHRINE_SLOT: i32 = 39; // java: byte (widened to int at every use)
/// java: private static final byte COACH_SLOT = 41;
pub const COACH_SLOT: i32 = 41; // java: byte (widened to int at every use)
/// java: private static final byte CPS_SLOT = 45;
pub const CPS_SLOT: i32 = 45; // java: byte (widened to int at every use)
/// java: private static final byte RABBIT_HITMAN_SLOT = 51;
pub const RABBIT_HITMAN_SLOT: i32 = 51; // java: byte (widened to int at every use)
/// java: private static StraySound ding = StraySound.NONE;
pub static DING: LazyLock<Mutex<StraySound>> = LazyLock::new(|| Mutex::new(StraySound::None));
/// java: private static int dingTick = 0;
pub static DING_TICK: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: public static final ChocolateFactorySolver INSTANCE = new ChocolateFactorySolver();
pub static INSTANCE: compat::MainThread<compat::Shared<ChocolateFactorySolver>> = compat::MainThread::new(|| ChocolateFactorySolver::new());

/// java: public class ChocolateFactorySolver extends SimpleContainerSolver implements TooltipAdder, SlotTextAdder
#[derive(Clone)]
pub struct ChocolateFactorySolver {
    /// java: extends SimpleContainerSolver
    pub base: SimpleContainerSolverBase,
    /// java: private final ObjectArrayList<Rabbit> cpsIncreaseFactors = new ObjectArrayList<>(8);
    pub cps_increase_factors: Vec<Rabbit>,
    /// java: private long totalChocolate = -1L;
    pub total_chocolate: i64,
    /// java: private double totalCps = -1.0;
    pub total_cps: f64,
    /// java: private double totalCpsMultiplier = -1.0;
    pub total_cps_multiplier: f64,
    /// java: private long requiredUntilNextPrestige = -1L;
    pub required_until_next_prestige: i64,
    /// java: private boolean canPrestige = false;
    pub can_prestige: bool,
    /// java: private boolean reachedMaxPrestige = false;
    pub reached_max_prestige: bool,
    /// java: private double timeTowerMultiplier = -1.0;
    pub time_tower_multiplier: f64,
    /// java: private boolean isTimeTowerMaxed = false;
    pub is_time_tower_maxed: bool,
    /// java: private boolean isTimeTowerActive = false;
    pub is_time_tower_active: bool,
    /// java: private int bestUpgrade = -1;
    pub best_upgrade: i32,
    /// java: private int bestAffordableUpgrade = -1;
    pub best_affordable_upgrade: i32,
    /// java: private int prestigeLevel = -1;
    pub prestige_level: i32,
    /// java: private int hitmanAvailableEggs = -1;
    pub hitman_available_eggs: i32,
    /// java: private int purchasedHitmanSlots = -1;
    pub purchased_hitman_slots: i32,
    /// java: private int maxHitmanSlots = -1;
    pub max_hitman_slots: i32,
    /// java: private int rabbitShrineLevel = -1;
    pub rabbit_shrine_level: i32,
    /// java: private int handBakedChocolateLevel = -1;
    pub hand_baked_chocolate_level: i32,
    /// java: private int rabbitBarnLevel = -1;
    pub rabbit_barn_level: i32,
    /// java: private int timeTowerLevel = -1;
    pub time_tower_level: i32,
}

impl std::ops::Deref for ChocolateFactorySolver {
    type Target = SimpleContainerSolverBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ChocolateFactorySolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ChocolateFactorySolver {
    /// java: private ChocolateFactorySolver()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleContainerSolverBase::new_in("^Chocolate Factory$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                cps_increase_factors: Vec::with_capacity((8) as usize),
                total_chocolate: -1,
                total_cps: -1.0,
                total_cps_multiplier: -1.0,
                required_until_next_prestige: -1,
                can_prestige: false,
                reached_max_prestige: false,
                time_tower_multiplier: -1.0,
                is_time_tower_maxed: false,
                is_time_tower_active: false,
                best_upgrade: -1,
                best_affordable_upgrade: -1,
                prestige_level: -1,
                hitman_available_eggs: -1,
                purchased_hitman_slots: -1,
                max_hitman_slots: -1,
                rabbit_shrine_level: -1,
                hand_baked_chocolate_level: -1,
                rabbit_barn_level: -1,
                time_tower_level: -1,
            });
            ((__h.clone() as compat::Shared<dyn SimpleContainerSolver>), __h)
        });
        client_tick_events::START_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| ChocolateFactorySolver::on_tick(minecraft)));
        __this
    }

    /// java: private static void onTick(Minecraft client)
    pub fn on_tick(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let ding = compat::get_state(&DING);
        if ding != StraySound::None {
            // java: dingTick = (++dingTick) % (ding == StraySound.NORMAL ? 5 : 3);
            let ding_tick = (compat::get_state(&DING_TICK) + 1) % (if ding == StraySound::Normal { 5 } else { 3 });
            compat::set_state(&DING_TICK, ding_tick);
            if ding_tick == 0 {
                // java: client.getSoundManager().play(SimpleSoundInstance.forUI(ding == StraySound.NORMAL ? SoundEvents.NOTE_BLOCK_PLING.value() : SoundEvents.NOTE_BLOCK_HARP.value(), 1.f, 1.f));
                // (`game::play_ui_sound_with_volume` is `getSoundManager().play(SimpleSoundInstance.forUI(event, pitch, volume))`)
                let _ = client;
                mcrust::mcrust::modding::game::play_ui_sound_with_volume(if ding == StraySound::Normal { mcrust::net::minecraft::sounds::sound_events::note_block_pling() } else { mcrust::net::minecraft::sounds::sound_events::note_block_harp() }, 1.0f32, 1.0f32);
            }
        }
    }

    /// java: private void updateFactoryInfo(Int2ObjectMap<ItemStack> slots)
    pub fn update_factory_info(__this: &compat::Shared<Self>, slots: &std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) {
        compat::write(__this, |__t| __t.cps_increase_factors.clear());
        for i in RABBITS_START..=(RABBITS_END) as i32 {
            let mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack = slots.get(&i).cloned().unwrap();
            if item.is_item(&items::items().get("player_head")) {
                if let Some(__a0) = ChocolateFactorySolver::get_rabbit(__this, &item, i) {
                    compat::write(__this, |__t| __t.cps_increase_factors.push(__a0));
                }
            }
        }
        if let Some(__a0) = ChocolateFactorySolver::get_coach(__this, &slots.get(&((COACH_SLOT) as i32)).cloned().unwrap()) {
            compat::write(__this, |__t| __t.cps_increase_factors.push(__a0));
        }
        if let Some(l) = RegexUtils::find_long_from_matcher(&mut CHOCOLATE_PATTERN.matcher(&(compat::component_of((slots.get(&((CHOCOLATE_SLOT) as i32)).cloned().unwrap()).get_hover_name())).get_string())) {
            { let __v = l; compat::write(__this, |__t| __t.total_chocolate = __v); }
        }
        let mut cps_item_lore: String = ItemUtils::get_concatenated_lore(&slots.get(&((CPS_SLOT) as i32)).cloned().unwrap());
        let mut cps_matcher: compat::regex::Matcher = CPS_PATTERN.matcher(&cps_item_lore);
        if let Some(d) = RegexUtils::find_double_from_matcher(&mut cps_matcher) {
            { let __v = d; compat::write(__this, |__t| __t.total_cps = __v); }
        }
        let mut multiplier_matcher: compat::regex::Matcher = TOTAL_MULTIPLIER_PATTERN.matcher(&cps_item_lore);
        // java: RegexUtils.findDoubleFromMatcher(multiplierMatcher, cpsMatcher.hasMatch() ? cpsMatcher.end() : 0).ifPresent(d -> totalCpsMultiplier = d);
        if let Some(d) = RegexUtils::find_double_from_matcher_2(&mut multiplier_matcher, if cps_matcher.has_match() { cps_matcher.end(0) } else { 0 }) {
            { let __v = d; compat::write(__this, |__t| __t.total_cps_multiplier = __v); }
        }
        let mut prestige_lore: String = ItemUtils::get_concatenated_lore(&slots.get(&((PRESTIGE_SLOT) as i32)).cloned().unwrap());
        let mut prestige_matcher: compat::regex::Matcher = PRESTIGE_REQUIREMENT_PATTERN.matcher(&prestige_lore);
        let mut current_chocolate: Option<i64> = RegexUtils::find_long_from_matcher(&mut prestige_matcher);
        if current_chocolate.is_some() {
            let mut requirement: String = prestige_matcher.group(2).unwrap();
            let mut amount_string: String = (requirement.replace("M", "000000")).replace("B", "000000000");
            // java: NumberUtils.isParsable(amountString) / Long.parseLong(amountString) (no trim: Java throws on whitespace)
            if is_parsable(&amount_string) {
                { let __v = (amount_string.parse::<i64>().expect("java.lang.NumberFormatException")).wrapping_sub(current_chocolate.clone().unwrap()); compat::write(__this, |__t| __t.required_until_next_prestige = __v); }
            }
            { let __v = false; compat::write(__this, |__t| __t.can_prestige = __v); }
        } else if prestige_lore.ends_with("Click to prestige!") {
            { let __v = true; compat::write(__this, |__t| __t.can_prestige = __v); }
            { let __v = false; compat::write(__this, |__t| __t.reached_max_prestige = __v); }
        } else if prestige_lore.ends_with("You have reached max prestige!") {
            { let __v = false; compat::write(__this, |__t| __t.can_prestige = __v); }
            { let __v = true; compat::write(__this, |__t| __t.reached_max_prestige = __v); }
        }
        let mut upgrade_matcher: compat::regex::Matcher = ROMAN_LEVEL_PATTERN.matcher(&(compat::component_of((slots.get(&((PRESTIGE_SLOT) as i32)).cloned().unwrap()).get_hover_name())).get_string());
        { let __v = RegexUtils::find_roman_numeral_from_matcher(&mut upgrade_matcher).unwrap_or(-1); compat::write(__this, |__t| __t.prestige_level = __v); }
        upgrade_matcher = ROMAN_LEVEL_PATTERN.matcher(&(compat::component_of((slots.get(&((HAND_BAKED_CHOCOLATE_SLOT) as i32)).cloned().unwrap()).get_hover_name())).get_string());
        { let __v = RegexUtils::find_roman_numeral_from_matcher(&mut upgrade_matcher).unwrap_or(-1); compat::write(__this, |__t| __t.hand_baked_chocolate_level = __v); }
        upgrade_matcher.reset_with(&(compat::component_of((slots.get(&((TIME_TOWER_SLOT) as i32)).cloned().unwrap()).get_hover_name())).get_string());
        { let __v = RegexUtils::find_roman_numeral_from_matcher(&mut upgrade_matcher).unwrap_or(-1); compat::write(__this, |__t| __t.time_tower_level = __v); }
        { let __v = (compat::read(__this, |__t| __t.time_tower_level) >= 15); compat::write(__this, |__t| __t.is_time_tower_maxed = __v); }
        let mut time_tower_lore: String = ItemUtils::get_concatenated_lore(&slots.get(&((TIME_TOWER_SLOT) as i32)).cloned().unwrap());
        let mut time_tower_multiplier_matcher: compat::regex::Matcher = TIME_TOWER_MULTIPLIER_PATTERN.matcher(&time_tower_lore);
        if let Some(d) = RegexUtils::find_double_from_matcher(&mut time_tower_multiplier_matcher) {
            { let __v = d; compat::write(__this, |__t| __t.time_tower_multiplier = __v); }
        }
        let mut time_tower_status_matcher: compat::regex::Matcher = TIME_TOWER_STATUS_PATTERN.matcher(&time_tower_lore);
        // java: if (timeTowerStatusMatcher.find(timeTowerMultiplierMatcher.hasMatch() ? timeTowerMultiplierMatcher.end() : 0))
        if time_tower_status_matcher.find_from(if time_tower_multiplier_matcher.has_match() { time_tower_multiplier_matcher.end(0) } else { 0 }) {
            { let __v = (time_tower_status_matcher.group(1).as_deref() == Some(&*("ACTIVE"))); compat::write(__this, |__t| __t.is_time_tower_active = __v); }
        }
        upgrade_matcher.reset_with(&(compat::component_of((slots.get(&((RABBIT_SHRINE_SLOT) as i32)).cloned().unwrap()).get_hover_name())).get_string());
        { let __v = RegexUtils::find_roman_numeral_from_matcher(&mut upgrade_matcher).unwrap_or(-1); compat::write(__this, |__t| __t.rabbit_shrine_level = __v); }
        upgrade_matcher.reset_with(&(compat::component_of((slots.get(&((RABBIT_BARN_SLOT) as i32)).cloned().unwrap()).get_hover_name())).get_string());
        { let __v = RegexUtils::find_roman_numeral_from_matcher(&mut upgrade_matcher).unwrap_or(-1); compat::write(__this, |__t| __t.rabbit_barn_level = __v); }
        let mut hitman_matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(&slots.get(&((RABBIT_HITMAN_SLOT) as i32)).cloned().unwrap(), &AVAILABLE_EGGS_PATTERN);
        if let Some(hitman_matcher) = &hitman_matcher {
            { let __v = RegexUtils::parse_int_from_matcher(hitman_matcher, 1); compat::write(__this, |__t| __t.hitman_available_eggs = __v); }
        }
        let mut purchased_slots_matcher: Option<compat::regex::Matcher> = ItemUtils::get_lore_line_if_match(&slots.get(&((RABBIT_HITMAN_SLOT) as i32)).cloned().unwrap(), &PURCHASED_SLOTS_PATTERN);
        if let Some(purchased_slots_matcher) = &purchased_slots_matcher {
            { let __v = RegexUtils::parse_int_from_matcher(purchased_slots_matcher, 1); compat::write(__this, |__t| __t.purchased_hitman_slots = __v); }
            { let __v = RegexUtils::parse_int_from_matcher(purchased_slots_matcher, 2); compat::write(__this, |__t| __t.max_hitman_slots = __v); }
        }
        // java: Comparator.comparingDouble(rabbit -> rabbit.cost() / rabbit.cpsIncrease()) - Double.compare order
        let key = |rabbit: &Rabbit| (rabbit.cost() as f64) / rabbit.cps_increase();
        compat::write(__this, |__t| __t.cps_increase_factors.sort_by(|a: &Rabbit, b: &Rabbit| key(a).total_cmp(&key(b))));
    }

    /// java: private Optional<Rabbit> getCoach(ItemStack coachItem)
    pub fn get_coach(__this: &compat::Shared<Self>, coach_item: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Option<Rabbit> {
        if !(coach_item.is_item(&items::items().get("player_head"))) {
            return None;
        }
        let mut coach_lore: String = ItemUtils::get_concatenated_lore(coach_item);
        if compat::read(__this, |__t| __t.total_cps) < (0 as f64) || compat::read(__this, |__t| __t.total_cps_multiplier) < (0 as f64) {
            return None;
        }
        let mut multiplier_increase_matcher: compat::regex::Matcher = MULTIPLIER_INCREASE_PATTERN.matcher(&coach_lore);
        let mut current_cps_multiplier: Option<f64> = RegexUtils::find_double_from_matcher(&mut multiplier_increase_matcher);
        if current_cps_multiplier.is_none() {
            return None;
        }
        let mut next_cps_multiplier: Option<f64> = RegexUtils::find_double_from_matcher(&mut multiplier_increase_matcher);
        if next_cps_multiplier.is_none() {
            next_cps_multiplier = current_cps_multiplier.clone();
            current_cps_multiplier = Some(0.0);
        }
        let mut level_matcher: compat::regex::Matcher = ROMAN_LEVEL_PATTERN.matcher(&compat::component_of(coach_item.get_hover_name()).get_string());
        let mut level: i32 = RegexUtils::find_roman_numeral_from_matcher(&mut level_matcher).unwrap_or(-1);
        let mut cost_matcher: compat::regex::Matcher = COST_PATTERN.matcher(&coach_lore);
        //Cost comes after the multiplier line
        let mut cost: Option<i64> = RegexUtils::find_long_from_matcher_2(&mut cost_matcher, if multiplier_increase_matcher.has_match() { multiplier_increase_matcher.end(0) } else { 0 });
        if cost.is_none() {
            return None;
        }
        return Some(Rabbit::new(((compat::read(__this, |__t| __t.total_cps) / compat::read(__this, |__t| __t.total_cps_multiplier)) * (next_cps_multiplier.clone().unwrap() - current_cps_multiplier.clone().unwrap())), cost.clone().unwrap(), (COACH_SLOT) as i32, level));
    }

    /// java: private Optional<Rabbit> getRabbit(ItemStack item, int slot)
    pub fn get_rabbit(__this: &compat::Shared<Self>, item: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot: i32) -> Option<Rabbit> {
        let mut lore: String = ItemUtils::get_concatenated_lore(item);
        let mut cps_matcher: compat::regex::Matcher = CPS_INCREASE_PATTERN.matcher(&lore);
        let mut current_cps: Option<i32> = RegexUtils::find_int_from_matcher(&mut cps_matcher);
        if current_cps.is_none() {
            return None;
        }
        let mut next_cps: Option<i32> = RegexUtils::find_int_from_matcher(&mut cps_matcher);
        if next_cps.is_none() {
            next_cps = current_cps.clone();
            current_cps = Some(0);
        }
        let mut cost_matcher: compat::regex::Matcher = COST_PATTERN.matcher(&lore);
        //Cost comes after the cps line
        let mut cost: Option<i64> = RegexUtils::find_long_from_matcher_2(&mut cost_matcher, if cps_matcher.has_match() { cps_matcher.end(0) } else { 0 });
        let mut level_matcher: compat::regex::Matcher = LEVEL_PATTERN.matcher(&lore);
        let mut level: i32 = (RegexUtils::find_int_from_matcher(&mut level_matcher).unwrap_or(0)).wrapping_sub(1);
        if cost.is_none() {
            return None;
        }
        return Some(Rabbit::new((((next_cps.clone().unwrap()).wrapping_sub(current_cps.clone().unwrap()) as f64) * (if compat::read(__this, |__t| __t.total_cps_multiplier) < (0 as f64) { (1) as f64 } else { compat::read(__this, |__t| __t.total_cps_multiplier) })), cost.clone().unwrap(), slot, level));
    }

    /// java: private Optional<ColorHighlight> getPrestigeHighlight()
    pub fn get_prestige_highlight(__this: &compat::Shared<Self>) -> Option<ColorHighlight> {
        if compat::read(__this, |__t| __t.reached_max_prestige) {
            return None;
        }
        if compat::read(__this, |__t| __t.can_prestige) {
            return Some(ColorHighlight::green((PRESTIGE_SLOT) as i32));
        }
        return Some(ColorHighlight::red((PRESTIGE_SLOT) as i32));
    }

    /// java: private List<ColorHighlight> getStrayRabbitHighlight(Int2ObjectMap<ItemStack> slots)
    pub fn get_stray_rabbit_highlight(__this: &compat::Shared<Self>, slots: &std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        compat::set_state(&DING, StraySound::None);
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        {
            let mut i: i32 = STRAY_RABBIT_START;
            while i <= STRAY_RABBIT_END {
                'continue_1: {
                    let mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack = slots.get(&((i) as i32)).cloned().unwrap();
                    if !(item.is_item(&items::items().get("player_head"))) {
                        break 'continue_1;
                    }
                    let mut name: String = compat::component_of(item.get_hover_name()).get_string();
                    if name == "CLICK ME!" || name.starts_with("Golden Rabbit - ") {
                        if SkyblockerConfigManager::get().helpers.chocolate_factory.stray_sound {
                            compat::set_state(&DING, (if name.starts_with("Golden") { StraySound::Golden } else { StraySound::Normal }));
                        }
                        highlights.push(ColorHighlight::green((i) as i32));
                    }
                }
                i += 1;
            }
        }
        return highlights;
    }

    /// java: private boolean addUpgradeTimerToLore(List<Component> lines, long cost)
    pub fn add_upgrade_timer_to_lore(__this: &compat::Shared<Self>, lines: &mut Vec<MutableComponent>, mut cost: i64) -> bool {
        if compat::read(__this, |__t| __t.total_chocolate) < 0 || compat::read(__this, |__t| __t.total_cps) < 0.0 {
            return false;
        }
        lines.push(component::empty().append(component::literal("Time until upgrade: ").with_style_format(ChatFormatting::Gray)).append(ChocolateFactorySolver::format_time(__this, ((cost.wrapping_sub(compat::read(__this, |__t| __t.total_chocolate)) as f64) / compat::read(__this, |__t| __t.total_cps)))));
        return true;
    }

    /// java: private boolean addPrestigeTimerToLore(List<Component> lines)
    pub fn add_prestige_timer_to_lore(__this: &compat::Shared<Self>, lines: &mut Vec<MutableComponent>) -> bool {
        if compat::read(__this, |__t| __t.total_cps) < 0.0 || compat::read(__this, |__t| __t.reached_max_prestige) {
            return false;
        }
        if compat::read(__this, |__t| __t.required_until_next_prestige) > (0 as i64) && !(compat::read(__this, |__t| __t.can_prestige)) {
            lines.push(component::empty().append(component::literal("Chocolate until next prestige: ").with_style_format(ChatFormatting::Gray)).append((component::literal(&formatters::FLOAT_NUMBERS.get_mut().format((compat::read(__this, |__t| __t.required_until_next_prestige)) as f64))).with_style_format(ChatFormatting::Gold)));
        }
        lines.push(component::empty().append(component::literal("Time until next prestige: ").with_style_format(ChatFormatting::Gray)).append(ChocolateFactorySolver::format_time(__this, ((compat::read(__this, |__t| __t.required_until_next_prestige) as f64) / compat::read(__this, |__t| __t.total_cps)))));
        return true;
    }

    /// java: private boolean addTimeTowerStatsToLore(List<Component> lines)
    pub fn add_time_tower_stats_to_lore(__this: &compat::Shared<Self>, lines: &mut Vec<MutableComponent>) -> bool {
        if compat::read(__this, |__t| __t.total_cps) < 0.0 || compat::read(__this, |__t| __t.total_cps_multiplier) < 0.0 || compat::read(__this, |__t| __t.time_tower_multiplier) < 0.0 {
            return false;
        }
        lines.push(component::literal("Current stats:").with_style_format(ChatFormatting::Gray));
        lines.push(component::empty().append(component::literal("  CPS increase: ").with_style_format(ChatFormatting::Gray)).append((component::literal(&formatters::FLOAT_NUMBERS.get_mut().format(((compat::read(__this, |__t| __t.total_cps) / compat::read(__this, |__t| __t.total_cps_multiplier)) * compat::read(__this, |__t| __t.time_tower_multiplier))))).with_style_format(ChatFormatting::Gold)));
        lines.push(component::empty().append(component::literal("  CPS when active: ").with_style_format(ChatFormatting::Gray)).append((component::literal(&formatters::FLOAT_NUMBERS.get_mut().format((if compat::read(__this, |__t| __t.is_time_tower_active) { compat::read(__this, |__t| __t.total_cps) } else { ((compat::read(__this, |__t| __t.total_cps) / compat::read(__this, |__t| __t.total_cps_multiplier)) * (compat::read(__this, |__t| __t.time_tower_multiplier) + compat::read(__this, |__t| __t.total_cps_multiplier))) })))).with_style_format(ChatFormatting::Gold)));
        if !(compat::read(__this, |__t| __t.is_time_tower_maxed)) {
            lines.push(component::literal("Stats after upgrade:").with_style_format(ChatFormatting::Gray));
            lines.push(component::empty().append(component::literal("  CPS increase: ").with_style_format(ChatFormatting::Gray)).append((component::literal(&formatters::FLOAT_NUMBERS.get_mut().format(((compat::read(__this, |__t| __t.total_cps) / compat::read(__this, |__t| __t.total_cps_multiplier)) * (compat::read(__this, |__t| __t.time_tower_multiplier) + 0.1))))).with_style_format(ChatFormatting::Gold)));
            lines.push(component::empty().append(component::literal("  CPS when active: ").with_style_format(ChatFormatting::Gray)).append((component::literal(&formatters::FLOAT_NUMBERS.get_mut().format((if compat::read(__this, |__t| __t.is_time_tower_active) { ((compat::read(__this, |__t| __t.total_cps) / compat::read(__this, |__t| __t.total_cps_multiplier)) * (compat::read(__this, |__t| __t.total_cps_multiplier) + 0.1)) } else { ((compat::read(__this, |__t| __t.total_cps) / compat::read(__this, |__t| __t.total_cps_multiplier)) * ((compat::read(__this, |__t| __t.time_tower_multiplier) + 0.1) + compat::read(__this, |__t| __t.total_cps_multiplier))) })))).with_style_format(ChatFormatting::Gold)));
        }
        return true;
    }

    /// java: private boolean addRabbitStatsToLore(List<Component> lines, int slot)
    pub fn add_rabbit_stats_to_lore(__this: &compat::Shared<Self>, lines: &mut Vec<MutableComponent>, mut slot: i32) -> bool {
        if compat::read(__this, |__t| __t.cps_increase_factors.is_empty()) {
            return false;
        }
        for rabbit in compat::read(__this, |__t| __t.cps_increase_factors.clone()) {
            if rabbit.slot == slot {
                lines.push(component::empty().append(component::literal("CPS Increase: ").with_style_format(ChatFormatting::Gray)).append(component::literal(&formatters::FLOAT_NUMBERS.get_mut().format(rabbit.cps_increase)).with_style_format(ChatFormatting::Gold)));
                lines.push(component::empty().append(component::literal("Cost per CPS: ").with_style_format(ChatFormatting::Gray)).append((component::literal(&formatters::FLOAT_NUMBERS.get_mut().format(((rabbit.cost as f64) / rabbit.cps_increase)))).with_style_format(ChatFormatting::Gold)));
                if rabbit.slot == compat::read(__this, |__t| __t.best_upgrade) {
                    if rabbit.cost <= compat::read(__this, |__t| __t.total_chocolate) {
                        lines.push(component::literal("Best upgrade").with_style_format(ChatFormatting::Green));
                    } else {
                        lines.push((component::literal("Best upgrade, can't afford")).with_style_format(ChatFormatting::Yellow));
                    }
                } else if rabbit.slot == compat::read(__this, |__t| __t.best_affordable_upgrade) && rabbit.cost <= compat::read(__this, |__t| __t.total_chocolate) {
                    lines.push(component::literal("Best upgrade you can afford").with_style_format(ChatFormatting::Green));
                }
                return true;
            }
        }
        return false;
    }

    /// java: private MutableComponent formatTime(double seconds)
    pub fn format_time(__this: &compat::Shared<Self>, mut seconds: f64) -> MutableComponent {
        return SkyblockTime::format_time(seconds).with_style_format(ChatFormatting::Gold);
    }

}

impl SimpleContainerSolver for ChocolateFactorySolver {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase {
        &self.base
    }
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase {
        &mut self.base
    }

}

impl RegexContainerMatcher for ChocolateFactorySolver {
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

impl ContainerMatcher for ChocolateFactorySolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        ChocolateFactorySolver::is_enabled_dyn
    }

    /// java: @Override public boolean test(Screen screen) (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_from::<Self>
    }

}

impl ContainerSolver for ChocolateFactorySolver {
    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots) (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        ChocolateFactorySolver::get_colors_dyn
    }

}

impl Resettable for ChocolateFactorySolver {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Resettable>) {
        ChocolateFactorySolver::reset_dyn
    }

}

impl TooltipAdder for ChocolateFactorySolver {
    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines) (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>) {
        ChocolateFactorySolver::add_to_tooltip_dyn
    }

    /// java: @Override public int getPriority() (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32 {
        ChocolateFactorySolver::get_priority_dyn
    }

}

impl SlotTextAdder for ChocolateFactorySolver {
    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId) (dispatch: the body for this class)
    fn get_text(&self) -> fn(&compat::Shared<dyn SlotTextAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32) -> Vec<SlotText> {
        ChocolateFactorySolver::get_text_dyn
    }

}

impl ChocolateFactorySolver {
    /// java: @Override public boolean isEnabled()
    pub fn is_enabled_impl(__this: &compat::Shared<Self>) -> bool {
        return SkyblockerConfigManager::get().helpers.chocolate_factory.enable_chocolate_factory_helper;
    }

    /// ChocolateFactorySolver.isEnabled as the dispatch of ContainerMatcher.isEnabled
    pub fn is_enabled_dyn(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        Self::is_enabled_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots)
    pub fn get_colors_impl(__this: &compat::Shared<Self>, slots: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        ChocolateFactorySolver::update_factory_info(__this, slots);
        let mut highlights: Vec<ColorHighlight> = Vec::new();
        if let Some(__a0) = ChocolateFactorySolver::get_prestige_highlight(__this) {
            highlights.push(__a0.clone());
        }
        highlights.extend(ChocolateFactorySolver::get_stray_rabbit_highlight(__this, slots).into_iter());
        if compat::read(__this, |__t| __t.total_chocolate) <= (0 as i64) || compat::read(__this, |__t| __t.cps_increase_factors.is_empty()) {
            return highlights;
        }
        let best_rabbit: Rabbit = compat::read(__this, |__t| __t.cps_increase_factors[0].clone());
        { let __v = best_rabbit.slot; compat::write(__this, |__t| __t.best_upgrade = __v); }
        if best_rabbit.cost <= compat::read(__this, |__t| __t.total_chocolate) {
            highlights.push(ColorHighlight::green(best_rabbit.slot));
            return highlights;
        }
        highlights.push(ColorHighlight::yellow(best_rabbit.slot));
        for rabbit in compat::read(__this, |__t| __t.cps_increase_factors[1..].to_vec()) {
            if rabbit.cost <= compat::read(__this, |__t| __t.total_chocolate) {
                { let __v = rabbit.slot; compat::write(__this, |__t| __t.best_affordable_upgrade = __v); }
                highlights.push(ColorHighlight::green(rabbit.slot));
                break;
            }
        }
        return highlights;
    }

    /// ChocolateFactorySolver.getColors as the dispatch of ContainerSolver.getColors
    pub fn get_colors_dyn(__this: &compat::Shared<dyn ContainerSolver>, __p0: &mut std::collections::BTreeMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight> {
        Self::get_colors_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void reset()
    pub fn reset_impl(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| __t.cps_increase_factors.clear());
        { let __v = -1; compat::write(__this, |__t| __t.total_chocolate = __v); }
        { let __v = -1.0; compat::write(__this, |__t| __t.total_cps = __v); }
        { let __v = -1.0; compat::write(__this, |__t| __t.total_cps_multiplier = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.required_until_next_prestige = __v); }
        { let __v = false; compat::write(__this, |__t| __t.can_prestige = __v); }
        { let __v = false; compat::write(__this, |__t| __t.reached_max_prestige = __v); }
        { let __v = -1.0; compat::write(__this, |__t| __t.time_tower_multiplier = __v); }
        { let __v = false; compat::write(__this, |__t| __t.is_time_tower_maxed = __v); }
        { let __v = false; compat::write(__this, |__t| __t.is_time_tower_active = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.best_upgrade = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.best_affordable_upgrade = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.prestige_level = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.hitman_available_eggs = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.purchased_hitman_slots = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.max_hitman_slots = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.rabbit_shrine_level = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.hand_baked_chocolate_level = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.rabbit_barn_level = __v); }
        { let __v = -1; compat::write(__this, |__t| __t.time_tower_level = __v); }
        compat::set_state(&DING_TICK, 0);
        compat::set_state(&DING, StraySound::None);
    }

    /// ChocolateFactorySolver.reset as the dispatch of Resettable.reset
    pub fn reset_dyn(__this: &compat::Shared<dyn Resettable>) {
        Self::reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines)
    pub fn add_to_tooltip_impl(__this: &compat::Shared<Self>, mut focused_slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, lines: &mut Vec<MutableComponent>) {
        let Some(mut focused_slot) = focused_slot.clone() else {
            return;
        };
        let mut line_index: i32 = (lines.len() as i32);
        let mut should_add_line: bool = false;
        let mut lore: String = ItemUtils::concatenate_lore(lines);
        let mut cost_matcher: compat::regex::Matcher = COST_PATTERN.matcher(&lore);
        let mut cost: Option<i64> = RegexUtils::find_long_from_matcher(&mut cost_matcher);
        if cost.is_some() {
            should_add_line = ((should_add_line | ChocolateFactorySolver::add_upgrade_timer_to_lore(__this, lines, cost.clone().unwrap()))) as bool;
        }
        let mut index: i32 = focused_slot.index;
        if index == (PRESTIGE_SLOT as i32) {
            should_add_line = ((should_add_line | ChocolateFactorySolver::add_prestige_timer_to_lore(__this, lines))) as bool;
        } else if index == (TIME_TOWER_SLOT as i32) {
            should_add_line = ((should_add_line | ChocolateFactorySolver::add_time_tower_stats_to_lore(__this, lines))) as bool;
        } else if index == (COACH_SLOT as i32) || index >= (RABBITS_START as i32) && index <= (RABBITS_END as i32) {
            should_add_line = ((should_add_line | ChocolateFactorySolver::add_rabbit_stats_to_lore(__this, lines, index))) as bool;
        }
        if should_add_line {
            lines.insert((line_index) as usize, LineSmoothener::create_smooth_line());
        }
    }

    /// ChocolateFactorySolver.addToTooltip as the dispatch of TooltipAdder.addToTooltip
    pub fn add_to_tooltip_dyn(__this: &compat::Shared<dyn TooltipAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: &mut Vec<MutableComponent>) {
        Self::add_to_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public int getPriority()
    pub fn get_priority_impl(__this: &compat::Shared<Self>) -> i32 {
        return 0;
    }

    /// ChocolateFactorySolver.getPriority as the dispatch of TooltipAdder.getPriority
    pub fn get_priority_dyn(__this: &compat::Shared<dyn TooltipAdder>) -> i32 {
        Self::get_priority_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<SlotText> getText(@Nullable Slot slot, ItemStack stack, int slotId)
    pub fn get_text_impl(__this: &compat::Shared<Self>, mut slot: Option<Slot>, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut slot_id: i32) -> Vec<SlotText> {
        for rabbit in compat::read(__this, |__t| __t.cps_increase_factors.clone()) {
            if slot_id == rabbit.slot() {
                let mut level_text: MutableComponent = component::literal(&rabbit.level().to_string()).with_style_format(ChatFormatting::Gold);
                return SlotText::top_left_list(&level_text);
            }
        }
        return match slot_id {
            HAND_BAKED_CHOCOLATE_SLOT => 'yield_2: {
                if compat::read(__this, |__t| __t.hand_baked_chocolate_level) < 0 || compat::read(__this, |__t| __t.hand_baked_chocolate_level) == 10 {
                    break 'yield_2 vec![];
                }
                let mut level_text: MutableComponent = (component::literal(&compat::read(__this, |__t| __t.hand_baked_chocolate_level).to_string())).with_style_format(ChatFormatting::Gold);
                break 'yield_2 SlotText::top_left_list(&level_text);
            }
            RABBIT_SHRINE_SLOT => 'yield_3: {
                if compat::read(__this, |__t| __t.rabbit_shrine_level) < 0 || compat::read(__this, |__t| __t.rabbit_shrine_level) == 20 {
                    break 'yield_3 vec![];
                }
                let mut level_text: MutableComponent = (component::literal(&compat::read(__this, |__t| __t.rabbit_shrine_level).to_string())).with_style_format(ChatFormatting::Gold);
                break 'yield_3 SlotText::top_left_list(&level_text);
            }
            RABBIT_BARN_SLOT => 'yield_4: {
                if compat::read(__this, |__t| __t.rabbit_barn_level) < 0 || compat::read(__this, |__t| __t.rabbit_barn_level) == 245 {
                    break 'yield_4 vec![];
                }
                let mut level_text: MutableComponent = (component::literal(&compat::read(__this, |__t| __t.rabbit_barn_level).to_string())).with_style_format(ChatFormatting::Gold);
                break 'yield_4 SlotText::top_left_list(&level_text);
            }
            TIME_TOWER_SLOT => 'yield_5: {
                if compat::read(__this, |__t| __t.time_tower_level) < 0 || compat::read(__this, |__t| __t.is_time_tower_maxed) {
                    break 'yield_5 vec![];
                }
                let mut level_text: MutableComponent = (component::literal(&compat::read(__this, |__t| __t.time_tower_level).to_string())).with_style_format(ChatFormatting::Gold);
                break 'yield_5 SlotText::top_left_list(&level_text);
            }
            RABBIT_HITMAN_SLOT => 'yield_6: {
                if compat::read(__this, |__t| __t.hitman_available_eggs) < 0 || compat::read(__this, |__t| __t.purchased_hitman_slots) < 0 || compat::read(__this, |__t| __t.max_hitman_slots) < 0 {
                    break 'yield_6 vec![];
                }
                if compat::read(__this, |__t| __t.purchased_hitman_slots) == 0 {
                    break 'yield_6 SlotText::top_left_list(&(component::literal("0/0")).with_style_format(ChatFormatting::Gray));
                }
                let mut level_text: MutableComponent = component::literal(&compat::read(__this, |__t| __t.hitman_available_eggs).to_string());
                if compat::read(__this, |__t| __t.hitman_available_eggs) == compat::read(__this, |__t| __t.purchased_hitman_slots) {
                    level_text = level_text.with_style_formats(&[ChatFormatting::Gold, ChatFormatting::Bold]);
                } else if compat::read(__this, |__t| __t.hitman_available_eggs) == 0 {
                    level_text = level_text.with_style_format(ChatFormatting::Gray);
                } else {
                    level_text = level_text.with_style_format(ChatFormatting::Yellow);
                }
                let mut result: MutableComponent = component::empty().append(level_text.clone());
                if compat::read(__this, |__t| __t.purchased_hitman_slots) < compat::read(__this, |__t| __t.max_hitman_slots) {
                    result = (result.append((component::literal("/")).with_style_format(ChatFormatting::Gray))).append((component::literal(&compat::read(__this, |__t| __t.purchased_hitman_slots).to_string())).with_style_format(ChatFormatting::Yellow));
                }
                break 'yield_6 SlotText::top_left_list(&result);
            }
            PRESTIGE_SLOT => 'yield_7: {
                if compat::read(__this, |__t| __t.prestige_level) < 0 {
                    break 'yield_7 vec![];
                }
                let mut level_text: MutableComponent = (component::literal(&compat::read(__this, |__t| __t.prestige_level).to_string())).with_style_format(ChatFormatting::Gold);
                break 'yield_7 SlotText::top_left_list(&level_text);
            }
            _ => vec![],
        };
    }

    /// ChocolateFactorySolver.getText as the dispatch of SlotTextAdder.getText
    pub fn get_text_dyn(__this: &compat::Shared<dyn SlotTextAdder>, __p0: Option<Slot>, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack, __p2: i32) -> Vec<SlotText> {
        Self::get_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}

/// java: private enum StraySound
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StraySound {
    None,
    Normal,
    Golden,
}

impl StraySound {
    pub const VALUES: [StraySound; 3] = [StraySound::None, StraySound::Normal, StraySound::Golden];

    /// java: values()
    pub fn values() -> Vec<StraySound> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            StraySound::None => "NONE",
            StraySound::Normal => "NORMAL",
            StraySound::Golden => "GOLDEN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> StraySound {
        match name {
            "NONE" => StraySound::None,
            "NORMAL" => StraySound::Normal,
            "GOLDEN" => StraySound::Golden,
            _ => panic!("IllegalArgumentException: No enum constant StraySound.{name}"),
        }
    }

}

impl std::fmt::Display for StraySound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: public record Rabbit(double cpsIncrease, long cost, int slot, int level)
#[derive(Clone, Debug, PartialEq)]
pub struct Rabbit {
    /// java: double cpsIncrease
    pub cps_increase: f64,
    /// java: long cost
    pub cost: i64,
    /// java: int slot
    pub slot: i32,
    /// java: int level
    pub level: i32,
}

impl Rabbit {
    /// java: the canonical record constructor
    pub fn new(mut cps_increase: f64, mut cost: i64, mut slot: i32, mut level: i32) -> Self {
        Self { cps_increase, cost, slot, level }
    }

    /// java: the record accessor `cpsIncrease()`
    pub fn cps_increase(&self) -> f64 {
        self.cps_increase
    }

    /// java: the record accessor `cost()`
    pub fn cost(&self) -> i64 {
        self.cost
    }

    /// java: the record accessor `slot()`
    pub fn slot(&self) -> i32 {
        self.slot
    }

    /// java: the record accessor `level()`
    pub fn level(&self) -> i32 {
        self.level
    }

}

/// java: org.apache.commons.lang3.math.NumberUtils.isParsable(String) - a private copy (no commons-lang in the port)
fn is_parsable(str: &str) -> bool {
    if str.is_empty() {
        return false;
    }
    if str.ends_with('.') {
        return false;
    }
    let chars: Vec<char> = str.chars().collect();
    let begin_idx = if chars[0] == '-' {
        if chars.len() == 1 {
            return false;
        }
        1
    } else {
        0
    };
    // withDecimalsParsing(str, beginIdx)
    let mut decimal_points = 0;
    for &c in &chars[begin_idx..] {
        let is_decimal_point = c == '.';
        if is_decimal_point {
            decimal_points += 1;
        }
        if decimal_points > 1 {
            return false;
        }
        if !is_decimal_point && !c.is_ascii_digit() {
            return false;
        }
    }
    true
}
