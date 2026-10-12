//! Ported from de.hysky.skyblocker.config.categories.HelperCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class HelperCategory
#[derive(Clone)]
pub struct HelperCategory {}

impl HelperCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/helpers"))
        //   java: 				.name(Component.translatable("skyblocker.config.helpers"))
        //   java:
        //   java: 				//Ungrouped Options
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.enableNewYearCakesHelper"))
        //   java: 						.description(Component.translatable("skyblocker.config.helpers.enableNewYearCakesHelper.@Tooltip"))
        //   java: 						.binding(defaults.helpers.enableNewYearCakesHelper,
        //   java: 								() -> config.helpers.enableNewYearCakesHelper,
        //   java: 								newValue -> config.helpers.enableNewYearCakesHelper = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				// Bits Helper
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.enableBitsHelper"))
        //   java: 						.description(Component.translatable("skyblocker.config.helpers.enableBitsHelper.@Tooltip"))
        //   java: 						.binding(defaults.helpers.enableBitsTooltip,
        //   java: 								() -> config.helpers.enableBitsTooltip,
        //   java: 								newValue -> config.helpers.enableBitsTooltip = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				// Date Calculator
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.enableDateCalculator"))
        //   java: 						.description(Component.translatable("skyblocker.config.helpers.enableDateCalculator.@Tooltip"))
        //   java: 						.binding(defaults.helpers.enableDateCalculator,
        //   java: 								() -> config.helpers.enableDateCalculator,
        //   java: 								newValue -> config.helpers.enableDateCalculator = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				// Copy Underbid Price
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.enableCopyUnderbidPrice"))
        //   java: 						.description(Component.translatable("skyblocker.config.helpers.enableCopyUnderbidPrice.@Tooltip"))
        //   java: 						.binding(defaults.helpers.enableCopyUnderbidPrice,
        //   java: 								() -> config.helpers.enableCopyUnderbidPrice,
        //   java: 								newValue -> config.helpers.enableCopyUnderbidPrice = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				// Anvil Helper
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.enableAnvilHelper"))
        //   java: 						.description(Component.translatable("skyblocker.config.helpers.enableAnvilHelper.@Tooltip"))
        //   java: 						.tags(CommonTags.ADDED_IN_5_10_0)
        //   java: 						.binding(defaults.helpers.enableAnvilHelper,
        //   java: 								() -> config.helpers.enableAnvilHelper,
        //   java: 								newValue -> config.helpers.enableAnvilHelper = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				// Accessories Helper Widget
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.enableAccessoriesHelperWidget"))
        //   java: 						.description(Component.translatable("skyblocker.config.helpers.enableAccessoriesHelperWidget.@Tooltip"))
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.binding(defaults.helpers.enableAccessoriesHelperWidget,
        //   java: 								() -> config.helpers.enableAccessoriesHelperWidget,
        //   java: 								newValue -> config.helpers.enableAccessoriesHelperWidget = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				// Builder's Wand and Ruler Preview
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.buildersWand"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.buildersWand.enableBuildersWandPreview"))
        //   java: 								.binding(defaults.helpers.buildersWand.enableBuildersWandPreview,
        //   java: 										() -> config.helpers.buildersWand.enableBuildersWandPreview,
        //   java: 										newValue -> config.helpers.buildersWand.enableBuildersWandPreview = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.buildersWand.previewOpacity"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_2_0)
        //   java: 								.binding(defaults.helpers.buildersWand.previewOpacity,
        //   java: 										() -> config.helpers.buildersWand.previewOpacity,
        //   java: 										newValue -> config.helpers.buildersWand.previewOpacity = newValue)
        //   java: 								.controller(FloatController.createBuilder().range(0f, 1f).slider(0.05f).build())
        //   java: 								.build())
        //   java: 						.build()
        //   java: 				)
        //   java:
        //   java: 				//Mythological Ritual
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.mythologicalRitual"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.mythologicalRitual.enableMythologicalRitualHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.mythologicalRitual.enableMythologicalRitualHelper.@Tooltip"))
        //   java: 								.binding(defaults.helpers.mythologicalRitual.enableMythologicalRitualHelper,
        //   java: 										() -> config.helpers.mythologicalRitual.enableMythologicalRitualHelper,
        //   java: 										newValue -> config.helpers.mythologicalRitual.enableMythologicalRitualHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Jerry Timer
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.jerry"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.jerry.enableJerryTimer"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.jerry.enableJerryTimer.@Tooltip"))
        //   java: 								.binding(defaults.helpers.jerry.enableJerryTimer,
        //   java: 										() -> config.helpers.jerry.enableJerryTimer,
        //   java: 										newValue -> config.helpers.jerry.enableJerryTimer = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Experiments Solver
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.experiments"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.experiments.enableChronomatronSolver"))
        //   java: 								.binding(defaults.helpers.experiments.enableChronomatronSolver,
        //   java: 										() -> config.helpers.experiments.enableChronomatronSolver,
        //   java: 										newValue -> config.helpers.experiments.enableChronomatronSolver = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.experiments.enableSuperpairsSolver"))
        //   java: 								.binding(defaults.helpers.experiments.enableSuperpairsSolver,
        //   java: 										() -> config.helpers.experiments.enableSuperpairsSolver,
        //   java: 										newValue -> config.helpers.experiments.enableSuperpairsSolver = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.experiments.enableUltrasequencerSolver"))
        //   java: 								.binding(defaults.helpers.experiments.enableUltrasequencerSolver,
        //   java: 										() -> config.helpers.experiments.enableUltrasequencerSolver,
        //   java: 										newValue -> config.helpers.experiments.enableUltrasequencerSolver = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.experiments.blockIncorrectClicks"))
        //   java: 								.binding(defaults.helpers.experiments.blockIncorrectClicks,
        //   java: 										() -> config.helpers.experiments.blockIncorrectClicks,
        //   java: 										newValue -> config.helpers.experiments.blockIncorrectClicks = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Fishing Helper
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.fishing"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fishing.enableFishingHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fishing.enableFishingHelper.@Tooltip"))
        //   java: 								.binding(defaults.helpers.fishing.enableFishingHelper,
        //   java: 										() -> config.helpers.fishing.enableFishingHelper,
        //   java: 										newValue -> config.helpers.fishing.enableFishingHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fishing.hideOtherPlayers"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fishing.hideOtherPlayers.@Tooltip"))
        //   java: 								.binding(defaults.helpers.fishing.hideOtherPlayersRods,
        //   java: 										() -> config.helpers.fishing.hideOtherPlayersRods,
        //   java: 										newValue -> config.helpers.fishing.hideOtherPlayersRods = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fishing.hud.screen"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.HUB, screen)))
        //   java: 								.build())
        //   java: 						.option(Option.<HelperConfig.Fishing.FishingHookDisplay>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fishing.fishingHookDisplay"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fishing.fishingHookDisplay.@Tooltip"))
        //   java: 								.binding(defaults.helpers.fishing.fishingHookDisplay,
        //   java: 										() -> config.helpers.fishing.fishingHookDisplay,
        //   java: 										newValue -> config.helpers.fishing.fishingHookDisplay = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fishing.timerLength"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fishing.timerLength.@Tooltip"))
        //   java: 								.binding(defaults.helpers.fishing.timerLength,
        //   java: 										() -> config.helpers.fishing.timerLength,
        //   java: 										newValue -> config.helpers.fishing.timerLength = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(1, 360).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fishing.seaCreatureTimerNotification"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fishing.seaCreatureTimerNotification.@Tooltip"))
        //   java: 								.binding(defaults.helpers.fishing.seaCreatureTimerNotification,
        //   java: 										() -> config.helpers.fishing.seaCreatureTimerNotification,
        //   java: 										newValue -> config.helpers.fishing.seaCreatureTimerNotification = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fishing.seaCreatureCapNotification"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fishing.seaCreatureCapNotification.@Tooltip"))
        //   java: 								.binding(defaults.helpers.fishing.seaCreatureCapNotification,
        //   java: 										() -> config.helpers.fishing.seaCreatureCapNotification,
        //   java: 										newValue -> config.helpers.fishing.seaCreatureCapNotification = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<SkyblockItemRarity>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fishing.minimumNotificationRarity"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fishing.minimumNotificationRarity.@Tooltip"))
        //   java: 								.binding(defaults.helpers.fishing.minimumNotificationRarity,
        //   java: 										() -> config.helpers.fishing.minimumNotificationRarity,
        //   java: 										rarity -> config.helpers.fishing.minimumNotificationRarity = rarity == SkyblockItemRarity.DIVINE ? SkyblockItemRarity.UNKNOWN : rarity)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java:
        //   java: 						.build())
        //   java:
        //   java: 				// Loadouts Helper
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.loadouts"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.loadouts.enableLoadoutsHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.loadouts.enableLoadoutsHelper.@Tooltip"))
        //   java: 								.tags(Component.translatable("skyblocker.config.helpers.loadouts.enableLoadoutsHelper.@Tag"), CommonTags.ADDED_IN_6_7_0)
        //   java: 								.binding(defaults.helpers.enableWardrobeHelper,
        //   java: 										() -> config.helpers.enableWardrobeHelper,
        //   java: 										newValue -> config.helpers.enableWardrobeHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.01"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_1)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.02"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_2)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.03"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_3)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.04"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_4)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.05"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_5)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.06"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_6)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.07"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_7)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.08"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_8)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.09"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_9)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.10"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_10)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.11"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_11)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.loadout.12"))
        //   java: 								.tags(ArrayUtils.add(CommonTags.KEY_MAPPING, CommonTags.ADDED_IN_6_7_0))
        //   java: 								.keyMapping(LoadoutKeybinds.LOADOUT_12)
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Fairy Souls Helper
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.fairySouls"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fairySouls.enableFairySoulsHelper"))
        //   java: 								.binding(defaults.helpers.fairySouls.enableFairySoulsHelper,
        //   java: 										() -> config.helpers.fairySouls.enableFairySoulsHelper,
        //   java: 										newValue -> config.helpers.fairySouls.enableFairySoulsHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fairySouls.highlightFoundSouls"))
        //   java: 								.binding(defaults.helpers.fairySouls.highlightFoundSouls,
        //   java: 										() -> config.helpers.fairySouls.highlightFoundSouls,
        //   java: 										newValue -> config.helpers.fairySouls.highlightFoundSouls = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fairySouls.highlightOnBingo"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fairySouls.highlightOnBingo.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_10_0)
        //   java: 								.binding(defaults.helpers.fairySouls.highlightOnBingo,
        //   java: 										() -> config.helpers.fairySouls.highlightOnBingo,
        //   java: 										newValue -> config.helpers.fairySouls.highlightOnBingo = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.fairySouls.highlightOnlyNearbySouls"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.fairySouls.highlightOnlyNearbySouls.@Tooltip"))
        //   java: 								.binding(defaults.helpers.fairySouls.highlightOnlyNearbySouls,
        //   java: 										() -> config.helpers.fairySouls.highlightOnlyNearbySouls,
        //   java: 										newValue -> config.helpers.fairySouls.highlightOnlyNearbySouls = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Chocolate Factory
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.chocolateFactory"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.chocolateFactory.enableChocolateFactoryHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.chocolateFactory.enableChocolateFactoryHelper.@Tooltip"))
        //   java: 								.binding(defaults.helpers.chocolateFactory.enableChocolateFactoryHelper,
        //   java: 										() -> config.helpers.chocolateFactory.enableChocolateFactoryHelper,
        //   java: 										newValue -> config.helpers.chocolateFactory.enableChocolateFactoryHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.chocolateFactory.enableTimeTowerReminder"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.chocolateFactory.enableTimeTowerReminder.@Tooltip"))
        //   java: 								.binding(defaults.helpers.chocolateFactory.enableTimeTowerReminder,
        //   java: 										() -> config.helpers.chocolateFactory.enableTimeTowerReminder,
        //   java: 										newValue -> config.helpers.chocolateFactory.enableTimeTowerReminder = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.chocolateFactory.straySound"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.chocolateFactory.straySound.@Tooltip"))
        //   java: 								.binding(defaults.helpers.chocolateFactory.straySound,
        //   java: 										() -> config.helpers.chocolateFactory.straySound,
        //   java: 										newValue -> config.helpers.chocolateFactory.straySound = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Hoppity's Hunt
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.hoppitysHunt"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.hoppitysHunt.enableEggFinder"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.hoppitysHunt.enableEggFinder.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.helpers.chocolateFactory.enableEggFinder,
        //   java: 										() -> config.helpers.chocolateFactory.enableEggFinder,
        //   java: 										newValue -> config.helpers.chocolateFactory.enableEggFinder = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.hoppitysHunt.sendEggFoundMessages"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.hoppitysHunt.sendEggFoundMessages.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.helpers.chocolateFactory.sendEggFoundMessages,
        //   java: 										() -> config.helpers.chocolateFactory.sendEggFoundMessages,
        //   java: 										newValue -> config.helpers.chocolateFactory.sendEggFoundMessages = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Waypoint.Type>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.hoppitysHunt.waypointType"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.helpers.chocolateFactory.waypointType,
        //   java: 										() -> config.helpers.chocolateFactory.waypointType,
        //   java: 										newValue -> config.helpers.chocolateFactory.waypointType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.hoppitysHunt.showThroughWalls"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.hoppitysHunt.showThroughWalls.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.helpers.chocolateFactory.showThroughWalls,
        //   java: 										() -> config.helpers.chocolateFactory.showThroughWalls,
        //   java: 										newValue -> config.helpers.chocolateFactory.showThroughWalls = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.carnival"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.carnival.catchAFishHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.carnival.catchAFishHelper.@Tooltip"))
        //   java: 								.binding(defaults.helpers.carnival.catchAFishHelper,
        //   java: 										() -> config.helpers.carnival.catchAFishHelper,
        //   java: 										newValue -> config.helpers.carnival.catchAFishHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.carnival.zombieShootoutHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.carnival.zombieShootoutHelper.@Tooltip"))
        //   java: 								.binding(defaults.helpers.carnival.zombieShootoutHelper,
        //   java: 										() -> config.helpers.carnival.zombieShootoutHelper,
        //   java: 										newValue -> config.helpers.carnival.zombieShootoutHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Bazaar
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.bazaar"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.bazaar.enableBazaarHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.bazaar.enableBazaarHelper.@Tooltip", BazaarHelper.getExpiringIcon(), BazaarHelper.getExpiredIcon(), BazaarHelper.getFilledIcon(69), BazaarHelper.getFilledIcon(100)))
        //   java: 								.binding(defaults.helpers.bazaar.enableBazaarHelper,
        //   java: 										() -> config.helpers.bazaar.enableBazaarHelper,
        //   java: 										newValue -> config.helpers.bazaar.enableBazaarHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.bazaar.enableReorderHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.bazaar.enableReorderHelper.@Tooltip"))
        //   java: 								.binding(defaults.helpers.bazaar.enableReorderHelper,
        //   java: 										() -> config.helpers.bazaar.enableReorderHelper,
        //   java: 										newValue -> config.helpers.bazaar.enableReorderHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.bazaar.enableOrderTracker"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.bazaar.enableOrderTracker.@Tooltip"))
        //   java: 								.binding(defaults.helpers.bazaar.enableOrderTracker,
        //   java: 										() -> config.helpers.bazaar.enableOrderTracker,
        //   java: 										newValue -> config.helpers.bazaar.enableOrderTracker = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.bazaar.enableBazaarMax"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.bazaar.enableBazaarMax.@Tooltip"))
        //   java: 								.binding(defaults.helpers.bazaar.enableBazaarMax,
        //   java: 										() -> config.helpers.bazaar.enableBazaarMax,
        //   java: 										newValue -> config.helpers.bazaar.enableBazaarMax = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.itemPrice.enableItemPriceLookup"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.itemPrice.enableItemPriceLookup.@Tooltip"))
        //   java: 								.binding(defaults.helpers.itemPrice.enableItemPriceLookup,
        //   java: 										() -> config.helpers.itemPrice.enableItemPriceLookup,
        //   java: 										newValue -> config.helpers.itemPrice.enableItemPriceLookup = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.itemPriceLookup"))
        //   java: 								.tags(CommonTags.KEY_MAPPING)
        //   java: 								.keyMapping(ItemPrice.ITEM_PRICE_LOOKUP)
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				// Great Spook Event
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.greatSpook"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.greatSpook.enableMathTeacherHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.greatSpook.enableMathTeacherHelper.@Tooltip"))
        //   java: 								.binding(defaults.helpers.greatSpookEvent.enableMathTeacherHelper,
        //   java: 										() -> config.helpers.greatSpookEvent.enableMathTeacherHelper,
        //   java: 										newValue -> config.helpers.greatSpookEvent.enableMathTeacherHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Century Raffle
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.helpers.centuryRaffle"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.centuryRaffle.enableRaffleTaskHighlight"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.centuryRaffle.enableRaffleTaskHighlight.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_6_0)
        //   java: 								.binding(defaults.helpers.centuryRaffle.enableRaffleTaskHighlight,
        //   java: 										() -> config.helpers.centuryRaffle.enableRaffleTaskHighlight,
        //   java: 										newValue -> config.helpers.centuryRaffle.enableRaffleTaskHighlight = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.helpers.centuryRaffle.enableRaffleRewardHighlight"))
        //   java: 								.description(Component.translatable("skyblocker.config.helpers.centuryRaffle.enableRaffleRewardHighlight.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_6_0)
        //   java: 								.binding(defaults.helpers.centuryRaffle.enableRaffleRewardHighlight,
        //   java: 										() -> config.helpers.centuryRaffle.enableRaffleRewardHighlight,
        //   java: 										newValue -> config.helpers.centuryRaffle.enableRaffleRewardHighlight = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
