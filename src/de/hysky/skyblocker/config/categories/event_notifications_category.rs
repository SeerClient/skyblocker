//! Ported from de.hysky.skyblocker.config.categories.EventNotificationsCategory.
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
use crate::de::hysky::skyblocker::config::configs::event_notifications_config::{Criterion, EventConfig, Sound};

/// java: private static boolean shouldPlaySound = false;
pub static SHOULD_PLAY_SOUND: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// java: public class EventNotificationsCategory
#[derive(Clone)]
pub struct EventNotificationsCategory {}

impl EventNotificationsCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &compat::Shared<SkyblockerConfig>, config: &compat::Shared<SkyblockerConfig>) -> Arc<dyn ConfigCategory> {
        let defaults: compat::Shared<SkyblockerConfig> = defaults.clone();
        let config: compat::Shared<SkyblockerConfig> = config.clone();
        //return null;
        SHOULD_PLAY_SOUND.store(false, std::sync::atomic::Ordering::Relaxed);
        return config_category::create_builder()
            .id(skyblocker_mod::id("config/eventnotifications"))
            .name(component::translatable("skyblocker.config.eventNotifications"))
            .option(option::create_builder::<Criterion>()
                .binding(compat::read(&defaults, |c| c.event_notifications.criterion.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.event_notifications.criterion.clone()) }, { let config = config.clone(); move |criterion: Criterion| compat::write(&config, |c| c.event_notifications.criterion = criterion) })
                .controller(ConfigUtils::create_enum_controller::<Criterion>())
                .name(component::translatable("skyblocker.config.eventNotifications.criterion"))
                .build())
            .option(option::create_builder::<Sound>()
                .binding(compat::read(&defaults, |c| c.event_notifications.reminder_sound.clone()), { let config = config.clone(); move || compat::read(&config, |c| c.event_notifications.reminder_sound.clone()) }, { let config = config.clone(); move |sound: Sound| compat::write(&config, |c| c.event_notifications.reminder_sound = sound) })
                .controller(ConfigUtils::create_enum_controller::<Sound>())
                .name(component::translatable("skyblocker.config.eventNotifications.notificationSound"))
                .listener(Arc::new(|sound_option: &dyn option::Option<Sound>, event: UpdateType| {
                    if event == UpdateType::ValueChange {
                        if !SHOULD_PLAY_SOUND.load(std::sync::atomic::Ordering::Relaxed) {
                            SHOULD_PLAY_SOUND.store(true, std::sync::atomic::Ordering::Relaxed);
                            return;
                        }
                        // java: if (soundOption.binding().get() != null)
                        //       Minecraft.getInstance().getSoundManager().play(SimpleSoundInstance.forUI(soundOption.binding().get().getSoundEvent(), 1f, 1f));
                        // (a `Sound` constant whose event is null - `NONE` - plays nothing, as `forUI(null)` would not either)
                        if let Some(sound_event) = sound_option.binding().get().get_sound_event() {
                            dandelion::compat::play_ui_sound(&sound_event);
                        }
                    }
                }))
                .build())
            .groups(EventNotificationsCategory::create_groups(&config))
            .build();
    }

    /// java: private static List<OptionGroup> createGroups(SkyblockerConfig config)
    pub fn create_groups(config: &compat::Shared<SkyblockerConfig>) -> Vec<Arc<dyn OptionGroup>> {
        let events_reminder_times: std::collections::HashMap<String, compat::Shared<EventConfig>> = compat::read(config, |c| c.event_notifications.events.clone());
        if events_reminder_times.is_empty() {
            return vec![option_group::create_builder()
                .option(label_option::create_builder().label(vec![component::translatable("skyblocker.config.eventNotifications.monologue")]).build())
                .build()];
        }
        let mut groups: Vec<Arc<dyn OptionGroup>> = Vec::with_capacity(events_reminder_times.len());
        for (key, value) in events_reminder_times.iter() {
            groups.push(option_group::create_builder()
                .name(component::literal(key))
                .option(option::create_builder::<bool>()
                    .name(component::translatable_args("skyblocker.config.eventNotifications.event.enabled", vec![FormattedText::of(key.clone())]))
                    .binding(
                        compat::read(&*crate::de::hysky::skyblocker::skyblock::events::event_notifications::DEFAULT_REMINDERS, |d| d.enabled),
                        { let value = value.clone(); move || compat::read(&value, |v| v.enabled) },
                        { let value = value.clone(); move |enabled: bool| compat::write(&value, |v| v.enabled = enabled) },
                    )
                    .controller(ConfigUtils::create_boolean_controller())
                    .build())
                .option(button_option::create_builder()
                    .name(component::translatable_args("skyblocker.config.eventNotifications.event.editReminders", vec![FormattedText::of(key.clone())]))
                    .prompt(component::translatable("skyblocker.config.eventNotifications.event.editReminders.prompt"))
                    .description(vec![component::translatable("skyblocker.config.eventNotifications.event.editReminders.tooltip")])
                    .action({
                        let (key, value) = (key.clone(), value.clone());
                        move |_s| mcrust::mcrust::modding::game::set_screen_keeping_parent(Box::new(compat::ScreenAdapter::new(
                            crate::de::hysky::skyblocker::config::screens::eventnotifications::event_config_times_edit_screen::EventConfigTimesEditScreen::new(None, key.clone(), value.clone()),
                        )))
                    })
                    .build())
                .build());
        }
        return groups;
    }
}
