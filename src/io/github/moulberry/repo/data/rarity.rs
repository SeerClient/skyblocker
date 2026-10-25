//! Ported from io.github.moulberry.repo.data.Rarity (neurepoparser 1.12.0).

/// java: public enum Rarity
///
/// The constants keep the Rust case the mod already names them by (`SkyblockItemRarity.toNeuRarity`); `name()` is
/// the Java constant name, and the declaration order (`ordinal`, `compareTo`) is the library's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Divine,
    Supreme,
    Special,
    VerySpecial,
    Unknown,
}

impl Rarity {
    /// java: Rarity.values()
    pub const VALUES: [Rarity; 11] = [
        Rarity::Common,
        Rarity::Uncommon,
        Rarity::Rare,
        Rarity::Epic,
        Rarity::Legendary,
        Rarity::Mythic,
        Rarity::Divine,
        Rarity::Supreme,
        Rarity::Special,
        Rarity::VerySpecial,
        Rarity::Unknown,
    ];

    /// java: Rarity.values()
    pub fn values() -> &'static [Rarity] {
        &Self::VALUES
    }

    /// java: Enum.name()
    pub fn name(&self) -> &'static str {
        match self {
            Rarity::Common => "COMMON",
            Rarity::Uncommon => "UNCOMMON",
            Rarity::Rare => "RARE",
            Rarity::Epic => "EPIC",
            Rarity::Legendary => "LEGENDARY",
            Rarity::Mythic => "MYTHIC",
            Rarity::Divine => "DIVINE",
            Rarity::Supreme => "SUPREME",
            Rarity::Special => "SPECIAL",
            Rarity::VerySpecial => "VERY_SPECIAL",
            Rarity::Unknown => "UNKNOWN",
        }
    }

    /// java: Enum.ordinal()
    pub fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// java: Rarity.valueOf(name) - `None` where Java throws `IllegalArgumentException` (and where Gson's enum adapter
    /// answers `null` for an unknown constant).
    pub fn value_of(name: &str) -> Option<Rarity> {
        Self::VALUES.iter().copied().find(|r| r.name() == name)
    }
}

impl std::fmt::Display for Rarity {
    /// java: Enum.toString() - the name.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
