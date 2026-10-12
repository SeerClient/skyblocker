//! Ported from de.hysky.skyblocker.skyblock.garden.FarmingHud by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::world_events;
use crate::de::hysky::skyblocker::skyblock::garden::farming_hud_widget::FarmingHudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::nbt::compound_tag;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::world::level::block::blocks;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: public static final NumberFormat NUMBER_FORMAT = NumberFormat.getInstance(Locale.US);
///
/// `NumberFormat.getInstance(Locale.US)` is the US NUMBER pattern `#,##0.###`.
pub static NUMBER_FORMAT: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| DecimalFormat::new("#,##0.###", mcrust::java::text::decimal_format_symbols::get_instance(Locale::Us)));
/// java: private static final Pattern FARMING_XP = Pattern.compile("\\+(?<xp>\\d+(?:\\.\\d+)?) Farming \\((?:(?<percent>[\\d,]+(?:\\.\\d+)?%)|(?<current>[\\d,]+)/(?<max>[\\d,]+k?))\\)");
pub static FARMING_XP: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\+(?<xp>\\d+(?:\\.\\d+)?) Farming \\((?:(?<percent>[\\d,]+(?:\\.\\d+)?%)|(?<current>[\\d,]+)/(?<max>[\\d,]+k?))\\)"));
/// java: private static final int STATS_WINDOW = 5_000;
pub const STATS_WINDOW: i32 = 5000;
/// java: private static CounterType counterType = CounterType.NONE;
pub static COUNTER_TYPE: LazyLock<Mutex<CounterType>> = LazyLock::new(|| Mutex::new(CounterType::None));
/// java: private static final Deque<LongLongPair> counter = new ArrayDeque<>();
pub static COUNTER: compat::MainThread<std::collections::VecDeque<(i64, i64)>> = compat::MainThread::new(|| std::collections::VecDeque::new());
// TODO(fabric2seer): `new LongArrayFIFOQueue(..)`
//   java: private static final LongPriorityQueue blockBreaks = new LongArrayFIFOQueue();
/// java: private static final LongPriorityQueue blockBreaks = new LongArrayFIFOQueue();
///
/// `LongArrayFIFOQueue`: first in, first out (`enqueue` at the back, `dequeueLong` / `firstLong` at the front).
pub static BLOCK_BREAKS: compat::MainThread<std::collections::VecDeque<i64>> = compat::MainThread::new(|| std::collections::VecDeque::new());
/// java: private static final Queue<FloatLongPair> farmingXp = new ArrayDeque<>();
pub static FARMING_XP_2: compat::MainThread<std::collections::VecDeque<(f32, i64)>> = compat::MainThread::new(|| std::collections::VecDeque::new());
/// java: private static float farmingXpPercentProgress;
pub static FARMING_XP_PERCENT_PROGRESS: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(0.0));

/// java: public class FarmingHud
#[derive(Clone)]
pub struct FarmingHud {}

impl FarmingHud {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            if FarmingHud::should_render() {
                if !(COUNTER.get_mut().is_empty()) && (COUNTER.get_mut().front().cloned().unwrap().1).wrapping_add(STATS_WINDOW as i64) < compat::millis() {
                    COUNTER.get_mut().pop_front();
                }
                if !(BLOCK_BREAKS.get_mut().is_empty()) && (BLOCK_BREAKS.get_mut().front().cloned().unwrap()).wrapping_add(STATS_WINDOW as i64) < compat::millis() {
                    BLOCK_BREAKS.get_mut().pop_front();
                }
                if !(FARMING_XP_2.get_mut().is_empty()) && (FARMING_XP_2.get_mut().front().cloned().unwrap().1).wrapping_add(STATS_WINDOW as i64) < compat::millis() {
                    FARMING_XP_2.get_mut().pop_front();
                }
                debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
                let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
                if FarmingHud::try_get_counter(&stack, CounterType::Cultivating) {
                    compat::set_state(&COUNTER_TYPE, CounterType::None);
                }
            }
        }));
        // TODO(fabric2seer): ClientPlayerBlockBreakEvents.AFTER.register: no SeerClient equivalent yet
        //   java: ClientPlayerBlockBreakEvents.AFTER.register((_, _, _, _) -> {
        //   java: 			if (shouldRender()) {
        //   java: 				blockBreaks.enqueue(System.currentTimeMillis());
        //   java: 			}
        //   java: 		});
        world_events::BLOCK_STATE_UPDATE.register(Arc::new(move |pos, old_state, new_state| {
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() || !(FarmingHud::should_render()) {
                return;
            }
            if false /* java: old_state == null: never null here */ {
                return;
            }
            if !new_state.is_air() || !(old_state.is_block(&blocks::blocks().get("CACTUS"))) {
                return;
            }
            if !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos.below()).is_block(&blocks::blocks().get("CACTUS"))) {
                return;
            }
            if mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.distance_to_sqr((pos.get_x()) as f64, (pos.get_y()) as f64, (pos.get_z()) as f64)).unwrap() > (64 as f64) {
                return;
            }
            if !(mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().get_neu_name().starts_with("CACTUS_KNIFE")).unwrap_or(false)) {
                return;
            }
            BLOCK_BREAKS.get_mut().push_back(compat::millis());
        }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register((message, overlay) -> {
        //   java: 			if (shouldRender() && overlay) {
        //   java: 				Matcher matcher = FARMING_XP.matcher(ChatFormatting.stripFormatting(message.getString()));
        //   java: 				if (matcher.find()) {
        //   java: 					try {
        //   java: 						farmingXp.offer(FloatLongPair.of(NUMBER_FORMAT.parse(matcher.group("xp")).floatValue(), System.currentTimeMillis()));
        //   java: 						if (matcher.group("percent") != null) {
        //   java: 							farmingXpPercentProgress = NUMBER_FORMAT.parse(matcher.group("percent")).floatValue();
        //   java: 						} else if (matcher.group("current") != null) {
        //   java: 							float max = NUMBER_FORMAT.parse(matcher.group("max")).floatValue();
        //   java: 							if (matcher.group("max").endsWith("k")) max *= 1000;
        //   java: 							farmingXpPercentProgress = NUMBER_FORMAT.parse(matcher.group("current")).floatValue() / max * 100;
        //   java: 						}
        //   java: 					} catch (ParseException e) {
        //   java: 						LOGGER.error("[Skyblocker Farming HUD] Failed to parse farming xp", e);
        //   java: 					}
        //   java: 				}
        //   java: 			}
        //   java:
        //   java: 			return true;
        //   java: 		});
    }

    /// java: @SuppressWarnings("SameParameterValue") private static boolean tryGetCounter(ItemStack stack, CounterType counterType)
    pub fn try_get_counter(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut counter_type: CounterType) -> bool {
        let mut custom_data: Tag = ItemUtils::get_custom_data(stack);
        if custom_data.compound_is_empty() || !(custom_data.get(&counter_type.fields().nbt_key).is_some_and(|tag| tag.is_numeric())) {
            return true;
        }
        let mut count: i64 = custom_data.get_long_or(&counter_type.fields().nbt_key, (0) as i64);
        if compat::get_state(&COUNTER_TYPE) != counter_type {
            COUNTER.get_mut().clear();
            compat::set_state(&COUNTER_TYPE, counter_type);
        }
        if COUNTER.get_mut().is_empty() || COUNTER.get_mut().back().cloned().unwrap().0 != count {
            COUNTER.get_mut().push_back((count, compat::millis()));
        }
        return false;
    }

    /// java: private static boolean shouldRender()
    pub fn should_render() -> bool {
        return (WidgetManager::is_widget_in_current_screen(&(FarmingHudWidget::get_instance() as compat::Shared<dyn HudWidget>)) && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && Utils::get_location() == Location::Garden);
    }

    /// java: public static CounterType counterType()
    pub fn counter_type() -> CounterType {
        return compat::get_state(&COUNTER_TYPE);
    }

    /// java: public static long counter()
    pub fn counter() -> i64 {
        return (if COUNTER.get_mut().is_empty() { ((0) as i64) as i64 } else { COUNTER.get_mut().back().cloned().unwrap().0 });
    }

    /// java: public static float cropsPerMinute()
    pub fn crops_per_minute() -> f32 {
        if COUNTER.get_mut().is_empty() {
            return ((0) as f32) as f32;
        }
        let mut first: (i64, i64) = COUNTER.get_mut().front().cloned().unwrap();
        let mut last: (i64, i64) = COUNTER.get_mut().back().cloned().unwrap();
        return ((((last.0.wrapping_sub(first.0)) as f32) / (last.1.wrapping_sub(first.1) as f32)) * 60000.0f32);
    }

    /// java: public static double blockBreaks()
    pub fn block_breaks() -> f64 {
        if BLOCK_BREAKS.get_mut().is_empty() {
            return ((0) as f64) as f64;
        }
        let mut first_timestamp: i64 = *BLOCK_BREAKS.get_mut().front().unwrap();
        let mut last_timestamp: i64 = *BLOCK_BREAKS.get_mut().back().unwrap();
        return (compat::java_round_f64((((BLOCK_BREAKS.get_mut().len() as i32) - 1) as f64) / ((last_timestamp.wrapping_sub(first_timestamp)) as f64) * (10000 as f64)) as f64) / 10.0;
    }

    /// java: public static float farmingXpPercentProgress()
    pub fn farming_xp_percent_progress() -> f32 {
        return (compat::get_state(&FARMING_XP_PERCENT_PROGRESS)).clamp((0 as f32), (100 as f32));
    }

    /// java: public static double farmingXpPerHour()
    pub fn farming_xp_per_hour() -> f64 {
        if FARMING_XP_2.get_mut().is_empty() {
            return ((0) as f64) as f64;
        }
        return (compat::java_round_f64((FARMING_XP_2.get_mut().front().unwrap().0 as f64) * FarmingHud::block_breaks() * (3600 as f64) * (10 as f64)) as f64) / 10.0;
    }

}

/// java: public enum CounterType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CounterType {
    None,
    Cultivating,
}

impl CounterType {
    pub const VALUES: [CounterType; 2] = [CounterType::None, CounterType::Cultivating];

    /// java: values()
    pub fn values() -> Vec<CounterType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            CounterType::None => "NONE",
            CounterType::Cultivating => "CULTIVATING",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> CounterType {
        match name {
            "NONE" => CounterType::None,
            "CULTIVATING" => CounterType::Cultivating,
            _ => panic!("IllegalArgumentException: No enum constant CounterType.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static CounterTypeFields {
        static FIELDS: LazyLock<Vec<CounterTypeFields>> = LazyLock::new(|| vec![
            CounterTypeFields::new("".to_string(), "Cultivating Needed".to_string()),
            CounterTypeFields::new("farmed_cultivating".to_string(), "Cultivating: ".to_string()),
        ]);
        &FIELDS[self as usize]
    }

}

impl std::fmt::Display for CounterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each CounterType constant
pub struct CounterTypeFields {
    pub nbt_key: String,
    pub text: String,
}

impl CounterTypeFields {
    fn new(mut nbt_key: String, mut text: String) -> Self {
        let mut this = Self {
            nbt_key: Default::default(),
            text: Default::default(),
        };
        this.nbt_key = nbt_key.clone();
        this.text = text.clone();
        this
    }

}
