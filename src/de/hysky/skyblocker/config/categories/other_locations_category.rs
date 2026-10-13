//! Ported from de.hysky.skyblocker.config.categories.OtherLocationsCategory.
//!
//! The category is built on Dandelion (`dandelion` crate, rendered through YACL). Java hands `create` the two
//! `SkyblockerConfig` objects; here they are the manager's shared instances (`compat::Shared`), and every option's
//! binding reads / writes its field through them, borrowing per statement.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::categories::{ChatFormattingEnum, KeyMappingNullable as _};
use crate::de::hysky::skyblocker::config::common_tags;
use crate::de::hysky::skyblocker::config::config_utils::ConfigUtils;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::entry::skyblocker_mod;
use dandelion::net::azureaaron::dandelion::api::button_option::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::config_category::{self, Builder as _, ConfigCategory};
use dandelion::net::azureaaron::dandelion::api::config_type::ConfigType;
use dandelion::net::azureaaron::dandelion::api::controllers::boolean_controller::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::controllers::colour_controller::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::controllers::controller::Builder as _;
use dandelion::net::azureaaron::dandelion::api::controllers::enum_controller::Builder as _;
use dandelion::net::azureaaron::dandelion::api::controllers::float_controller;
use dandelion::net::azureaaron::dandelion::api::controllers::integer_controller;
use dandelion::net::azureaaron::dandelion::api::controllers::item_controller;
use dandelion::net::azureaaron::dandelion::api::controllers::number_controller::Builder as _;
use dandelion::net::azureaaron::dandelion::api::controllers::string_controller;
use dandelion::net::azureaaron::dandelion::api::key_mapping_option::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::label_option::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::list_option::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::option::{self, AnyOption, Builder as _};
use dandelion::net::azureaaron::dandelion::api::option_adder::OptionAdder as _;
use dandelion::net::azureaaron::dandelion::api::option_group::{self, Builder as _, OptionGroup};
use dandelion::net::azureaaron::dandelion::api::option_listener::UpdateType;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class OtherLocationsCategory
#[derive(Clone)]
pub struct OtherLocationsCategory {}

impl OtherLocationsCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/otherlocations"))
            .name(component::translatable("skyblocker.config.otherLocations"))
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.otherLocations.barn"))
                .collapsed(false)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.barn.enableGlowingMushroomHelper"))
                    .binding(compat::read(&defaults, |c| c.other_locations.barn.enable_glowing_mushroom_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.barn.enable_glowing_mushroom_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.barn.enable_glowing_mushroom_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.barn.solveHungryHiker"))
                    .binding(compat::read(&defaults, |c| c.other_locations.barn.solve_hungry_hiker.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.barn.solve_hungry_hiker.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.barn.solve_hungry_hiker = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.barn.solveTreasureHunter"))
                    .binding(compat::read(&defaults, |c| c.other_locations.barn.solve_treasure_hunter.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.barn.solve_treasure_hunter.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.barn.solve_treasure_hunter = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.barn.callTrevor"))
                    .description(vec![component::translatable("skyblocker.config.otherLocations.barn.callTrevor.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.other_locations.barn.enable_call_trevor_message.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.barn.enable_call_trevor_message.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.barn.enable_call_trevor_message = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.barn.enablePeltAnimalHighlighter"))
                    .description(vec![component::translatable("skyblocker.config.otherLocations.barn.enablePeltAnimalHighlighter.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_2_0.get().clone(), component::literal("pelt animal")])
                    .binding(compat::read(&defaults, |c| c.other_locations.barn.enable_pelt_animal_highlighter.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.barn.enable_pelt_animal_highlighter.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.barn.enable_pelt_animal_highlighter = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.otherLocations.rift"))
                .collapsed(false)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.rift.mirrorverseWaypoints"))
                    .binding(compat::read(&defaults, |c| c.other_locations.rift.mirrorverse_waypoints.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.rift.mirrorverse_waypoints.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.rift.mirrorverse_waypoints = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.rift.blobbercystGlow"))
                    .description(vec![component::translatable("skyblocker.config.otherLocations.rift.blobbercystGlow.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.other_locations.rift.blobbercyst_glow.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.rift.blobbercyst_glow.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.rift.blobbercyst_glow = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.rift.enigmaSoulWaypoints"))
                    .description(vec![component::translatable("skyblocker.config.otherLocations.rift.enigmaSoulWaypoints.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.other_locations.rift.enigma_soul_waypoints.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.rift.enigma_soul_waypoints.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.rift.enigma_soul_waypoints = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.rift.highlightFoundEnigmaSouls"))
                    .binding(compat::read(&defaults, |c| c.other_locations.rift.highlight_found_enigma_souls.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.rift.highlight_found_enigma_souls.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.rift.highlight_found_enigma_souls = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.rift.autoDetectMcGrubber"))
                    .description(vec![component::translatable("skyblocker.config.otherLocations.rift.autoDetectMcGrubber.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_4_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.other_locations.rift.auto_detect_mc_grubber.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.rift.auto_detect_mc_grubber.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.rift.auto_detect_mc_grubber = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<i32>()
                    .name(component::translatable("skyblocker.config.otherLocations.rift.mcGrubberStacks"))
                    .description(vec![component::translatable("skyblocker.config.otherLocations.rift.mcGrubberStacks.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.other_locations.rift.mc_grubber_stacks.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.rift.mc_grubber_stacks.clone()) }, { let config = config.clone(); move |new_value: i32| compat::write(&config, |c| c.other_locations.rift.mc_grubber_stacks = new_value) })
                    .controller(integer_controller::create_builder()
                        .range(0, 5)
                        .slider(1)
                        .build())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.otherLocations.end"))
                .collapsed(false)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.end.enableEnderNodeHelper"))
                    .binding(compat::read(&defaults, |c| c.other_locations.end.enable_ender_node_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.end.enable_ender_node_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.end.enable_ender_node_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type>()
                    .name(component::translatable("skyblocker.config.otherLocations.end.enderNodeWaypointType"))
                    .description(vec![component::translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.other_locations.end.ender_node_waypoint_type.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.end.ender_node_waypoint_type.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type| compat::write(&config, |c| c.other_locations.end.ender_node_waypoint_type = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type>())
                    .build())
                .option(button_option::create_builder()
                    .name(component::translatable("skyblocker.config.otherLocations.end.screen"))
                    .description(vec![component::translatable("skyblocker.config.hud.movedMessage")])
                    .prompt(component::translatable("text.skyblocker.open"))
                    .action(|_screen| mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen::new_2(crate::de::hysky::skyblocker::utils::location::Location::TheEnd, None)))))
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.end.waypoint"))
                    .binding(compat::read(&defaults, |c| c.other_locations.end.waypoint.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.end.waypoint.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.end.waypoint = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(button_option::create_builder()
                    .name(component::translatable("skyblocker.config.otherLocations.end.resetName"))
                    .prompt(component::translatable("skyblocker.config.otherLocations.end.resetText"))
                    .action(|_| { crate::de::hysky::skyblocker::utils::data::profiled_data::ProfiledData::put(crate::de::hysky::skyblocker::skyblock::end::the_end::PROFILES_STATS.get(), (crate::de::hysky::skyblocker::skyblock::end::the_end::END_STATS_EMPTY.get())()); })
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.end.muteEndermanSounds"))
                    .binding(compat::read(&defaults, |c| c.other_locations.end.mute_enderman_sounds.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.end.mute_enderman_sounds.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.end.mute_enderman_sounds = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.otherLocations.spidersDen"))
                .collapsed(false)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.spidersDen.relics.enableRelicsHelper"))
                    .binding(compat::read(&defaults, |c| c.other_locations.spiders_den.relics.enable_relics_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.spiders_den.relics.enable_relics_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.spiders_den.relics.enable_relics_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.otherLocations.spidersDen.relics.highlightFoundRelics"))
                    .binding(compat::read(&defaults, |c| c.other_locations.spiders_den.relics.highlight_found_relics.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.other_locations.spiders_den.relics.highlight_found_relics.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.other_locations.spiders_den.relics.highlight_found_relics = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .build();
    }
}
