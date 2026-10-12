//! Ported from de.hysky.skyblocker.config.categories.DungeonsCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class DungeonsCategory
#[derive(Clone)]
pub struct DungeonsCategory {}

impl DungeonsCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/dungeons"))
        //   java: 				.name(Component.translatable("skyblocker.config.dungeons"))
        //   java:
        //   java: 				//Ungrouped Options
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.fancyPartyFinder"))
        //   java: 						.binding(defaults.dungeons.fancyPartyFinder,
        //   java: 								() -> config.dungeons.fancyPartyFinder,
        //   java: 								newValue -> config.dungeons.fancyPartyFinder = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.croesusHelper"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.croesusHelper.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.croesusHelper,
        //   java: 								() -> config.dungeons.croesusHelper,
        //   java: 								newValue -> config.dungeons.croesusHelper = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.salvageHelper"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.salvageHelper.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.salvageHelper,
        //   java: 								() -> config.dungeons.salvageHelper,
        //   java: 								newValue -> config.dungeons.salvageHelper = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.salvageHelper.onlyDonated"))
        //   java: 						.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 						.binding(defaults.dungeons.onlyHighlightDonatedItems,
        //   java: 								() -> config.dungeons.onlyHighlightDonatedItems,
        //   java: 								newValue -> config.dungeons.onlyHighlightDonatedItems = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.sellableItemsHighlighter"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.sellableItemsHighlighter.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.sellableItemsHighlighter,
        //   java: 								() -> config.dungeons.sellableItemsHighlighter,
        //   java: 								newValue -> config.dungeons.sellableItemsHighlighter = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.playerSecretsTracker"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.playerSecretsTracker.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.playerSecretsTracker,
        //   java: 								() -> config.dungeons.playerSecretsTracker,
        //   java: 								newValue -> config.dungeons.playerSecretsTracker = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.classBasedPlayerGlow"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.classBasedPlayerGlow.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.classBasedPlayerGlow,
        //   java: 								() -> config.dungeons.classBasedPlayerGlow,
        //   java: 								newValue -> config.dungeons.classBasedPlayerGlow = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.starredMobGlow"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.starredMobGlow.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.starredMobGlow,
        //   java: 								() -> config.dungeons.starredMobGlow,
        //   java: 								newValue -> config.dungeons.starredMobGlow = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.starredMobBoundingBoxes"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.starredMobBoundingBoxes.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.starredMobBoundingBoxes,
        //   java: 								() -> config.dungeons.starredMobBoundingBoxes,
        //   java: 								newValue -> config.dungeons.starredMobBoundingBoxes = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.highlightDoorKeys"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.highlightDoorKeys.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.highlightDoorKeys,
        //   java: 								() -> config.dungeons.highlightDoorKeys,
        //   java: 								newValue -> config.dungeons.highlightDoorKeys = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.allowDroppingProtectedItems"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.allowDroppingProtectedItems.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.allowDroppingProtectedItems,
        //   java: 								() -> config.dungeons.allowDroppingProtectedItems,
        //   java: 								newValue -> config.dungeons.allowDroppingProtectedItems = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.hideSoulweaverSkulls"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.hideSoulweaverSkulls.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.hideSoulweaverSkulls,
        //   java: 								() -> config.dungeons.hideSoulweaverSkulls,
        //   java: 								newValue -> config.dungeons.hideSoulweaverSkulls = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(ButtonOption.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.dungeonSplits"))
        //   java: 						.description(Component.translatable("skyblocker.config.hud.movedMessage"), Component.translatable("skyblocker.config.dungeons.dungeonSplits.@Tooltip"))
        //   java: 						.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.DUNGEON, screen)))
        //   java: 						.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.bloodCampHelper"))
        //   java: 						.description(Component.translatable("skyblocker.config.dungeons.bloodCampHelper.@Tooltip"))
        //   java: 						.binding(defaults.dungeons.bloodCampHelper,
        //   java: 								() -> config.dungeons.bloodCampHelper,
        //   java: 								newValue -> config.dungeons.bloodCampHelper = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				// Map
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.map"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.enableMap"))
        //   java: 								.binding(defaults.dungeons.dungeonMap.enableMap,
        //   java: 										() -> config.dungeons.dungeonMap.enableMap,
        //   java: 										newValue -> config.dungeons.dungeonMap.enableMap = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.fancyMap"))
        //   java: 								.binding(defaults.dungeons.dungeonMap.fancyMap,
        //   java: 										() -> config.dungeons.dungeonMap.fancyMap,
        //   java: 										newValue -> config.dungeons.dungeonMap.fancyMap = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.showSelfHead"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.map.showSelfHead.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonMap.showSelfHead,
        //   java: 										() -> config.dungeons.dungeonMap.showSelfHead,
        //   java: 										newValue -> config.dungeons.dungeonMap.showSelfHead = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.showRoomLabels"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.dungeons.dungeonMap.showRoomLabels,
        //   java: 										() -> config.dungeons.dungeonMap.showRoomLabels,
        //   java: 										newValue -> config.dungeons.dungeonMap.showRoomLabels = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<DungeonMapLabels.RoomLabelType>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.roomLabelType"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.map.roomLabelType.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.dungeonMap.roomLabelType,
        //   java: 										() -> config.dungeons.dungeonMap.roomLabelType,
        //   java: 										newValue -> config.dungeons.dungeonMap.roomLabelType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.hideCheckmarks"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.map.hideCheckmarks.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.dungeonMap.hideCheckmarks,
        //   java: 										() -> config.dungeons.dungeonMap.hideCheckmarks,
        //   java: 										newValue -> config.dungeons.dungeonMap.hideCheckmarks = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.showOutline"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.dungeonMap.showOutline,
        //   java: 										() -> config.dungeons.dungeonMap.showOutline,
        //   java: 										newValue -> config.dungeons.dungeonMap.showOutline = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.backgroundBlur"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.dungeonMap.backgroundBlur,
        //   java: 										() -> config.dungeons.dungeonMap.backgroundBlur,
        //   java: 										newValue -> config.dungeons.dungeonMap.backgroundBlur = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.mapScaling"))
        //   java: 								.binding(defaults.dungeons.dungeonMap.mapScaling,
        //   java: 										() -> config.dungeons.dungeonMap.mapScaling,
        //   java: 										newValue -> config.dungeons.dungeonMap.mapScaling = newValue)
        //   java: 								.controller(FloatController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.map.mapScreen"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new DungeonMapConfigScreen(screen)))
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Spirit Leap Overlay
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.enableLeapOverlay"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.enableLeapOverlay.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.leapOverlay.enableLeapOverlay,
        //   java: 										() -> config.dungeons.leapOverlay.enableLeapOverlay,
        //   java: 										newValue -> config.dungeons.leapOverlay.enableLeapOverlay = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.leapKeybinds"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.leapKeybinds.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.leapOverlay.leapKeybinds,
        //   java: 										() -> config.dungeons.leapOverlay.leapKeybinds,
        //   java: 										newValue -> config.dungeons.leapOverlay.leapKeybinds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.showMap"))
        //   java: 								.binding(defaults.dungeons.leapOverlay.showMap,
        //   java: 										() -> config.dungeons.leapOverlay.showMap,
        //   java: 										newValue -> config.dungeons.leapOverlay.showMap = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.scale"))
        //   java: 								.binding(defaults.dungeons.leapOverlay.scale,
        //   java: 										() -> config.dungeons.leapOverlay.scale,
        //   java: 										newValue -> config.dungeons.leapOverlay.scale = newValue)
        //   java: 								.controller(FloatController.createBuilder().range(1f, 2f).slider(0.05f).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.enableLeapMessage"))
        //   java: 								.description(
        //   java: 										Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.enableLeapMessage.@Tooltip[0]"),
        //   java: 										Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.enableLeapMessage.@Tooltip[1]").withColor(TextColor.RED)
        //   java: 								)
        //   java: 								.binding(defaults.dungeons.leapOverlay.enableLeapMessage,
        //   java: 										() -> config.dungeons.leapOverlay.enableLeapMessage,
        //   java: 										newValue -> config.dungeons.leapOverlay.enableLeapMessage = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<String>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.leapMessage"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.spiritLeapOverlay.leapMessage.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.leapOverlay.leapMessage,
        //   java: 										() -> config.dungeons.leapOverlay.leapMessage,
        //   java: 										newValue -> config.dungeons.leapOverlay.leapMessage = newValue)
        //   java: 								.controller(StringController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Puzzle Solver
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.puzzle"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.solveTicTacToe"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.puzzle.solveTicTacToe.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.solveTicTacToe,
        //   java: 										() -> config.dungeons.puzzleSolvers.solveTicTacToe,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.solveTicTacToe = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.solveThreeWeirdos"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.solveThreeWeirdos,
        //   java: 										() -> config.dungeons.puzzleSolvers.solveThreeWeirdos,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.solveThreeWeirdos = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.creeperSolver"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.puzzle.creeperSolver.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.creeperSolver,
        //   java: 										() -> config.dungeons.puzzleSolvers.creeperSolver,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.creeperSolver = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.solveWaterboard"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.puzzle.solveWaterboard.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.waterboardOneFlow,
        //   java: 										() -> config.dungeons.puzzleSolvers.waterboardOneFlow,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.waterboardOneFlow = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.previewWaterPath"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.previewWaterPath,
        //   java: 										() -> config.dungeons.puzzleSolvers.previewWaterPath,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.previewWaterPath = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.previewLeverEffects"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.previewLeverEffects,
        //   java: 										() -> config.dungeons.puzzleSolvers.previewLeverEffects,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.previewLeverEffects = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.blazeSolver"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.puzzle.blazeSolver.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.blazeSolver,
        //   java: 										() -> config.dungeons.puzzleSolvers.blazeSolver,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.blazeSolver = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.solveBoulder"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.puzzle.solveBoulder.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.solveBoulder,
        //   java: 										() -> config.dungeons.puzzleSolvers.solveBoulder,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.solveBoulder = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.solveIceFill"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.solveIceFill,
        //   java: 										() -> config.dungeons.puzzleSolvers.solveIceFill,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.solveIceFill = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.solveSilverfish"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.solveSilverfish,
        //   java: 										() -> config.dungeons.puzzleSolvers.solveSilverfish,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.solveSilverfish = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.solveTrivia"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.solveTrivia,
        //   java: 										() -> config.dungeons.puzzleSolvers.solveTrivia,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.solveTrivia = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.puzzle.solveTeleportMaze"))
        //   java: 								.binding(defaults.dungeons.puzzleSolvers.solveTeleportMaze,
        //   java: 										() -> config.dungeons.puzzleSolvers.solveTeleportMaze,
        //   java: 										newValue -> config.dungeons.puzzleSolvers.solveTeleportMaze = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// The Professor (F3/M3)
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.professor"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.professor.fireFreezeStaffTimer"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.professor.fireFreezeStaffTimer.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.theProfessor.fireFreezeStaffTimer,
        //   java: 										() -> config.dungeons.theProfessor.fireFreezeStaffTimer,
        //   java: 										newValue -> config.dungeons.theProfessor.fireFreezeStaffTimer = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.professor.floor3GuardianHealthDisplay"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.professor.floor3GuardianHealthDisplay.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.theProfessor.floor3GuardianHealthDisplay,
        //   java: 										() -> config.dungeons.theProfessor.floor3GuardianHealthDisplay,
        //   java: 										newValue -> config.dungeons.theProfessor.floor3GuardianHealthDisplay = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Livid (F5/M5)
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.livid"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.livid.enableSolidColor"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.livid.enableSolidColor.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.livid.enableSolidColor,
        //   java: 										() -> config.dungeons.livid.enableSolidColor,
        //   java: 										newValue -> config.dungeons.livid.enableSolidColor = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.livid.customColor"))
        //   java: 								.binding(defaults.dungeons.livid.customColor,
        //   java: 										() -> config.dungeons.livid.customColor,
        //   java: 										newValue -> config.dungeons.livid.customColor = newValue)
        //   java: 								.controller(ConfigUtils.createColourController(false))
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.livid.enableLividColorGlow"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.livid.enableLividColorGlow.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.livid.enableLividColorGlow,
        //   java: 										() -> config.dungeons.livid.enableLividColorGlow,
        //   java: 										newValue -> config.dungeons.livid.enableLividColorGlow = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.livid.enableLividColorBoundingBox"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.livid.enableLividColorBoundingBox.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.livid.enableLividColorBoundingBox,
        //   java: 										() -> config.dungeons.livid.enableLividColorBoundingBox,
        //   java: 										newValue -> config.dungeons.livid.enableLividColorBoundingBox = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.livid.enableLividColorTitle"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.livid.enableLividColorTitle.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.livid.enableLividColorTitle,
        //   java: 										() -> config.dungeons.livid.enableLividColorTitle,
        //   java: 										newValue -> config.dungeons.livid.enableLividColorTitle = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.livid.enableLividColorText"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.livid.enableLividColorText.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.livid.enableLividColorText,
        //   java: 										() -> config.dungeons.livid.enableLividColorText,
        //   java: 										newValue -> config.dungeons.livid.enableLividColorText = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<String>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.livid.lividColorText"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.livid.lividColorText.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.livid.lividColorText,
        //   java: 										() -> config.dungeons.livid.lividColorText,
        //   java: 										newValue -> config.dungeons.livid.lividColorText = newValue)
        //   java: 								.controller(StringController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Terminal (F7/M7)
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.terminals"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminals.solveColor"))
        //   java: 								.binding(defaults.dungeons.terminals.solveColor,
        //   java: 										() -> config.dungeons.terminals.solveColor,
        //   java: 										newValue -> config.dungeons.terminals.solveColor = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminals.solveOrder"))
        //   java: 								.binding(defaults.dungeons.terminals.solveOrder,
        //   java: 										() -> config.dungeons.terminals.solveOrder,
        //   java: 										newValue -> config.dungeons.terminals.solveOrder = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminals.solveStartsWith"))
        //   java: 								.binding(defaults.dungeons.terminals.solveStartsWith,
        //   java: 										() -> config.dungeons.terminals.solveStartsWith,
        //   java: 										newValue -> config.dungeons.terminals.solveStartsWith = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminals.solveSameColor"))
        //   java: 								.binding(defaults.dungeons.terminals.solveSameColor,
        //   java: 										() -> config.dungeons.terminals.solveSameColor,
        //   java: 										newValue -> config.dungeons.terminals.solveSameColor = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminals.blockIncorrectClicks"))
        //   java: 								.binding(defaults.dungeons.terminals.blockIncorrectClicks,
        //   java: 										() -> config.dungeons.terminals.blockIncorrectClicks,
        //   java: 										newValue -> config.dungeons.terminals.blockIncorrectClicks = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Devices (F7/M7)
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.devices"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.devices.solveSimonSays"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.devices.solveSimonSays.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.devices.solveSimonSays,
        //   java: 										() -> config.dungeons.devices.solveSimonSays,
        //   java: 										newValue -> config.dungeons.devices.solveSimonSays = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.devices.solveLightsOn"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.devices.solveLightsOn.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.devices.solveLightsOn,
        //   java: 										() -> config.dungeons.devices.solveLightsOn,
        //   java: 										newValue -> config.dungeons.devices.solveLightsOn = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.devices.solveArrowAlign"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.devices.solveArrowAlign.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.devices.solveArrowAlign,
        //   java: 										() -> config.dungeons.devices.solveArrowAlign,
        //   java: 										newValue -> config.dungeons.devices.solveArrowAlign = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.devices.solveTargetPractice"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.devices.solveTargetPractice.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.devices.solveTargetPractice,
        //   java: 										() -> config.dungeons.devices.solveTargetPractice,
        //   java: 										newValue -> config.dungeons.devices.solveTargetPractice = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Waypoints for goldor phase in f7/m7
        //   java: 				.group(OptionGroup.createBuilder().name(Component.translatable("skyblocker.config.dungeons.goldorWaypoints"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.goldorWaypoints.enableGoldorWaypoints"))
        //   java: 								.binding(defaults.dungeons.goldor.enableGoldorWaypoints,
        //   java: 										() -> config.dungeons.goldor.enableGoldorWaypoints,
        //   java: 										newValue -> config.dungeons.goldor.enableGoldorWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Type>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.goldorWaypoints.waypointType"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.goldor.waypointType,
        //   java: 										() -> config.dungeons.goldor.waypointType,
        //   java: 										newValue -> config.dungeons.goldor.waypointType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// F7/M7 Terminal Hud
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.terminalHud"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminalHud.enabled"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.terminalHud.enableTerminalHud,
        //   java: 										() -> config.dungeons.terminalHud.enableTerminalHud,
        //   java: 										newValue -> config.dungeons.terminalHud.enableTerminalHud = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminalHud.showTerminalStatus"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.terminalHud.showTerminalStatus.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.terminalHud.showTerminalStatus,
        //   java: 										() -> config.dungeons.terminalHud.showTerminalStatus,
        //   java: 										newValue -> config.dungeons.terminalHud.showTerminalStatus = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminalHud.determineInProgressStatus"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.terminalHud.determineInProgressStatus.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.terminalHud.showPlayerAtTerminal,
        //   java: 										() -> config.dungeons.terminalHud.showPlayerAtTerminal,
        //   java: 										newValue -> config.dungeons.terminalHud.showPlayerAtTerminal = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminalHud.showTerminals"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.terminalHud.showTerminals,
        //   java: 										() -> config.dungeons.terminalHud.showTerminals,
        //   java: 										newValue -> config.dungeons.terminalHud.showTerminals = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminalHud.showDevice"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.terminalHud.showDevice,
        //   java: 										() -> config.dungeons.terminalHud.showDevice,
        //   java: 										newValue -> config.dungeons.terminalHud.showDevice = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminalHud.showLevers"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.terminalHud.showLevers,
        //   java: 										() -> config.dungeons.terminalHud.showLevers,
        //   java: 										newValue -> config.dungeons.terminalHud.showLevers = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.terminalHud.showGate"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.terminalHud.showGate,
        //   java: 										() -> config.dungeons.terminalHud.showGate,
        //   java: 										newValue -> config.dungeons.terminalHud.showGate = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Dungeon Secret Waypoints
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableSecretWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableSecretWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableSecretWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableSecretWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Type>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.waypointType"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.waypointType,
        //   java: 										() -> config.dungeons.secretWaypoints.waypointType,
        //   java: 										newValue -> config.dungeons.secretWaypoints.waypointType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.adaptiveBoxSize"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.secretWaypoints.adaptiveBoxSize.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.dungeons.secretWaypoints.adaptiveBoxSize,
        //   java: 										() -> config.dungeons.secretWaypoints.adaptiveBoxSize,
        //   java: 										newValue -> config.dungeons.secretWaypoints.adaptiveBoxSize = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.showSecretText"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.showSecretText,
        //   java: 										() -> config.dungeons.secretWaypoints.showSecretText,
        //   java: 										newValue -> config.dungeons.secretWaypoints.showSecretText = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableEntranceWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableEntranceWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableEntranceWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableEntranceWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableSuperboomWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableSuperboomWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableSuperboomWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableSuperboomWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableChestWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableChestWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableChestWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableChestWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableItemWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableItemWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableItemWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableItemWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableBatWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableBatWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableBatWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableBatWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableWitherWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableWitherWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableWitherWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableWitherWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableRedstoneKeyWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableRedstoneKeyWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableRedstoneKeyWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableRedstoneKeyWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableLeverWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableLeverWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableLeverWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableLeverWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableFairySoulWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableFairySoulWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableFairySoulWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableFairySoulWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableStonkWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableStonkWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableStonkWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableStonkWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableAotvWaypoints"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableAotvWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableAotvWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableAotvWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enablePearlWaypoints"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enablePearlWaypoints.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enablePearlWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enablePearlWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enablePearlWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enablePrinceWaypoints"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enablePrinceWaypoints.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enablePrinceWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enablePrinceWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enablePrinceWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableDefaultWaypoints"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.secretWaypoints.enableDefaultWaypoints.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.secretWaypoints.enableDefaultWaypoints,
        //   java: 										() -> config.dungeons.secretWaypoints.enableDefaultWaypoints,
        //   java: 										newValue -> config.dungeons.secretWaypoints.enableDefaultWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Secret Sync
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.secretSync"))
        //   java: 						.collapsed(true)
        //   java: 						// TODO: Add description when labels work properly on MoulConfig
        //   java: 						// Soon?
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretSync.receiveMatchedRooms"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_10_0)
        //   java: 								.binding(defaults.dungeons.secretSync.receiveRoomMatch,
        //   java: 										() -> config.dungeons.secretSync.receiveRoomMatch,
        //   java: 										newValue -> config.dungeons.secretSync.receiveRoomMatch = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretSync.receiveRoomSecretCount"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_10_0)
        //   java: 								.binding(defaults.dungeons.secretSync.receiveRoomSecretCount,
        //   java: 										() -> config.dungeons.secretSync.receiveRoomSecretCount,
        //   java: 										newValue -> config.dungeons.secretSync.receiveRoomSecretCount = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.secretSync.hideReceivedWaypoints"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_10_0)
        //   java: 								.binding(defaults.dungeons.secretSync.hideReceivedWaypoints,
        //   java: 										() -> config.dungeons.secretSync.hideReceivedWaypoints,
        //   java: 										newValue -> config.dungeons.secretSync.hideReceivedWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Rare Room Alert
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.rareRoomAlert"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_2_0)
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.rareRoomAlert.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.rareRoomAlert.enabled.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_2_0)
        //   java: 								.binding(defaults.dungeons.rareRoomAlert.enabled,
        //   java: 										() -> config.dungeons.rareRoomAlert.enabled,
        //   java: 										newValue -> config.dungeons.rareRoomAlert.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.rareRoomAlert.showForTrinity"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_2_0)
        //   java: 								.binding(defaults.dungeons.rareRoomAlert.showForTrinity,
        //   java: 										() -> config.dungeons.rareRoomAlert.showForTrinity,
        //   java: 										newValue -> config.dungeons.rareRoomAlert.showForTrinity = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.rareRoomAlert.showForTomioka"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_2_0)
        //   java: 								.binding(defaults.dungeons.rareRoomAlert.showForTomioka,
        //   java: 										() -> config.dungeons.rareRoomAlert.showForTomioka,
        //   java: 										newValue -> config.dungeons.rareRoomAlert.showForTomioka = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.rareRoomAlert.showForDuncan"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_2_0)
        //   java: 								.binding(defaults.dungeons.rareRoomAlert.showForDuncan,
        //   java: 										() -> config.dungeons.rareRoomAlert.showForDuncan,
        //   java: 										newValue -> config.dungeons.rareRoomAlert.showForDuncan = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Dungeon Messages
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.dungeonMessages"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.mimicMessage.sendMimicMessage"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.mimicMessage.sendMimicMessage.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.mimicMessage.sendMimicMessage,
        //   java: 										() -> config.dungeons.mimicMessage.sendMimicMessage,
        //   java: 										newValue -> config.dungeons.mimicMessage.sendMimicMessage = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.princeMessage.sendPrinceMessage"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.princeMessage.sendPrinceMessage.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.princeMessage.sendPrinceMessage,
        //   java: 										() -> config.dungeons.princeMessage.sendPrinceMessage,
        //   java: 										newValue -> config.dungeons.princeMessage.sendPrinceMessage = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.batMessage.sendBatMessage"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.batMessage.sendBatMessage.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.dungeons.batMessage.sendBatMessage,
        //   java: 										() -> config.dungeons.batMessage.sendBatMessage,
        //   java: 										newValue -> config.dungeons.batMessage.sendBatMessage = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Door Highlight
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.doorHighlight"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.doorHighlight.enableDoorHighlight"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.doorHighlight.enableDoorHighlight.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.doorHighlight.enableDoorHighlight,
        //   java: 										() -> config.dungeons.doorHighlight.enableDoorHighlight,
        //   java: 										newValue -> config.dungeons.doorHighlight.enableDoorHighlight = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<DungeonsConfig.DoorHighlight.Type>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.doorHighlight.doorHighlightType"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.doorHighlight.doorHighlightType.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.doorHighlight.doorHighlightType,
        //   java: 										() -> config.dungeons.doorHighlight.doorHighlightType,
        //   java: 										newValue -> config.dungeons.doorHighlight.doorHighlightType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Dungeon Score
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.dungeonScore"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreMessage", 270))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreMessage.@Tooltip", 270))
        //   java: 								.binding(defaults.dungeons.dungeonScore.enableDungeonScore270Message,
        //   java: 										() -> config.dungeons.dungeonScore.enableDungeonScore270Message,
        //   java: 										newValue -> config.dungeons.dungeonScore.enableDungeonScore270Message = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreTitle", 270))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreTitle.@Tooltip", 270))
        //   java: 								.binding(defaults.dungeons.dungeonScore.enableDungeonScore270Title,
        //   java: 										() -> config.dungeons.dungeonScore.enableDungeonScore270Title,
        //   java: 										newValue -> config.dungeons.dungeonScore.enableDungeonScore270Title = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreSound", 270))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreSound.@Tooltip", 270))
        //   java: 								.binding(defaults.dungeons.dungeonScore.enableDungeonScore270Sound,
        //   java: 										() -> config.dungeons.dungeonScore.enableDungeonScore270Sound,
        //   java: 										newValue -> config.dungeons.dungeonScore.enableDungeonScore270Sound = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<String>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.dungeonScoreMessage", 270))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.dungeonScoreMessage.@Tooltip", 270, 270))
        //   java: 								.binding(defaults.dungeons.dungeonScore.dungeonScore270Message,
        //   java: 										() -> config.dungeons.dungeonScore.dungeonScore270Message,
        //   java: 										newValue -> config.dungeons.dungeonScore.dungeonScore270Message = newValue)
        //   java: 								.controller(StringController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreMessage", 300))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreMessage.@Tooltip", 300))
        //   java: 								.binding(defaults.dungeons.dungeonScore.enableDungeonScore300Message,
        //   java: 										() -> config.dungeons.dungeonScore.enableDungeonScore300Message,
        //   java: 										newValue -> config.dungeons.dungeonScore.enableDungeonScore300Message = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreTitle", 300))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreTitle.@Tooltip", 300))
        //   java: 								.binding(defaults.dungeons.dungeonScore.enableDungeonScore300Title,
        //   java: 										() -> config.dungeons.dungeonScore.enableDungeonScore300Title,
        //   java: 										newValue -> config.dungeons.dungeonScore.enableDungeonScore300Title = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreSound", 300))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonScoreSound.@Tooltip", 300))
        //   java: 								.binding(defaults.dungeons.dungeonScore.enableDungeonScore300Sound,
        //   java: 										() -> config.dungeons.dungeonScore.enableDungeonScore300Sound,
        //   java: 										newValue -> config.dungeons.dungeonScore.enableDungeonScore300Sound = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<String>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.dungeonScoreMessage", 300))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.dungeonScoreMessage.@Tooltip", 300, 300))
        //   java: 								.binding(defaults.dungeons.dungeonScore.dungeonScore300Message,
        //   java: 										() -> config.dungeons.dungeonScore.dungeonScore300Message,
        //   java: 										newValue -> config.dungeons.dungeonScore.dungeonScore300Message = newValue)
        //   java: 								.controller(StringController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonCryptsMessage"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableDungeonCryptsMessage.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonScore.enableDungeonCryptsMessage,
        //   java: 										() -> config.dungeons.dungeonScore.enableDungeonCryptsMessage,
        //   java: 										newValue -> config.dungeons.dungeonScore.enableDungeonCryptsMessage = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.dungeonCryptsMessageThreshold"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.dungeonCryptsMessageThreshold.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonScore.dungeonCryptsMessageThreshold,
        //   java: 										() -> config.dungeons.dungeonScore.dungeonCryptsMessageThreshold,
        //   java: 										newValue -> config.dungeons.dungeonScore.dungeonCryptsMessageThreshold = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(0, 330).build())
        //   java: 								.build())
        //   java: 						.option(Option.<String>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.dungeonCryptsMessage"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.dungeonCryptsMessage.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonScore.dungeonCryptsMessage,
        //   java: 										() -> config.dungeons.dungeonScore.dungeonCryptsMessage,
        //   java: 										newValue -> config.dungeons.dungeonScore.dungeonCryptsMessage = newValue)
        //   java: 								.controller(StringController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableScoreHUD"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonScore.enableScoreHUD.@Tooltip"),
        //   java: 										Component.translatable("skyblocker.config.dungeons.dungeonScore.enableScoreHUD.deathMessagesNote"))
        //   java: 								.binding(defaults.dungeons.dungeonScore.enableScoreHUD,
        //   java: 										() -> config.dungeons.dungeonScore.enableScoreHUD,
        //   java: 										newValue -> config.dungeons.dungeonScore.enableScoreHUD = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonScore.scoreScaling"))
        //   java: 								.binding(defaults.dungeons.dungeonScore.scoreScaling,
        //   java: 										() -> config.dungeons.dungeonScore.scoreScaling,
        //   java: 										newValue -> {
        //   java: 											config.dungeons.dungeonScore.scoreX = config.dungeons.dungeonScore.scoreX + (int) ((config.dungeons.dungeonScore.scoreScaling - newValue) * 38.0);
        //   java: 											config.dungeons.dungeonScore.scoreY = config.dungeons.dungeonScore.scoreY + (int) ((config.dungeons.dungeonScore.scoreScaling - newValue) * Minecraft.getInstance().font.lineHeight / 2.0);
        //   java: 											config.dungeons.dungeonScore.scoreScaling = newValue;
        //   java: 										})
        //   java: 								.controller(FloatController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Dungeon Chest Profit Calculator
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.enableProfitCalculator"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.enableProfitCalculator.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.enableProfitCalculator,
        //   java: 										() -> config.dungeons.dungeonChestProfit.enableProfitCalculator,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.enableProfitCalculator = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.croesusProfit"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.croesusProfit.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.croesusProfit,
        //   java: 										() -> config.dungeons.dungeonChestProfit.croesusProfit,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.croesusProfit = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.includeKismet"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.includeKismet.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.includeKismet,
        //   java: 										() -> config.dungeons.dungeonChestProfit.includeKismet,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.includeKismet = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.includeEssence"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.includeEssence.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.includeEssence,
        //   java: 										() -> config.dungeons.dungeonChestProfit.includeEssence,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.includeEssence = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						//FIXME maybe use color controller
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.neutralThreshold"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.neutralThreshold.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.neutralThreshold,
        //   java: 										() -> config.dungeons.dungeonChestProfit.neutralThreshold,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.neutralThreshold = newValue)
        //   java: 								.controller(IntegerController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFormatting>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.neutralColor"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.neutralColor,
        //   java: 										() -> config.dungeons.dungeonChestProfit.neutralColor,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.neutralColor = newValue)
        //   java: 								.controller(ConfigUtils.createEnumDropdownController(ConfigUtils.FORMATTING_FORMATTER))
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFormatting>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.profitColor"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.profitColor,
        //   java: 										() -> config.dungeons.dungeonChestProfit.profitColor,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.profitColor = newValue)
        //   java: 								.controller(ConfigUtils.createEnumDropdownController(ConfigUtils.FORMATTING_FORMATTER))
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFormatting>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.lossColor"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.lossColor,
        //   java: 										() -> config.dungeons.dungeonChestProfit.lossColor,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.lossColor = newValue)
        //   java: 								.controller(ConfigUtils.createEnumDropdownController(ConfigUtils.FORMATTING_FORMATTER))
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFormatting>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.incompleteColor"))
        //   java: 								.description(Component.translatable("skyblocker.config.dungeons.dungeonChestProfit.incompleteColor.@Tooltip"))
        //   java: 								.binding(defaults.dungeons.dungeonChestProfit.incompleteColor,
        //   java: 										() -> config.dungeons.dungeonChestProfit.incompleteColor,
        //   java: 										newValue -> config.dungeons.dungeonChestProfit.incompleteColor = newValue)
        //   java: 								.controller(ConfigUtils.createEnumDropdownController(ConfigUtils.FORMATTING_FORMATTER))
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
