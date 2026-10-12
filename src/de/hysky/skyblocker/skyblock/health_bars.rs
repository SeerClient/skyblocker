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
pub static HEALTH_VALUES: compat::MainThread<std::collections::HashMap<ArmorStand, f32>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Object2LongOpenHashMap<ArmorStand> mobStartingHealth = new Object2LongOpenHashMap<>();
pub static MOB_STARTING_HEALTH: compat::MainThread<std::collections::HashMap<ArmorStand, i64>> = compat::MainThread::new(|| std::collections::HashMap::new());

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
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of ClientLevel#removeEntity (`net.minecraft.client.multiplayer.ClientLevel#removeEntity`: exists)
        //   java: ClientEntityEvents.ENTITY_UNLOAD.register(HealthBars::onEntityDespawn);
    }

    /// java: private static void reset()
    pub fn reset() {
        HEALTH_VALUES.get_mut().clear();
        MOB_STARTING_HEALTH.get_mut().clear();
    }

    /// java: public static void onEntityDespawn(Entity entity, ClientLevel clientWorld)
    pub fn on_entity_despawn(entity: &dyn Entity, client_world: &ClientLevel) {
        if let Some(armor_stand_entity) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() {
            // TODO(fabric2seer): Map.removeFloat(..)
            //   java: healthValues.removeFloat(armorStandEntity);
            // TODO(fabric2seer): Map.removeLong(..)
            //   java: mobStartingHealth.removeLong(armorStandEntity);
        }
    }

    /// java: private static boolean isDisallowedMob(String name)
    pub fn is_disallowed_mob(name: &str) -> bool {
        return (Utils::is_in_dungeons() && DISALLOWED_DUNGEON_MOBS.iter().cloned().any(|__a0| name.contains(&__a0)));
    }

    /// java: public static void healthBar(ArmorStand armorStand)
    pub fn health_bar(mut armor_stand: ArmorStand) {
        if !armor_stand.is_invisible_field() || !armor_stand.base.base.has_custom_name() || !armor_stand.base.base.is_custom_name_visible() || !(SkyblockerConfigManager::get().ui_and_visuals.health_bars.enabled) {
            return;
        }
        if armor_stand.base.is_dead_or_dying() {
            // TODO(fabric2seer): Map.removeFloat(..)
            //   java: healthValues.removeFloat(armorStand);
            // TODO(fabric2seer): Map.removeLong(..)
            //   java: mobStartingHealth.removeLong(armorStand);
            return;
        }
        if armor_stand.base.base.get_custom_name().is_none() {
            return;
        }
        let mut health_matcher: compat::regex::Matcher = HEALTH_PATTERN.matcher(&armor_stand.base.base.get_custom_name().unwrap().get_string());
        if !health_matcher.find() {
            // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
            // rust: HealthBars::health_only_check(armor_stand.clone());
            return;
        }
        // TODO(fabric2seer): `.floatValue(..)` on a Number
        //   java: float firstValue = Formatters.parseNumber(healthMatcher.group(1).toUpperCase(Locale.ENGLISH)).floatValue();
        let mut first_value: f32 = todo!("fabric2seer: the initialiser of `firstValue` (TODO above)");
        // TODO(fabric2seer): `.floatValue(..)` on a Number
        //   java: float secondValue = Formatters.parseNumber(healthMatcher.group(4).toUpperCase(Locale.ENGLISH)).floatValue();
        let mut second_value: f32 = todo!("fabric2seer: the initialiser of `secondValue` (TODO above)");
        let mut health: f32 = (first_value / second_value);
        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
        // rust: HEALTH_VALUES.get_mut().insert(armor_stand.clone(), health);
        let mut remove_value: bool = SkyblockerConfigManager::get().ui_and_visuals.health_bars.remove_health_from_name;
        let mut remove_max: bool = SkyblockerConfigManager::get().ui_and_visuals.health_bars.remove_max_health_from_name;
        if !remove_value && !remove_max {
            return;
        }
        if HealthBars::is_disallowed_mob(&armor_stand.base.base.get_custom_name().unwrap().get_string()) {
            return;
        }
        let mut cleaned_text: MutableComponent = component::empty();
        let parts = armor_stand.base.base.get_custom_name().unwrap().get_siblings();
        let mut health_start_index: i32 = -1;
        for i in 0..(parts.len() as i32) {
            // TODO(fabric2seer): the condition: static call StringUtils.join(..)
            //   java: i < parts.size() - 4 && StringUtils.join(parts.subList(i + 1, i + 5).stream().map(Component::getString).toArray(), "").equals(healthMatcher.group(0))
            if todo!("fabric2seer: the condition (TODO above)") {
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
            cleaned_text.append(parts[(i) as usize].clone());
        }
        // TODO(fabric2seer): `ArmorStand.skyblocker$setCustomName` has no Rust method in the port (looked for `skyblocker_set_custom_name` in ArmorStand and JavaSrc)
        // rust: armor_stand.skyblocker_set_custom_name(&cleaned_text);
    }

    /// java: private static void healthOnlyCheck(ArmorStand armorStand)
    pub fn health_only_check(mut armor_stand: ArmorStand) {
        if !(SkyblockerConfigManager::get().ui_and_visuals.health_bars.apply_to_health_only_mobs) || armor_stand.base.base.get_custom_name().is_none() {
            return;
        }
        let mut health_only_matcher: compat::regex::Matcher = HEALTH_ONLY_PATTERN.matcher(&armor_stand.base.base.get_custom_name().unwrap().get_string());
        if !health_only_matcher.find() {
            return;
        }
        // TODO(fabric2seer): `.longValue(..)` on a Number
        //   java: long currentHealth = Formatters.parseNumber(healthOnlyMatcher.group(1).toUpperCase(Locale.ENGLISH)).longValue();
        let mut current_health: i64 = todo!("fabric2seer: the initialiser of `currentHealth` (TODO above)");
        if !(MOB_STARTING_HEALTH.get_mut().contains_key(&armor_stand)) {
            // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
            // rust: MOB_STARTING_HEALTH.get_mut().insert(armor_stand.clone(), current_health);
        }
        let mut health: f32 = (((current_health) as f32) / (MOB_STARTING_HEALTH.get_mut().get(&armor_stand).copied().unwrap_or_default() as f32));
        // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
        // rust: HEALTH_VALUES.get_mut().insert(armor_stand.clone(), health);
        if !(SkyblockerConfigManager::get().ui_and_visuals.health_bars.remove_health_from_name) {
            return;
        }
        if HealthBars::is_disallowed_mob(&armor_stand.base.base.get_custom_name().unwrap().get_string()) {
            return;
        }
        let mut cleaned_text: MutableComponent = component::empty();
        let parts = armor_stand.base.base.get_custom_name().unwrap().get_siblings();
        {
            let mut i: i32 = 0;
            while i < (parts.len() as i32) {
                'continue_1: {
                    if i < (parts.len() as i32).wrapping_sub(2) && Some(&*(parts[(i.wrapping_add(1)) as usize..(i.wrapping_add(3)) as usize].to_vec().into_iter().map(|__a0| __a0.get_string()).map(|s| s.to_string()).collect::<Vec<String>>().join(""))) == health_only_matcher.group(0).as_deref() {
                        i = i.wrapping_add(2);
                        break 'continue_1;
                    }
                    cleaned_text.append(parts[(i) as usize].clone());
                }
                i += 1;
            }
        }
        // TODO(fabric2seer): `ArmorStand.skyblocker$setCustomName` has no Rust method in the port (looked for `skyblocker_set_custom_name` in ArmorStand and JavaSrc)
        // rust: armor_stand.skyblocker_set_custom_name(&cleaned_text);
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
        // TODO(fabric2seer): `DeltaTracker.getGameTimeDeltaPartialTick` has no Rust method in the port (looked for `get_game_time_delta_partial_tick` in DeltaTracker and its supertypes)
        // rust: let mut tick_delta: f32 = RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false);
        let mut tick_delta: f32 = todo!("fabric2seer: the initialiser of `tickDelta` (TODO above)");
        let mut width: f32 = scale;
        let mut height: f32 = (scale * 0.1f32);
        // TODO(fabric2seer): Map.object2FloatEntrySet(..)
        //   java: for (Object2FloatMap.Entry<ArmorStand> healthValue : healthValues.object2FloatEntrySet()) {
        //   java: 			//if the health bar is full and the setting is enabled to hide it stop rendering it
        //   java: 			float health = healthValue.getFloatValue();
        //   java: 			if (hideFullHealth && health == 1) {
        //   java: 				continue;
        //   java: 			}
        //   java:
        //   java: 			ArmorStand armorStand = healthValue.getKey();
        //   java: 			//only render health bar if name is visible
        //   java: 			if (!armorStand.shouldShowName()) {
        //   java: 				return;
        //   java: 			}
        //   java: 			//gets the mixed color of the health bar
        //   java: 			int mixedColor = ColorUtils.interpolate(health, emptyColor.getRGB(), halfColor.getRGB(), fullColor.getRGB());
        //   java: 			float[] components = ColorUtils.getFloatComponents(mixedColor);
        //   java: 			// Render the health bar texture with scaling based on health percentage
        //   java: 			collector.submitTexturedQuad(armorStand.getEyePosition(tickDelta).add(0, 0.25 - height, 0), width, height, 1f, 1f, new Vec3(width * -0.5f, 0, 0), HEALTH_BAR_BACKGROUND_TEXTURE, components, 1f, true);
        //   java: 			collector.submitTexturedQuad(armorStand.getEyePosition(tickDelta).add(0, 0.25 - height, 0), width * health, height, health, 1f, new Vec3(width * -0.5f, 0, -0.003f), HEALTH_BAR_TEXTURE, components, 1f, true);
        //   java: 		}
    }

}
