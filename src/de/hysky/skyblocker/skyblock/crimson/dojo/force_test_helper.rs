//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.ForceTestHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::monster::zombie::zombie::Zombie;

/// java: private static final DecimalFormat FORMATTER = new DecimalFormat("0.0");
///
/// `new DecimalFormat(pattern)` takes the default locale's symbols; the port models no default locale (`java::text`
/// has ROOT and US, whose CLDR symbols are the same), so US is used.
pub static FORMATTER: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| DecimalFormat::new("0.0", mcrust::java::text::decimal_format_symbols::get_instance(mcrust::java::text::decimal_format_symbols::Locale::Us)));
/// java: private static final int ZOMBIE_LIFE_TIME = 10100;
pub const ZOMBIE_LIFE_TIME: i32 = 10100;
/// java: private static final Object2LongOpenHashMap<Zombie> zombies = new Object2LongOpenHashMap<>();
///
/// The zombies by entity id (Java's key is the live object, compared by identity).
pub static ZOMBIES: compat::MainThread<std::collections::HashMap<i32, i64>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class ForceTestHelper
#[derive(Clone)]
pub struct ForceTestHelper {}

impl ForceTestHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void reset()
    pub fn reset() {
        ZOMBIES.get_mut().clear();
    }

    /// java: protected static boolean shouldGlow(String name)
    pub fn should_glow(name: &str) -> bool {
        return name.contains("-");
    }

    /// java: protected static int getColor()
    pub fn get_color() -> i32 {
        return mcrust::java::awt::color::red().get_rgb();
    }

    /// java: protected static void onEntitySpawn(Entity entity)
    pub fn on_entity_spawn(entity: &dyn Entity) {
        if entity.as_any().is_some_and(|a| a.is::<Zombie>()) {
            ZOMBIES.get_mut().insert(entity.base().get_id(), (compat::millis()).wrapping_add(ZOMBIE_LIFE_TIME as i64));
        }
    }

    /// java: protected static void onEntityAttacked(Entity entity)
    pub fn on_entity_attacked(entity: &dyn Entity) {
        if entity.as_any().is_some_and(|a| a.is::<Zombie>()) {
            if ZOMBIES.get_mut().contains_key(&entity.base().get_id()) {
                ZOMBIES.get_mut().insert(entity.base().get_id(), (compat::millis()).wrapping_add(ZOMBIE_LIFE_TIME as i64));
            }
        }
    }

    /// java: protected static void onEntityDespawn(Entity entity)
    pub fn on_entity_despawn(entity: &dyn Entity) {
        if entity.as_any().is_some_and(|a| a.is::<Zombie>()) {
            ZOMBIES.get_mut().remove(&entity.base().get_id());
        }
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let mut current_time: i64 = compat::millis();
        let zombies: Vec<(i32, i64)> = ZOMBIES.get().iter().map(|(k, v)| (*k, *v)).collect();
        for (zombie, value) in zombies {
            let mut seconds_time: f32 = ((value.wrapping_sub(current_time) as f32) / 1000f32).max(0.0);
            let mut text: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent = mcrust::net::minecraft::network::chat::component::literal(&FORMATTER.get().format(seconds_time as f64));
            if seconds_time > 1.0 {
                text = text.with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Green);
            } else if seconds_time > 0.0 {
                text = text.with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Yellow);
            } else {
                text = text.with_style_format(mcrust::net::minecraft::chat_formatting::ChatFormatting::Red);
            }
            // the zombie by id; one the level dropped is no longer drawn (Java keeps the dead object until it despawns)
            let partial_tick = crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false);
            let Some(label_pos) = mcrust::mcrust::modding::game::with_entity(zombie, |e| e.base().get_eye_position_partial(partial_tick)) else {
                continue;
            };
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_4()); __f(collector, &text, &label_pos, 1.5f32, true) });
        }
    }

}
