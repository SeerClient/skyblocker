//! Ported from de.hysky.skyblocker.config.categories.ChatCategory.
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

/// java: public class ChatCategory
#[derive(Clone)]
pub struct ChatCategory {}

impl ChatCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/chat"))
            .name(component::translatable("skyblocker.config.chat"))
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.chat.skyblockXpMessages"))
                .description(vec![component::translatable("skyblocker.config.chat.skyblockXpMessages.@Tooltip")])
                .binding(compat::read(&defaults, |c| c.chat.skyblock_xp_messages.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.skyblock_xp_messages.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.chat.skyblock_xp_messages = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .option(option::create_builder::<bool>()
                .name(component::translatable("skyblocker.config.chat.confirmationPromptHelper"))
                .description(vec![component::translatable("skyblocker.config.chat.confirmationPromptHelper.@Tooltip")])
                .binding(compat::read(&defaults, |c| c.chat.confirmation_prompt_helper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.confirmation_prompt_helper.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.chat.confirmation_prompt_helper = new_value) })
                .controller(ConfigUtils::create_boolean_controller())
                .build())
            .option(option::create_builder::<f32>()
                .name(component::translatable("skyblocker.config.chat.toastDuration"))
                .description(vec![component::translatable("skyblocker.config.chat.toastDuration.@Tooltip")])
                .tags(vec![common_tags::ADDED_IN_6_0_0.get().clone()])
                .binding(compat::read(&defaults, |c| c.chat.toast_display_duration.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.toast_display_duration.clone()) }, { let config = config.clone(); move |new_value: f32| compat::write(&config, |c| c.chat.toast_display_duration = new_value) })
                .controller(float_controller::create_builder()
                    .range(1.0f32, 10.0f32)
                    .slider(0.1f32)
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.chat.filter"))
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideAbility"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_ability.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_ability.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_ability = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideHeal"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_heal.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_heal.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_heal = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideAOTE"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideAOTE.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.chat.hide_aote.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_aote.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_aote = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideImplosion"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_implosion.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_implosion.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_implosion = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideSpiritSceptre"))
                    .tags(vec![common_tags::ADDED_IN_6_4_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.chat.hide_spirit_sceptre.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_spirit_sceptre.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_spirit_sceptre = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideMoltenWave"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_molten_wave.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_molten_wave.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_molten_wave = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideRareCrops"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_rare_crops.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_rare_crops.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_rare_crops = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideAds"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_ads.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_ads.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_ads = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideTeleportPad"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_teleport_pad.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_teleport_pad.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_teleport_pad = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideCombo"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_combo.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_combo.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_combo = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideAutopet"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_autopet.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_autopet.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_autopet = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideShowOff"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideShowOff.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.chat.hide_show_off.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_show_off.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_show_off = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideToggleSkyMall"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideToggleSkyMall.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.chat.hide_toggle_sky_mall.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_toggle_sky_mall.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_toggle_sky_mall = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideToggleLottery"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideToggleLottery.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.chat.hide_toggle_lottery.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_toggle_lottery.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_toggle_lottery = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideToggleBeekeeper"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideToggleBeekeeper.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_9_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.chat.hide_toggle_beekeeper.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_toggle_beekeeper.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_toggle_beekeeper = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<bool>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideMana"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideMana.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.chat.hide_mana.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_mana.clone()) }, { let config = config.clone(); move |new_value: bool| compat::write(&config, |c| c.chat.hide_mana = new_value) })
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideMimicKill"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideMimicKill.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.chat.hide_mimic_kill.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_mimic_kill.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_mimic_kill = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideDeath"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideDeath.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.chat.hide_death.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_death.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_death = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideDungeonBreaker"))
                    .binding(compat::read(&defaults, |c| c.chat.hide_dungeon_breaker.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_dungeon_breaker.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_dungeon_breaker = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .option(option::create_builder::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>()
                    .name(component::translatable("skyblocker.config.chat.filter.hideCritterCapture"))
                    .description(vec![component::translatable("skyblocker.config.chat.filter.hideCritterCapture.@Tooltip")])
                    .tags(vec![common_tags::ADDED_IN_6_10_0.get().clone()])
                    .binding(compat::read(&defaults, |c| c.chat.hide_critter_capture.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.hide_critter_capture.clone()) }, { let config = config.clone(); move |new_value: crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult| compat::write(&config, |c| c.chat.hide_critter_capture = new_value) })
                    .controller(ConfigUtils::create_enum_controller::<crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult>())
                    .build())
                .build())
            .group(option_group::create_builder()
                .name(component::translatable("skyblocker.config.chat.chatRules"))
                .collapsed(false)
                .option(button_option::create_builder()
                    .name(component::translatable("skyblocker.config.chat.chatRules.screen"))
                    .prompt(component::translatable("text.skyblocker.open"))
                    .action(|_screen| mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::skyblock::chat::chat_rules_config_screen::ChatRulesConfigScreen::new(None)))))
                    .build())
                .option(option::create_builder::<i32>()
                    .name(component::translatable("skyblocker.config.chat.chatRules.announcementLength"))
                    .description(vec![component::translatable("skyblocker.config.chat.chatRules.announcementLength.@Tooltip")])
                    .binding(compat::read(&defaults, |c| c.chat.chat_rule_config.announcement_length.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.chat.chat_rule_config.announcement_length.clone()) }, { let config = config.clone(); move |new_value: i32| compat::write(&config, |c| c.chat.chat_rule_config.announcement_length = new_value) })
                    .controller(integer_controller::create_builder()
                        .range(5, 200)
                        .slider(1)
                        .build())
                    .build())
                .build())
            .build();
    }
}
