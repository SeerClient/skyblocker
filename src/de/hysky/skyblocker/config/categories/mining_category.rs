//! Ported from de.hysky.skyblocker.config.categories.MiningCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class MiningCategory
#[derive(Clone)]
pub struct MiningCategory {}

impl MiningCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/mining"))
        //   java: 				.name(Component.translatable("skyblocker.config.mining"))
        //   java:
        //   java: 				//Uncategorized Options
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.enableDrillFuel"))
        //   java: 						.description(Component.translatable("skyblocker.config.mining.enableDrillFuel.@Tooltip"))
        //   java: 						.binding(defaults.mining.enableDrillFuel,
        //   java: 								() -> config.mining.enableDrillFuel,
        //   java: 								newValue -> config.mining.enableDrillFuel = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.commissionHighlight"))
        //   java: 						.binding(defaults.mining.commissionHighlight,
        //   java: 								() -> config.mining.commissionHighlight,
        //   java: 								newValue -> config.mining.commissionHighlight = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.callMismyla"))
        //   java: 						.description(Component.translatable("skyblocker.config.mining.callMismyla.@Tooltip"))
        //   java: 						.binding(defaults.mining.callMismyla,
        //   java: 								() -> config.mining.callMismyla,
        //   java: 								newValue -> config.mining.callMismyla = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.redialOnBadSignal"))
        //   java: 						.description(Component.translatable("skyblocker.config.mining.redialOnBadSignal.@Tooltip"))
        //   java: 						.binding(defaults.mining.redialOnBadSignal,
        //   java: 								() -> config.mining.redialOnBadSignal,
        //   java: 								newValue -> config.mining.redialOnBadSignal = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(ButtonOption.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.dwarvenHud.screen"))
        //   java: 						.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 						.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 						.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.DWARVEN_MINES, screen)))
        //   java: 						.build())
        //   java:
        //   java: 				//Pickobulus Helper
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.pickobulusHelper"))
        //   java: 						.tags(CommonTags.ADDED_IN_5_11_0)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.pickobulusHelper.enable"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.pickobulusHelper.enable.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_10_0)
        //   java: 								.binding(defaults.mining.pickobulusHelper.enablePickobulusHelper,
        //   java: 										() -> config.mining.pickobulusHelper.enablePickobulusHelper,
        //   java: 										newValue -> config.mining.pickobulusHelper.enablePickobulusHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.pickobulusHelper.enableHud"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_11_0)
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.DWARVEN_MINES, screen)))
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.pickobulusHelper.hideOnCooldown"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.pickobulusHelper.hideOnCooldown.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_11_0)
        //   java: 								.binding(defaults.mining.pickobulusHelper.hideHudOnCooldown,
        //   java: 										() -> config.mining.pickobulusHelper.hideHudOnCooldown,
        //   java: 										newValue -> config.mining.pickobulusHelper.hideHudOnCooldown = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Dwarven Mines
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.dwarvenMines"))
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.dwarvenMines.solveFetchur"))
        //   java: 								.binding(defaults.mining.dwarvenMines.solveFetchur,
        //   java: 										() -> config.mining.dwarvenMines.solveFetchur,
        //   java: 										newValue -> config.mining.dwarvenMines.solveFetchur = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.dwarvenMines.solvePuzzler"))
        //   java: 								.binding(defaults.mining.dwarvenMines.solvePuzzler,
        //   java: 										() -> config.mining.dwarvenMines.solvePuzzler,
        //   java: 										newValue -> config.mining.dwarvenMines.solvePuzzler = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.dwarvenMines.enableCarpetHighlight"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.dwarvenMines.enableCarpetHighlight.@Tooltip"))
        //   java: 								.binding(defaults.mining.dwarvenMines.enableCarpetHighlighter,
        //   java: 										() -> config.mining.dwarvenMines.enableCarpetHighlighter,
        //   java: 									newValue -> config.mining.dwarvenMines.enableCarpetHighlighter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.dwarvenMines.carpetHighlightColor"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.dwarvenMines.carpetHighlightColor.@Tooltip"))
        //   java: 								.binding(defaults.mining.dwarvenMines.carpetHighlightColor,
        //   java: 										() -> config.mining.dwarvenMines.carpetHighlightColor,
        //   java: 										newValue -> {
        //   java: 											config.mining.dwarvenMines.carpetHighlightColor = newValue;
        //   java: 											CarpetHighlighter.INSTANCE.configCallback(newValue);
        //   java: 										})
        //   java: 								.controller(ConfigUtils.createColourController(true))
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Crystal Hollows
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.crystalHollows"))
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalHollows.metalDetectorHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalHollows.metalDetectorHelper.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalHollows.metalDetectorHelper,
        //   java: 										() -> config.mining.crystalHollows.metalDetectorHelper,
        //   java: 										newValue -> config.mining.crystalHollows.metalDetectorHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalHollows.nucleusWaypoints"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalHollows.nucleusWaypoints.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalHollows.nucleusWaypoints,
        //   java: 										() -> config.mining.crystalHollows.nucleusWaypoints,
        //   java: 										newValue -> config.mining.crystalHollows.nucleusWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalHollows.chestHighlighter"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalHollows.chestHighlighter.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalHollows.chestHighlighter,
        //   java: 										() -> config.mining.crystalHollows.chestHighlighter,
        //   java: 										newValue -> config.mining.crystalHollows.chestHighlighter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalHollows.chestHighlighter.color"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalHollows.chestHighlighter.color.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalHollows.chestHighlightColor,
        //   java: 										() -> config.mining.crystalHollows.chestHighlightColor,
        //   java: 										newValue -> config.mining.crystalHollows.chestHighlightColor = newValue)
        //   java: 								.controller(ConfigUtils.createColourController(true))
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalHollows.enablePowderTracker"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalHollows.enablePowderTracker.@Tooltip"), Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.CRYSTAL_HOLLOWS, screen)))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalHollows.powderTrackerFilter"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalHollows.powderTrackerFilter.@Tooltip"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new PowderFilterConfigScreen(screen, new ObjectImmutableList<>(PowderMiningTracker.getName2IdMap().keySet()))))
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Crystal Hollows Map
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.crystalsHud"))
        //   java: 						.collapsed(false)
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalsHud.screen"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.CRYSTAL_HOLLOWS, screen)))
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalsHud.mapScaling"))
        //   java: 								.binding(defaults.mining.crystalsHud.mapScaling,
        //   java: 										() -> config.mining.crystalsHud.mapScaling,
        //   java: 										newValue -> config.mining.crystalsHud.mapScaling = newValue)
        //   java: 								.controller(FloatController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalsHud.showLocations"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalsHud.showLocations.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalsHud.showLocations,
        //   java: 										() -> config.mining.crystalsHud.showLocations,
        //   java: 										newValue -> config.mining.crystalsHud.showLocations = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalsHud.showLocations.locationSize"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalsHud.showLocations.locationSize.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalsHud.locationSize,
        //   java: 										() -> config.mining.crystalsHud.locationSize,
        //   java: 										newValue -> config.mining.crystalsHud.locationSize = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(4, 12).slider(2).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Crystal Hollows waypoints
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.crystalsWaypoints"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalsWaypoints.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalsWaypoints.enabled.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalsWaypoints.enabled,
        //   java: 										() -> config.mining.crystalsWaypoints.enabled,
        //   java: 										newValue -> config.mining.crystalsWaypoints.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalsWaypoints.findInChat"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalsWaypoints.findInChat.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalsWaypoints.findInChat,
        //   java: 										() -> config.mining.crystalsWaypoints.findInChat,
        //   java: 										newValue -> config.mining.crystalsWaypoints.findInChat = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalsWaypoints.wishingCompassSolver,
        //   java: 										() -> config.mining.crystalsWaypoints.wishingCompassSolver,
        //   java: 										newValue -> config.mining.crystalsWaypoints.wishingCompassSolver = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.crystalsWaypoints.shareFairyGrotto"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.crystalsWaypoints.shareFairyGrotto.@Tooltip"))
        //   java: 								.binding(defaults.mining.crystalsWaypoints.shareFairyGrotto,
        //   java: 										() -> config.mining.crystalsWaypoints.shareFairyGrotto,
        //   java: 										newValue -> config.mining.crystalsWaypoints.shareFairyGrotto = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//commission waypoints
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.commissionWaypoints"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<MiningConfig.CommissionWaypointMode>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.commissionWaypoints.mode"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.commissionWaypoints.mode.@Tooltip[0]"),
        //   java: 										Component.translatable("skyblocker.config.mining.commissionWaypoints.mode.@Tooltip[1]"),
        //   java: 										Component.translatable("skyblocker.config.mining.commissionWaypoints.mode.@Tooltip[2]"),
        //   java: 										Component.translatable("skyblocker.config.mining.commissionWaypoints.mode.@Tooltip[3]"),
        //   java: 										Component.translatable("skyblocker.config.mining.commissionWaypoints.mode.@Tooltip[4]"))
        //   java: 								.binding(defaults.mining.commissionWaypoints.mode,
        //   java: 										() -> config.mining.commissionWaypoints.mode,
        //   java: 										newValue -> config.mining.commissionWaypoints.mode = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.commissionWaypoints.useColor"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.commissionWaypoints.useColor.@Tooltip"))
        //   java: 								.binding(defaults.mining.commissionWaypoints.useColor,
        //   java: 										() -> config.mining.commissionWaypoints.useColor,
        //   java: 										newValue -> config.mining.commissionWaypoints.useColor = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.commissionWaypoints.showBaseCamp"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.commissionWaypoints.showBaseCamp.@Tooltip"))
        //   java: 								.binding(defaults.mining.commissionWaypoints.showBaseCamp,
        //   java: 										() -> config.mining.commissionWaypoints.showBaseCamp,
        //   java: 										newValue -> config.mining.commissionWaypoints.showBaseCamp = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.commissionWaypoints.showEmissary"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.commissionWaypoints.showEmissary.@Tooltip"))
        //   java: 								.binding(defaults.mining.commissionWaypoints.showEmissary,
        //   java: 										() -> config.mining.commissionWaypoints.showEmissary,
        //   java: 										newValue -> config.mining.commissionWaypoints.showEmissary = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.commissionWaypoints.hideEmissaryOnPigeon"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.commissionWaypoints.hideEmissaryOnPigeon.@Tooltip"))
        //   java: 								.binding(defaults.mining.commissionWaypoints.hideEmissaryOnPigeon,
        //   java: 										() -> config.mining.commissionWaypoints.hideEmissaryOnPigeon,
        //   java: 										newValue -> config.mining.commissionWaypoints.hideEmissaryOnPigeon = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Glacite Tunnels
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.glacite"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.glacite.coldOverlay"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.glacite.coldOverlay.@Tooltip"))
        //   java: 								.binding(defaults.mining.glacite.coldOverlay,
        //   java: 										() -> config.mining.glacite.coldOverlay,
        //   java: 										newValue -> config.mining.glacite.coldOverlay = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.glacite.fossilSolver"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.glacite.fossilSolver.@Tooltip"))
        //   java: 								.binding(defaults.mining.glacite.fossilSolver,
        //   java: 												() -> config.mining.glacite.fossilSolver,
        //   java: 												newValue -> config.mining.glacite.fossilSolver = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.glacite.solveFossilMuncher"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_6_0)
        //   java: 								.binding(defaults.mining.glacite.solveFossilMuncher,
        //   java: 										() -> config.mining.glacite.solveFossilMuncher,
        //   java: 										newValue -> config.mining.glacite.solveFossilMuncher = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.glacite.enableCorpseFinder"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.glacite.enableCorpseFinder.@Tooltip"))
        //   java: 								.binding(defaults.mining.glacite.enableCorpseFinder,
        //   java: 										() -> config.mining.glacite.enableCorpseFinder,
        //   java: 										newValue -> config.mining.glacite.enableCorpseFinder = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.glacite.enableParsingChatCorpseFinder"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.glacite.enableParsingChatCorpseFinder.@Tooltip"))
        //   java: 								.binding(defaults.mining.glacite.enableParsingChatCorpseFinder,
        //   java: 										() -> config.mining.glacite.enableParsingChatCorpseFinder,
        //   java: 										newValue -> config.mining.glacite.enableParsingChatCorpseFinder = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.glacite.autoShareCorpses"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.glacite.autoShareCorpses.@Tooltip"))
        //   java: 								.binding(defaults.mining.glacite.autoShareCorpses,
        //   java: 										() -> config.mining.glacite.autoShareCorpses,
        //   java: 										newValue -> config.mining.glacite.autoShareCorpses = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.glacite.enableCorpseProfitTracker"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.glacite.enableCorpseProfitTracker.@Tooltip"))
        //   java: 								.binding(defaults.mining.glacite.enableCorpseProfitTracker,
        //   java: 										() -> config.mining.glacite.enableCorpseProfitTracker,
        //   java: 										newValue -> config.mining.glacite.enableCorpseProfitTracker = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.glacite.forceEnglishCorpseProfitTracker"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.glacite.forceEnglishCorpseProfitTracker.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.mining.glacite.forceEnglishCorpseProfitTracker,
        //   java: 										() -> config.mining.glacite.forceEnglishCorpseProfitTracker,
        //   java: 										newValue -> config.mining.glacite.forceEnglishCorpseProfitTracker = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				//Block break prediction
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.mining.blockBreakPrediction"))
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.blockBreakPrediction.enabled"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.mining.blockBreakPrediction.enabled,
        //   java: 										() -> config.mining.blockBreakPrediction.enabled,
        //   java: 										newValue -> config.mining.blockBreakPrediction.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.mining.blockBreakPrediction.playSound"))
        //   java: 								.description(Component.translatable("skyblocker.config.mining.blockBreakPrediction.playSound.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.mining.blockBreakPrediction.playSound,
        //   java: 										() -> config.mining.blockBreakPrediction.playSound,
        //   java: 										newValue -> config.mining.blockBreakPrediction.playSound = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java:
        //   java: 						.build())
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
