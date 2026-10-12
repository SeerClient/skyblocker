//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.GardenPlayerData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class GardenPlayerData
#[derive(Clone)]
pub struct GardenPlayerData {
    /// java: public int copper;
    pub copper: i32,
    /// java: @SerializedName("larva_consumed") public int larvaConsumed;
    pub larva_consumed: i32,
    /// java: @SerializedName("analyzed_greenhouse_crops") public List<String> analyzedGreenhouseMutations = List.of();
    pub analyzed_greenhouse_mutations: Vec<String>,
    /// java: @SerializedName("discovered_greenhouse_crops") public List<String> discoveredGreenhouseMutations = List.of();
    pub discovered_greenhouse_mutations: Vec<String>,
}

impl GardenPlayerData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            copper: 0,
            larva_consumed: 0,
            analyzed_greenhouse_mutations: vec![],
            discovered_greenhouse_mutations: vec![],
        };
        this
    }

}
