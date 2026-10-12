//! Ported from de.hysky.skyblocker.skyblock.fishing.SeaCreatureCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public enum SeaCreatureCategory
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SeaCreatureCategory {
    Basic,
    BackwaterBayou,
    LotusAtoll,
    MoongladeMarsh,
    TorrhusCanyon,
    CrimsonIsle,
    WinterIsland,
    Spooky,
    Shark,
    Hotspot,
    Special,
}

impl SeaCreatureCategory {
    pub const VALUES: [SeaCreatureCategory; 11] = [SeaCreatureCategory::Basic, SeaCreatureCategory::BackwaterBayou, SeaCreatureCategory::LotusAtoll, SeaCreatureCategory::MoongladeMarsh, SeaCreatureCategory::TorrhusCanyon, SeaCreatureCategory::CrimsonIsle, SeaCreatureCategory::WinterIsland, SeaCreatureCategory::Spooky, SeaCreatureCategory::Shark, SeaCreatureCategory::Hotspot, SeaCreatureCategory::Special];

    /// java: values()
    pub fn values() -> Vec<SeaCreatureCategory> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SeaCreatureCategory::Basic => "BASIC",
            SeaCreatureCategory::BackwaterBayou => "BACKWATER_BAYOU",
            SeaCreatureCategory::LotusAtoll => "LOTUS_ATOLL",
            SeaCreatureCategory::MoongladeMarsh => "MOONGLADE_MARSH",
            SeaCreatureCategory::TorrhusCanyon => "TORRHUS_CANYON",
            SeaCreatureCategory::CrimsonIsle => "CRIMSON_ISLE",
            SeaCreatureCategory::WinterIsland => "WINTER_ISLAND",
            SeaCreatureCategory::Spooky => "SPOOKY",
            SeaCreatureCategory::Shark => "SHARK",
            SeaCreatureCategory::Hotspot => "HOTSPOT",
            SeaCreatureCategory::Special => "SPECIAL",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SeaCreatureCategory {
        match name {
            "BASIC" => SeaCreatureCategory::Basic,
            "BACKWATER_BAYOU" => SeaCreatureCategory::BackwaterBayou,
            "LOTUS_ATOLL" => SeaCreatureCategory::LotusAtoll,
            "MOONGLADE_MARSH" => SeaCreatureCategory::MoongladeMarsh,
            "TORRHUS_CANYON" => SeaCreatureCategory::TorrhusCanyon,
            "CRIMSON_ISLE" => SeaCreatureCategory::CrimsonIsle,
            "WINTER_ISLAND" => SeaCreatureCategory::WinterIsland,
            "SPOOKY" => SeaCreatureCategory::Spooky,
            "SHARK" => SeaCreatureCategory::Shark,
            "HOTSPOT" => SeaCreatureCategory::Hotspot,
            "SPECIAL" => SeaCreatureCategory::Special,
            _ => panic!("IllegalArgumentException: No enum constant SeaCreatureCategory.{name}"),
        }
    }

}

impl std::fmt::Display for SeaCreatureCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
