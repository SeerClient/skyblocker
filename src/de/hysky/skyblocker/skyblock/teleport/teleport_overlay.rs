//! Ported from de.hysky.skyblocker.skyblock.teleport.TeleportOverlay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::HighlightType;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::PredictiveSmoothAOTE;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static float[] colorComponents;
pub static COLOR_COMPONENTS: LazyLock<Mutex<Vec<f32>>> = LazyLock::new(|| Mutex::new(Default::default()));

/// java: public class TeleportOverlay
#[derive(Clone)]
pub struct TeleportOverlay {}

impl TeleportOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        TeleportOverlay::config_callback(&SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.teleport_overlay_color);
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { TeleportOverlay::extract_rendering(&__a0); }));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if Utils::is_on_skyblock() && SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_teleport_overlays && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::level().is_some() {
            let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
            let mut item_id: String = held_item.get_skyblock_id();
            let mut custom_data: Tag = ItemUtils::get_custom_data(&held_item);
            match item_id.as_str() {
                "ASPECT_OF_THE_LEECH_1" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_weird_transmission {
                        TeleportOverlay::extract_rendering_3(collector, 3, false);
                    }
                }
                "ASPECT_OF_THE_LEECH_2" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_weird_transmission {
                        TeleportOverlay::extract_rendering_3(collector, 4, false);
                    }
                }
                "ASPECT_OF_THE_LEECH_3" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_weird_transmission {
                        TeleportOverlay::extract_rendering_3(collector, 5, false);
                    }
                }
                "ASPECT_OF_THE_END" | "ASPECT_OF_THE_VOID" => {
                    // TODO(fabric2seer): `Options.keyShift` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
                    // TODO(fabric2seer): `is_down` takes 1 argument(s) in the port (`name: &str`), Java passes 0
                    if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_ether_transmission && key_mapping::is_down() && custom_data.get_int_or("ethermerge", 0) == 1 {
                        TeleportOverlay::extract_rendering_4(collector, &custom_data, 57, true);
                    } else if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_instant_transmission {
                        TeleportOverlay::extract_rendering_4(collector, &custom_data, 8, false);
                    }
                }
                "ETHERWARP_CONDUIT" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_ether_transmission {
                        TeleportOverlay::extract_rendering_4(collector, &custom_data, 57, true);
                    }
                }
                "SINSEEKER_SCYTHE" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_sinrecall_transmission {
                        TeleportOverlay::extract_rendering_4(collector, &custom_data, 4, false);
                    }
                }
                "NECRON_BLADE" | "ASTRAEA" | "HYPERION" | "SCYLLA" | "VALKYRIE" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_wither_impact {
                        TeleportOverlay::extract_rendering_3(collector, 10, false);
                    }
                }
                _ => {}
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector, CompoundTag customData, int baseRange, boolean isEtherwarp)
    pub fn extract_rendering_4(collector: &compat::Shared<dyn PrimitiveCollector>, custom_data: &Tag, mut base_range: i32, mut is_etherwarp: bool) {
        TeleportOverlay::extract_rendering_3(collector, (if true /* java: custom_data != null: never null here */ && custom_data.contains("tuned_transmission") { base_range.wrapping_add(custom_data.get_int_or("tuned_transmission", 0)) } else { base_range }), is_etherwarp);
    }

    /// java: private static void extractRendering(PrimitiveCollector collector, int range, boolean isEtherwarp)
    pub fn extract_rendering_3(collector: &compat::Shared<dyn PrimitiveCollector>, mut range: i32, mut is_etherwarp: bool) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let mut pitch: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x_rot()).unwrap();
        let mut yaw: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_y_rot()).unwrap();
        let mut look: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.calculate_view_vector(pitch, yaw)).unwrap();
        let mut start_pos: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.position().add((0) as f64, (Utils::get_eye_height(player)) as f64, (0) as f64)).unwrap();
        let mut raycast: Option<Vec3> = PredictiveSmoothAOTE::raycast(range, look.clone(), &mut start_pos, is_etherwarp);
        if let Some(raycast) = &raycast {
            let mut target: BlockPos = BlockPos::containing_position(&start_pos.add_vec(raycast));
            if is_etherwarp {
                if !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&target.above()).is_air()) {
                    return;
                }
                if !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&target.above_n(2)).is_air()) {
                    return;
                }
            } else {
                target = target.below();
            }
            let config = &SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay;
            if !config.show_when_in_air && mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&target).is_air() {
                return;
            }
            if config.highlight_type != HighlightType::Outline {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &target, &compat::get_state(&COLOR_COMPONENTS), compat::lock(&COLOR_COMPONENTS)[(3) as usize], false) });
            }
            if config.highlight_type != HighlightType::Highlight {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box()); __f(collector, &target, &compat::get_state(&COLOR_COMPONENTS), 4.0f32, false) });
            }
        }
    }

    /// java: public static void configCallback(Color color)
    pub fn config_callback(color: &Color) {
        // TODO(fabric2seer): `.getRGBComponents(..)` on a Color
        //   java: colorComponents = color.getRGBComponents(null);
    }

}
