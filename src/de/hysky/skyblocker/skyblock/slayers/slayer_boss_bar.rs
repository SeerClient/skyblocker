//! Ported from de.hysky.skyblocker.skyblock.slayers.SlayerBossBar by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::BossFight;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::uuid::Uuid;
use mcrust::net::minecraft::client::gui::components::lerping_boss_event::LerpingBossEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::boss_event::BossEvent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Pattern HEALTH_PATTERN = Pattern.compile("(\\d{1,3}(?:,\\d{3})*(?:\\.\\d+)?[kM]?)(?=❤)");
pub static HEALTH_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(\\d{1,3}(?:,\\d{3})*(?:\\.\\d+)?[kM]?)(?=❤)"));
/// java: private static int bossMaxHealth = -1;
pub static BOSS_MAX_HEALTH: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));
/// java: private static @Nullable LerpingBossEvent bossBar;
pub static BOSS_BAR: compat::MainThread<Option<LerpingBossEvent>> = compat::MainThread::new(|| None);

/// java: public class SlayerBossBar
#[derive(Clone)]
pub struct SlayerBossBar {}

impl SlayerBossBar {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean shouldRenderBossBar()
    pub fn should_render_boss_bar() -> bool {
        if !(SlayerManager::is_fighting_slayer()) {
            *BOSS_BAR.get_mut() = None;
            compat::set_state(&BOSS_MAX_HEALTH, -1);
            return false;
        }
        if compat::get_state(&BOSS_MAX_HEALTH) == -1 {
            let mut boss_fight: Option<compat::Shared<BossFight>> = SlayerManager::get_boss_fight();
            if let Some(boss_fight) = &boss_fight {
                // the stand by id; a stand the level dropped has no name to read
                let Some(mut boss_name) = mcrust::mcrust::modding::game::with_entity(compat::read(boss_fight, |__t| __t.armor_stand), |e| e.get_name().get_string()) else {
                    return true;
                };
                let mut max_health_matcher: compat::regex::Matcher = HEALTH_PATTERN.matcher(&boss_name);
                if max_health_matcher.find() {
                    let mut current_health: i32 = SlayerBossBar::convert_to_int(max_health_matcher.group(0));
                    let mut max_health: i32 = boss_fight.lock().borrow().slayer_type.get_health(&boss_name, compat::read(boss_fight, |__t| __t.slayer_tier));
                    compat::set_state(&BOSS_MAX_HEALTH, max_health.max(current_health));
                }
            }
        }
        return true;
    }

    /// java: public static LerpingBossEvent updateBossBar()
    pub fn update_boss_bar() -> LerpingBossEvent {
        // the stand by id (read through `game::with_entity`)
        let mut slayer_armor_stand: Option<i32> = SlayerManager::get_slayer_armor_stand();
        debug_assert!(slayer_armor_stand.is_some());
        let mut name: MutableComponent = mcrust::mcrust::modding::game::with_entity(slayer_armor_stand.unwrap(), |e| compat::component_of(e.get_name())).expect("NullPointerException: the slayer armor stand");
        if BOSS_BAR.get_mut().is_none() {
            // TODO(fabric2seer): field `PURPLE` of a value of unknown type
            //   java: bossBar = new LerpingBossEvent(UUID.randomUUID(), name, 1f, BossEvent.BossBarColor.PURPLE, BossEvent.BossBarOverlay.PROGRESS, false, false, false);
        }
        let mut health_matcher: compat::regex::Matcher = HEALTH_PATTERN.matcher(&name.get_string());
        if health_matcher.find() {
            let mut health: i32 = SlayerBossBar::convert_to_int(health_matcher.group(1));
            if health > compat::get_state(&BOSS_MAX_HEALTH) {
                compat::set_state(&BOSS_MAX_HEALTH, health);
            }
            // java: bossBar.setProgress(bossMaxHealth < 1 ? 1f : (float) health / bossMaxHealth); - `LerpingBossEvent.setProgress`
            // starts its lerp at `Util.getMillis()`
            { let __progress = if compat::get_state(&BOSS_MAX_HEALTH) < 1 { 1.0f32 } else { (health as f32) / (compat::get_state(&BOSS_MAX_HEALTH) as f32) }; BOSS_BAR.get_mut().as_mut().unwrap().set_progress(__progress, mcrust::net::minecraft::util::util::get_millis()); }
            // java: bossBar.setColor(BossEvent.BossBarColor.PINK);
            BOSS_BAR.get_mut().as_mut().unwrap().base.set_color(mcrust::net::minecraft::world::boss_event::BossBarColor::Pink);
            BOSS_BAR.get_mut().as_mut().unwrap().base.set_name(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(name.clone()));
            // java: bossBar.setOverlay(BossEvent.BossBarOverlay.NOTCHED_10);
            BOSS_BAR.get_mut().as_mut().unwrap().base.set_overlay(mcrust::net::minecraft::world::boss_event::BossBarOverlay::Notched10);
        } else {
            // java: bossBar.setColor(BossEvent.BossBarColor.RED);
            BOSS_BAR.get_mut().as_mut().unwrap().base.set_color(mcrust::net::minecraft::world::boss_event::BossBarColor::Red);
            // java: bossBar.setOverlay(BossEvent.BossBarOverlay.PROGRESS);
            BOSS_BAR.get_mut().as_mut().unwrap().base.set_overlay(mcrust::net::minecraft::world::boss_event::BossBarOverlay::Progress);
            BOSS_BAR.get_mut().as_mut().unwrap().base.set_name(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(name.clone()));
        }
        return BOSS_BAR.get_mut().clone().unwrap();
    }

    /// java: private static int convertToInt(@Nullable String value)
    pub fn convert_to_int(mut value: Option<String>) -> i32 {
        if value.is_none() || value.as_ref().unwrap().is_empty() {
            return 0;
        }
        value = Some((compat::trim(&(value.as_ref().unwrap().replace(",", "")))).to_lowercase());
        let mut multiplier: i32 = (if value.as_ref().unwrap().ends_with("m") { 1000000 } else { (if value.as_ref().unwrap().ends_with("k") { 1000 } else { 1 }) });
        if multiplier > 1 {
            value = Some(compat::substring2(&value.as_ref().unwrap(), 0, (compat::str_len(&value.as_ref().unwrap())).wrapping_sub(1)));
        }
        // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut numeric_value: f64 = value.as_ref().unwrap().trim().parse::<f64>().expect("NumberFormatException");
            return (((numeric_value * (multiplier as f64))) as i32);
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return 0;
            }
        }
    }

}
