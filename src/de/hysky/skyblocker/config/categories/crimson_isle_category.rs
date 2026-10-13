//! Ported from de.hysky.skyblocker.config.categories.CrimsonIsleCategory.
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

/// java: public class CrimsonIsleCategory
#[derive(Clone)]
pub struct CrimsonIsleCategory {}

impl CrimsonIsleCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/crimsonisle"))
            .name(component::translatable("skyblocker.config.crimsonIsle"))
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.crimsonIsle.kuudra"))
                .collapsed(false)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.supplyWaypoints"))
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.supply_waypoints.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.supply_waypoints.clone())) }, { let config = config.clone(); move |new_value: bool| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.supply_waypoints = new_value)) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.fuelWaypoints"))
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.fuel_waypoints.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.fuel_waypoints.clone())) }, { let config = config.clone(); move |new_value: bool| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.fuel_waypoints = new_value)) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.suppliesAndFuelWaypointType"))
                    .description(vec![component::translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.@Tooltip")])
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.supplies_and_fuel_waypoint_type.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.supplies_and_fuel_waypoint_type.clone())) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.supplies_and_fuel_waypoint_type = new_value)) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type>())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.ballistaBuildWaypoints"))
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.ballista_build_waypoints.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.ballista_build_waypoints.clone())) }, { let config = config.clone(); move |new_value: bool| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.ballista_build_waypoints = new_value)) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.safeSpotWaypoints"))
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.safe_spot_waypoints.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.safe_spot_waypoints.clone())) }, { let config = config.clone(); move |new_value: bool| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.safe_spot_waypoints = new_value)) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.pearlWaypoints"))
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.pearl_waypoints.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.pearl_waypoints.clone())) }, { let config = config.clone(); move |new_value: bool| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.pearl_waypoints = new_value)) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.noArrowPoisonWarning"))
                    .description(vec![component::translatable("skyblocker.config.crimsonIsle.kuudra.noArrowPoisonWarning.@Tooltip")])
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.no_arrow_poison_warning.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.no_arrow_poison_warning.clone())) }, { let config = config.clone(); move |new_value: bool| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.no_arrow_poison_warning = new_value)) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<i32>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.arrowPoisonThreshold"))
                    .description(vec![component::translatable("skyblocker.config.crimsonIsle.kuudra.arrowPoisonThreshold.@Tooltip")])
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.arrow_poison_threshold.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.arrow_poison_threshold.clone())) }, { let config = config.clone(); move |new_value: i32| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.arrow_poison_threshold = new_value)) })
                    .controller(integer_controller::create_builder()
                        .build())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.kuudraGlow"))
                    .description(vec![component::translatable("skyblocker.config.crimsonIsle.kuudra.kuudraGlow.@Tooltip")])
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.kuudra_glow.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.kuudra_glow.clone())) }, { let config = config.clone(); move |new_value: bool| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.kuudra_glow = new_value)) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.crimsonIsle.kuudra.dangerWarning"))
                    .description(vec![component::translatable("skyblocker.config.crimsonIsle.kuudra.dangerWarning.@Tooltip")])
                    .binding(compat::read(&defaults, |c| compat::read(&c.crimson_isle.kuudra, |c| c.danger_warning.clone())), { let config = config.clone(); move || compat::read(&config, |c| compat::read(&c.crimson_isle.kuudra, |c| c.danger_warning.clone())) }, { let config = config.clone(); move |new_value: bool| compat::read(&config, |c| compat::write(&c.crimson_isle.kuudra, |c| c.danger_warning = new_value)) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.crimson.dojo"))
                .collapsed(false)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.crimson.dojo.forceHelper"))
                    .description(vec![component::translatable("skyblocker.crimson.dojo.forceHelper.@Tooltip")])
                    .binding(compat::read(&config, |c| c.crimson_isle.dojo.enable_force_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.crimson_isle.dojo.enable_force_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.crimson_isle.dojo.enable_force_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.crimson.dojo.staminaHelper"))
                    .description(vec![component::translatable("skyblocker.crimson.dojo.staminaHelper.@Tooltip")])
                    .binding(compat::read(&config, |c| c.crimson_isle.dojo.enable_stamina_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.crimson_isle.dojo.enable_stamina_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.crimson_isle.dojo.enable_stamina_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.crimson.dojo.masteryHelper"))
                    .description(vec![component::translatable("skyblocker.crimson.dojo.masteryHelper.@Tooltip")])
                    .binding(compat::read(&config, |c| c.crimson_isle.dojo.enable_mastery_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.crimson_isle.dojo.enable_mastery_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.crimson_isle.dojo.enable_mastery_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.crimson.dojo.disciplineHelper"))
                    .description(vec![component::translatable("skyblocker.crimson.dojo.disciplineHelper.@Tooltip")])
                    .binding(compat::read(&config, |c| c.crimson_isle.dojo.enable_discipline_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.crimson_isle.dojo.enable_discipline_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.crimson_isle.dojo.enable_discipline_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.crimson.dojo.swiftnessHelper"))
                    .description(vec![component::translatable("skyblocker.crimson.dojo.swiftnessHelper.@Tooltip")])
                    .binding(compat::read(&config, |c| c.crimson_isle.dojo.enable_swiftness_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.crimson_isle.dojo.enable_swiftness_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.crimson_isle.dojo.enable_swiftness_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.crimson.dojo.controlHelper"))
                    .description(vec![component::translatable("skyblocker.crimson.dojo.controlHelper.@Tooltip")])
                    .binding(compat::read(&config, |c| c.crimson_isle.dojo.enable_control_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.crimson_isle.dojo.enable_control_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.crimson_isle.dojo.enable_control_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.crimson.dojo.tenacityHelper"))
                    .description(vec![component::translatable("skyblocker.crimson.dojo.tenacityHelper.@Tooltip")])
                    .binding(compat::read(&config, |c| c.crimson_isle.dojo.enable_tenacity_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.crimson_isle.dojo.enable_tenacity_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.crimson_isle.dojo.enable_tenacity_helper = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.crimsonIsle.extendNetherFog"))
                .description(vec![component::translatable("skyblocker.config.crimsonIsle.extendNetherFog.@Tooltip")])
                .binding(compat::read(&config, |c| c.crimson_isle.extend_nether_fog.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.crimson_isle.extend_nether_fog.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.crimson_isle.extend_nether_fog = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .build();
    }
}
