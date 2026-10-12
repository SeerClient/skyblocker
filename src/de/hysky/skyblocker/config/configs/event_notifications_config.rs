//! Ported from de.hysky.skyblocker.config.configs.EventNotificationsConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;

/// java: public class EventNotificationsConfig
#[derive(Clone)]
pub struct EventNotificationsConfig {
    /// java: public Criterion criterion = Criterion.SKYBLOCK;
    pub criterion: Criterion,
    /// java: public Sound reminderSound = Sound.PLING;
    pub reminder_sound: Sound,
    /// java: public Map<String, EventConfig> events = new HashMap<>();
    pub events: std::collections::HashMap<String, compat::Shared<EventConfig>>,
}

impl EventNotificationsConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            criterion: Criterion::Skyblock,
            reminder_sound: Sound::Pling,
            events: std::collections::HashMap::new(),
        };
        this
    }

}

/// java: public enum Criterion
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Criterion {
    None,
    Skyblock,
    Hypixel,
    Everywhere,
}

impl Criterion {
    pub const VALUES: [Criterion; 4] = [Criterion::None, Criterion::Skyblock, Criterion::Hypixel, Criterion::Everywhere];

    /// java: values()
    pub fn values() -> Vec<Criterion> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Criterion::None => "NONE",
            Criterion::Skyblock => "SKYBLOCK",
            Criterion::Hypixel => "HYPIXEL",
            Criterion::Everywhere => "EVERYWHERE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Criterion {
        match name {
            "NONE" => Criterion::None,
            "SKYBLOCK" => Criterion::Skyblock,
            "HYPIXEL" => Criterion::Hypixel,
            "EVERYWHERE" => Criterion::Everywhere,
            _ => panic!("IllegalArgumentException: No enum constant Criterion.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.eventNotifications.criterion." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for Criterion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Criterion::to_string(*self))
    }
}

/// java: public static class EventConfig
#[derive(Clone)]
pub struct EventConfig {
    /// java: public boolean enabled;
    pub enabled: bool,
    /// java: public IntArrayList reminderTimes;
    pub reminder_times: Vec<i32>,
}

impl EventConfig {
    /// java: public EventConfig()
    pub fn new() -> compat::Shared<Self> {
        let __init_enabled = true;
        let __init_reminder_times = Vec::new();
        let __this = compat::shared(Self {
            enabled: __init_enabled,
            reminder_times: __init_reminder_times,
        });
        __this
    }

    /// java: @GenToString @Override public native String toString();
    pub fn to_string(&self) -> String {
        todo!("native method: its body is generated at build time by an annotation processor (@GenToString)")
    }

}

impl std::fmt::Display for EventConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", EventConfig::to_string(self))
    }
}

/// java: public enum Sound
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sound {
    None,
    Bell,
    Ding,
    Pling,
    Goat,
}

impl Sound {
    pub const VALUES: [Sound; 5] = [Sound::None, Sound::Bell, Sound::Ding, Sound::Pling, Sound::Goat];

    /// java: values()
    pub fn values() -> Vec<Sound> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Sound::None => "NONE",
            Sound::Bell => "BELL",
            Sound::Ding => "DING",
            Sound::Pling => "PLING",
            Sound::Goat => "GOAT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Sound {
        match name {
            "NONE" => Sound::None,
            "BELL" => Sound::Bell,
            "DING" => Sound::Ding,
            "PLING" => Sound::Pling,
            "GOAT" => Sound::Goat,
            _ => panic!("IllegalArgumentException: No enum constant Sound.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SoundFields {
        static FIELDS: LazyLock<Vec<SoundFields>> = LazyLock::new(|| vec![
            SoundFields::new(None),
            SoundFields::new(Some(mcrust::net::minecraft::sounds::sound_events::bell_block().clone())),
            SoundFields::new(Some(mcrust::net::minecraft::sounds::sound_events::arrow_hit_player().clone())),
            SoundFields::new(Some(mcrust::net::minecraft::sounds::sound_events::note_block_pling().value())),
            todo!("GOAT"),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public @Nullable SoundEvent getSoundEvent()
    pub fn get_sound_event(&self) -> Option<SoundEvent> {
        return Some(self.fields().sound_event.clone());
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.eventNotifications.notificationSound.sound." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Sound::to_string(*self))
    }
}

/// the constructor data of each Sound constant
pub struct SoundFields {
    pub sound_event: Option<SoundEvent>,
}

impl SoundFields {
    fn new(mut sound_event: Option<SoundEvent>) -> Self {
        let mut this = Self {
            sound_event: None,
        };
        this.sound_event = sound_event.clone().unwrap();
        this
    }

}
