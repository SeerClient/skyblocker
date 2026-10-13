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
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.eventNotifications.criterion.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Criterion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Criterion::to_string(self))
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
        // @GenToString writes the record-style form: the class's simple name and each field, `IntArrayList.toString()`
        //   being `[a, b, ..]`
        let times: Vec<String> = self.reminder_times.iter().map(|t| t.to_string()).collect();
        format!("EventConfig[enabled={}, reminderTimes=[{}]]", self.enabled, times.join(", "))
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
            SoundFields::new(Some(mcrust::net::minecraft::sounds::sound_events::note_block_pling().clone())),
            // java: GOAT(SoundEvents.GOAT_HORN_SOUND_VARIANTS.getFirst().value()) - the first variant, `item.goat_horn.sound.0`
            SoundFields::new(mcrust::net::minecraft::sounds::sound_events::registry_entries().iter()
                .find(|(id, _)| *id == mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("item.goat_horn.sound.0"))
                .map(|(_, event)| (*event).clone())),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public @Nullable SoundEvent getSoundEvent()
    pub fn get_sound_event(&self) -> Option<SoundEvent> {
        return self.fields().sound_event.clone();
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.eventNotifications.notificationSound.sound.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Sound::to_string(self))
    }
}

/// the constructor data of each Sound constant
pub struct SoundFields {
    pub sound_event: Option<SoundEvent>,
}

impl SoundFields {
    fn new(mut sound_event: Option<SoundEvent>) -> Self {
        Self { sound_event }
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/event_notifications_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// EventNotificationsConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl EventNotificationsConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("criterion".to_string(), dandelion::compat::GsonValue::str(((&self.criterion)).name())),
            ("reminderSound".to_string(), dandelion::compat::GsonValue::str(((&self.reminder_sound)).name())),
            ("events".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.events)).iter().map(|(__k0, __x0)| (__k0.clone(), compat::read(__x0, |__t| __t.__gson_write()))).collect())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<EventNotificationsConfig> {
        let __o = __v.as_object()?;
        let mut __this = EventNotificationsConfig::new();
        if let Some(__v) = __o.get("criterion") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Criterion::values().into_iter().find(|__c| __c.name() == __s)) { __this.criterion = __x; }
        }
        if let Some(__v) = __o.get("reminderSound") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Sound::values().into_iter().find(|__c| __c.name() == __s)) { __this.reminder_sound = __x; }
        }
        if let Some(__v) = __o.get("events") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, EventConfig::__gson_read(__y0)?))).collect()) { __this.events = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "criterion" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Criterion::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.criterion = __c; true } None => false } } else { false },
                "reminderSound" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Sound::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.reminder_sound = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `EventNotificationsConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<EventNotificationsConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.EventNotificationsConfig",
            new: EventNotificationsConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: EventNotificationsConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Criterion's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Criterion {
    pub fn __class_of() -> dandelion::compat::ClassOf<Criterion> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.EventNotificationsConfig$Criterion",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| Criterion::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || Criterion::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// EventConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl EventConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("reminderTimes".to_string(), dandelion::compat::GsonValue::Arr(((&self.reminder_times)).iter().map(|__x0| dandelion::compat::GsonValue::int(*__x0 as i64)).collect())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<EventConfig>> {
        let __o = __v.as_object()?;
        let __h = EventConfig::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("enabled") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
            }
            if let Some(__v) = __o.get("reminderTimes") {
                if let Some(__x) = (__v).as_array().map(|__a| __a.iter().filter_map(|__y0| dandelion::compat::gson_i64(__y0).map(|__n| __n as i32)).collect()) { __this.reminder_times = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `EventConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<EventConfig>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.EventNotificationsConfig$EventConfig",
            new: EventConfig::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: EventConfig::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Sound's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Sound {
    pub fn __class_of() -> dandelion::compat::ClassOf<Sound> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.EventNotificationsConfig$Sound",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| Sound::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || Sound::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}
