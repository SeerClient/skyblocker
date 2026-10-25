//! Ported from de.hysky.skyblocker.skyblock.GyroOverlay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::client::resources::language::i18n;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static int color;
pub static COLOR: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final int GYRO_RADIUS = 10;
pub const GYRO_RADIUS: i32 = 10;
/// java: private static final int SEGMENTS = 32;
pub const SEGMENTS: i32 = 32;
/// java: private static final int MAX_REACH = 24;
pub const MAX_REACH: i32 = 24;

/// java: public class GyroOverlay
#[derive(Clone)]
pub struct GyroOverlay {}

impl GyroOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        GyroOverlay::config_callback(&SkyblockerConfigManager::get().ui_and_visuals.gyro_overlay.gyro_overlay_color);
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { GyroOverlay::extract_rendering(&__a0); }));
    }

    /// java: public static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() || compat::minecraft().get_camera_entity().is_none() {
            return;
        }
        if !(Utils::is_on_skyblock()) {
            return;
        }
        if SkyblockerConfigManager::get().ui_and_visuals.gyro_overlay.gyro_overlay_mode == Mode::Off {
            return;
        }
        // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the held id compared BORROWED
        // (`with_skyblock_id`), not cloned into a `String` every frame; the same id and the same cache write on a miss.
        // (The null check at the top stays where it is: folding it into this read would move `getCameraEntity`, a
        // hooked call, ahead of it.) `SKYBLOCKER_VANILLA_COLLECTOR=1` clones as before. Was:
        //     let mut held_item: String = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().get_skyblock_id()).unwrap();
        //     if !(held_item == "GYROKINETIC_WAND") {
        let holds_gyro: bool = if crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector_impl::VANILLA_COLLECTOR.on() {
            let held_item: String = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().get_skyblock_id()).unwrap();
            held_item == "GYROKINETIC_WAND"
        } else {
            mcrust::mcrust::modding::game::with_player(|player| crate::mixins::item_stack_mixin::with_skyblock_id(player.get_main_hand_item(), |held_item: &str| held_item == "GYROKINETIC_WAND")).unwrap()
        };
        if !holds_gyro {
            return;
        }
        //   java: HitResult hit = CLIENT.getCameraEntity().pick(MAX_REACH, CLIENT.getDeltaTracker().getGameTimeDeltaPartialTick(false), false);
        // (the port's `Entity.pick` is handed the level it clips against - `this.level()` in Java)
        let partial_tick: f32 = compat::minecraft().get_delta_tracker().get_game_time_delta_partial_tick(false);
        let level = mcrust::mcrust::modding::game::level().unwrap();
        let Some(mut hit) = mcrust::mcrust::modding::game::with_entity(compat::minecraft().get_camera_entity().unwrap(), |camera_entity| mcrust::net::minecraft::world::entity::entity::pick(&*camera_entity, &*level, (MAX_REACH) as f64, partial_tick, false)) else {
            return;
        };
        //   java: if (hit.getType() == HitResult.Type.MISS) return;
        if hit.get_type() == mcrust::net::minecraft::world::phys::hit_result::Type::Miss {
            return;
        }
        match SkyblockerConfigManager::get().ui_and_visuals.gyro_overlay.gyro_overlay_mode {
            Mode::Off => {
            }
            Mode::CircleOutline => {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_circle()); __f(collector, &hit.get_location().add_vec(&Vec3::new((0) as f64, 0.1, (0) as f64)), ((GYRO_RADIUS) as f32) as f32, 0.25f32, SEGMENTS, compat::get_state(&COLOR)) });
            }
            Mode::Circle => {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_circle()); __f(collector, &hit.get_location().add_vec(&Vec3::new((0) as f64, 0.1, (0) as f64)), ((GYRO_RADIUS) as f32) as f32, SEGMENTS, compat::get_state(&COLOR)) });
            }
            Mode::Sphere => {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_sphere()); __f(collector, &hit.get_location(), ((GYRO_RADIUS) as f32) as f32, SEGMENTS, SEGMENTS, compat::get_state(&COLOR)) });
            }
            _ => {}
        }
    }

    /// java: public static void configCallback(Color colorObj)
    pub fn config_callback(color_obj: &Color) {
        //   java: float[] colorComponents = colorObj.getRGBComponents(null);
        let mut color_components: Vec<f32> = color_obj.get_rgb_components(None).to_vec();
        compat::set_state(&COLOR, argb::color_from_float(color_components[(3) as usize], color_components[(0) as usize], color_components[(1) as usize], color_components[(2) as usize]));
    }

}

/// java: public enum Mode implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mode {
    Off,
    Circle,
    CircleOutline,
    Sphere,
}

impl Mode {
    pub const VALUES: [Mode; 4] = [Mode::Off, Mode::Circle, Mode::CircleOutline, Mode::Sphere];

    /// java: values()
    pub fn values() -> Vec<Mode> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Mode::Off => "OFF",
            Mode::Circle => "CIRCLE",
            Mode::CircleOutline => "CIRCLE_OUTLINE",
            Mode::Sphere => "SPHERE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Mode {
        match name {
            "OFF" => Mode::Off,
            "CIRCLE" => Mode::Circle,
            "CIRCLE_OUTLINE" => Mode::CircleOutline,
            "SPHERE" => Mode::Sphere,
            _ => panic!("IllegalArgumentException: No enum constant Mode.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static ModeFields {
        static FIELDS: LazyLock<Vec<ModeFields>> = LazyLock::new(|| vec![
            ModeFields::new("OFF".to_string()),
            ModeFields::new("CIRCLE".to_string()),
            ModeFields::new("CIRCLE_OUTLINE".to_string()),
            ModeFields::new("SPHERE".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        //   java: return name().toLowerCase(Locale.ENGLISH);
        return self.name().to_lowercase();
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        //   java: return I18n.get(key);
        return i18n::get(&self.fields().key, &[]);
    }

}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Mode::to_string(self))
    }
}

/// the constructor data of each Mode constant
pub struct ModeFields {
    pub key: String,
}

impl ModeFields {
    fn new(mut key: String) -> Self {
        let mut this = Self {
            key: Default::default(),
        };
        this.key = format!("skyblocker.config.uiAndVisuals.gyroOverlay.mode.{}", key);
        this
    }

}
