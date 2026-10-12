//! Ported from de.hysky.skyblocker.skyblock.crimson.kuudra.KuudraPhase by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public enum KuudraPhase
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KuudraPhase {
    Other,
    RetrieveSupplies,
    Dps,
    KuudraLair,
}

impl KuudraPhase {
    pub const VALUES: [KuudraPhase; 4] = [KuudraPhase::Other, KuudraPhase::RetrieveSupplies, KuudraPhase::Dps, KuudraPhase::KuudraLair];

    /// java: values()
    pub fn values() -> Vec<KuudraPhase> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            KuudraPhase::Other => "OTHER",
            KuudraPhase::RetrieveSupplies => "RETRIEVE_SUPPLIES",
            KuudraPhase::Dps => "DPS",
            KuudraPhase::KuudraLair => "KUUDRA_LAIR",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> KuudraPhase {
        match name {
            "OTHER" => KuudraPhase::Other,
            "RETRIEVE_SUPPLIES" => KuudraPhase::RetrieveSupplies,
            "DPS" => KuudraPhase::Dps,
            "KUUDRA_LAIR" => KuudraPhase::KuudraLair,
            _ => panic!("IllegalArgumentException: No enum constant KuudraPhase.{name}"),
        }
    }

}

impl std::fmt::Display for KuudraPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
