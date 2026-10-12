//! Ported from de.hysky.skyblocker.skyblock.dwarven.NucleusWaypoints by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::world::phys::vec3;

/// java: private static final List<Waypoint> WAYPOINTS = List.of( new Waypoint(new BlockPos(551, 116, 551), "Precursor Remnants", DyeColor.LIGHT_BLUE), new Waypoint(new BlockPos(551, 116, 475), "Mithril Deposi
pub static WAYPOINTS: compat::MainThread<Vec<Waypoint>> = compat::MainThread::new(|| vec![Waypoint::new(BlockPos::new(551, 116, 551), "Precursor Remnants".to_string(), DyeColor::LightBlue), Waypoint::new(BlockPos::new(551, 116, 475), "Mithril Deposits".to_string(), DyeColor::Lime), Waypoint::new(BlockPos::new(475, 116, 551), "Goblin Holdout".to_string(), DyeColor::Orange), Waypoint::new(BlockPos::new(475, 116, 475), "Jungle".to_string(), DyeColor::Purple), Waypoint::new(BlockPos::new(513, 106, 524), "Nucleus".to_string(), DyeColor::Red)]);

/// java: public class NucleusWaypoints
#[derive(Clone)]
pub struct NucleusWaypoints {}

impl NucleusWaypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut enabled: bool = SkyblockerConfigManager::get().mining.crystal_hollows.nucleus_waypoints;
            let mut in_crystal_hollows: bool = Utils::is_in_crystal_hollows();
            if enabled && in_crystal_hollows {
                for waypoint in WAYPOINTS.get_mut().iter() {
                    let mut rgb: i32 = waypoint.color.get_firework_color();
                    let mut text_color: TextColor = TextColor::from_rgb(rgb);
                    let mut text: MutableComponent = component::literal(&waypoint.name).set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_opt(Some(text_color.clone())));
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_4()); __f(collector, &text, &vec3::at_center_of(&waypoint.position).add((0) as f64, (5) as f64, (0) as f64), ((8) as f32) as f32, true) });
                }
            }
        }));
        if let Err(e) = __result {
            // TODO(fabric2seer): Logger.getName
            //   java: LOGGER.error("[{}] Error occurred while rendering Nucleus waypoints. {}", LOGGER.getName(), e);
        }
    }

}

/// java: private static class Waypoint
#[derive(Clone)]
pub struct Waypoint {
    /// java: BlockPos position;
    pub position: BlockPos,
    /// java: String name;
    pub name: String,
    /// java: DyeColor color;
    pub color: DyeColor,
}

impl Waypoint {
    /// java: Waypoint(BlockPos position, String name, DyeColor color)
    pub fn new(mut position: BlockPos, mut name: String, mut color: DyeColor) -> Self {
        let __init_position = position.clone();
        let __init_name = name.clone();
        let __init_color = color;
        let mut this = Self {
            position: __init_position,
            name: __init_name,
            color: __init_color,
        };
        this
    }

}
