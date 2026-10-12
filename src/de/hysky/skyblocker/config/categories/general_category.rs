//! Ported from de.hysky.skyblocker.config.categories.GeneralCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class GeneralCategory
#[derive(Clone)]
pub struct GeneralCategory {}

impl GeneralCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/general"))
        //   java: 				.name(Component.translatable("skyblocker.config.general"))
        //   java:
        //   java: 				//Skyblocker Screen
        //   java: 				.option(ButtonOption.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.skyblockerScreen"))
        //   java: 						.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 						.action(_ -> Minecraft.getInstance().gui.setScreen(new SkyblockerScreen()))
        //   java: 						.build())
        //   java:
        //   java: 				// Disable All
        //   java: 				.option(ButtonOption.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.disableAll.popup.title"))
        //   java: 						.description(Component.translatable("skyblocker.disableAll.popup.description"))
        //   java: 						.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 						.action(screen -> new DisableAllPopup().open(screen))
        //   java: 						.build())
        //   java:
        //   java: 				//Ungrouped Options
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.enableTips"))
        //   java: 						.binding(defaults.general.enableTips,
        //   java: 								() -> config.general.enableTips,
        //   java: 								newValue -> config.general.enableTips = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.updateNotifications"))
        //   java: 						.description(Component.translatable("skyblocker.config.general.updateNotifications.@Tooltip"))
        //   java: 						.binding(UpdateNotifications.Config.DEFAULT.enabled(),
        //   java: 								() -> UpdateNotifications.getConfig().enabled(),
        //   java: 								newValue -> UpdateNotifications.setConfig(updateNotificationsConfig -> updateNotificationsConfig.withEnabled(newValue)))
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<UpdateNotifications.Channel>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.updateNotifications.updateChannel"))
        //   java: 						.description(Component.translatable("skyblocker.config.general.updateNotifications.updateChannel.@Tooltip"))
        //   java: 						.binding(UpdateNotifications.Config.DEFAULT.channel(),
        //   java: 								() -> UpdateNotifications.getConfig().channel(),
        //   java: 								newValue -> UpdateNotifications.setConfig(updateNotificationsConfig -> updateNotificationsConfig.withChannel(newValue)))
        //   java: 						.controller(ConfigUtils.createEnumController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.acceptReparty"))
        //   java: 						.binding(defaults.general.acceptReparty,
        //   java: 								() -> config.general.acceptReparty,
        //   java: 								newValue -> config.general.acceptReparty = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				//Config Backups
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.backup"))
        //   java: 						.collapsed(true)
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.backup.manage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new ConfigBackupScreen(screen)))
        //   java: 								.build())
        //   java: 						.build())
        //   java: 				// Speed Presets
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.speedPresets"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.speedPresets.enableSpeedPresets"))
        //   java: 								.binding(defaults.general.speedPresets.enableSpeedPresets,
        //   java: 										() -> config.general.speedPresets.enableSpeedPresets,
        //   java: 										newValue -> config.general.speedPresets.enableSpeedPresets = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.speedPresets.config"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new SpeedPresetsScreen(screen)))
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Shortcuts
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.shortcuts"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.shortcuts.enableShortcuts"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.shortcuts.enableShortcuts.@Tooltip"))
        //   java: 								.binding(defaults.general.shortcuts.enableShortcuts,
        //   java: 										() -> config.general.shortcuts.enableShortcuts,
        //   java: 										newValue -> config.general.shortcuts.enableShortcuts = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.shortcuts.enableCommandShortcuts"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.shortcuts.enableCommandShortcuts.@Tooltip"))
        //   java: 								.binding(defaults.general.shortcuts.enableCommandShortcuts,
        //   java: 										() -> config.general.shortcuts.enableCommandShortcuts,
        //   java: 										newValue -> config.general.shortcuts.enableCommandShortcuts = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.shortcuts.enableCommandArgShortcuts"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.shortcuts.enableCommandArgShortcuts.@Tooltip"))
        //   java: 								.binding(defaults.general.shortcuts.enableCommandArgShortcuts,
        //   java: 										() -> config.general.shortcuts.enableCommandArgShortcuts,
        //   java: 										newValue -> config.general.shortcuts.enableCommandArgShortcuts = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.shortcuts.enableKeyBindingShortcuts"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.shortcuts.enableKeyBindingShortcuts.@Tooltip"))
        //   java: 								.binding(defaults.general.shortcuts.enableKeyBindingShortcuts,
        //   java: 										() -> config.general.shortcuts.enableKeyBindingShortcuts,
        //   java: 										newValue -> config.general.shortcuts.enableKeyBindingShortcuts = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.shortcuts.config"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new ShortcutsConfigScreen(screen)))
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Quiver Warning
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.quiverWarning"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.quiverWarning.enableQuiverWarning"))
        //   java: 								.binding(defaults.general.quiverWarning.enableQuiverWarning,
        //   java: 										() -> config.general.quiverWarning.enableQuiverWarning,
        //   java: 										newValue -> config.general.quiverWarning.enableQuiverWarning = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.quiverWarning.enableQuiverWarningInDungeons"))
        //   java: 								.binding(defaults.general.quiverWarning.enableQuiverWarningInDungeons,
        //   java: 										() -> config.general.quiverWarning.enableQuiverWarningInDungeons,
        //   java: 										newValue -> config.general.quiverWarning.enableQuiverWarningInDungeons = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.quiverWarning.enableQuiverWarningAfterDungeon"))
        //   java: 								.binding(defaults.general.quiverWarning.enableQuiverWarningAfterDungeon,
        //   java: 										() -> config.general.quiverWarning.enableQuiverWarningAfterDungeon,
        //   java: 										newValue -> config.general.quiverWarning.enableQuiverWarningAfterDungeon = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Item List
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.itemList"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemList.enableRecipeBook"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemList.enableRecipeBook.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_3_0)
        //   java: 								.binding(defaults.general.itemList.enableRecipeBook,
        //   java: 										() -> config.general.itemList.enableRecipeBook,
        //   java: 										newValue -> config.general.itemList.enableRecipeBook = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemList.enableItemList"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemList.enableItemList.@Tooltip"))
        //   java: 								.binding(defaults.general.itemList.enableItemList,
        //   java: 										() -> config.general.itemList.enableItemList,
        //   java: 										newValue -> config.general.itemList.enableItemList = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemList.enableCollapsibleEntries"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemList.enableCollapsibleEntries.@Tooltip"))
        //   java: 								.binding(defaults.general.itemList.enableCollapsibleEntries,
        //   java: 										() -> config.general.itemList.enableCollapsibleEntries,
        //   java: 										newValue -> config.general.itemList.enableCollapsibleEntries = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Item Tooltip
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.itemTooltip"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableNPCPrice"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enablePrice.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableNPCPrice,
        //   java: 										() -> config.general.itemTooltip.enableNPCPrice,
        //   java: 										newValue -> config.general.itemTooltip.enableNPCPrice = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableMotesPrice"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enableMotesPrice.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableMotesPrice,
        //   java: 										() -> config.general.itemTooltip.enableMotesPrice,
        //   java: 										newValue -> config.general.itemTooltip.enableMotesPrice = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableAvgBIN"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enablePrice.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableAvgBIN,
        //   java: 										() -> config.general.itemTooltip.enableAvgBIN,
        //   java: 										newValue -> config.general.itemTooltip.enableAvgBIN = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<GeneralConfig.Average>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.avg"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.avg.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.avg,
        //   java: 										() -> config.general.itemTooltip.avg,
        //   java: 										newValue -> config.general.itemTooltip.avg = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableLowestBIN"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enablePrice.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableLowestBIN,
        //   java: 										() -> config.general.itemTooltip.enableLowestBIN,
        //   java: 										newValue -> config.general.itemTooltip.enableLowestBIN = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableBazaarPrice"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enablePrice.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableBazaarPrice,
        //   java: 										() -> config.general.itemTooltip.enableBazaarPrice,
        //   java: 										newValue -> config.general.itemTooltip.enableBazaarPrice = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<GeneralConfig.Craft>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.craft"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.craft.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableCraftingCost,
        //   java: 										() -> config.general.itemTooltip.enableCraftingCost,
        //   java: 										newValue -> config.general.itemTooltip.enableCraftingCost = newValue)
        //   java: 								.listener((_, updateType) -> {
        //   java: 									if (updateType == UpdateType.VALUE_CHANGE) CraftPriceTooltip.clearCache();
        //   java: 								})
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableObtainedDate"))
        //   java: 								.binding(defaults.general.itemTooltip.enableObtainedDate,
        //   java: 										() -> config.general.itemTooltip.enableObtainedDate,
        //   java: 										newValue -> config.general.itemTooltip.enableObtainedDate = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableMuseumInfo"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enableMuseumInfo.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableMuseumInfo,
        //   java: 										() -> config.general.itemTooltip.enableMuseumInfo,
        //   java: 										newValue -> config.general.itemTooltip.enableMuseumInfo = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableExoticTooltip"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enableExoticTooltip.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableExoticTooltip,
        //   java: 										() -> config.general.itemTooltip.enableExoticTooltip,
        //   java: 										newValue -> config.general.itemTooltip.enableExoticTooltip = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableAccessoriesHelper"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enableAccessoriesHelper.@Tooltip[0]"),
        //   java: 										Component.literal("\n\n✔ Collected").withColor(AccessoryTooltip.COLLECTED_COLOUR),
        //   java: 										Component.translatable("skyblocker.config.general.itemTooltip.enableAccessoriesHelper.@Tooltip[1]"),
        //   java: 										Component.literal("\n✦ Upgrade").withColor(AccessoryTooltip.UPGRADE_COLOUR),
        //   java: 										Component.translatable("skyblocker.config.general.itemTooltip.enableAccessoriesHelper.@Tooltip[2]"),
        //   java: 										Component.literal("\n↑ Upgradable").withColor(AccessoryTooltip.UPGRADABLE_COLOUR),
        //   java: 										Component.translatable("skyblocker.config.general.itemTooltip.enableAccessoriesHelper.@Tooltip[3]"),
        //   java: 										Component.literal("\n↓ Downgrade").withColor(AccessoryTooltip.DOWNGRADE_COLOUR),
        //   java: 										Component.translatable("skyblocker.config.general.itemTooltip.enableAccessoriesHelper.@Tooltip[4]"),
        //   java: 										Component.literal("\n✖ Missing").withColor(AccessoryTooltip.MISSING_COLOUR),
        //   java: 										Component.translatable("skyblocker.config.general.itemTooltip.enableAccessoriesHelper.@Tooltip[5]"))
        //   java: 								.binding(defaults.general.itemTooltip.enableAccessoriesHelper,
        //   java: 										() -> config.general.itemTooltip.enableAccessoriesHelper,
        //   java: 										newValue -> config.general.itemTooltip.enableAccessoriesHelper = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.dungeonQuality"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.dungeonQuality.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.dungeonQuality,
        //   java: 										() -> config.general.itemTooltip.dungeonQuality,
        //   java: 										newValue -> config.general.itemTooltip.dungeonQuality = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.showEssenceCost"))
        //   java: 								.binding(defaults.general.itemTooltip.showEssenceCost,
        //   java: 										() -> config.general.itemTooltip.showEssenceCost,
        //   java: 										newValue -> config.general.itemTooltip.showEssenceCost = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableEstimatedItemValue"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enableEstimatedItemValue.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableEstimatedItemValue,
        //   java: 										() -> config.general.itemTooltip.enableEstimatedItemValue,
        //   java: 										newValue -> config.general.itemTooltip.enableEstimatedItemValue = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableStackingEnchantProgress"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enableStackingEnchantProgress.@Tooltip"))
        //   java: 								.binding(defaults.general.itemTooltip.enableStackingEnchantProgress,
        //   java: 										() -> config.general.itemTooltip.enableStackingEnchantProgress,
        //   java: 										newValue -> config.general.itemTooltip.enableStackingEnchantProgress = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableEvolvingItemProgress"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemTooltip.enableEvolvingItemProgress.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.general.itemTooltip.enableEvolvingItemProgress,
        //   java: 										() -> config.general.itemTooltip.enableEvolvingItemProgress,
        //   java: 										newValue -> config.general.itemTooltip.enableEvolvingItemProgress = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemTooltip.enableGeorgePrice"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_4_0)
        //   java: 								.binding(defaults.general.itemTooltip.enableGeorgePrice,
        //   java: 										() -> config.general.itemTooltip.enableGeorgePrice,
        //   java: 										newValue -> config.general.itemTooltip.enableGeorgePrice = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Item Info Display
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.itemInfoDisplay"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<GeneralConfig.ItemBackgroundStyle>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemInfoDisplay.itemBackgroundStyle"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemInfoDisplay.itemBackgroundStyle.@Tooltip"))
        //   java: 								.binding(defaults.general.itemInfoDisplay.itemBackgroundStyle,
        //   java: 										() -> config.general.itemInfoDisplay.itemBackgroundStyle,
        //   java: 										newValue -> config.general.itemInfoDisplay.itemBackgroundStyle = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemInfoDisplay.itemBackgroundOpacity"))
        //   java: 								.binding(defaults.general.itemInfoDisplay.itemBackgroundOpacity,
        //   java: 										() -> config.general.itemInfoDisplay.itemBackgroundOpacity,
        //   java: 										newValue -> config.general.itemInfoDisplay.itemBackgroundOpacity = newValue)
        //   java: 								.controller(FloatController.createBuilder().range(0f, 1f).slider(0.05f).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemInfoDisplay.itemRarityBackgrounds"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemInfoDisplay.itemRarityBackgrounds.@Tooltip"))
        //   java: 								.binding(defaults.general.itemInfoDisplay.itemRarityBackgrounds,
        //   java: 										() -> config.general.itemInfoDisplay.itemRarityBackgrounds,
        //   java: 										newValue -> config.general.itemInfoDisplay.itemRarityBackgrounds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemInfoDisplay.itemRarityBackgroundsLegacyColors"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemInfoDisplay.itemRarityBackgroundsLegacyColors.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_8_0)
        //   java: 								.binding(defaults.general.itemInfoDisplay.itemRarityBackgroundsLegacyColors,
        //   java: 										() -> config.general.itemInfoDisplay.itemRarityBackgroundsLegacyColors,
        //   java: 										newValue -> config.general.itemInfoDisplay.itemRarityBackgroundsLegacyColors = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemInfoDisplay.jacobMedalBackgrounds"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemInfoDisplay.jacobMedalBackgrounds.@Tooltip"))
        //   java: 								.binding(defaults.general.itemInfoDisplay.jacobMedalBackgrounds,
        //   java: 										() -> config.general.itemInfoDisplay.jacobMedalBackgrounds,
        //   java: 										newValue -> config.general.itemInfoDisplay.jacobMedalBackgrounds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemInfoDisplay.legacyAttributeBackgrounds"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemInfoDisplay.legacyAttributeBackgrounds.@Tooltip"))
        //   java: 								.binding(defaults.general.itemInfoDisplay.legacyAttributeBackgrounds,
        //   java: 										() -> config.general.itemInfoDisplay.legacyAttributeBackgrounds,
        //   java: 										newValue -> config.general.itemInfoDisplay.legacyAttributeBackgrounds = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Item Protection
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.itemProtection"))
        //   java: 						.collapsed(true)
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.hotbarSlotLock"))
        //   java: 								.tags(CommonTags.KEY_MAPPING)
        //   java: 								.keyMapping(HotbarSlotLock.hotbarSlotLock)
        //   java: 								.build())
        //   java: 						.option(Option.<GeneralConfig.SlotLockStyle>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemProtection.slotLockStyle"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemProtection.slotLockStyle.@Tooltip"))
        //   java: 								.binding(defaults.general.itemProtection.slotLockStyle,
        //   java: 										() -> config.general.itemProtection.slotLockStyle,
        //   java: 										newValue -> config.general.itemProtection.slotLockStyle = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemProtection.protectValuableConsumables"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemProtection.protectValuableConsumables.@Tooltip"))
        //   java: 								.binding(defaults.general.itemProtection.protectValuableConsumables,
        //   java: 										() -> config.general.itemProtection.protectValuableConsumables,
        //   java: 										newValue -> config.general.itemProtection.protectValuableConsumables = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.itemProtection"))
        //   java: 								.tags(CommonTags.KEY_MAPPING)
        //   java: 								.keyMapping(ItemProtection.itemProtection)
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.itemProtection.displayChatNotification"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.itemProtection.displayChatNotification.@Tooltip"))
        //   java: 								.binding(defaults.general.itemProtection.displayChatNotification,
        //   java: 										() -> config.general.itemProtection.displayChatNotification,
        //   java: 										newValue -> config.general.itemProtection.displayChatNotification = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Wiki Lookup
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.wikiLookup"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.wikiLookup.enableWikiLookup"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.wikiLookup.enableWikiLookup.@Tooltip",
        //   java: 										WikiLookupManager.independentWikiLookup.getTranslatedKeyMessage()))
        //   java: 								.binding(defaults.general.wikiLookup.enableWikiLookup,
        //   java: 										() -> config.general.wikiLookup.enableWikiLookup,
        //   java: 										newValue -> config.general.wikiLookup.enableWikiLookup = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.wikiLookup.independent"))
        //   java: 								.tags(CommonTags.KEY_MAPPING)
        //   java: 								.keyMapping(WikiLookupManager.independentWikiLookup)
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Special Effects
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.general.specialEffects"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.specialEffects.rareDungeonDropEffects"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.specialEffects.rareDungeonDropEffects.@Tooltip"))
        //   java: 								.binding(defaults.general.specialEffects.rareDungeonDropEffects,
        //   java: 										() -> config.general.specialEffects.rareDungeonDropEffects,
        //   java: 										newValue -> config.general.specialEffects.rareDungeonDropEffects = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.specialEffects.rareDropEffects"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.specialEffects.rareDropEffects.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_11_0)
        //   java: 								.binding(defaults.general.specialEffects.rareDropEffects,
        //   java: 										() -> config.general.specialEffects.rareDropEffects,
        //   java: 										newValue -> config.general.specialEffects.rareDropEffects = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.specialEffects.rareDyeDropEffects"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.specialEffects.rareDyeDropEffects.@Tooltip"))
        //   java: 								.binding(defaults.general.specialEffects.rareDyeDropEffects,
        //   java: 										() -> config.general.specialEffects.rareDyeDropEffects,
        //   java: 										newValue -> config.general.specialEffects.rareDyeDropEffects = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.specialEffects.trophyDropEffects"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.specialEffects.trophyDropEffects.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_5_1)
        //   java: 								.binding(defaults.general.specialEffects.trophyDropEffects,
        //   java: 										() -> config.general.specialEffects.trophyDropEffects,
        //   java: 										newValue -> config.general.specialEffects.trophyDropEffects = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.general.specialEffects.displayItemName"))
        //   java: 								.description(Component.translatable("skyblocker.config.general.specialEffects.displayItemName.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_7_0)
        //   java: 								.binding(defaults.general.specialEffects.displayItemName,
        //   java: 										() -> config.general.specialEffects.displayItemName,
        //   java: 										newValue -> config.general.specialEffects.displayItemName = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
