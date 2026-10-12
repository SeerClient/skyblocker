//! Ported from de.hysky.skyblocker.skyblock.rift.EffigyWaypoints by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::area::TheRift;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final List<BlockPos> EFFIGIES = List.of( new BlockPos(150, 79, 95), //Effigy 1 new BlockPos(193, 93, 119), //Effigy 2 new BlockPos(235, 110, 147), //Effigy 3 new BlockPos(293, 96, 134),
pub static EFFIGIES: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| vec![BlockPos::new(150, 79, 95), BlockPos::new(193, 93, 119), BlockPos::new(235, 110, 147), BlockPos::new(293, 96, 134), BlockPos::new(262, 99, 94), BlockPos::new(240, 129, 118)]);
/// java: private static final List<BlockPos> UNBROKEN_EFFIGIES = new ArrayList<>();
pub static UNBROKEN_EFFIGIES: compat::MainThread<Vec<BlockPos>> = compat::MainThread::new(|| Vec::new());
/// java: private static final float[] RED = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));

/// java: public class EffigyWaypoints
#[derive(Clone)]
pub struct EffigyWaypoints {}

impl EffigyWaypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void updateEffigies()
    pub fn update_effigies() {
        if !(SkyblockerConfigManager::get().slayers.vampire_slayer.enable_effigy_waypoints) || !(Utils::is_on_skyblock()) || !(Utils::is_in_the_rift()) || !compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<TheRift>().is_some_and(|__e| *__e == TheRift::Chateau)) {
            return;
        }
        UNBROKEN_EFFIGIES.get_mut().clear();
        // java: try {..} catch (NullPointerException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for i in 0..(compat::lock(&utils::STRING_SCOREBOARD).len() as i32) {
                let mut line: String = compat::lock(&utils::STRING_SCOREBOARD)[(i) as usize].clone();
                if line.contains("Effigies") {
                    let mut effigies_text: Vec<MutableComponent> = Vec::new();
                    let prefix_and_suffix = (utils::TEXT_SCOREBOARD.get_mut()[(i) as usize]).get_siblings();
                    effigies_text.extend(prefix_and_suffix[0].get_siblings().iter().cloned());
                    effigies_text.extend(((prefix_and_suffix[(1) as usize]).get_siblings()).iter().cloned());
                    for i2 in 1..(effigies_text.len() as i32) {
                        if ((effigies_text[(i2) as usize]).get_style()).get_color().unwrap() == TextColor::from_legacy_format(ChatFormatting::Gray).unwrap() {
                            UNBROKEN_EFFIGIES.get_mut().push(EFFIGIES.get_mut()[(i2.wrapping_sub(1)) as usize].clone());
                        }
                    }
                }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Error while updating effigies."));
        }
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if SkyblockerConfigManager::get().slayers.vampire_slayer.enable_effigy_waypoints && compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<TheRift>().is_some_and(|__e| *__e == TheRift::Chateau)) {
            for effigy in UNBROKEN_EFFIGIES.get_mut().iter() {
                if SkyblockerConfigManager::get().slayers.vampire_slayer.compact_effigy_waypoints {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_with_beacon_beam()); __f(collector, &effigy.below_n(6), &RED, 0.5f32, true) });
                } else {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_with_beacon_beam()); __f(collector, effigy, &RED, 0.5f32, true) });
                    for i in 1..6 {
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &effigy.below_n(i), &RED, (0.5f32 - (0.075f32 * (i as f32))), true) });
                    }
                }
            }
        }
    }

}
