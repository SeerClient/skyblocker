//! Ported from de.hysky.skyblocker.config.categories.OtherLocationsCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class OtherLocationsCategory
#[derive(Clone)]
pub struct OtherLocationsCategory {}

impl OtherLocationsCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/otherlocations"))
        //   java: 				.name(Component.translatable("skyblocker.config.otherLocations"))
        //   java:
        //   java: 				//Barn
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.otherLocations.barn"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.barn.enableGlowingMushroomHelper"))
        //   java: 								.binding(defaults.otherLocations.barn.enableGlowingMushroomHelper,
        //   java: 										() -> config.otherLocations.barn.enableGlowingMushroomHelper,
        //   java: 										newValue -> config.otherLocations.barn.enableGlowingMushroomHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.barn.solveHungryHiker"))
        //   java: 								.binding(defaults.otherLocations.barn.solveHungryHiker,
        //   java: 										() -> config.otherLocations.barn.solveHungryHiker,
        //   java: 										newValue -> config.otherLocations.barn.solveHungryHiker = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.barn.solveTreasureHunter"))
        //   java: 								.binding(defaults.otherLocations.barn.solveTreasureHunter,
        //   java: 										() -> config.otherLocations.barn.solveTreasureHunter,
        //   java: 										newValue -> config.otherLocations.barn.solveTreasureHunter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.barn.callTrevor"))
        //   java: 								.description(Component.translatable("skyblocker.config.otherLocations.barn.callTrevor.@Tooltip"))
        //   java: 								.binding(defaults.otherLocations.barn.enableCallTrevorMessage,
        //   java: 										() -> config.otherLocations.barn.enableCallTrevorMessage,
        //   java: 										newValue -> config.otherLocations.barn.enableCallTrevorMessage = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.barn.enablePeltAnimalHighlighter"))
        //   java: 								.description(Component.translatable("skyblocker.config.otherLocations.barn.enablePeltAnimalHighlighter.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_2_0, Component.literal("pelt animal"))
        //   java: 								.binding(defaults.otherLocations.barn.enablePeltAnimalHighlighter,
        //   java: 										() -> config.otherLocations.barn.enablePeltAnimalHighlighter,
        //   java: 										newValue -> config.otherLocations.barn.enablePeltAnimalHighlighter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//The Rift
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.otherLocations.rift"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.rift.mirrorverseWaypoints"))
        //   java: 								.binding(defaults.otherLocations.rift.mirrorverseWaypoints,
        //   java: 										() -> config.otherLocations.rift.mirrorverseWaypoints,
        //   java: 										newValue -> config.otherLocations.rift.mirrorverseWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.rift.blobbercystGlow"))
        //   java: 								.description(Component.translatable("skyblocker.config.otherLocations.rift.blobbercystGlow.@Tooltip"))
        //   java: 								.binding(defaults.otherLocations.rift.blobbercystGlow,
        //   java: 										() -> config.otherLocations.rift.blobbercystGlow,
        //   java: 										newValue -> config.otherLocations.rift.blobbercystGlow = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.rift.enigmaSoulWaypoints"))
        //   java: 								.description(Component.translatable("skyblocker.config.otherLocations.rift.enigmaSoulWaypoints.@Tooltip"))
        //   java: 								.binding(defaults.otherLocations.rift.enigmaSoulWaypoints,
        //   java: 										() -> config.otherLocations.rift.enigmaSoulWaypoints,
        //   java: 										newValue -> config.otherLocations.rift.enigmaSoulWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.rift.highlightFoundEnigmaSouls"))
        //   java: 								.binding(defaults.otherLocations.rift.highlightFoundEnigmaSouls,
        //   java: 										() -> config.otherLocations.rift.highlightFoundEnigmaSouls,
        //   java: 										newValue -> config.otherLocations.rift.highlightFoundEnigmaSouls = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.rift.autoDetectMcGrubber"))
        //   java: 								.description(Component.translatable("skyblocker.config.otherLocations.rift.autoDetectMcGrubber.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_4_0)
        //   java: 								.binding(defaults.otherLocations.rift.autoDetectMcGrubber,
        //   java: 										() -> config.otherLocations.rift.autoDetectMcGrubber,
        //   java: 										newValue -> config.otherLocations.rift.autoDetectMcGrubber = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.rift.mcGrubberStacks"))
        //   java: 								.description(Component.translatable("skyblocker.config.otherLocations.rift.mcGrubberStacks.@Tooltip"))
        //   java: 								.binding(defaults.otherLocations.rift.mcGrubberStacks,
        //   java: 										() -> config.otherLocations.rift.mcGrubberStacks,
        //   java: 										newValue -> config.otherLocations.rift.mcGrubberStacks = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(0, 5).slider(1).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// The end
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.otherLocations.end"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.end.enableEnderNodeHelper"))
        //   java: 								.binding(defaults.otherLocations.end.enableEnderNodeHelper,
        //   java: 										() -> config.otherLocations.end.enableEnderNodeHelper,
        //   java: 										newValue -> config.otherLocations.end.enableEnderNodeHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Waypoint.Type>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.end.enderNodeWaypointType"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.@Tooltip"))
        //   java: 								.binding(defaults.otherLocations.end.enderNodeWaypointType,
        //   java: 										() -> config.otherLocations.end.enderNodeWaypointType,
        //   java: 										newValue -> config.otherLocations.end.enderNodeWaypointType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.end.screen"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open")) // Reusing again lol
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.THE_END, screen)))
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.end.waypoint"))
        //   java: 								.binding(defaults.otherLocations.end.waypoint,
        //   java: 										() -> config.otherLocations.end.waypoint,
        //   java: 										newValue -> config.otherLocations.end.waypoint = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.end.resetName"))
        //   java: 								.prompt(Component.translatable("skyblocker.config.otherLocations.end.resetText"))
        //   java: 								.action(_ -> TheEnd.PROFILES_STATS.put(TheEnd.EndStats.EMPTY.get()))
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.end.muteEndermanSounds"))
        //   java: 								.binding(defaults.otherLocations.end.muteEndermanSounds,
        //   java: 										() -> config.otherLocations.end.muteEndermanSounds,
        //   java: 										newValue -> config.otherLocations.end.muteEndermanSounds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Spider's Den
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.otherLocations.spidersDen"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.spidersDen.relics.enableRelicsHelper"))
        //   java: 								.binding(defaults.otherLocations.spidersDen.relics.enableRelicsHelper,
        //   java: 										() -> config.otherLocations.spidersDen.relics.enableRelicsHelper,
        //   java: 										newValue -> config.otherLocations.spidersDen.relics.enableRelicsHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.otherLocations.spidersDen.relics.highlightFoundRelics"))
        //   java: 								.binding(defaults.otherLocations.spidersDen.relics.highlightFoundRelics,
        //   java: 										() -> config.otherLocations.spidersDen.relics.highlightFoundRelics,
        //   java: 										newValue -> config.otherLocations.spidersDen.relics.highlightFoundRelics = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
