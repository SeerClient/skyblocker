//! Ported from de.hysky.skyblocker.config.categories.QuickNavigationCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::quick_navigation_config::QuickNavItem;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: public class QuickNavigationCategory
#[derive(Clone)]
pub struct QuickNavigationCategory {}

impl QuickNavigationCategory {
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
        //   java: 				.id(SkyblockerMod.id("config/quicknav"))
        //   java: 				.name(Component.translatable("skyblocker.config.quickNav"))
        //   java:
        //   java: 				//Toggle
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.enableQuickNav"))
        //   java: 						.description(Component.translatable("skyblocker.config.quickNav.enableQuickNav.@Tooltip"))
        //   java: 						.binding(defaults.quickNav.enableQuickNav,
        //   java: 								() -> config.quickNav.enableQuickNav,
        //   java: 								newValue -> config.quickNav.enableQuickNav = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java:
        //   java: 				//Buttons
        //   java: 				.group(quickNavButton(defaults.quickNav.button1, config.quickNav.button1, 1))
        //   java: 				.group(quickNavButton(defaults.quickNav.button2, config.quickNav.button2, 2))
        //   java: 				.group(quickNavButton(defaults.quickNav.button3, config.quickNav.button3, 3))
        //   java: 				.group(quickNavButton(defaults.quickNav.button4, config.quickNav.button4, 4))
        //   java: 				.group(quickNavButton(defaults.quickNav.button5, config.quickNav.button5, 5))
        //   java: 				.group(quickNavButton(defaults.quickNav.button6, config.quickNav.button6, 6))
        //   java: 				.group(quickNavButton(defaults.quickNav.button7, config.quickNav.button7, 7))
        //   java: 				.group(quickNavButton(defaults.quickNav.button8, config.quickNav.button8, 8))
        //   java: 				.group(quickNavButton(defaults.quickNav.button9, config.quickNav.button9, 9))
        //   java: 				.group(quickNavButton(defaults.quickNav.button10, config.quickNav.button10, 10))
        //   java: 				.group(quickNavButton(defaults.quickNav.button11, config.quickNav.button11, 11))
        //   java: 				.group(quickNavButton(defaults.quickNav.button12, config.quickNav.button12, 12))
        //   java: 				.group(quickNavButton(defaults.quickNav.button13, config.quickNav.button13, 13))
        //   java: 				.group(quickNavButton(defaults.quickNav.button14, config.quickNav.button14, 14))
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static OptionGroup quickNavButton(QuickNavigationConfig.QuickNavItem defaultButton, QuickNavigationConfig.QuickNavItem button, int index)
    pub fn quick_nav_button(default_button: &compat::Shared<QuickNavItem>, button: &compat::Shared<QuickNavItem>, mut index: i32) -> crate::unresolved::OptionGroup {
        // TODO(fabric2seer): static call OptionGroup.createBuilder(..)
        //   java: return OptionGroup.createBuilder()
        //   java: 				.name(Component.translatable("skyblocker.config.quickNav.button", index))
        //   java: 				.collapsed(true)
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.render"))
        //   java: 						.binding(defaultButton.render,
        //   java: 								() -> button.render,
        //   java: 								newValue -> button.render = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.option(Option.<Boolean>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.doubleClick"))
        //   java: 						.binding(defaultButton.doubleClick,
        //   java: 								() -> button.doubleClick,
        //   java: 								newValue -> button.doubleClick = newValue)
        //   java: 						.controller(ConfigUtils.createBooleanController())
        //   java: 						.build())
        //   java: 				.optionIf(Minecraft.getInstance().level != null, ButtonOption.createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.chooseSkyblockItem"))
        //   java: 						.description(Component.translatable("skyblocker.config.quickNav.button.chooseSkyblockItem.@Tooltip"))
        //   java: 						.action(screen -> Minecraft.getInstance().gui.setScreen(new ItemSelectionPopup(screen, item -> {
        //   java: 							if (item == null) return;
        //   java: 							button.itemData.item = item.getItem();
        //   java: 							button.itemData.components = ItemStackComponentizationFixer.componentsAsString(item);
        //   java: 						})))
        //   java: 						.prompt(Component.translatable("text.skyblocker.open"))
        //   java: 						.build())
        //   java: 				.option(Option.<Item>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.item.itemName"))
        //   java: 						.binding(defaultButton.itemData.item,
        //   java: 								() -> button.itemData.item,
        //   java: 								newValue -> button.itemData.item = newValue)
        //   java: 						.controller(ItemController.createBuilder().build())
        //   java: 						.build())
        //   java: 				.option(Option.<Integer>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.item.count"))
        //   java: 						.binding(defaultButton.itemData.count,
        //   java: 								() -> button.itemData.count,
        //   java: 								newValue -> button.itemData.count = newValue)
        //   java: 						.controller(IntegerController.createBuilder().range(1, 99).build())
        //   java: 						.build())
        //   java: 				.option(Option.<String>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.item.components"))
        //   java: 						.description(Component.translatable("skyblocker.config.quickNav.button.item.components.@Tooltip"))
        //   java: 						.binding(defaultButton.itemData.components,
        //   java: 								() -> button.itemData.components,
        //   java: 								newValue -> button.itemData.components = newValue)
        //   java: 						.controller(StringController.createBuilder().build())
        //   java: 						.build())
        //   java: 				.option(Option.<String>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.uiTitle"))
        //   java: 						.binding(defaultButton.uiTitle,
        //   java: 								() -> button.uiTitle,
        //   java: 								newValue -> button.uiTitle = newValue)
        //   java: 						.controller(StringController.createBuilder().build())
        //   java: 						.build())
        //   java: 				.option(Option.<String>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.tooltip"))
        //   java: 						.description(Component.translatable("skyblocker.config.quickNav.button.tooltip.@Tooltip"))
        //   java: 						.binding(defaultButton.tooltip,
        //   java: 								() -> button.tooltip,
        //   java: 								newValue -> button.tooltip = newValue)
        //   java: 						.controller(StringController.createBuilder().build())
        //   java: 						.build())
        //   java: 				.option(Option.<String>createBuilder()
        //   java: 						.name(Component.translatable("skyblocker.config.quickNav.button.clickEvent"))
        //   java: 						.binding(defaultButton.clickEvent,
        //   java: 								() -> button.clickEvent,
        //   java: 								newValue -> button.clickEvent = newValue)
        //   java: 						.controller(StringController.createBuilder().build())
        //   java: 						.build())
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
