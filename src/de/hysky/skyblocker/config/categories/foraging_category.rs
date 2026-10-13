//! Ported from de.hysky.skyblocker.config.categories.ForagingCategory.
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

/// java: public class ForagingCategory
#[derive(Clone)]
pub struct ForagingCategory {}

impl ForagingCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/foraging"))
            .name(component::translatable("skyblocker.config.foraging"))
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.foraging.enableTreeFelledNotification"))
                .description(vec![component::translatable("skyblocker.config.foraging.enableTreeFelledNotification.@Tooltip")])
                .tags(vec![common_tags::ADDED_IN_6_9_1.get().clone()])
                .binding(compat::read(&defaults, |c| c.foraging.enable_tree_felled_notification.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.enable_tree_felled_notification.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.enable_tree_felled_notification = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.foraging.moongladeMarsh"))
                .collapsed(false)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.moongladeMarsh.solveForestTemplePuzzle"))
                    .description(vec![component::translatable("skyblocker.config.foraging.moongladeMarsh.solveForestTemplePuzzle.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.foraging.moonglade_marsh.solve_forest_temple_puzzle.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.moonglade_marsh.solve_forest_temple_puzzle.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.moonglade_marsh.solve_forest_temple_puzzle = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.moongladeMarsh.enableLushlilacHighlighter"))
                    .description(vec![component::translatable("skyblocker.config.foraging.moongladeMarsh.enableLushlilacHighlighter.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.foraging.moonglade_marsh.enable_lushlilac_highlighter.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.moonglade_marsh.enable_lushlilac_highlighter.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.moonglade_marsh.enable_lushlilac_highlighter = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.moongladeMarsh.enableSeaLumiesHighlighter"))
                    .description(vec![component::translatable("skyblocker.config.foraging.moongladeMarsh.enableSeaLumiesHighlighter.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.foraging.moonglade_marsh.enable_sea_lumies_highlighter.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.moonglade_marsh.enable_sea_lumies_highlighter.clone()) }, { let config = config.clone(); move |new_value: bool| { compat::write(&config, |c| c.foraging.moonglade_marsh.enable_sea_lumies_highlighter = new_value); crate::de::hysky::skyblocker::skyblock::foraging::galatea::sea_lumies_highlighter::SeaLumiesHighlighter::config_callback(crate::de::hysky::skyblocker::skyblock::foraging::galatea::sea_lumies_highlighter::INSTANCE.get()); } })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<i32>()
                    .name(component::translatable("skyblocker.config.foraging.moongladeMarsh.seaLumieMinCount"))
                    .description(vec![component::translatable("skyblocker.config.foraging.moongladeMarsh.seaLumieMinCount.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.foraging.moonglade_marsh.sea_lumies_minimum_count.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.moonglade_marsh.sea_lumies_minimum_count.clone()) }, { let config = config.clone(); move |new_value: i32| { compat::write(&config, |c| c.foraging.moonglade_marsh.sea_lumies_minimum_count = new_value); crate::de::hysky::skyblocker::skyblock::foraging::galatea::sea_lumies_highlighter::SeaLumiesHighlighter::config_callback(crate::de::hysky::skyblocker::skyblock::foraging::galatea::sea_lumies_highlighter::INSTANCE.get()); } })
                    .controller(integer_controller::create_builder()
                        .range(1, 4)
                        .slider(1)
                        .build())
                    .build())
                .option(button_option::create_builder()
                    .name(component::translatable("skyblocker.config.foraging.moongladeMarsh.enableTreeBreakHud"))
                    .description(vec![component::translatable("skyblocker.config.hud.movedMessage")])
                    .prompt(component::translatable("text.skyblocker.open"))
                    .action(|_screen| mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen::new_2(crate::de::hysky::skyblocker::utils::location::Location::Galatea, None)))))
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.moongladeMarsh.enableTunerSolver"))
                    .description(vec![component::translatable("skyblocker.config.foraging.moongladeMarsh.enableTunerSolver.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.foraging.moonglade_marsh.enable_tuner_solver.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.moonglade_marsh.enable_tuner_solver.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.moonglade_marsh.enable_tuner_solver = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(button_option::create_builder()
                    .name(component::translatable("skyblocker.config.foraging.moongladeMarsh.enableSweepDetailsWidget"))
                    .description(vec![component::translatable("skyblocker.config.hud.movedMessage"), component::translatable("skyblocker.config.foraging.moongladeMarsh.enableSweepDetailsWidget.@Tooltip")])
                    .action(|_screen| mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::tabhud::config::widgets_configuration_screen::WidgetsConfigurationScreen::new_2(crate::de::hysky::skyblocker::utils::location::Location::Galatea, None)))))
                    .prompt(component::translatable("text.skyblocker.open"))
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.foraging.torrhusCanyon"))
                .collapsed(false)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.torrhusCanyon.solveDesertTemplePuzzles"))
                    .description(vec![component::translatable("skyblocker.config.foraging.torrhusCanyon.solveDesertTemplePuzzles.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_9_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.foraging.torrhus_canyon.solve_desert_temple_puzzles.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.torrhus_canyon.solve_desert_temple_puzzles.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.torrhus_canyon.solve_desert_temple_puzzles = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.torrhusCanyon.enableRubyVeilshroomHighlighter"))
                    .description(vec![component::translatable("skyblocker.config.foraging.torrhusCanyon.enableRubyVeilshroomHighlighter.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_9_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.foraging.torrhus_canyon.enable_ruby_veilshroom_highlighter.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.torrhus_canyon.enable_ruby_veilshroom_highlighter.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.torrhus_canyon.enable_ruby_veilshroom_highlighter = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.torrhusCanyon.enableHoneyhiveHighlighter"))
                    .description(vec![component::translatable("skyblocker.config.foraging.torrhusCanyon.enableHoneyhiveHighlighter.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_9_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.foraging.torrhus_canyon.enable_honeyhive_highlighter.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.torrhus_canyon.enable_honeyhive_highlighter.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.torrhus_canyon.enable_honeyhive_highlighter = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.foraging.sweepOverlay"))
                .collapsed(true)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.sweepOverlay.enableSweepOverlay"))
                    .description(vec![component::translatable("skyblocker.config.foraging.sweepOverlay.enableSweepOverlay.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.foraging.sweep_overlay.enable_sweep_overlay.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.sweep_overlay.enable_sweep_overlay.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.sweep_overlay.enable_sweep_overlay = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.foraging.sweepOverlay.enableThrownAbilityOverlay"))
                    .description(vec![component::translatable("skyblocker.config.foraging.sweepOverlay.enableThrownAbilityOverlay.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.foraging.sweep_overlay.enable_thrown_ability_overlay.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.sweep_overlay.enable_thrown_ability_overlay.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.foraging.sweep_overlay.enable_thrown_ability_overlay = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<Color>()
                    .name(component::translatable("skyblocker.config.foraging.sweepOverlay.sweepOverlayColor"))
                    .binding(compat::read(&defaults, |c| c.foraging.sweep_overlay.sweep_overlay_color.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.foraging.sweep_overlay.sweep_overlay_color.clone()) }, { let config = config.clone(); move |new_value: Color| { compat::write(&config, |c| c.foraging.sweep_overlay.sweep_overlay_color = new_value.clone()); crate::de::hysky::skyblocker::skyblock::foraging::sweep_overlay::SweepOverlay::config_callback(&new_value); } })
                    .controller(colour_controller::create_builder()
                        .has_alpha(true)
                        .build())
                    .build())
                .build())
            .build();
    }
}
