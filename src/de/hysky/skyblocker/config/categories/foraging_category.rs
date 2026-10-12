//! Ported from de.hysky.skyblocker.config.categories.ForagingCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class ForagingCategory
#[derive(Clone)]
pub struct ForagingCategory {}

impl ForagingCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/foraging"))
        //   java: 				.name(Component.translatable("skyblocker.config.foraging"))
        //   java:
        //   java: 				// Ungrouped Options
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.foraging.enableTreeFelledNotification"))
        //   java: 						.description(Component.translatable("skyblocker.config.foraging.enableTreeFelledNotification.@Tooltip"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_9_1)
        //   java: 						.binding(defaults.foraging.enableTreeFelledNotification,
        //   java: 								() -> config.foraging.enableTreeFelledNotification,
        //   java: 								newValue -> config.foraging.enableTreeFelledNotification = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				// Moonglade Marsh
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.foraging.moongladeMarsh"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.moongladeMarsh.solveForestTemplePuzzle"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.moongladeMarsh.solveForestTemplePuzzle.@Tooltip"))
        //   java: 								.binding(defaults.foraging.moongladeMarsh.solveForestTemplePuzzle,
        //   java: 										() -> config.foraging.moongladeMarsh.solveForestTemplePuzzle,
        //   java: 										newValue -> config.foraging.moongladeMarsh.solveForestTemplePuzzle = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableLushlilacHighlighter"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableLushlilacHighlighter.@Tooltip"))
        //   java: 								.binding(defaults.foraging.moongladeMarsh.enableLushlilacHighlighter,
        //   java: 										() -> config.foraging.moongladeMarsh.enableLushlilacHighlighter,
        //   java: 										newValue -> config.foraging.moongladeMarsh.enableLushlilacHighlighter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableSeaLumiesHighlighter"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableSeaLumiesHighlighter.@Tooltip"))
        //   java: 								.binding(defaults.foraging.moongladeMarsh.enableSeaLumiesHighlighter,
        //   java: 										() -> config.foraging.moongladeMarsh.enableSeaLumiesHighlighter,
        //   java: 										newValue -> {
        //   java: 											config.foraging.moongladeMarsh.enableSeaLumiesHighlighter = newValue;
        //   java: 											SeaLumiesHighlighter.INSTANCE.configCallback();
        //   java: 										})
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.moongladeMarsh.seaLumieMinCount"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.moongladeMarsh.seaLumieMinCount.@Tooltip"))
        //   java: 								.binding(defaults.foraging.moongladeMarsh.seaLumiesMinimumCount,
        //   java: 										() -> config.foraging.moongladeMarsh.seaLumiesMinimumCount,
        //   java: 										newValue -> {
        //   java: 											config.foraging.moongladeMarsh.seaLumiesMinimumCount = newValue;
        //   java: 											SeaLumiesHighlighter.INSTANCE.configCallback();
        //   java: 										})
        //   java: 								.controller(IntegerController.createBuilder().range(1, 4).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableTreeBreakHud"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.GALATEA, screen)))
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableTunerSolver"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableTunerSolver.@Tooltip"))
        //   java: 								.binding(defaults.foraging.moongladeMarsh.enableTunerSolver,
        //   java: 										() -> config.foraging.moongladeMarsh.enableTunerSolver,
        //   java: 										newValue -> config.foraging.moongladeMarsh.enableTunerSolver = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableSweepDetailsWidget"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"), Component.translatable("skyblocker.config.foraging.moongladeMarsh.enableSweepDetailsWidget.@Tooltip"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.GALATEA, screen)))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Torrhus Canyon
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.foraging.torrhusCanyon"))
        //   java: 						.collapsed(false)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.torrhusCanyon.solveDesertTemplePuzzles"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.torrhusCanyon.solveDesertTemplePuzzles.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.foraging.torrhusCanyon.solveDesertTemplePuzzles,
        //   java: 										() -> config.foraging.torrhusCanyon.solveDesertTemplePuzzles,
        //   java: 										newValue -> config.foraging.torrhusCanyon.solveDesertTemplePuzzles = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.torrhusCanyon.enableRubyVeilshroomHighlighter"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.torrhusCanyon.enableRubyVeilshroomHighlighter.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.foraging.torrhusCanyon.enableRubyVeilshroomHighlighter,
        //   java: 										() -> config.foraging.torrhusCanyon.enableRubyVeilshroomHighlighter,
        //   java: 										newValue -> config.foraging.torrhusCanyon.enableRubyVeilshroomHighlighter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.torrhusCanyon.enableHoneyhiveHighlighter"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.torrhusCanyon.enableHoneyhiveHighlighter.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_9_0)
        //   java: 								.binding(defaults.foraging.torrhusCanyon.enableHoneyhiveHighlighter,
        //   java: 										() -> config.foraging.torrhusCanyon.enableHoneyhiveHighlighter,
        //   java: 										newValue -> config.foraging.torrhusCanyon.enableHoneyhiveHighlighter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Sweep Overlay
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.foraging.sweepOverlay"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.sweepOverlay.enableSweepOverlay"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.sweepOverlay.enableSweepOverlay.@Tooltip"))
        //   java: 								.binding(defaults.foraging.sweepOverlay.enableSweepOverlay,
        //   java: 										() -> config.foraging.sweepOverlay.enableSweepOverlay,
        //   java: 										newValue -> config.foraging.sweepOverlay.enableSweepOverlay = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.sweepOverlay.enableThrownAbilityOverlay"))
        //   java: 								.description(Component.translatable("skyblocker.config.foraging.sweepOverlay.enableThrownAbilityOverlay.@Tooltip"))
        //   java: 								.binding(defaults.foraging.sweepOverlay.enableThrownAbilityOverlay,
        //   java: 										() -> config.foraging.sweepOverlay.enableThrownAbilityOverlay,
        //   java: 										newValue -> config.foraging.sweepOverlay.enableThrownAbilityOverlay = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.foraging.sweepOverlay.sweepOverlayColor"))
        //   java: 								.binding(defaults.foraging.sweepOverlay.sweepOverlayColor,
        //   java: 										() -> config.foraging.sweepOverlay.sweepOverlayColor,
        //   java: 										newValue -> {
        //   java: 											config.foraging.sweepOverlay.sweepOverlayColor = newValue;
        //   java: 											SweepOverlay.configCallback(newValue);
        //   java: 										})
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(true).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
