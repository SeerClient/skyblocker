//! Ported from de.hysky.skyblocker.config.categories.UIAndVisualsCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_manager::SlotTextManager;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu_manager::RadialMenuManager;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;

/// java: public class UIAndVisualsCategory
#[derive(Clone)]
pub struct UIAndVisualsCategory {}

impl UIAndVisualsCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/uiandvisuals"))
        //   java: 				.name(Component.translatable("skyblocker.config.uiAndVisuals"))
        //   java:
        //   java: 				//Ungrouped Options
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.swingOnAbilities"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.swingOnAbilities.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.swingOnAbilities,
        //   java: 								() -> config.uiAndVisuals.swingOnAbilities,
        //   java: 								newValue -> config.uiAndVisuals.swingOnAbilities = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Integer>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.nightVisionStrength"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.nightVisionStrength.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.nightVisionStrength,
        //   java: 								() -> config.uiAndVisuals.nightVisionStrength,
        //   java: 								newValue -> config.uiAndVisuals.nightVisionStrength = newValue)
        //   java: 						.controller(IntegerController.createBuilder().range(0, 100).slider(1).build())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.compactorDeletorPreview"))
        //   java: 						.binding(defaults.uiAndVisuals.compactorDeletorPreview,
        //   java: 								() -> config.uiAndVisuals.compactorDeletorPreview,
        //   java: 								newValue -> config.uiAndVisuals.compactorDeletorPreview = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.dontStripSkinAlphaValues"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.dontStripSkinAlphaValues.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.dontStripSkinAlphaValues,
        //   java: 								() -> config.uiAndVisuals.dontStripSkinAlphaValues,
        //   java: 								newValue -> config.uiAndVisuals.dontStripSkinAlphaValues = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.flags(OptionFlag.ASSET_RELOAD)
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.backpackPreviewWithoutShift"))
        //   java: 						.binding(defaults.uiAndVisuals.backpackPreviewWithoutShift,
        //   java: 								() -> config.uiAndVisuals.backpackPreviewWithoutShift,
        //   java: 								newValue -> config.uiAndVisuals.backpackPreviewWithoutShift = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.hideEmptyTooltips"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.hideEmptyTooltips.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.hideEmptyTooltips,
        //   java: 								() -> config.uiAndVisuals.hideEmptyTooltips,
        //   java: 								newValue -> config.uiAndVisuals.hideEmptyTooltips = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.fancyCraftingTable"))
        //   java: 						.binding(defaults.uiAndVisuals.fancyCraftingTable,
        //   java: 								() -> config.uiAndVisuals.fancyCraftingTable,
        //   java: 								newValue -> config.uiAndVisuals.fancyCraftingTable = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.hideStatusEffectOverlay"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.hideStatusEffectOverlay.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.hideStatusEffectOverlay,
        //   java: 								() -> config.uiAndVisuals.hideStatusEffectOverlay,
        //   java: 								newValue -> config.uiAndVisuals.hideStatusEffectOverlay = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.museumOverlay"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.museumOverlay.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.museumOverlay,
        //   java: 								() -> config.uiAndVisuals.museumOverlay,
        //   java: 								newValue -> config.uiAndVisuals.museumOverlay = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.cancelComponentUpdateAnimation"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.cancelComponentUpdateAnimation.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.cancelComponentUpdateAnimation,
        //   java: 								() -> config.uiAndVisuals.cancelComponentUpdateAnimation,
        //   java: 								newValue -> config.uiAndVisuals.cancelComponentUpdateAnimation = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.showCustomizeButton"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.showCustomizeButton.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.showCustomizeButton,
        //   java: 								() -> config.uiAndVisuals.showCustomizeButton,
        //   java: 								newValue -> config.uiAndVisuals.showCustomizeButton = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.showConfigButton"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.showConfigButton.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.showConfigButton,
        //   java: 								() -> config.uiAndVisuals.showConfigButton,
        //   java: 								newValue -> config.uiAndVisuals.showConfigButton = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.trueQuiverCount"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.trueQuiverCount.@Tooltip"))
        //   java: 						.binding(defaults.uiAndVisuals.trueQuiverCount,
        //   java: 								() -> config.uiAndVisuals.trueQuiverCount,
        //   java: 								newValue -> config.uiAndVisuals.trueQuiverCount = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				// SkyBlock Inventory Screen
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.skyblockInventoryScreen"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.skyblockInventoryScreen.showEquipmentInInventory"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.skyblockInventoryScreen.showEquipmentInInventory.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.showEquipmentInInventory,
        //   java: 										() -> config.uiAndVisuals.showEquipmentInInventory,
        //   java: 										newValue -> config.uiAndVisuals.showEquipmentInInventory = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.skyblockInventoryScreen.openEquipmentToStats"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.skyblockInventoryScreen.openEquipmentToStats.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_7_0)
        //   java: 								.binding(defaults.uiAndVisuals.skyblockInventoryScreen.openEquipmentToStatsPage,
        //   java: 										() -> config.uiAndVisuals.skyblockInventoryScreen.openEquipmentToStatsPage,
        //   java: 										newValue -> config.uiAndVisuals.skyblockInventoryScreen.openEquipmentToStatsPage = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Chest Value FIXME change dropdown to color controller
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.chestValue"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.chestValue.enableChestValue"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.chestValue.enableChestValue.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.chestValue.enableChestValue,
        //   java: 										() -> config.uiAndVisuals.chestValue.enableChestValue,
        //   java: 										newValue -> config.uiAndVisuals.chestValue.enableChestValue = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFormatting>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.chestValue.color"))
        //   java: 								.binding(defaults.uiAndVisuals.chestValue.color,
        //   java: 										() -> config.uiAndVisuals.chestValue.color,
        //   java: 										newValue -> config.uiAndVisuals.chestValue.color = newValue)
        //   java: 								.controller(ConfigUtils.createEnumDropdownController(ConfigUtils.FORMATTING_FORMATTER))
        //   java: 								.build())
        //   java: 						.option(Option.<ChatFormatting>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.chestValue.incompleteColor"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.chestValue.incompleteColor.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.chestValue.incompleteColor,
        //   java: 										() -> config.uiAndVisuals.chestValue.incompleteColor,
        //   java: 										newValue -> config.uiAndVisuals.chestValue.incompleteColor = newValue)
        //   java: 								.controller(ConfigUtils.createEnumDropdownController(ConfigUtils.FORMATTING_FORMATTER))
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.valueBreadownPopup"))
        //   java: 								.tags(CommonTags.KEY_MAPPING)
        //   java: 								.keyMapping(ValueBreakdownPopup.KEY_BINDING)
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Item Cooldown
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.itemCooldown"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.itemCooldown.enableItemCooldowns"))
        //   java: 								.binding(defaults.uiAndVisuals.itemCooldown.enableItemCooldowns,
        //   java: 										() -> config.uiAndVisuals.itemCooldown.enableItemCooldowns,
        //   java: 										newValue -> config.uiAndVisuals.itemCooldown.enableItemCooldowns = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Slot Text
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.slotText"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<SlotTextMode>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.slotText"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.slotText.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.slotText.slotTextMode,
        //   java: 										() -> config.uiAndVisuals.slotText.slotTextMode,
        //   java: 										newValue -> config.uiAndVisuals.slotText.slotTextMode = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.slottext"))
        //   java: 								.tags(CommonTags.KEY_MAPPING)
        //   java: 								.keyMapping(SlotTextManager.KEY_MAPPING)
        //   java: 								.build())
        //   java: 						.option(LabelOption.createBuilder()
        //   java: 								.label(Component.translatable("skyblocker.config.uiAndVisuals.slotText.separator"))
        //   java: 								.build())
        //   java: 						.options(createSlotTextToggles(config))
        //   java: 						.build())
        //   java:
        //   java: 				// Storage Overlay
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.enabled.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_8_0)
        //   java: 								.binding(defaults.uiAndVisuals.storageOverlay.enabled,
        //   java: 										() -> config.uiAndVisuals.storageOverlay.enabled,
        //   java: 										newValue -> config.uiAndVisuals.storageOverlay.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.storagesPerRow"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.storagesPerRow.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_8_1)
        //   java: 								.binding(defaults.uiAndVisuals.storageOverlay.storagesPerRow,
        //   java: 										() -> config.uiAndVisuals.storageOverlay.storagesPerRow,
        //   java: 										newValue -> config.uiAndVisuals.storageOverlay.storagesPerRow = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(0, 20).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.backpackWidth"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.backpackWidth.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_8_0)
        //   java: 								.binding(defaults.uiAndVisuals.storageOverlay.backpackWidth,
        //   java: 										() -> config.uiAndVisuals.storageOverlay.backpackWidth,
        //   java: 										newValue -> config.uiAndVisuals.storageOverlay.backpackWidth = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(4, 45).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.rememberSearch"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.rememberSearch.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_8_0)
        //   java: 								.binding(defaults.uiAndVisuals.storageOverlay.rememberSearch,
        //   java: 										() -> config.uiAndVisuals.storageOverlay.rememberSearch,
        //   java: 										newValue -> config.uiAndVisuals.storageOverlay.rememberSearch = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.rememberOpened"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.storageOverlay.rememberOpened.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_8_2)
        //   java: 								.binding(defaults.uiAndVisuals.storageOverlay.rememberOpened,
        //   java: 										() -> config.uiAndVisuals.storageOverlay.rememberOpened,
        //   java: 										newValue -> config.uiAndVisuals.storageOverlay.rememberOpened = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build()
        //   java: 				)
        //   java:
        //   java: 				// Radial Menus
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.radialMenu"))
        //   java: 						.collapsed(true)
        //   java: 						.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.radialMenu.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.radialMenu.enabled.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.radialMenu.enabled,
        //   java: 										() -> config.uiAndVisuals.radialMenu.enabled,
        //   java: 										newValue -> config.uiAndVisuals.radialMenu.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.radialMenu.tooltipsWithoutShift"))
        //   java: 								.binding(defaults.uiAndVisuals.radialMenu.tooltipsWithoutShift,
        //   java: 										() -> config.uiAndVisuals.radialMenu.tooltipsWithoutShift,
        //   java: 										newValue -> config.uiAndVisuals.radialMenu.tooltipsWithoutShift = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.options(createRadialToggles(config))
        //   java: 						.build()
        //   java: 				)
        //   java:
        //   java: 				// Inventory Search
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.inventorySearch"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<UIAndVisualsConfig.InventorySearchConfig.EnableState>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.inventorySearch.enabled"))
        //   java: 								.binding(defaults.uiAndVisuals.inventorySearch.enabled,
        //   java: 										() -> config.uiAndVisuals.inventorySearch.enabled,
        //   java: 										newValue -> config.uiAndVisuals.inventorySearch.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(InputQuirks.REPLACE_CTRL_KEY_WITH_CMD_KEY ? Component.translatable("skyblocker.config.uiAndVisuals.inventorySearch.cmdK") : Component.translatable("skyblocker.config.uiAndVisuals.inventorySearch.ctrlK"))
        //   java: 								.binding(defaults.uiAndVisuals.inventorySearch.ctrlK,
        //   java: 										() -> config.uiAndVisuals.inventorySearch.ctrlK,
        //   java: 										newValue -> config.uiAndVisuals.inventorySearch.ctrlK = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.inventorySearch.showClickableText"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.inventorySearch.showClickableText.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.inventorySearch.clickableText,
        //   java: 										() -> config.uiAndVisuals.inventorySearch.clickableText,
        //   java: 										newValue -> config.uiAndVisuals.inventorySearch.clickableText = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Title Container
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.titleContainer"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.titleContainer.@Tooltip"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.titleContainer.titleContainerScale"))
        //   java: 								.binding(defaults.uiAndVisuals.titleContainer.titleContainerScale,
        //   java: 										() -> config.uiAndVisuals.titleContainer.titleContainerScale,
        //   java: 										newValue -> config.uiAndVisuals.titleContainer.titleContainerScale = newValue)
        //   java: 								.controller(FloatController.createBuilder().range(TitleContainerConfigScreen.MIN_TITLE_SCALE, TitleContainerConfigScreen.MAX_TITLE_SCALE).build())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.titleContainer.config"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new TitleContainerConfigScreen(screen)))
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Tab Hud
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.enableFancyTab"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.enableFancyTab.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.tabHud.tabHudEnabled,
        //   java: 										() -> config.uiAndVisuals.tabHud.tabHudEnabled,
        //   java: 										newValue -> config.uiAndVisuals.tabHud.tabHudEnabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.configScreen"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.configScreen.@Tooltip"))
        //   java: 								.tags(ArrayUtils.addAll(WidgetManager.WIDGET_INSTANCES.values().stream().map(w -> w.getInformation().displayName()).toArray(Component[]::new), Component.translatable("skyblocker.config.uiAndVisuals.tabHud.configScreen.@Tag"), CommonTags.ADDED_IN_6_10_0))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(_ -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen()))
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.tabHudScale"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.tabHudScale.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.tabHud.tabHudScale,
        //   java: 										() -> config.uiAndVisuals.tabHud.tabHudScale,
        //   java: 										newValue -> config.uiAndVisuals.tabHud.tabHudScale = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(10, 200).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.showVanillaTabByDefault"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.showVanillaTabByDefault.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.tabHud.showVanillaTabByDefault,
        //   java: 										() -> config.uiAndVisuals.tabHud.showVanillaTabByDefault,
        //   java: 										newValue -> config.uiAndVisuals.tabHud.showVanillaTabByDefault = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.defaultTgl"))
        //   java: 								.tags(CommonTags.KEY_MAPPING)
        //   java: 								.keyMapping(TabHud.defaultTgl)
        //   java: 								.build())
        //   java: 						.option(KeyMappingOption.createBuilder()
        //   java: 								.name(Component.translatable("key.skyblocker.toggleA"))
        //   java: 								.tags(CommonTags.KEY_MAPPING)
        //   java: 								.keyMapping(TabHud.toggleSecondary)
        //   java: 								.build())
        //   java: 						.option(Option.<UIAndVisualsConfig.TabHudStyle>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.style"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[0]"),
        //   java: 										Component.translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[1]"),
        //   java: 										Component.translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[2]"),
        //   java: 										Component.translatable("skyblocker.config.uiAndVisuals.tabHud.style.@Tooltip[3]"))
        //   java: 								.binding(defaults.uiAndVisuals.tabHud.style,
        //   java: 										() -> config.uiAndVisuals.tabHud.style,
        //   java: 										newValue -> config.uiAndVisuals.tabHud.style = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.displayIcons"))
        //   java: 								.binding(defaults.uiAndVisuals.tabHud.displayIcons,
        //   java: 										() -> config.uiAndVisuals.tabHud.displayIcons,
        //   java: 										newValue -> config.uiAndVisuals.tabHud.displayIcons = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.compactWidgets"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.compactWidgets.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.tabHud.compactWidgets,
        //   java: 										() -> config.uiAndVisuals.tabHud.compactWidgets,
        //   java: 										newValue -> config.uiAndVisuals.tabHud.compactWidgets = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.enableHudBackground"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.enableHudBackground.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.tabHud.enableHudBackground,
        //   java: 										() -> config.uiAndVisuals.tabHud.enableHudBackground,
        //   java: 										newValue -> config.uiAndVisuals.tabHud.enableHudBackground = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Positioner>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.tabHud.defaultPositioning"))
        //   java: 								.binding(defaults.uiAndVisuals.tabHud.defaultPositioning,
        //   java: 										() -> config.uiAndVisuals.tabHud.defaultPositioning,
        //   java: 										newValue -> config.uiAndVisuals.tabHud.defaultPositioning = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Fancy Auction House
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.fancyAuctionHouse"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.fancyAuctionHouse.enabled"))
        //   java: 								.binding(defaults.uiAndVisuals.fancyAuctionHouse.enabled,
        //   java: 										() -> config.uiAndVisuals.fancyAuctionHouse.enabled,
        //   java: 										newValue -> config.uiAndVisuals.fancyAuctionHouse.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.fancyAuctionHouse.highlightUnderAvgPrice"))
        //   java: 								.binding(defaults.uiAndVisuals.fancyAuctionHouse.highlightCheapBIN,
        //   java: 										() -> config.uiAndVisuals.fancyAuctionHouse.highlightCheapBIN,
        //   java: 										newValue -> config.uiAndVisuals.fancyAuctionHouse.highlightCheapBIN = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Fancy Bars
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.bars"))
        //   java: 						.tags(Component.translatable("skyblocker.config.uiAndVisuals.bars.@Tag[0]"), Component.translatable("skyblocker.config.uiAndVisuals.bars.@Tag[1]"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.enableBars"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.bars.enableBars.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.bars.enableBars,
        //   java: 										() -> config.uiAndVisuals.bars.enableBars,
        //   java: 										newValue -> config.uiAndVisuals.bars.enableBars = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.enableBarsRift"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.uiAndVisuals.bars.enableBarsRift,
        //   java: 										() -> config.uiAndVisuals.bars.enableBarsRift,
        //   java: 										newValue -> config.uiAndVisuals.bars.enableBarsRift = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.riftHealthHP"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.bars.riftHealthHP.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.uiAndVisuals.bars.riftHealthHP,
        //   java: 										() -> config.uiAndVisuals.bars.riftHealthHP,
        //   java: 										newValue -> config.uiAndVisuals.bars.riftHealthHP = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.enableVanillaStyleManaBar"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.uiAndVisuals.bars.enableVanillaStyleManaBar,
        //   java: 										() -> config.uiAndVisuals.bars.enableVanillaStyleManaBar,
        //   java: 										newValue -> config.uiAndVisuals.bars.enableVanillaStyleManaBar = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.enableVanillaStyleManaBarBlinking"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_6_0)
        //   java: 								.binding(defaults.uiAndVisuals.bars.enableVanillaStyleManaBarBlinking,
        //   java: 										() -> config.uiAndVisuals.bars.enableVanillaStyleManaBarBlinking,
        //   java: 										newValue -> config.uiAndVisuals.bars.enableVanillaStyleManaBarBlinking = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.useHungerBarSprites"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_6_0)
        //   java: 								.binding(defaults.uiAndVisuals.bars.useHungerBarSprites,
        //   java: 										() -> config.uiAndVisuals.bars.useHungerBarSprites,
        //   java: 										newValue -> config.uiAndVisuals.bars.useHungerBarSprites = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.openScreen"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(_ -> Minecraft.getInstance().gui.setScreen(new StatusBarsConfigScreen(FancyStatusBars.INSTANCE)))
        //   java: 								.build())
        //   java: 						.option(Option.<UIAndVisualsConfig.IntelligenceDisplay>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.intelligenceDisplay"))
        //   java: 								.binding(defaults.uiAndVisuals.bars.intelligenceDisplay,
        //   java: 										() -> config.uiAndVisuals.bars.intelligenceDisplay,
        //   java: 										newValue -> config.uiAndVisuals.bars.intelligenceDisplay = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController(intelligenceDisplay -> Component.translatable("skyblocker.config.uiAndVisuals.bars.intelligenceDisplay." + intelligenceDisplay.name())))
        //   java: 								.build()
        //   java: 						).option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bars.showEstimatedTilde"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.bars.showEstimatedTilde.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_8_1)
        //   java: 								.binding(defaults.uiAndVisuals.bars.showEstimatedTilde,
        //   java: 										() -> config.uiAndVisuals.bars.showEstimatedTilde,
        //   java: 										newValue -> config.uiAndVisuals.bars.showEstimatedTilde = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Waypoints
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.enableWaypoints"))
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.enableWaypoints,
        //   java: 										() -> config.uiAndVisuals.waypoints.enableWaypoints,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.enableWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Waypoint.Type>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.@Tooltip"),
        //   java: 										Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointType.generalNote"))
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.waypointType,
        //   java: 										() -> config.uiAndVisuals.waypoints.waypointType,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.waypointType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.renderLine"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.renderLine.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.renderLine,
        //   java: 										() -> config.uiAndVisuals.waypoints.renderLine,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.renderLine = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.lineColor"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.lineColor,
        //   java: 										() -> config.uiAndVisuals.waypoints.lineColor,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.lineColor = newValue)
        //   java: 								.controller(ConfigUtils.createColourController(true))
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.lineWidth"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.lineWidth,
        //   java: 										() -> config.uiAndVisuals.waypoints.lineWidth,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.lineWidth = newValue)
        //   java: 								.controller(FloatController.createBuilder().range(1f, 15f).slider(0.5f).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.allowSkippingWaypoints"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.allowSkippingWaypoints.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.allowSkippingWaypoints,
        //   java: 										() -> config.uiAndVisuals.waypoints.allowSkippingWaypoints,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.allowSkippingWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointActivationRadius"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.waypointActivationRadius.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_3_0)
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.waypointActivationRadius,
        //   java: 										() -> config.uiAndVisuals.waypoints.waypointActivationRadius,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.waypointActivationRadius = newValue)
        //   java: 								.controller(FloatController.createBuilder().range(1f, 10f).slider(0.5f).build())
        //   java: 								.build()
        //   java: 						)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.allowGoingBackwards"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.allowGoingBackwards.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_5_9_0)
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.allowGoingBackwards,
        //   java: 										() -> config.uiAndVisuals.waypoints.allowGoingBackwards,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.allowGoingBackwards = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.waypoints.config"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WaypointsScreen(screen)))
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.enableChatWaypoints"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.waypoints.enableChatWaypoints.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.waypoints.enableChatWaypoints,
        //   java: 										() -> config.uiAndVisuals.waypoints.enableChatWaypoints,
        //   java: 										newValue -> config.uiAndVisuals.waypoints.enableChatWaypoints = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Teleport Overlays
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.enableTeleportOverlays"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.enableTeleportOverlays,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.enableTeleportOverlays,
        //   java: 										newValue -> config.uiAndVisuals.teleportOverlay.enableTeleportOverlays = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.showWhenInAir"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.showWhenInAir,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.showWhenInAir,
        //   java: 										newValue -> config.uiAndVisuals.teleportOverlay.showWhenInAir = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.teleportOverlayColor"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.teleportOverlayColor,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.teleportOverlayColor,
        //   java: 										newValue -> {
        //   java: 											config.uiAndVisuals.teleportOverlay.teleportOverlayColor = newValue;
        //   java: 											TeleportOverlay.configCallback(newValue);
        //   java: 										})
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(true).build())
        //   java: 								.build())
        //   java: 						.option(Option.<UIAndVisualsConfig.TeleportOverlay.HighlightType>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.highlightType"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.highlightType.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.highlightType,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.highlightType,
        //   java: 										newValue -> config.uiAndVisuals.teleportOverlay.highlightType = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.enableWeirdTransmission"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableWeirdTransmission.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.enableWeirdTransmission,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.enableWeirdTransmission,
        //   java: 										newValue -> config.uiAndVisuals.teleportOverlay.enableWeirdTransmission = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.enableInstantTransmission"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableInstantTransmission.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.enableInstantTransmission,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.enableInstantTransmission,
        //   java: 										newValue -> config.uiAndVisuals.teleportOverlay.enableInstantTransmission = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.enableEtherTransmission"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableEtherTransmission.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.enableEtherTransmission,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.enableEtherTransmission,
        //   java: 										newValue -> config.uiAndVisuals.teleportOverlay.enableEtherTransmission = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.enableSinrecallTransmission"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableSinrecallTransmission.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.enableSinrecallTransmission,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.enableSinrecallTransmission,
        //   java: 										newValue -> config.uiAndVisuals.teleportOverlay.enableSinrecallTransmission = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.teleportOverlay.enableWitherImpact"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableWitherImpact.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.teleportOverlay.enableWitherImpact,
        //   java: 										() -> config.uiAndVisuals.teleportOverlay.enableWitherImpact,
        //   java: 										newValue -> config.uiAndVisuals.teleportOverlay.enableWitherImpact = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Smooth AOTE
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE"))
        //   java: 						.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.@Tooltip"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.predictive"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.predictive.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.smoothAOTE.predictive,
        //   java: 										() -> config.uiAndVisuals.smoothAOTE.predictive,
        //   java: 										newValue -> config.uiAndVisuals.smoothAOTE.predictive = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.thirdPerson"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.thirdPerson.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_6_0)
        //   java: 								.binding(defaults.uiAndVisuals.smoothAOTE.thirdPerson,
        //   java: 										() -> config.uiAndVisuals.smoothAOTE.thirdPerson,
        //   java: 										newValue -> config.uiAndVisuals.smoothAOTE.thirdPerson = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableWeirdTransmission"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableWeirdTransmission.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.smoothAOTE.enableWeirdTransmission,
        //   java: 										() -> config.uiAndVisuals.smoothAOTE.enableWeirdTransmission,
        //   java: 										newValue -> config.uiAndVisuals.smoothAOTE.enableWeirdTransmission = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableInstantTransmission"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableInstantTransmission.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.smoothAOTE.enableInstantTransmission,
        //   java: 										() -> config.uiAndVisuals.smoothAOTE.enableInstantTransmission,
        //   java: 										newValue -> config.uiAndVisuals.smoothAOTE.enableInstantTransmission = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableEtherTransmission"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableEtherTransmission.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.smoothAOTE.enableEtherTransmission,
        //   java: 										() -> config.uiAndVisuals.smoothAOTE.enableEtherTransmission,
        //   java: 										newValue -> config.uiAndVisuals.smoothAOTE.enableEtherTransmission = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableSinrecallTransmission"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableSinrecallTransmission.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.smoothAOTE.enableSinrecallTransmission,
        //   java: 										() -> config.uiAndVisuals.smoothAOTE.enableSinrecallTransmission,
        //   java: 										newValue -> config.uiAndVisuals.smoothAOTE.enableSinrecallTransmission = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableWitherImpact"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.enableWitherImpact.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.smoothAOTE.enableWitherImpact,
        //   java: 										() -> config.uiAndVisuals.smoothAOTE.enableWitherImpact,
        //   java: 										newValue -> config.uiAndVisuals.smoothAOTE.enableWitherImpact = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.maximumAddedLag"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.smoothAOTE.maximumAddedLag.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.smoothAOTE.maximumAddedLag,
        //   java: 										() -> config.uiAndVisuals.smoothAOTE.maximumAddedLag,
        //   java: 										newValue -> config.uiAndVisuals.smoothAOTE.maximumAddedLag = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(0, 500).slider(1).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Search overlay
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableBazaar"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableBazaar.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.enableBazaar,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.enableBazaar,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.enableBazaar = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableAuctionHouse"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableAuctionHouse.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.enableAuctionHouse,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.enableAuctionHouse,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.enableAuctionHouse = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableIronmanAuctionHouse"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableIronmanAuctionHouse.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.enableIronmanAuctionHouse,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.enableIronmanAuctionHouse,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.enableIronmanAuctionHouse = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableMuseum"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableMuseum.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.enableMuseum,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.enableMuseum,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.enableMuseum = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.keepPreviousSearches"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.keepPreviousSearches.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.keepPreviousSearches,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.keepPreviousSearches,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.keepPreviousSearches = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.maxSuggestions"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.maxSuggestions.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.maxSuggestions,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.maxSuggestions,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.maxSuggestions = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(0, 5).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.historyLength"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.historyLength.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.historyLength,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.historyLength,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.historyLength = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(0, 5).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableCommands"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.enableCommands.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.enableCommands,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.enableCommands,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.enableCommands = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.commandAutocomplete"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.searchOverlay.commandAutocomplete.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_8_0)
        //   java: 								.binding(defaults.uiAndVisuals.searchOverlay.commandAutocomplete,
        //   java: 										() -> config.uiAndVisuals.searchOverlay.commandAutocomplete,
        //   java: 										newValue -> config.uiAndVisuals.searchOverlay.commandAutocomplete = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				// Bazaar Quick Quantities
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.bazaarQuickQuantities"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bazaarQuickQuantities.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.bazaarQuickQuantities.enabled.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.bazaarQuickQuantities.enabled,
        //   java: 										() -> config.uiAndVisuals.bazaarQuickQuantities.enabled,
        //   java: 										newValue -> config.uiAndVisuals.bazaarQuickQuantities.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bazaarQuickQuantities.closeSignOnUse"))
        //   java: 								.binding(defaults.uiAndVisuals.bazaarQuickQuantities.closeSignOnUse,
        //   java: 										() -> config.uiAndVisuals.bazaarQuickQuantities.closeSignOnUse,
        //   java: 										newValue -> config.uiAndVisuals.bazaarQuickQuantities.closeSignOnUse = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bazaarQuickQuantities.slotQuantity", 1))
        //   java: 								.binding(defaults.uiAndVisuals.bazaarQuickQuantities.slot1Quantity,
        //   java: 										() -> config.uiAndVisuals.bazaarQuickQuantities.slot1Quantity,
        //   java: 										newValue -> config.uiAndVisuals.bazaarQuickQuantities.slot1Quantity = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(1, 71680).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bazaarQuickQuantities.slotQuantity", 2))
        //   java: 								.binding(defaults.uiAndVisuals.bazaarQuickQuantities.slot2Quantity,
        //   java: 										() -> config.uiAndVisuals.bazaarQuickQuantities.slot2Quantity,
        //   java: 										newValue -> config.uiAndVisuals.bazaarQuickQuantities.slot2Quantity = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(1, 71680).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.bazaarQuickQuantities.slotQuantity", 3))
        //   java: 								.binding(defaults.uiAndVisuals.bazaarQuickQuantities.slot3Quantity,
        //   java: 										() -> config.uiAndVisuals.bazaarQuickQuantities.slot3Quantity,
        //   java: 										newValue -> config.uiAndVisuals.bazaarQuickQuantities.slot3Quantity = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(1, 71680).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Input Calculator
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.inputCalculator"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.inputCalculator.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.inputCalculator.enabled.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.inputCalculator.enabled,
        //   java: 										() -> config.uiAndVisuals.inputCalculator.enabled,
        //   java: 										newValue -> config.uiAndVisuals.inputCalculator.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.inputCalculator.requiresEquals"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.inputCalculator.requiresEquals.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.inputCalculator.requiresEquals,
        //   java: 										() -> config.uiAndVisuals.inputCalculator.requiresEquals,
        //   java: 										newValue -> config.uiAndVisuals.inputCalculator.requiresEquals = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.inputCalculator.closeSignsWithEnter"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.inputCalculator.closeSignsWithEnter.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.inputCalculator.closeSignsWithEnter,
        //   java: 										() -> config.uiAndVisuals.inputCalculator.closeSignsWithEnter,
        //   java: 										newValue -> config.uiAndVisuals.inputCalculator.closeSignsWithEnter = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Flame Overlay
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.flameOverlay"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.flameOverlay.flameHeight"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.flameOverlay.flameHeight.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.flameOverlay.flameHeight,
        //   java: 										() -> config.uiAndVisuals.flameOverlay.flameHeight,
        //   java: 										newValue -> config.uiAndVisuals.flameOverlay.flameHeight = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(0, 100).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.flameOverlay.flameOpacity"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.flameOverlay.flameOpacity.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.flameOverlay.flameOpacity,
        //   java: 										() -> config.uiAndVisuals.flameOverlay.flameOpacity,
        //   java: 										newValue -> config.uiAndVisuals.flameOverlay.flameOpacity = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(0, 100).slider(1).build())
        //   java: 								.build())
        //   java: 						.build())
        //   java:
        //   java: 				//Compact Damage Numbers
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.compactDamage"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.compactDamage.enabled"))
        //   java: 								.binding(defaults.uiAndVisuals.compactDamage.enabled,
        //   java: 										() -> config.uiAndVisuals.compactDamage.enabled,
        //   java: 										newValue -> config.uiAndVisuals.compactDamage.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Integer>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.compactDamage.precision"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.compactDamage.precision.@Tooltip"))
        //   java: 								.tags(CommonTags.ADDED_IN_6_0_0)
        //   java: 								.binding(defaults.uiAndVisuals.compactDamage.maxPrecision,
        //   java: 										() -> config.uiAndVisuals.compactDamage.maxPrecision,
        //   java: 										newValue -> config.uiAndVisuals.compactDamage.maxPrecision = newValue)
        //   java: 								.controller(IntegerController.createBuilder().range(3, 10).slider(1).build())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.compactDamage.normalDamageColor"))
        //   java: 								.binding(defaults.uiAndVisuals.compactDamage.normalDamageColor,
        //   java: 										() -> config.uiAndVisuals.compactDamage.normalDamageColor,
        //   java: 										newValue -> config.uiAndVisuals.compactDamage.normalDamageColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.compactDamage.critDamageGradientStart"))
        //   java: 								.binding(defaults.uiAndVisuals.compactDamage.critDamageGradientStart,
        //   java: 										() -> config.uiAndVisuals.compactDamage.critDamageGradientStart,
        //   java: 										newValue -> config.uiAndVisuals.compactDamage.critDamageGradientStart = newValue)
        //   java: 								.controller(ColourController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.compactDamage.critDamageGradientEnd"))
        //   java: 								.binding(defaults.uiAndVisuals.compactDamage.critDamageGradientEnd,
        //   java: 										() -> config.uiAndVisuals.compactDamage.critDamageGradientEnd,
        //   java: 										newValue -> config.uiAndVisuals.compactDamage.critDamageGradientEnd = newValue)
        //   java: 								.controller(ColourController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.build()
        //   java: 				)
        //   java:
        //   java: 				//Custom Health bars
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.enabled"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.enabled.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.enabled,
        //   java: 										() -> config.uiAndVisuals.healthBars.enabled,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.enabled = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Float>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.scale"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.scale,
        //   java: 										() -> config.uiAndVisuals.healthBars.scale,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.scale = newValue)
        //   java: 								.controller(FloatController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.removeHealthFromName"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.removeHealthFromName.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.removeHealthFromName,
        //   java: 										() -> config.uiAndVisuals.healthBars.removeHealthFromName,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.removeHealthFromName = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.removeMaxHealthFromName"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.removeMaxHealthFromName.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.removeMaxHealthFromName,
        //   java: 										() -> config.uiAndVisuals.healthBars.removeMaxHealthFromName,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.removeMaxHealthFromName = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.applyToHealthOnlyMobs"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.applyToHealthOnlyMobs.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.applyToHealthOnlyMobs,
        //   java: 										() -> config.uiAndVisuals.healthBars.applyToHealthOnlyMobs,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.applyToHealthOnlyMobs = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Boolean>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.hideFullHealth"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.hideFullHealth.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.hideFullHealth,
        //   java: 										() -> config.uiAndVisuals.healthBars.hideFullHealth,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.hideFullHealth = newValue)
        //   java: 								.controller(ConfigUtils.createBooleanController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.fullBarColor"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.fullBarColor.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.fullBarColor,
        //   java: 										() -> config.uiAndVisuals.healthBars.fullBarColor,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.fullBarColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.halfBarColor"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.halfBarColor.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.halfBarColor,
        //   java: 										() -> config.uiAndVisuals.healthBars.halfBarColor,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.halfBarColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.emptyBarColor"))
        //   java: 								.description(Component.translatable("skyblocker.config.uiAndVisuals.healthBars.emptyBarColor.@Tooltip"))
        //   java: 								.binding(defaults.uiAndVisuals.healthBars.emptyBarColor,
        //   java: 										() -> config.uiAndVisuals.healthBars.emptyBarColor,
        //   java: 										newValue -> config.uiAndVisuals.healthBars.emptyBarColor = newValue)
        //   java: 								.controller(ColourController.createBuilder().build())
        //   java: 								.build())
        //   java: 						.build()
        //   java: 				)
        //   java: 				//Gyro Overlay
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.gyroOverlay"))
        //   java: 						.collapsed(true)
        //   java: 						.option(Option.<GyroOverlay.Mode>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.gyroOverlay.modeSelect"))
        //   java: 								.binding(defaults.uiAndVisuals.gyroOverlay.gyroOverlayMode,
        //   java: 										() -> config.uiAndVisuals.gyroOverlay.gyroOverlayMode,
        //   java: 										newValue -> config.uiAndVisuals.gyroOverlay.gyroOverlayMode = newValue)
        //   java: 								.controller(ConfigUtils.createEnumController())
        //   java: 								.build())
        //   java: 						.option(Option.<Color>createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.gyroOverlay.Color"))
        //   java: 								.binding(defaults.uiAndVisuals.gyroOverlay.gyroOverlayColor,
        //   java: 										() -> config.uiAndVisuals.gyroOverlay.gyroOverlayColor,
        //   java: 										newValue -> {
        //   java: 											config.uiAndVisuals.gyroOverlay.gyroOverlayColor = newValue;
        //   java: 											GyroOverlay.configCallback(newValue);
        //   java: 										})
        //   java: 								.controller(ColourController.createBuilder().hasAlpha(true).build())
        //   java: 								.build())
        //   java: 						.build()
        //   java: 				)
        //   java:
        //   java: 				//item pickup widget
        //   java: 				.group(OptionGroup.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.uiAndVisuals.itemPickup"))
        //   java: 						.collapsed(true)
        //   java: 						.option(ButtonOption.createBuilder()
        //   java: 								.name(Component.translatable("skyblocker.config.uiAndVisuals.itemPickup.hud.screen"))
        //   java: 								.description(Component.translatable("skyblocker.config.hud.movedMessage"))
        //   java: 								.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 								.action(screen -> Minecraft.getInstance().gui.setScreen(new WidgetsConfigurationScreen(Location.HUB, screen)))
        //   java: 								.build())
        //   java: 						.build()
        //   java: 				)
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static List<Option<Boolean>> createSlotTextToggles(SkyblockerConfig config)
    pub fn create_slot_text_toggles(config: &SkyblockerConfig) -> Vec<crate::unresolved::Option> {
        // TODO(fabric2seer): `.name(..)` on a Option
        //   java: return SlotTextManager.getAdderStream().map(SlotTextAdder::getConfigInformation).filter(Objects::nonNull).distinct()
        //   java: 				.map(configInfo -> configInfo.getOption(config))
        //   java: 				.sorted(Comparator.comparing(option -> option.name().getString())).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static List<Option<Boolean>> createRadialToggles(SkyblockerConfig config)
    pub fn create_radial_toggles(config: &SkyblockerConfig) -> Vec<crate::unresolved::Option> {
        // TODO(fabric2seer): not a lambda
        //   java: return RadialMenuManager.getMenuStream().filter(distinctByKey(RadialMenu::getConfigId))
        //   java: 				.map(configInfo -> configInfo.getOption(config))
        //   java: 				.sorted(Comparator.comparing(option -> option.name().getString())).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <T> Predicate<T> distinctByKey(Function<? super T, ?> keyExtractor)
    pub fn distinct_by_key<T: Clone + 'static>(key_extractor: Arc<dyn Fn(T) -> compat::Object>) -> Arc<dyn Fn(T) -> bool> {
        // TODO(fabric2seer): static call ConcurrentHashMap.newKeySet(..)
        //   java: Set<Object> seen = ConcurrentHashMap.newKeySet();
        let mut seen: std::collections::HashSet<compat::Object> = todo!("fabric2seer: the initialiser of `seen` (TODO above)");
        return Arc::new(move |t| seen.insert((key_extractor)(t.clone())));
    }

}
