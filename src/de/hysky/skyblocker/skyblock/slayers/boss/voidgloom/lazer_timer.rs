//! Ported from de.hysky.skyblocker.skyblock.slayers.boss.voidgloom.LazerTimer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::BossFight;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static long lastPhaseTime;
pub static LAST_PHASE_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static double remainingTime;
pub static REMAINING_TIME: LazyLock<Mutex<f64>> = LazyLock::new(|| Mutex::new(0.0));
/// java: private static boolean active;
pub static ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class LazerTimer
#[derive(Clone)]
pub struct LazerTimer {}

impl LazerTimer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { LazerTimer::extract_rendering(&__a0); }));
    }

    /// java: public static void tick()
    pub fn tick() {
        compat::set_state(&REMAINING_TIME, (compat::get_state(&REMAINING_TIME) - 0.05));
        compat::set_state(&ACTIVE, (compat::get_state(&REMAINING_TIME) > (0 as f64)));
    }

    /// java: public static void activate()
    pub fn activate() {
        if (compat::millis()).wrapping_sub(compat::get_state(&LAST_PHASE_TIME)) >= (10000 as i64) {
            compat::set_state(&LAST_PHASE_TIME, compat::millis());
            compat::set_state(&REMAINING_TIME, ((8) as f64) as f64);
            compat::set_state(&ACTIVE, true);
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if compat::get_state(&ACTIVE) {
            let mut boss_fight: Option<compat::Shared<BossFight>> = SlayerManager::get_boss_fight();
            if boss_fight.is_some() && boss_fight.as_ref().unwrap().lock().borrow().boss.get_vehicle().is_some() {
                let mut text: MutableComponent = (component::literal(&format!("{:.1}s", compat::get_state(&REMAINING_TIME)))).with_style_format(ChatFormatting::Aqua);
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_4()); __f(collector, &text, &{ let (__q0, __q1, __q2,) = ((0) as f64, 1.5, (0) as f64,); boss_fight.as_ref().unwrap().lock().borrow().boss.position().add(__q0, __q1, __q2) }, ((3) as f32) as f32, true) });
            }
        }
    }

    /// java: public static boolean isActive()
    pub fn is_active() -> bool {
        return compat::get_state(&ACTIVE);
    }

}
