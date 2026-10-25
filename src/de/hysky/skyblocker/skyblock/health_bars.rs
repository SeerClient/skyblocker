//! Ported from de.hysky.skyblocker.skyblock.HealthBars by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::formatters::Formatters;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::awt::color::Color;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier HEALTH_BAR_BACKGROUND_TEXTURE = Identifier.withDefaultNamespace("textures/gui/sprites/boss_bar/white_background.png");
pub static HEALTH_BAR_BACKGROUND_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/gui/sprites/boss_bar/white_background.png"));
/// java: private static final Identifier HEALTH_BAR_TEXTURE = Identifier.withDefaultNamespace("textures/gui/sprites/boss_bar/white_progress.png");
pub static HEALTH_BAR_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/gui/sprites/boss_bar/white_progress.png"));
/// java: protected static final Pattern HEALTH_PATTERN = Pattern.compile("(\\d{1,3}(,\\d{3})*(\\.\\d+)?[kKmMbBtT]?)/(\\d{1,3}(,\\d{3})*(\\.\\d+)?[kKmMbBtT]?)❤");
pub static HEALTH_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(\\d{1,3}(,\\d{3})*(\\.\\d+)?[kKmMbBtT]?)/(\\d{1,3}(,\\d{3})*(\\.\\d+)?[kKmMbBtT]?)❤"));
/// java: protected static final Pattern HEALTH_ONLY_PATTERN = Pattern.compile("(\\d{1,3}(,\\d{3})*(\\.\\d+)?[kKmMbBtT]?)❤");
pub static HEALTH_ONLY_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(\\d{1,3}(,\\d{3})*(\\.\\d+)?[kKmMbBtT]?)❤"));
/// java: private static final List<String> DISALLOWED_DUNGEON_MOBS = List.of("Blaze", "The Professor", "Guardian");
pub static DISALLOWED_DUNGEON_MOBS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Blaze".to_string(), "The Professor".to_string(), "Guardian".to_string()]);
/// java: private static final Object2FloatOpenHashMap<ArmorStand> healthValues = new Object2FloatOpenHashMap<>();
///
/// Keyed by entity id (Java's key is the live stand, compared by identity).
pub static HEALTH_VALUES: compat::MainThread<std::collections::HashMap<i32, f32>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Object2LongOpenHashMap<ArmorStand> mobStartingHealth = new Object2LongOpenHashMap<>();
///
/// Keyed by entity id, as `HEALTH_VALUES`.
pub static MOB_STARTING_HEALTH: compat::MainThread<std::collections::HashMap<i32, i64>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class HealthBars
#[derive(Clone)]
pub struct HealthBars {}

impl HealthBars {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            HealthBars::reset();
        }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { HealthBars::extract_rendering(&__a0); }));
        // java: ClientEntityEvents.ENTITY_UNLOAD.register(HealthBars::onEntityDespawn);
        mcrust::mcrust::modding::events::client_entity_events::register_entity_unload(|entity, client_world| HealthBars::on_entity_despawn(entity, client_world));
    }

    /// java: private static void reset()
    pub fn reset() {
        HEALTH_VALUES.get_mut().clear();
        MOB_STARTING_HEALTH.get_mut().clear();
    }

    /// java: public static void onEntityDespawn(Entity entity, ClientLevel clientWorld)
    pub fn on_entity_despawn(entity: &dyn Entity, client_world: &ClientLevel) {
        if entity.as_any().is_some_and(|a| a.is::<ArmorStand>()) {
            HEALTH_VALUES.get_mut().remove(&entity.base().get_id());
            MOB_STARTING_HEALTH.get_mut().remove(&entity.base().get_id());
        }
    }

    /// java: private static boolean isDisallowedMob(String name)
    pub fn is_disallowed_mob(name: &str) -> bool {
        return (Utils::is_in_dungeons() && DISALLOWED_DUNGEON_MOBS.iter().cloned().any(|__a0| name.contains(&__a0)));
    }

    /// java: public static void healthBar(ArmorStand armorStand)
    ///
    /// `armor_stand` is lent by the packet handler that holds it.
    pub fn health_bar(armor_stand: &ArmorStand) {
        let id: i32 = armor_stand.base.base.get_id();
        if !armor_stand.base.base.is_invisible() || !armor_stand.base.base.has_custom_name() || !armor_stand.base.base.is_custom_name_visible() || !(SkyblockerConfigManager::get().ui_and_visuals.health_bars.enabled) {
            return;
        }
        if armor_stand.base.is_dead_or_dying() {
            HEALTH_VALUES.get_mut().remove(&id);
            MOB_STARTING_HEALTH.get_mut().remove(&id);
            return;
        }
        let Some(custom_name) = armor_stand.base.base.get_custom_name().map(compat::component_of) else {
            return;
        };
        let mut health_matcher: compat::regex::Matcher = HEALTH_PATTERN.matcher(&custom_name.get_string());
        if !health_matcher.find() {
            HealthBars::health_only_check(armor_stand);
            return;
        }
        let mut first_value: f32 = Formatters::parse_number(&health_matcher.group(1).unwrap().to_uppercase()).float_value();
        let mut second_value: f32 = Formatters::parse_number(&health_matcher.group(4).unwrap().to_uppercase()).float_value();
        let mut health: f32 = (first_value / second_value);
        HEALTH_VALUES.get_mut().insert(id, health);
        let mut remove_value: bool = SkyblockerConfigManager::get().ui_and_visuals.health_bars.remove_health_from_name;
        let mut remove_max: bool = SkyblockerConfigManager::get().ui_and_visuals.health_bars.remove_max_health_from_name;
        if !remove_value && !remove_max {
            return;
        }
        if HealthBars::is_disallowed_mob(&custom_name.get_string()) {
            return;
        }
        let mut cleaned_text: MutableComponent = component::empty();
        let parts = custom_name.get_siblings();
        let mut health_start_index: i32 = -1;
        for i in 0..(parts.len() as i32) {
            // StringUtils.join(parts.subList(i + 1, i + 5).stream().map(Component::getString).toArray(), "")
            if i < (parts.len() as i32).wrapping_sub(4) && Some(parts[(i.wrapping_add(1)) as usize..(i.wrapping_add(5)) as usize].iter().map(|part| part.get_string()).collect::<Vec<String>>().join("")) == health_matcher.group(0) {
                health_start_index = i;
            }
            if health_start_index != -1 {
                match i.wrapping_sub(health_start_index) {
                    0 => {
                        if remove_max && remove_value {
                            continue;
                        }
                    }
                    1 => {
                        if remove_value {
                            continue;
                        }
                    }
                    2 => {
                        if remove_max {
                            continue;
                        }
                    }
                    3 => {
                        if remove_max {
                            continue;
                        }
                    }
                    4 => {
                        if remove_max && remove_value {
                            continue;
                        }
                    }
                    _ => {}
                }
            }
            cleaned_text = cleaned_text.append(parts[(i) as usize].clone());
        }
        armor_stand.skyblocker_set_custom_name(&cleaned_text);
    }

    /// java: private static void healthOnlyCheck(ArmorStand armorStand)
    pub fn health_only_check(armor_stand: &ArmorStand) {
        let id: i32 = armor_stand.base.base.get_id();
        if !(SkyblockerConfigManager::get().ui_and_visuals.health_bars.apply_to_health_only_mobs) || armor_stand.base.base.get_custom_name().is_none() {
            return;
        }
        let custom_name: MutableComponent = compat::component_of(armor_stand.base.base.get_custom_name().unwrap());
        let mut health_only_matcher: compat::regex::Matcher = HEALTH_ONLY_PATTERN.matcher(&custom_name.get_string());
        if !health_only_matcher.find() {
            return;
        }
        let mut current_health: i64 = Formatters::parse_number(&health_only_matcher.group(1).unwrap().to_uppercase()).long_value();
        if !(MOB_STARTING_HEALTH.get_mut().contains_key(&id)) {
            MOB_STARTING_HEALTH.get_mut().insert(id, current_health);
        }
        let mut health: f32 = (((current_health) as f32) / (MOB_STARTING_HEALTH.get_mut().get(&id).copied().unwrap_or_default() as f32));
        HEALTH_VALUES.get_mut().insert(id, health);
        if !(SkyblockerConfigManager::get().ui_and_visuals.health_bars.remove_health_from_name) {
            return;
        }
        if HealthBars::is_disallowed_mob(&custom_name.get_string()) {
            return;
        }
        let mut cleaned_text: MutableComponent = component::empty();
        let parts = custom_name.get_siblings();
        {
            let mut i: i32 = 0;
            while i < (parts.len() as i32) {
                'continue_1: {
                    if i < (parts.len() as i32).wrapping_sub(2) && Some(parts[(i.wrapping_add(1)) as usize..(i.wrapping_add(3)) as usize].iter().map(|part| part.get_string()).collect::<Vec<String>>().join("")) == health_only_matcher.group(0) {
                        i = i.wrapping_add(2);
                        break 'continue_1;
                    }
                    cleaned_text = cleaned_text.append(parts[(i) as usize].clone());
                }
                i += 1;
            }
        }
        armor_stand.skyblocker_set_custom_name(&cleaned_text);
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().ui_and_visuals.health_bars.enabled) || HEALTH_VALUES.get_mut().is_empty() {
            return;
        }
        let full_color = &SkyblockerConfigManager::get().ui_and_visuals.health_bars.full_bar_color;
        let half_color = &SkyblockerConfigManager::get().ui_and_visuals.health_bars.half_bar_color;
        let empty_color = &SkyblockerConfigManager::get().ui_and_visuals.health_bars.empty_bar_color;
        let mut hide_full_health: bool = SkyblockerConfigManager::get().ui_and_visuals.health_bars.hide_full_health;
        let mut scale: f32 = SkyblockerConfigManager::get().ui_and_visuals.health_bars.scale;
        let mut tick_delta: f32 = RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false);
        let mut width: f32 = scale;
        let mut height: f32 = (scale * 0.1f32);
        let health_values: Vec<(i32, f32)> = HEALTH_VALUES.get().iter().map(|(k, v)| (*k, *v)).collect();
        for (armor_stand, health) in health_values {
            if hide_full_health && health == 1.0 {
                continue;
            }
            // the stand by id: (shouldShowName, eye position); a stand the level dropped is not drawn
            let Some((should_show_name, eye_position)) = mcrust::mcrust::modding::game::with_entity(armor_stand, |e| (e.should_show_name(), e.base().get_eye_position_partial(tick_delta))) else {
                continue;
            };
            if !should_show_name {
                return;
            }
            let mut mixed_color: i32 = ColorUtils::interpolate_2((health) as f64, &vec![empty_color.get_rgb(), half_color.get_rgb(), full_color.get_rgb()]);
            let mut components: Vec<f32> = ColorUtils::get_float_components(mixed_color);
            let position = eye_position.add((0) as f64, (0.25 - (height as f64)), (0) as f64);
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_textured_quad()); __f(collector, &position, width, height, 1.0f32, 1.0f32, &Vec3::new(((width * -0.5f32) as f64), (0) as f64, (0) as f64), HEALTH_BAR_BACKGROUND_TEXTURE.get(), &components, 1.0f32, true) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_textured_quad()); __f(collector, &position, (width * health), height, health, 1.0f32, &Vec3::new(((width * -0.5f32) as f64), (0) as f64, (-0.003f32) as f64), HEALTH_BAR_TEXTURE.get(), &components, 1.0f32, true) });
        }
    }

}
