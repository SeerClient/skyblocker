//! Ported from de.hysky.skyblocker.config.categories.FarmingCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class FarmingCategory
#[derive(Clone)]
pub struct FarmingCategory {}

impl FarmingCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/farming"))
        //   java: 				.name(Component.translatable("skyblocker.config.farming"))
        //   java:
        //   java: 				// Farming HUD
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.farming.farmingHud"))
        //   java: 						.collapsed(true)
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.farmingHud.config"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.GARDEN, screen)))
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				// Pest Highlighter
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.farming.pestHighlighter"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.pestHighlighter.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.pestHighlighter.enabled.@Tooltip"))
        //   java: 								.binding(defaults.farming.pestHighlighter.enabled,
        //   java: 										() -> config.farming.pestHighlighter.enabled,
        //   java: 										newValue -> config.farming.pestHighlighter.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.pestHighlighter.contestHighlighter"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.pestHighlighter.contestHighlighter.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_5_0)
        //   java: 								.binding(defaults.farming.pestHighlighter.contestHighlighter,
        //   java: 										() -> config.farming.pestHighlighter.contestHighlighter,
        //   java: 										newValue -> config.farming.pestHighlighter.contestHighlighter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.pestHighlighter.vinylHighlighter"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.pestHighlighter.vinylHighlighter.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_2_0)
        //   java: 								.binding(defaults.farming.pestHighlighter.vinylHighlighter,
        //   java: 										() -> config.farming.pestHighlighter.vinylHighlighter,
        //   java: 										newValue -> config.farming.pestHighlighter.vinylHighlighter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.pestHighlighter.stereoHarmony"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.pestHighlighter.stereoHarmony.@Tooltip"))
        //   java: 								.binding(defaults.farming.pestHighlighter.enableStereoHarmonyHelperForContest,
        //   java: 										() -> config.farming.pestHighlighter.enableStereoHarmonyHelperForContest,
        //   java: 										newValue -> config.farming.pestHighlighter.enableStereoHarmonyHelperForContest = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				// Mouse Lock
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.farming.mouseLock"))
        //   java: 						.collapsed(true)
        //   java: 						.tags(Component.literal("camera lock"), Component.literal("sensitivity"), Component.literal("yaw pitch"))
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.mouseLock.lockMouseTool"))
        //   java: 								.binding(defaults.farming.mouseLock.lockMouseTool,
        //   java: 										() -> config.farming.mouseLock.lockMouseTool,
        //   java: 										newValue -> config.farming.mouseLock.lockMouseTool = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.mouseLock.lockMouseGround"))
        //   java: 								.binding(defaults.farming.mouseLock.lockMouseGroundOnly,
        //   java: 										() -> config.farming.mouseLock.lockMouseGroundOnly,
        //   java: 										newValue -> config.farming.mouseLock.lockMouseGroundOnly = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				// Garden Plots Widget
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.farming.plotsWidget"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.plotsWidget.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.plotsWidget.enabled.@Tooltip"))
        //   java: 								.binding(defaults.farming.plotsWidget.enabled,
        //   java: 										() -> config.farming.plotsWidget.enabled,
        //   java: 										newValue -> config.farming.plotsWidget.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.plotsWidget.closeScreenOnPlotClick"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.plotsWidget.closeScreenOnPlotClick.@Tooltip"))
        //   java: 								.binding(defaults.farming.plotsWidget.closeScreenOnPlotClick,
        //   java: 										() -> config.farming.plotsWidget.closeScreenOnPlotClick,
        //   java: 										newValue -> config.farming.plotsWidget.closeScreenOnPlotClick = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				// Visitor Helper
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.farming.visitorHelper"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.visitorHelper.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.visitorHelper.enabled.@Tooltip"))
        //   java: 								.binding(defaults.farming.visitorHelper.enabled,
        //   java: 										() -> config.farming.visitorHelper.enabled,
        //   java: 										newValue -> config.farming.visitorHelper.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.visitorHelper.showInGardenOnly"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.visitorHelper.showInGardenOnly.@Tooltip"))
        //   java: 								.binding(defaults.farming.visitorHelper.showInGardenOnly,
        //   java: 										() -> config.farming.visitorHelper.showInGardenOnly,
        //   java: 										newValue -> config.farming.visitorHelper.showInGardenOnly = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.visitorHelper.showInStacks"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.visitorHelper.showInStacks.@Tooltip"))
        //   java: 								.binding(defaults.farming.visitorHelper.showInStacks,
        //   java: 										() -> config.farming.visitorHelper.showInStacks,
        //   java: 										newValue -> config.farming.visitorHelper.showInStacks = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Greenhouse
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.farming.greenhouse"))
        //   java: 						.collapsed(true)
        //   java: 						.tags(CommonTags.ADDED_IN_6_5_0)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.greenhouse.greenhousePaste.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.greenhouse.greenhousePaste.enabled.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_5_0)
        //   java: 								.binding(defaults.farming.greenhouse.enabled,
        //   java: 										() -> config.farming.greenhouse.enabled,
        //   java: 										newValue -> config.farming.greenhouse.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.greenhouse.greenhousePresets"))
        //   java: 								.prompt(Component.translatable("skyblocker.config.farming.greenhouse.greenhousePresets.@Prompt"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.greenhouse.greenhousePresets.@Tooltip"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new GreenhousePresetsScreen(screen)))
        //   java: 								.build()
        //   java: 						)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.farming.greenhouse.showMutationSlot.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.farming.greenhouse.showMutationSlot.enabled.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_5_0)
        //   java: 								.binding(defaults.farming.greenhouse.showMutationSlot,
        //   java: 										() -> config.farming.greenhouse.showMutationSlot,
        //   java: 										newValue -> config.farming.greenhouse.showMutationSlot = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
