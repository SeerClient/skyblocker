//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Currencies by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Currencies
#[derive(Clone)]
pub struct Currencies {
    /// java: @SerializedName("coin_purse") public double coinsInPurse;
    pub coins_in_purse: f64,
    /// java: @SerializedName("motes_purse") public double riftMotes;
    pub rift_motes: f64,
    /// java: public Map<String, EssenceCurrency> essence = Map.of();
    pub essence: std::collections::HashMap<String, EssenceCurrency>,
}

impl Currencies {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            coins_in_purse: 0.0,
            rift_motes: 0.0,
            essence: std::collections::HashMap::from([]),
        };
        this
    }

}

/// java: public static class EssenceCurrency
#[derive(Clone)]
pub struct EssenceCurrency {
    /// java: public int current;
    pub current: i32,
}

impl EssenceCurrency {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            current: 0,
        };
        this
    }

}
