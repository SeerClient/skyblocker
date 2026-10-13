//! Ported from de.hysky.skyblocker.config.categories.FarmingCategory.
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

/// java: public class FarmingCategory
#[derive(Clone)]
pub struct FarmingCategory {}

impl FarmingCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/farming"))
            .name(component::translatable("skyblocker.config.farming"))
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.farming.farmingHud"))
                .collapsed(true)
                .option(button_option::create_builder()
                    .name(component::translatable("skyblocker.config.farming.farmingHud.config"))
                    .description(vec![component::translatable("skyblocker.config.hud.movedMessage")])
                    .prompt(component::translatable("text.skyblocker.open"))
                    .action(|_screen| mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen::new_2(crate::de::hysky::skyblocker::utils::location::Location::Garden, None)))))
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.farming.pestHighlighter"))
                .collapsed(true)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.pestHighlighter.enabled"))
                    .description(vec![component::translatable("skyblocker.config.farming.pestHighlighter.enabled.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.farming.pest_highlighter.enabled.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.pest_highlighter.enabled.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.pest_highlighter.enabled = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.pestHighlighter.contestHighlighter"))
                    .description(vec![component::translatable("skyblocker.config.farming.pestHighlighter.contestHighlighter.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_5_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.farming.pest_highlighter.contest_highlighter.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.pest_highlighter.contest_highlighter.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.pest_highlighter.contest_highlighter = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.pestHighlighter.vinylHighlighter"))
                    .description(vec![component::translatable("skyblocker.config.farming.pestHighlighter.vinylHighlighter.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_2_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.farming.pest_highlighter.vinyl_highlighter.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.pest_highlighter.vinyl_highlighter.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.pest_highlighter.vinyl_highlighter = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.pestHighlighter.stereoHarmony"))
                    .description(vec![component::translatable("skyblocker.config.farming.pestHighlighter.stereoHarmony.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.farming.pest_highlighter.enable_stereo_harmony_helper_for_contest.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.pest_highlighter.enable_stereo_harmony_helper_for_contest.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.pest_highlighter.enable_stereo_harmony_helper_for_contest = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.farming.mouseLock"))
                .collapsed(true)
                .tags(vec![component::literal("camera lock"), component::literal("sensitivity"), component::literal("yaw pitch")])
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.mouseLock.lockMouseTool"))
                    .binding(compat::read(&defaults, |c| c.farming.mouse_lock.lock_mouse_tool.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.mouse_lock.lock_mouse_tool.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.mouse_lock.lock_mouse_tool = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.mouseLock.lockMouseGround"))
                    .binding(compat::read(&defaults, |c| c.farming.mouse_lock.lock_mouse_ground_only.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.mouse_lock.lock_mouse_ground_only.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.mouse_lock.lock_mouse_ground_only = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.farming.plotsWidget"))
                .collapsed(true)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.plotsWidget.enabled"))
                    .description(vec![component::translatable("skyblocker.config.farming.plotsWidget.enabled.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.farming.plots_widget.enabled.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.plots_widget.enabled.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.plots_widget.enabled = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.plotsWidget.closeScreenOnPlotClick"))
                    .description(vec![component::translatable("skyblocker.config.farming.plotsWidget.closeScreenOnPlotClick.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.farming.plots_widget.close_screen_on_plot_click.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.plots_widget.close_screen_on_plot_click.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.plots_widget.close_screen_on_plot_click = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.farming.visitorHelper"))
                .collapsed(true)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.visitorHelper.enabled"))
                    .description(vec![component::translatable("skyblocker.config.farming.visitorHelper.enabled.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.farming.visitor_helper.enabled.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.visitor_helper.enabled.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.visitor_helper.enabled = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.visitorHelper.showInGardenOnly"))
                    .description(vec![component::translatable("skyblocker.config.farming.visitorHelper.showInGardenOnly.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.farming.visitor_helper.show_in_garden_only.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.visitor_helper.show_in_garden_only.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.visitor_helper.show_in_garden_only = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.visitorHelper.showInStacks"))
                    .description(vec![component::translatable("skyblocker.config.farming.visitorHelper.showInStacks.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.farming.visitor_helper.show_in_stacks.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.visitor_helper.show_in_stacks.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.visitor_helper.show_in_stacks = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.farming.greenhouse"))
                .collapsed(true)
                .tags(vec![common_tags::ADDED_IN_6_5_0.get().clone()])
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.greenhouse.greenhousePaste.enabled"))
                    .description(vec![component::translatable("skyblocker.config.farming.greenhouse.greenhousePaste.enabled.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_5_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.farming.greenhouse.enabled.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.greenhouse.enabled.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.greenhouse.enabled = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(button_option::create_builder()
                    .name(component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets"))
                    .prompt(component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.@Prompt"))
                    .description(vec![component::translatable("skyblocker.config.farming.greenhouse.greenhousePresets.@Tooltip")])
                    .action(|_screen| mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::config::screens::greenhouse::greenhouse_presets_screen::GreenhousePresetsScreen::new(None)))))
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.farming.greenhouse.showMutationSlot.enabled"))
                    .description(vec![component::translatable("skyblocker.config.farming.greenhouse.showMutationSlot.enabled.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_5_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.farming.greenhouse.show_mutation_slot.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.farming.greenhouse.show_mutation_slot.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.farming.greenhouse.show_mutation_slot = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .build();
    }
}
