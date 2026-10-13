//! Ported from de.hysky.skyblocker.config.categories.DebugCategory.
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

/// java: public class DebugCategory
#[derive(Clone)]
pub struct DebugCategory {}

impl DebugCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/debug"))
            .name(component::translatable("skyblocker.config.debug"))
            .option_if_boolean_supplier(crate::de::hysky::skyblocker::debug::debug::DUMP_HOVERED_ITEM_KEY.get().is_some(), || key_mapping_option::create_builder()
                .name(component::translatable("key.skyblocker.debug.dumpHoveredItem"))
                .tags(common_tags::KEY_MAPPING.get().clone())
                .key_mapping_nullable(crate::de::hysky::skyblocker::debug::debug::DUMP_HOVERED_ITEM_KEY.get().clone())
                .build() as Arc<dyn AnyOption>)
            .option_if_boolean_supplier(crate::de::hysky::skyblocker::debug::debug::DUMP_NEARBY_ENTITIES_KEY.get().is_some(), || key_mapping_option::create_builder()
                .name(component::translatable("key.skyblocker.debug.dumpNearbyEntities"))
                .tags(common_tags::KEY_MAPPING.get().clone())
                .key_mapping_nullable(crate::de::hysky::skyblocker::debug::debug::DUMP_NEARBY_ENTITIES_KEY.get().clone())
                .build() as Arc<dyn AnyOption>)
            .option(option::create_builder::<i32>()
                .name(component::translatable("skyblocker.config.debug.dumpRange"))
                .description(vec![component::translatable("skyblocker.config.debug.dumpRange.@Tooltip")])
                .binding(compat::read(&defaults, |c| c.debug.dump_range.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.debug.dump_range.clone()) }, { let config = config.clone(); move |new_value: i32| compat::write(&config, |c| c.debug.dump_range = new_value) })
                .controller(integer_controller::create_builder()
                    .range(1, 25)
                    .slider(1)
                    .build())
                .build())
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.debug.showInvisibleArmorStands"))
                .binding(compat::read(&defaults, |c| c.debug.show_invisible_armor_stands.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.debug.show_invisible_armor_stands.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.debug.show_invisible_armor_stands = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.debug.debugWebSockets"))
                .binding(compat::read(&defaults, |c| c.debug.web_socket_debug.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.debug.web_socket_debug.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.debug.web_socket_debug = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .option(option::create_builder::<crate::de::hysky::skyblocker::debug::debug::DumpFormat>()
                .name(component::translatable("skyblocker.config.debug.dumpFormat"))
                .description(vec![component::translatable("skyblocker.config.debug.dumpFormat.@Tooltip")])
                .binding(compat::read(&defaults, |c| c.debug.dump_format.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.debug.dump_format.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::debug::debug::DumpFormat| compat::write(&config, |c| c.debug.dump_format = new_value) })
                .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::debug::debug::DumpFormat>())
                .build())
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.debug.corpseFinderDebug"))
                .binding(compat::read(&defaults, |c| c.debug.corpse_finder_debug.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.debug.corpse_finder_debug.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.debug.corpse_finder_debug = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.debug.enableRepoDev"))
                .binding(compat::read(&defaults, |c| c.debug.enable_repo_dev.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.debug.enable_repo_dev.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.debug.enable_repo_dev = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.debug.skyblockResourcePack"))
                .collapsed(true)
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.debug.skyblockResourcePack.downloadResourcePack"))
                    .description(vec![component::translatable("skyblocker.config.debug.skyblockResourcePack.downloadResourcePack.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.debug.skyblock_resource_pack.download_resource_pack.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.debug.skyblock_resource_pack.download_resource_pack.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.debug.skyblock_resource_pack.download_resource_pack = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .listener(Arc::new(|option: &dyn option::Option<bool>, update_type: UpdateType| { if update_type == UpdateType::ValueChange && option.binding().get() { crate::de::hysky::skyblocker::debug::sky_block_resource_pack_downloader::SkyBlockResourcePackDownloader::download_resource_pack(); } }))
                    .build())
                .build())
            .build();
    }
}
