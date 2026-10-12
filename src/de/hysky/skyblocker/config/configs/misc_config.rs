//! Ported from de.hysky.skyblocker.config.configs.MiscConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class MiscConfig
pub struct MiscConfig {
    /// java: public RichPresence richPresence = new RichPresence();
    pub rich_presence: RichPresence,
    /// java: public ConfigType configBackend = ConfigType.MOUL_CONFIG;
    pub config_backend: crate::unresolved::ConfigType,
    /// java: public boolean cat = true;
    pub cat: bool,
}

impl MiscConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            rich_presence: RichPresence::new(),
            config_backend: crate::unresolved::ConfigType::MoulConfig,
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
