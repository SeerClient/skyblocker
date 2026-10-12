//! Ported from de.hysky.skyblocker.config.categories.EventNotificationsCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::event_notifications_config::EventConfig;
use crate::de::hysky::skyblocker::config::skyblocker_config::SkyblockerConfig;

/// java: private static boolean shouldPlaySound = false;
pub static SHOULD_PLAY_SOUND: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class EventNotificationsCategory
#[derive(Clone)]
pub struct EventNotificationsCategory {}

impl EventNotificationsCategory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static ConfigCategory create(SkyblockerConfig defaults, SkyblockerConfig config)
    pub fn create(defaults: &SkyblockerConfig, config: &SkyblockerConfig) -> crate::unresolved::ConfigCategory {
        compat::set_state(&SHOULD_PLAY_SOUND, false);
        // TODO(fabric2seer): static call ConfigCategory.createBuilder(..)
        //   java: return ConfigCategory.createBuilder()
        //   java: 				.id(SkyblockerMod.id("config/eventnotifications"))
        //   java: 				.name(Component.translatable("skyblocker.config.eventNotifications"))
        //   java: 				.option(Option.<EventNotificationsConfig.Criterion>createBuilder()
        //   java: 						.binding(defaults.eventNotifications.criterion,
        //   java: 								() -> config.eventNotifications.criterion,
        //   java: 								criterion -> config.eventNotifications.criterion = criterion)
        //   java: 						.controller(ConfigUtils.createEnumController())
        //   java: 						.name(Component.translatable("skyblocker.config.eventNotifications.criterion"))
        //   java: 						.build())
        //   java: 				.option(Option.<EventNotificationsConfig.Sound>createBuilder()
        //   java: 						.binding(defaults.eventNotifications.reminderSound,
        //   java: 								() -> config.eventNotifications.reminderSound,
        //   java: 								sound -> config.eventNotifications.reminderSound = sound)
        //   java: 						.controller(ConfigUtils.createEnumController())
        //   java: 						.name(Component.translatable("skyblocker.config.eventNotifications.notificationSound"))
        //   java: 						.listener((soundOption, event) -> {
        //   java: 							if (event == UpdateType.VALUE_CHANGE) {
        //   java: 								if (!shouldPlaySound) {
        //   java: 									shouldPlaySound = true;
        //   java: 									return;
        //   java: 								}
        //   java: 								if (soundOption.binding().get() != null)
        //   java: 									Minecraft.getInstance().getSoundManager().play(SimpleSoundInstance.forUI(soundOption.binding().get().getSoundEvent(), 1f, 1f));
        //   java: 							}
        //   java: 						})
        //   java: 						.build())
        //   java: 				.groups(createGroups(config))
        //   java: 				.build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static List<OptionGroup> createGroups(SkyblockerConfig config)
    pub fn create_groups(config: &SkyblockerConfig) -> Vec<crate::unresolved::OptionGroup> {
        let events_reminder_times = &config.event_notifications.events;
        if events_reminder_times.is_empty() {
            // TODO(fabric2seer): static call OptionGroup.createBuilder(..)
            //   java: return List.of(OptionGroup.createBuilder().option(LabelOption.createBuilder().label(Component.translatable("skyblocker.config.eventNotifications.monologue")).build()).build());
        }
        let mut groups: Vec<crate::unresolved::OptionGroup> = Vec::with_capacity(((events_reminder_times.len() as i32)) as usize);
        for entry in events_reminder_times.iter().map(|(k, v)| (k.clone(), v.clone())) {
            // TODO(fabric2seer): static call OptionGroup.createBuilder(..)
            //   java: groups.add(OptionGroup.createBuilder()
            //   java: 					.name(Component.literal(entry.getKey()))
            //   java: 					.option(Option.<Boolean>createBuilder()
            //   java: 							.name(Component.translatable("skyblocker.config.eventNotifications.event.enabled", entry.getKey()))
            //   java: 							.binding(EventNotifications.DEFAULT_REMINDERS.enabled,
            //   java: 									() -> entry.getValue().enabled,
            //   java: 									enabled -> entry.getValue().enabled = enabled
            //   java: 							)
            //   java: 							.controller(ConfigUtils.createBooleanController())
            //   java: 							.build()
            //   java: 					)
            //   java: 					.option(ButtonOption.createBuilder()
            //   java: 							.name(Component.translatable("skyblocker.config.eventNotifications.event.editReminders", entry.getKey()))
            //   java: 							.prompt(Component.translatable("skyblocker.config.eventNotifications.event.editReminders.prompt"))
            //   java: 							.description(Component.translatable("skyblocker.config.eventNotifications.event.editReminders.tooltip"))
            //   java: 							.action(s -> Minecraft.getInstance().gui.setScreen(new EventConfigTimesEditScreen(s, entry.getKey(), entry.getValue())))
            //   java: 							.build()
            //   java: 					)
            //   java: 					.build()
            //   java: 			);
        }
        return groups;
    }

}
