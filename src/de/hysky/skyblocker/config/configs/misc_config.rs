//! Ported from de.hysky.skyblocker.config.configs.MiscConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use dandelion::net::azureaaron::dandelion::api::config_type::ConfigType;

/// java: public class MiscConfig
#[derive(Clone)]
pub struct MiscConfig {
    /// java: public RichPresence richPresence = new RichPresence();
    pub rich_presence: RichPresence,
    /// java: public ConfigType configBackend = ConfigType.MOUL_CONFIG;
    pub config_backend: ConfigType,
    /// java: public boolean cat = true;
    pub cat: bool,
}

impl MiscConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            rich_presence: RichPresence::new(),
            config_backend: ConfigType::MoulConfig,
            cat: true,
        };
        this
    }

}

/// java: public static class RichPresence
#[derive(Clone)]
pub struct RichPresence {
    /// java: public boolean enableRichPresence = false;
    pub enable_rich_presence: bool,
    /// java: public Info info = Info.LOCATION;
    pub info: Info,
    /// java: public boolean cycleMode = false;
    pub cycle_mode: bool,
    /// java: public String customMessage = "Playing Skyblock";
    pub custom_message: String,
}

impl RichPresence {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_rich_presence: false,
            info: Info::Location,
            cycle_mode: false,
            custom_message: "Playing Skyblock".to_string(),
        };
        this
    }

}

/// java: public enum Info
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Info {
    Purse,
    Bits,
    Location,
}

impl Info {
    pub const VALUES: [Info; 3] = [Info::Purse, Info::Bits, Info::Location];

    /// java: values()
    pub fn values() -> Vec<Info> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Info::Purse => "PURSE",
            Info::Bits => "BITS",
            Info::Location => "LOCATION",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Info {
        match name {
            "PURSE" => Info::Purse,
            "BITS" => Info::Bits,
            "LOCATION" => Info::Location,
            _ => panic!("IllegalArgumentException: No enum constant Info.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.misc.richPresence.info.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Info::to_string(self))
    }
}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/misc_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// MiscConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl MiscConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("richPresence".to_string(), ((&self.rich_presence)).__gson_write()),
            ("configBackend".to_string(), dandelion::compat::GsonValue::str(((&self.config_backend)).name())),
            ("cat".to_string(), dandelion::compat::GsonValue::Bool(*(&self.cat))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<MiscConfig> {
        let __o = __v.as_object()?;
        let mut __this = MiscConfig::new();
        if let Some(__v) = __o.get("richPresence") {
            if let Some(__x) = RichPresence::__gson_read(__v) { __this.rich_presence = __x; }
        }
        if let Some(__v) = __o.get("configBackend") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ConfigType::values().into_iter().find(|__c| __c.name() == __s)) { __this.config_backend = __x; }
        }
        if let Some(__v) = __o.get("cat") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.cat = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "configBackend" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ConfigType::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.config_backend = __c; true } None => false } } else { false },
                "cat" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.cat = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "richPresence" => self.rich_presence.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `MiscConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<MiscConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiscConfig",
            new: MiscConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: MiscConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// RichPresence's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl RichPresence {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableRichPresence".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_rich_presence))),
            ("info".to_string(), dandelion::compat::GsonValue::str(((&self.info)).name())),
            ("cycleMode".to_string(), dandelion::compat::GsonValue::Bool(*(&self.cycle_mode))),
            ("customMessage".to_string(), dandelion::compat::GsonValue::str(&(&self.custom_message))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<RichPresence> {
        let __o = __v.as_object()?;
        let mut __this = RichPresence::new();
        if let Some(__v) = __o.get("enableRichPresence") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_rich_presence = __x; }
        }
        if let Some(__v) = __o.get("info") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Info::values().into_iter().find(|__c| __c.name() == __s)) { __this.info = __x; }
        }
        if let Some(__v) = __o.get("cycleMode") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.cycle_mode = __x; }
        }
        if let Some(__v) = __o.get("customMessage") {
            if let Some(__x) = dandelion::compat::gson_string(__v) { __this.custom_message = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableRichPresence" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_rich_presence = *__b; true } else { false },
                "info" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Info::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.info = __c; true } None => false } } else { false },
                "cycleMode" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.cycle_mode = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `RichPresence.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<RichPresence> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiscConfig$RichPresence",
            new: RichPresence::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: RichPresence::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Info's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Info {
    pub fn __class_of() -> dandelion::compat::ClassOf<Info> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiscConfig$Info",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| Info::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || Info::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}
