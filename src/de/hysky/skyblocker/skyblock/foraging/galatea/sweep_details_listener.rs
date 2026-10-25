//! Ported from de.hysky.skyblocker.skyblock.foraging.galatea.SweepDetailsListener by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::foraging::galatea::sweep_details_hud_widget;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: protected static final Pattern SWEEP_DETAILS = Pattern.compile(String.format("Sweep Details: ([\\d,.]+)[∮%s] Sweep", SkyBlockIcons.SWEEP));
pub static SWEEP_DETAILS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("Sweep Details: ([\\d,.]+)[∮{}] Sweep", sky_block_icons::SWEEP)));
/// java: protected static final Pattern TREE_TOUGHNESS = Pattern.compile(" (.+?) Tree Toughness: ([\\d.]+) ([\\d.]+) Logs");
pub static TREE_TOUGHNESS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("  (.+?) Tree Toughness: ([\\d.]+) ([\\d.]+) Logs"));
/// java: protected static final Pattern AXE_THROW_PENALTY = Pattern.compile(" Axe throw: (-\\d+)% Sweep ([\\d.]+) Logs");
pub static AXE_THROW_PENALTY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("  Axe throw: (-\\d+)% Sweep ([\\d.]+) Logs"));
/// java: protected static final Pattern WRONG_STYLE_PENALTY = Pattern.compile(" Wrong Style: (-\\d+)% Sweep ([\\d.]+) Logs ([a-zA-Z ]*)!!");
pub static WRONG_STYLE_PENALTY: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("  Wrong Style: (-\\d+)% Sweep ([\\d.]+) Logs ([a-zA-Z ]*)!!"));
/// java: protected static final long TIMEOUT_MS = 5_000;
pub const TIMEOUT_MS: i64 = ((5000) as i64) as i64;
/// java: public static boolean active = false;
pub static ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static long lastMatch = -1;
pub static LAST_MATCH: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(((-1) as i64) as i64));
/// java: public static float maxSweep = -1;
pub static MAX_SWEEP: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(((-1) as f32) as f32));
/// java: public static float lastSweep = -1;
pub static LAST_SWEEP: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(((-1) as f32) as f32));
/// java: public static String lastTreeType = "Unknown";
pub static LAST_TREE_TYPE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("Unknown".to_string()));
/// java: public static String toughness = "";
pub static TOUGHNESS: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: public static String logs = "";
pub static LOGS: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));
/// java: public static boolean axePenalty;
pub static AXE_PENALTY: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static float axePenaltyAmount;
pub static AXE_PENALTY_AMOUNT: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(0.0));
/// java: public static boolean stylePenalty;
pub static STYLE_PENALTY: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static float stylePenaltyAmount;
pub static STYLE_PENALTY_AMOUNT: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(0.0));
/// java: public static String correctStyle = "";
pub static CORRECT_STYLE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

/// java: @SuppressWarnings("RegExpRepeatedSpace") // followup messages have 2 leading spaces public class SweepDetailsListener implements ChatMessageListener
#[derive(Clone)]
pub struct SweepDetailsListener {}

impl SweepDetailsListener {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

    /// java: protected static void resetStats()
    pub fn reset_stats() {
        compat::set_state(&ACTIVE, false);
        compat::set_state(&LAST_MATCH, ((-1) as i64) as i64);
        compat::set_state(&MAX_SWEEP, ((-1) as f32) as f32);
        compat::set_state(&LAST_SWEEP, ((-1) as f32) as f32);
        compat::set_state(&LAST_TREE_TYPE, "Unknown".to_string());
        compat::set_state(&AXE_PENALTY, false);
        compat::set_state(&STYLE_PENALTY, false);
        compat::set_state(&TOUGHNESS, "".to_string());
        compat::set_state(&LOGS, "".to_string());
        compat::set_state(&AXE_PENALTY_AMOUNT, ((-1) as f32) as f32);
        compat::set_state(&STYLE_PENALTY_AMOUNT, ((-1) as f32) as f32);
        compat::set_state(&CORRECT_STYLE, "".to_string());
    }

    /// java: private static float parsePenalty(String rawPenaltyAmount)
    pub fn parse_penalty(raw_penalty_amount: &str) -> f32 {
        let mut penalty_amount: f32;
        if compat::number_utils_is_creatable(raw_penalty_amount) {
            penalty_amount = raw_penalty_amount.trim().parse::<f32>().expect("NumberFormatException");
            compat::set_state(&LAST_SWEEP, (compat::get_state(&LAST_SWEEP) * ((1 as f32) - ((-penalty_amount) / 100.0f32))));
        } else {
            penalty_amount = -1.0f32;
        }
        return penalty_amount;
    }

    /// java: private static void logMessage(String chatMessage)
    pub fn log_message(chat_message: &str) {
        logger().info(&format!("[CHAT] {}", chat_message));
    }

}

impl ChatMessageListener for SweepDetailsListener {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        SweepDetailsListener::on_message_dyn
    }

}

impl SweepDetailsListener {
    /// java: @Override public ChatFilterResult onMessage(Component message, String asString)
    pub fn on_message_impl(__this: &compat::Shared<Self>, message: &MutableComponent, as_string: &str) -> ChatFilterResult {
        if !(sweep_details_hud_widget::LOCATIONS.contains(&Utils::get_location())) {
            return ChatFilterResult::Pass;
        }
        if !(sweep_details_hud_widget::INSTANCE.get().clone().map_or(false, |__w| WidgetManager::is_widget_in_current_screen(&(__w as compat::Shared<dyn HudWidget>)))) {
            return ChatFilterResult::Pass;
        }
        let mut msg: String = message.get_string();
        let mut sweep_details: compat::regex::Matcher = SWEEP_DETAILS.matcher(&msg);
        if sweep_details.matches() {
            SweepDetailsListener::reset_stats();
            compat::set_state(&ACTIVE, true);
            compat::set_state(&LAST_MATCH, compat::millis());
            let mut raw_max_sweep: String = sweep_details.group(1).unwrap().replace(",", "");
            if compat::number_utils_is_creatable(&raw_max_sweep) {
                compat::set_state(&MAX_SWEEP, raw_max_sweep.trim().parse::<f32>().expect("NumberFormatException"));
            } else {
                compat::set_state(&MAX_SWEEP, ((-1) as f32) as f32);
            }
            compat::set_state(&LAST_SWEEP, compat::get_state(&MAX_SWEEP));
            SweepDetailsListener::log_message(&msg);
            return ChatFilterResult::Filter;
        }
        if compat::get_state(&ACTIVE) && compat::millis() > (compat::get_state(&LAST_MATCH)).wrapping_add(TIMEOUT_MS) {
            SweepDetailsListener::reset_stats();
        }
        if !(compat::get_state(&ACTIVE)) {
            return ChatFilterResult::Pass;
        }
        let mut tree_toughness: compat::regex::Matcher = TREE_TOUGHNESS.matcher(&msg);
        if tree_toughness.matches() {
            compat::set_state(&LAST_TREE_TYPE, tree_toughness.group(1).unwrap());
            compat::set_state(&TOUGHNESS, tree_toughness.group(2).unwrap());
            compat::set_state(&LOGS, tree_toughness.group(3).unwrap());
            SweepDetailsListener::log_message(&msg);
            return ChatFilterResult::Filter;
        }
        let mut axe_throw: compat::regex::Matcher = AXE_THROW_PENALTY.matcher(&msg);
        if axe_throw.matches() {
            compat::set_state(&AXE_PENALTY, true);
            compat::set_state(&AXE_PENALTY_AMOUNT, SweepDetailsListener::parse_penalty(axe_throw.group(1).as_deref().unwrap()));
            compat::set_state(&LOGS, axe_throw.group(2).unwrap());
            SweepDetailsListener::log_message(&msg);
            return ChatFilterResult::Filter;
        }
        let mut wrong_style: compat::regex::Matcher = WRONG_STYLE_PENALTY.matcher(&msg);
        if wrong_style.matches() {
            compat::set_state(&STYLE_PENALTY, true);
            compat::set_state(&STYLE_PENALTY_AMOUNT, SweepDetailsListener::parse_penalty(wrong_style.group(1).as_deref().unwrap()));
            compat::set_state(&LOGS, wrong_style.group(2).unwrap());
            compat::set_state(&CORRECT_STYLE, wrong_style.group(3).unwrap());
            SweepDetailsListener::log_message(&msg);
            return ChatFilterResult::Filter;
        }
        return ChatFilterResult::Pass;
    }

    /// SweepDetailsListener.onMessage as the dispatch of ChatMessageListener.onMessage
    pub fn on_message_dyn(__this: &compat::Shared<dyn ChatMessageListener>, __p0: &MutableComponent, __p1: &str) -> ChatFilterResult {
        Self::on_message_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
