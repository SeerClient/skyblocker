//! Ported from de.hysky.skyblocker.config.categories.QuickNavigationCategory.
//!
//! The category is built on Dandelion (`dandelion` crate, rendered through YACL). Java hands `create` the two
//! `SkyblockerConfig` objects; here they are the manager's shared instances (`compat::Shared`), and every option's
//! binding reads / writes its field through them, borrowing per statement.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::categories::{ChatFormattingEnum, KeyMappingNullable as _};
use crate::de::hysky::skyblocker::config::common_tags;
use crate::de::hysky::skyblocker::config::config_utils::ConfigUtils;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;
use crate::entry::skyblocker_mod;
use dandelion::net::azureaaron::dandelion::api::button_option::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::config_category::{self, Builder as _, ConfigCategory};
use dandelion::net::azureaaron::dandelion::api::config_type::ConfigType;
use dandelion::net::azureaaron::dandelion::api::controllers::boolean_controller::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::controllers::colour_controller::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::controllers::controller::Builder as _;
use dandelion::net::azureaaron::dandelion::api::controllers::enum_controller::Builder as _;
use dandelion::net::azureaaron::dandelion::api::controllers::float_controller;
use dandelion::net::azureaaron::dandelion::api::controllers::integer_controller;
use dandelion::net::azureaaron::dandelion::api::controllers::item_controller;
use dandelion::net::azureaaron::dandelion::api::controllers::number_controller::Builder as _;
use dandelion::net::azureaaron::dandelion::api::controllers::string_controller;
use dandelion::net::azureaaron::dandelion::api::key_mapping_option::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::label_option::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::list_option::{self, Builder as _};
use dandelion::net::azureaaron::dandelion::api::option::{self, AnyOption, Builder as _};
use dandelion::net::azureaaron::dandelion::api::option_adder::OptionAdder as _;
use dandelion::net::azureaaron::dandelion::api::option_group::{self, Builder as _, OptionGroup};
use dandelion::net::azureaaron::dandelion::api::option_listener::UpdateType;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::item::Item;
use crate::de::hysky::skyblocker::config::configs::quick_navigation_config::QuickNavItem;

/// java: public class QuickNavigationCategory
#[derive(Clone)]
pub struct QuickNavigationCategory {}

impl QuickNavigationCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/quicknav"))
            .name(component::translatable("skyblocker.config.quickNav"))
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.quickNav.enableQuickNav"))
                .description(vec![component::translatable("skyblocker.config.quickNav.enableQuickNav.@Tooltip")])
                .binding(compat::read(&defaults, |c| c.quick_nav.enable_quick_nav.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.quick_nav.enable_quick_nav.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.quick_nav.enable_quick_nav = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button1.clone()), compat::read(&config, |c| c.quick_nav.button1.clone()), 1))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button2.clone()), compat::read(&config, |c| c.quick_nav.button2.clone()), 2))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button3.clone()), compat::read(&config, |c| c.quick_nav.button3.clone()), 3))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button4.clone()), compat::read(&config, |c| c.quick_nav.button4.clone()), 4))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button5.clone()), compat::read(&config, |c| c.quick_nav.button5.clone()), 5))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button6.clone()), compat::read(&config, |c| c.quick_nav.button6.clone()), 6))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button7.clone()), compat::read(&config, |c| c.quick_nav.button7.clone()), 7))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button8.clone()), compat::read(&config, |c| c.quick_nav.button8.clone()), 8))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button9.clone()), compat::read(&config, |c| c.quick_nav.button9.clone()), 9))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button10.clone()), compat::read(&config, |c| c.quick_nav.button10.clone()), 10))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button11.clone()), compat::read(&config, |c| c.quick_nav.button11.clone()), 11))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button12.clone()), compat::read(&config, |c| c.quick_nav.button12.clone()), 12))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button13.clone()), compat::read(&config, |c| c.quick_nav.button13.clone()), 13))
            .group(QuickNavigationCategory::quick_nav_button(compat::read(&defaults, |c| c.quick_nav.button14.clone()), compat::read(&config, |c| c.quick_nav.button14.clone()), 14))
            .build();
    }

    /// java: private static OptionGroup quickNavButton(QuickNavigationConfig.QuickNavItem defaultButton, QuickNavigationConfig.QuickNavItem button, int index)
    ///
    /// The two buttons are the config objects themselves (`compat::Shared`), as Java hands them over; every binding
    /// reads and writes the button it was given. `itemData` is never null for a button the config holds (only the
    /// Gson no-argument constructor leaves it null, and Gson then fills it); a null one reads as air / empty and drops
    /// a write, where Java would throw.
    pub fn quick_nav_button(default_button: compat::Shared<QuickNavItem>, button: compat::Shared<QuickNavItem>, index: i32) -> Arc<dyn OptionGroup> {
        let item_of = |b: &QuickNavItem| -> dandelion::compat::JavaRef<Item> {
            dandelion::compat::JavaRef(b.item_data.as_ref().and_then(|d| d.item.clone()).unwrap_or_else(|| mcrust::net::minecraft::world::item::items::items().get("air")))
        };
        return option_group::create_builder()
            .name(component::translatable_args("skyblocker.config.quickNav.button", vec![FormattedText::of(index.to_string())]))
            .collapsed(true)
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.quickNav.button.render"))
                .binding(compat::read(&default_button, |b| b.render), { let button = button.clone(); move || compat::read(&button, |b| b.render) }, { let button = button.clone(); move |new_value: bool| compat::write(&button, |b| b.render = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.quickNav.button.doubleClick"))
                .binding(compat::read(&default_button, |b| b.double_click), { let button = button.clone(); move || compat::read(&button, |b| b.double_click) }, { let button = button.clone(); move |new_value: bool| compat::write(&button, |b| b.double_click = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .option_if(mcrust::mcrust::modding::game::level().is_some(), button_option::create_builder()
                .name(component::translatable("skyblocker.config.quickNav.button.chooseSkyblockItem"))
                .description(vec![component::translatable("skyblocker.config.quickNav.button.chooseSkyblockItem.@Tooltip")])
                .action({
                    let button = button.clone();
                    move |_screen| {
                        // java: Minecraft.getInstance().gui.setScreen(new ItemSelectionPopup(screen, item -> {..}))
                        let button = button.clone();
                        let popup = crate::de::hysky::skyblocker::utils::render::gui::item_selection_popup::ItemSelectionPopup::new(
                            mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
                            Arc::new(move |item: std::option::Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>| {
                                // java: if (item == null) return;
                                let Some(mut item) = item else { return; };
                                // java: button.itemData.item = item.getItem();
                                //       button.itemData.components = ItemStackComponentizationFixer.componentsAsString(item);
                                let new_item = item.get_item();
                                let components = crate::de::hysky::skyblocker::utils::datafixer::item_stack_componentization_fixer::ItemStackComponentizationFixer::components_as_string(&mut item);
                                compat::write(&button, |b| {
                                    if let Some(item_data) = b.item_data.as_mut() {
                                        item_data.item = Some(new_item);
                                        item_data.components = components;
                                    }
                                });
                            }),
                        );
                        mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(popup)));
                    }
                })
                .prompt(component::translatable("text.skyblocker.open"))
                .build())
            .option(option::create_builder::<dandelion::compat::JavaRef<Item>>()
                .name(component::translatable("skyblocker.config.quickNav.button.item.itemName"))
                .binding(compat::read(&default_button, |b| item_of(b)), { let button = button.clone(); move || compat::read(&button, |b| item_of(b)) }, { let button = button.clone(); move |new_value: dandelion::compat::JavaRef<Item>| compat::write(&button, |b| {
                    if let Some(item_data) = b.item_data.as_mut() {
                        item_data.item = Some(new_value.0);
                    }
                }) })
                .controller(item_controller::create_builder().build())
                .build())
            .option(option::create_builder::<i32>()
                .name(component::translatable("skyblocker.config.quickNav.button.item.count"))
                .binding(compat::read(&default_button, |b| b.item_data.as_ref().map(|d| d.count).unwrap_or(1)), { let button = button.clone(); move || compat::read(&button, |b| b.item_data.as_ref().map(|d| d.count).unwrap_or(1)) }, { let button = button.clone(); move |new_value: i32| compat::write(&button, |b| {
                    if let Some(item_data) = b.item_data.as_mut() {
                        item_data.count = new_value;
                    }
                }) })
                .controller(integer_controller::create_builder().range(1, 99).build())
                .build())
            .option(option::create_builder::<String>()
                .name(component::translatable("skyblocker.config.quickNav.button.item.components"))
                .description(vec![component::translatable("skyblocker.config.quickNav.button.item.components.@Tooltip")])
                .binding(compat::read(&default_button, |b| b.item_data.as_ref().map(|d| d.components.clone()).unwrap_or_default()), { let button = button.clone(); move || compat::read(&button, |b| b.item_data.as_ref().map(|d| d.components.clone()).unwrap_or_default()) }, { let button = button.clone(); move |new_value: String| compat::write(&button, |b| {
                    if let Some(item_data) = b.item_data.as_mut() {
                        item_data.components = new_value;
                    }
                }) })
                .controller(string_controller::create_builder().build())
                .build())
            .option(option::create_builder::<String>()
                .name(component::translatable("skyblocker.config.quickNav.button.uiTitle"))
                .binding(compat::read(&default_button, |b| b.ui_title.clone()), { let button = button.clone(); move || compat::read(&button, |b| b.ui_title.clone()) }, { let button = button.clone(); move |new_value: String| compat::write(&button, |b| b.ui_title = new_value) })
                .controller(string_controller::create_builder().build())
                .build())
            .option(option::create_builder::<String>()
                .name(component::translatable("skyblocker.config.quickNav.button.tooltip"))
                .description(vec![component::translatable("skyblocker.config.quickNav.button.tooltip.@Tooltip")])
                .binding(compat::read(&default_button, |b| b.tooltip.clone()), { let button = button.clone(); move || compat::read(&button, |b| b.tooltip.clone()) }, { let button = button.clone(); move |new_value: String| compat::write(&button, |b| b.tooltip = new_value) })
                .controller(string_controller::create_builder().build())
                .build())
            .option(option::create_builder::<String>()
                .name(component::translatable("skyblocker.config.quickNav.button.clickEvent"))
                .binding(compat::read(&default_button, |b| b.click_event.clone()), { let button = button.clone(); move || compat::read(&button, |b| b.click_event.clone()) }, { let button = button.clone(); move |new_value: String| compat::write(&button, |b| b.click_event = new_value) })
                .controller(string_controller::create_builder().build())
                .build())
            .build();
    }
}
