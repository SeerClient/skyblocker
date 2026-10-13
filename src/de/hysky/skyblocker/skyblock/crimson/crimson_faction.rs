//! Ported from de.hysky.skyblocker.skyblock.crimson.CrimsonFaction by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: public static final Codec<CrimsonFaction> CODEC = StringRepresentable.fromValues(CrimsonFaction::values);
pub static CODEC: compat::MainThread<CodecRef<CrimsonFaction>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_values(CrimsonFaction::values(), |faction: &CrimsonFaction| faction.get_serialized_name()));

/// java: public enum CrimsonFaction implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrimsonFaction {
    Mage,
    Barbarian,
}

impl CrimsonFaction {
    pub const VALUES: [CrimsonFaction; 2] = [CrimsonFaction::Mage, CrimsonFaction::Barbarian];

    /// java: values()
    pub fn values() -> Vec<CrimsonFaction> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            CrimsonFaction::Mage => "MAGE",
            CrimsonFaction::Barbarian => "BARBARIAN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> CrimsonFaction {
        match name {
            "MAGE" => CrimsonFaction::Mage,
            "BARBARIAN" => CrimsonFaction::Barbarian,
            _ => panic!("IllegalArgumentException: No enum constant CrimsonFaction.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        //   java: return this.name();
        return self.name().to_string();
    }

}

impl std::fmt::Display for CrimsonFaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
