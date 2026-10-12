//! Ported from de.hysky.skyblocker.config.categories.CrimsonIsleCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class CrimsonIsleCategory
#[derive(Clone)]
pub struct CrimsonIsleCategory {}

impl CrimsonIsleCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &SkyblockerConfig, config: &SkyblockerConfig) -> crate::unresolved::ConfigCategory {
        // TODO(fabric2seer): static call ConfigCategory.createBuilder(..)
        //   java: return ConfigCategory.createBuilder()
        //   java: 				.id(SkyblockerMod.id("config/crimsonisle"))
        //   java: 				.name(Component.translatable("skyblocker.config.crimsonIsle"))
        //   java:
        //   java: 				//Kuudra
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.supplyWaypoints"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.supplyWaypoints,
        //   java: 										() -> config.crimsonIsle.kuudra.supplyWaypoints,
        //   java: 										newValue -> config.crimsonIsle.kuudra.supplyWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.fuelWaypoints"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.fuelWaypoints,
        //   java: 										() -> config.crimsonIsle.kuudra.fuelWaypoints,
        //   java: 										newValue -> config.crimsonIsle.kuudra.fuelWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Waypoint.Type>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.suppliesAndFuelWaypointType"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.@Tooltip"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.suppliesAndFuelWaypointType,
        //   java: 										() -> config.crimsonIsle.kuudra.suppliesAndFuelWaypointType,
        //   java: 										newValue -> config.crimsonIsle.kuudra.suppliesAndFuelWaypointType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.ballistaBuildWaypoints"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.ballistaBuildWaypoints,
        //   java: 										() -> config.crimsonIsle.kuudra.ballistaBuildWaypoints,
        //   java: 										newValue -> config.crimsonIsle.kuudra.ballistaBuildWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.safeSpotWaypoints"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.safeSpotWaypoints,
        //   java: 										() -> config.crimsonIsle.kuudra.safeSpotWaypoints,
        //   java: 										newValue -> config.crimsonIsle.kuudra.safeSpotWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.pearlWaypoints"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.pearlWaypoints,
        //   java: 										() -> config.crimsonIsle.kuudra.pearlWaypoints,
        //   java: 										newValue -> config.crimsonIsle.kuudra.pearlWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.noArrowPoisonWarning"))
        //   java: 								.description(Component.translatable("skyblocker.config.crimsonIsle.kuudra.noArrowPoisonWarning.@Tooltip"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.noArrowPoisonWarning,
        //   java: 										() -> config.crimsonIsle.kuudra.noArrowPoisonWarning,
        //   java: 										newValue -> config.crimsonIsle.kuudra.noArrowPoisonWarning = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.arrowPoisonThreshold"))
        //   java: 								.description(Component.translatable("skyblocker.config.crimsonIsle.kuudra.arrowPoisonThreshold.@Tooltip"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.arrowPoisonThreshold,
        //   java: 										() -> config.crimsonIsle.kuudra.arrowPoisonThreshold,
        //   java: 										newValue -> config.crimsonIsle.kuudra.arrowPoisonThreshold = newValue)
        //   java: 								.controller(IntegerController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.kuudraGlow"))
        //   java: 								.description(Component.translatable("skyblocker.config.crimsonIsle.kuudra.kuudraGlow.@Tooltip"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.kuudraGlow,
        //   java: 										() -> config.crimsonIsle.kuudra.kuudraGlow,
        //   java: 										newValue -> config.crimsonIsle.kuudra.kuudraGlow = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.crimsonIsle.kuudra.dangerWarning"))
        //   java: 								.description(Component.translatable("skyblocker.config.crimsonIsle.kuudra.dangerWarning.@Tooltip"))
        //   java: 								.binding(defaults.crimsonIsle.kuudra.dangerWarning,
        //   java: 										() -> config.crimsonIsle.kuudra.dangerWarning,
        //   java: 										newValue -> config.crimsonIsle.kuudra.dangerWarning = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				//dojo
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.crimson.dojo"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.crimson.dojo.forceHelper"))
        //   java: 								.description(Component.translatable("skyblocker.crimson.dojo.forceHelper.@Tooltip"))
        //   java: 								.binding(config.crimsonIsle.dojo.enableForceHelper,
        //   java: 										() -> config.crimsonIsle.dojo.enableForceHelper,
        //   java: 										newValue -> config.crimsonIsle.dojo.enableForceHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.crimson.dojo.staminaHelper"))
        //   java: 								.description(Component.translatable("skyblocker.crimson.dojo.staminaHelper.@Tooltip"))
        //   java: 								.binding(config.crimsonIsle.dojo.enableStaminaHelper,
        //   java: 										() -> config.crimsonIsle.dojo.enableStaminaHelper,
        //   java: 										newValue -> config.crimsonIsle.dojo.enableStaminaHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.crimson.dojo.masteryHelper"))
        //   java: 								.description(Component.translatable("skyblocker.crimson.dojo.masteryHelper.@Tooltip"))
        //   java: 								.binding(config.crimsonIsle.dojo.enableMasteryHelper,
        //   java: 										() -> config.crimsonIsle.dojo.enableMasteryHelper,
        //   java: 										newValue -> config.crimsonIsle.dojo.enableMasteryHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.crimson.dojo.disciplineHelper"))
        //   java: 								.description(Component.translatable("skyblocker.crimson.dojo.disciplineHelper.@Tooltip"))
        //   java: 								.binding(config.crimsonIsle.dojo.enableDisciplineHelper,
        //   java: 										() -> config.crimsonIsle.dojo.enableDisciplineHelper,
        //   java: 										newValue -> config.crimsonIsle.dojo.enableDisciplineHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.crimson.dojo.swiftnessHelper"))
        //   java: 								.description(Component.translatable("skyblocker.crimson.dojo.swiftnessHelper.@Tooltip"))
        //   java: 								.binding(config.crimsonIsle.dojo.enableSwiftnessHelper,
        //   java: 										() -> config.crimsonIsle.dojo.enableSwiftnessHelper,
        //   java: 										newValue -> config.crimsonIsle.dojo.enableSwiftnessHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.crimson.dojo.controlHelper"))
        //   java: 								.description(Component.translatable("skyblocker.crimson.dojo.controlHelper.@Tooltip"))
        //   java: 								.binding(config.crimsonIsle.dojo.enableControlHelper,
        //   java: 										() -> config.crimsonIsle.dojo.enableControlHelper,
        //   java: 										newValue -> config.crimsonIsle.dojo.enableControlHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.crimson.dojo.tenacityHelper"))
        //   java: 								.description(Component.translatable("skyblocker.crimson.dojo.tenacityHelper.@Tooltip"))
        //   java: 								.binding(config.crimsonIsle.dojo.enableTenacityHelper,
        //   java: 										() -> config.crimsonIsle.dojo.enableTenacityHelper,
        //   java: 										newValue -> config.crimsonIsle.dojo.enableTenacityHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Extend nether fog
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.crimsonIsle.extendNetherFog"))
        //   java: 						.description(Component.translatable("skyblocker.config.crimsonIsle.extendNetherFog.@Tooltip"))
        //   java: 						.binding(config.crimsonIsle.extendNetherFog,
        //   java: 								() -> config.crimsonIsle.extendNetherFog,
        //   java: 								newValue -> config.crimsonIsle.extendNetherFog = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
