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
        // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): `SKYBLOCKER_VANILLA_COLLECTOR=1` keeps the
        // separate player null check and the held id as a `String` (`extract_rendering_vanilla`).
        if crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector_impl::VANILLA_COLLECTOR.on() {
            TeleportOverlay::extract_rendering_vanilla(collector);
            return;
        }
        // The player null check folded into the read that follows it: `with_player` answers `None` for a null player
        // and nothing is read then, as the `&&` did (the level test, which reads nothing, moves ahead of it). The held
        // id is matched BORROWED (`with_skyblock_id`) to the arm it selects - no `String` per frame - and the arm runs
        // outside the player's lock, as before.
        if Utils::is_on_skyblock() && SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_teleport_overlays && mcrust::mcrust::modding::game::level().is_some()
            && let Some((arm, __custom_data)) = mcrust::mcrust::modding::game::with_player(|player| {
                let held_item: &mcrust::net::minecraft::world::item::item_stack::ItemStack = player.get_main_hand_item();
                (crate::mixins::item_stack_mixin::with_skyblock_id(held_item, HeldArm::of), ItemUtils::custom_data_component(held_item))
            })
        {
            let custom_data: &Tag = ItemUtils::custom_data_tag(&__custom_data);
            TeleportOverlay::extract_rendering_arm(collector, arm, custom_data);
        }
    }

    /// `extractRendering(collector)`'s switch, on the arm the held id selected.
    fn extract_rendering_arm(collector: &compat::Shared<dyn PrimitiveCollector>, arm: HeldArm, custom_data: &Tag) {
        match arm {
            HeldArm::Leech1 => {
                if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_weird_transmission {
                    TeleportOverlay::extract_rendering_3(collector, 3, false);
                }
            }
            HeldArm::Leech2 => {
                if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_weird_transmission {
                    TeleportOverlay::extract_rendering_3(collector, 4, false);
                }
            }
            HeldArm::Leech3 => {
                if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_weird_transmission {
                    TeleportOverlay::extract_rendering_3(collector, 5, false);
                }
            }
            HeldArm::AspectOfTheEndOrVoid => {
                // java: client.options.keyShift.isDown()
                if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_ether_transmission && key_mapping::is_down("key.sneak") && custom_data.get_int_or("ethermerge", 0) == 1 {
                    TeleportOverlay::extract_rendering_4(collector, &custom_data, 57, true);
                } else if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_instant_transmission {
                    TeleportOverlay::extract_rendering_4(collector, &custom_data, 8, false);
                }
            }
            HeldArm::EtherwarpConduit => {
                if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_ether_transmission {
                    TeleportOverlay::extract_rendering_4(collector, &custom_data, 57, true);
                }
            }
            HeldArm::SinseekerScythe => {
                if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_sinrecall_transmission {
                    TeleportOverlay::extract_rendering_4(collector, &custom_data, 4, false);
                }
            }
            HeldArm::WitherImpact => {
                if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_wither_impact {
                    TeleportOverlay::extract_rendering_3(collector, 10, false);
                }
            }
            HeldArm::Other => {}
        }
    }

    /// `extractRendering(collector)` as it was before perf round 1 (`SKYBLOCKER_VANILLA_COLLECTOR=1`).
    fn extract_rendering_vanilla(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if Utils::is_on_skyblock() && SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_teleport_overlays && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::level().is_some() {
            // PERF (port-only, same result): the held stack's id and custom data are read off the player's own
            // stack - not a `clone()` of it (a deep copy of its components) - and the custom data in place
            // (`ItemUtils.getCustomData` is the component's own tag in Java, not a copy), every frame.
            let (mut item_id, __custom_data): (String, mcrust::com::mojang::serialization::Obj) = mcrust::mcrust::modding::game::with_player(|player| {
                let held_item: &mcrust::net::minecraft::world::item::item_stack::ItemStack = player.get_main_hand_item();
                (held_item.get_skyblock_id(), ItemUtils::custom_data_component(held_item))
            }).unwrap();
            let custom_data: &Tag = ItemUtils::custom_data_tag(&__custom_data);
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
                    // java: client.options.keyShift.isDown()
                    if SkyblockerConfigManager::get().ui_and_visuals.teleport_overlay.enable_ether_transmission && key_mapping::is_down("key.sneak") && custom_data.get_int_or("ethermerge", 0) == 1 {
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
        // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the null check and the four reads in ONE
        // `with_player` - the same reads, in the same order, with nothing between them; `None` (a null player) returns
        // before any read, as the null check did (the level test, which reads nothing, moves ahead of it). Was:
        //     if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
        //         return;
        //     }
        //     let mut pitch: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x_rot()).unwrap();
        //     let mut yaw: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_y_rot()).unwrap();
        //     let mut look: Vec3 = mcrust::mcrust::modding::game::with_player(|player| mcrust::net::minecraft::world::entity::entity::EntityBase::calculate_view_vector(pitch, yaw)).unwrap();
        //     let mut start_pos: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.position().add((0) as f64, (Utils::get_eye_height(player)) as f64, (0) as f64)).unwrap();
        let (mut pitch, mut yaw, mut look, mut start_pos): (f32, f32, Vec3, Vec3) = if crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector_impl::VANILLA_COLLECTOR.on() {
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
                return;
            }
            let pitch: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x_rot()).unwrap();
            let yaw: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_y_rot()).unwrap();
            let look: Vec3 = mcrust::mcrust::modding::game::with_player(|player| mcrust::net::minecraft::world::entity::entity::EntityBase::calculate_view_vector(pitch, yaw)).unwrap();
            let start_pos: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.position().add((0) as f64, (Utils::get_eye_height(player)) as f64, (0) as f64)).unwrap();
            (pitch, yaw, look, start_pos)
        } else {
            if mcrust::mcrust::modding::game::level().is_none() {
                return;
            }
            let Some(read) = mcrust::mcrust::modding::game::with_player(|player| {
                let pitch: f32 = player.base.base.base.base.get_x_rot();
                let yaw: f32 = player.base.base.base.base.get_y_rot();
                let look: Vec3 = mcrust::net::minecraft::world::entity::entity::EntityBase::calculate_view_vector(pitch, yaw);
                let start_pos: Vec3 = player.position().add((0) as f64, (Utils::get_eye_height(player)) as f64, (0) as f64);
                (pitch, yaw, look, start_pos)
            }) else {
                return;
            };
            read
        };
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
        // java: colorComponents = color.getRGBComponents(null);
        *compat::lock(&COLOR_COMPONENTS) = color.get_rgb_components(None).to_vec();
    }

}

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the arm of `extractRendering`'s
/// `switch (itemId)` a held skyblock id selects - decided on the BORROWED id, so no `String` is built per frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HeldArm {
    Leech1,
    Leech2,
    Leech3,
    AspectOfTheEndOrVoid,
    EtherwarpConduit,
    SinseekerScythe,
    WitherImpact,
    Other,
}

impl HeldArm {
    /// The `case` labels of the switch, exactly.
    fn of(item_id: &str) -> HeldArm {
        match item_id {
            "ASPECT_OF_THE_LEECH_1" => HeldArm::Leech1,
            "ASPECT_OF_THE_LEECH_2" => HeldArm::Leech2,
            "ASPECT_OF_THE_LEECH_3" => HeldArm::Leech3,
            "ASPECT_OF_THE_END" | "ASPECT_OF_THE_VOID" => HeldArm::AspectOfTheEndOrVoid,
            "ETHERWARP_CONDUIT" => HeldArm::EtherwarpConduit,
            "SINSEEKER_SCYTHE" => HeldArm::SinseekerScythe,
            "NECRON_BLADE" | "ASTRAEA" | "HYPERION" | "SCYLLA" | "VALKYRIE" => HeldArm::WitherImpact,
            _ => HeldArm::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HeldArm;

    /// The arms are the vanilla switch's labels, one for one.
    #[test]
    fn every_case_label_selects_its_arm() {
        assert_eq!(HeldArm::of("ASPECT_OF_THE_LEECH_1"), HeldArm::Leech1);
        assert_eq!(HeldArm::of("ASPECT_OF_THE_LEECH_2"), HeldArm::Leech2);
        assert_eq!(HeldArm::of("ASPECT_OF_THE_LEECH_3"), HeldArm::Leech3);
        assert_eq!(HeldArm::of("ASPECT_OF_THE_END"), HeldArm::AspectOfTheEndOrVoid);
        assert_eq!(HeldArm::of("ASPECT_OF_THE_VOID"), HeldArm::AspectOfTheEndOrVoid);
        assert_eq!(HeldArm::of("ETHERWARP_CONDUIT"), HeldArm::EtherwarpConduit);
        assert_eq!(HeldArm::of("SINSEEKER_SCYTHE"), HeldArm::SinseekerScythe);
        for wither in ["NECRON_BLADE", "ASTRAEA", "HYPERION", "SCYLLA", "VALKYRIE"] {
            assert_eq!(HeldArm::of(wither), HeldArm::WitherImpact);
        }
        for other in ["", "ASPECT_OF_THE_LEECH_4", "aspect_of_the_end", "GYROKINETIC_WAND"] {
            assert_eq!(HeldArm::of(other), HeldArm::Other);
        }
    }
}
