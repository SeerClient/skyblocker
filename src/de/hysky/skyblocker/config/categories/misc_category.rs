//! Ported from de.hysky.skyblocker.config.categories.MiscCategory.
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

/// java: public class MiscCategory
#[derive(Clone)]
pub struct MiscCategory {}

impl MiscCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/misc"))
            .name(component::translatable("skyblocker.config.misc"))
            .option(option::create_builder::<ConfigType>()
                .name(component::translatable("skyblocker.config.misc.configBackend"))
                .description(vec![component::translatable("skyblocker.config.misc.configBackend.@Tooltip")])
                .binding(compat::read(&defaults, |c| c.misc.config_backend.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.misc.config_backend.clone()) }, { let config = config.clone(); move |new_value: ConfigType| compat::write(&config, |c| c.misc.config_backend = new_value) })
                .controller(ConfigUtils::create_enum_controller::<ConfigType>())
                .build())
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.misc.cat"))
                .description(vec![component::translatable("skyblocker.config.misc.cat.@Tooltip")])
                .binding(compat::read(&defaults, |c| c.misc.cat.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.misc.cat.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.misc.cat = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.misc.richPresence"))
                .collapsed(true)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.misc.richPresence.enableRichPresence"))
                    .binding(compat::read(&defaults, |c| c.misc.rich_presence.enable_rich_presence.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.misc.rich_presence.enable_rich_presence.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.misc.rich_presence.enable_rich_presence = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::config::configs::misc_config::Info>()
                    .name(component::translatable("skyblocker.config.misc.richPresence.info"))
                    .description(vec![component::translatable("skyblocker.config.misc.richPresence.info.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.misc.rich_presence.info.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.misc.rich_presence.info.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::config::configs::misc_config::Info| compat::write(&config, |c| c.misc.rich_presence.info = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::config::configs::misc_config::Info>())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.misc.richPresence.cycleMode"))
                    .binding(compat::read(&defaults, |c| c.misc.rich_presence.cycle_mode.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.misc.rich_presence.cycle_mode.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.misc.rich_presence.cycle_mode = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<String>()
                    .name(component::translatable("skyblocker.config.misc.richPresence.customMessage"))
                    .binding(compat::read(&defaults, |c| c.misc.rich_presence.custom_message.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.misc.rich_presence.custom_message.clone()) }, { let config = config.clone(); move |new_value: String| compat::write(&config, |c| c.misc.rich_presence.custom_message = new_value) })
                    .controller(string_controller::create_builder()
                        .build())
                    .build())
                .build())
            .build();
    }
}
