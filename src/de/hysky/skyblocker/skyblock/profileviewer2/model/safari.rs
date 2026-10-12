//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Safari by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Safari
#[derive(Clone)]
pub struct Safari {
    /// java: public Tickets tickets = new Tickets();
    pub tickets: Tickets,
    /// java: @SerializedName("milestone_claimed_tiers") public AbstractBiomeData milestoneTiersClaimed = new AbstractBiomeData();
    pub milestone_tiers_claimed: compat::Shared<AbstractBiomeData>,
    /// java: @SerializedName("biome_captures") public AbstractBiomeData biomeCaptures = new AbstractBiomeData();
    pub biome_captures: compat::Shared<AbstractBiomeData>,
    /// java: @SerializedName("discovered_critters") public List<String> discoveredCritters = List.of();
    pub discovered_critters: Vec<String>,
    /// java: @SerializedName("discovered_sparkling_critters") public List<String> discoveredSparklingCritters = List.of();
    pub discovered_sparkling_critters: Vec<String>,
    /// java: @SerializedName("total_captured_sparkling_critters") public int totalSparklingCrittersCaptured;
    pub total_sparkling_critters_captured: i32,
}

impl Safari {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            tickets: Tickets::new(),
            milestone_tiers_claimed: AbstractBiomeData::new(),
            biome_captures: AbstractBiomeData::new(),
            discovered_critters: vec![],
            discovered_sparkling_critters: vec![],
            total_sparkling_critters_captured: 0,
        };
        this
    }

}

/// java: public static class Tickets
#[derive(Clone)]
pub struct Tickets {
    /// java: public int basic;
    pub basic: i32,
    /// java: public int economy;
    pub economy: i32,
    /// java: public int premium;
    pub premium: i32,
    /// java: @SerializedName("first_class") public int firstClass;
    pub first_class: i32,
}

impl Tickets {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            basic: 0,
            economy: 0,
            premium: 0,
            first_class: 0,
        };
        this
    }

}

/// java: public static class AbstractBiomeData
#[derive(Clone)]
pub struct AbstractBiomeData {
    /// java: public int cavern;
    pub cavern: i32,
    /// java: public int forest;
    pub forest: i32,
    /// java: public int haunted;
    pub haunted: i32,
    /// java: public int icy;
    pub icy: i32,
}

impl AbstractBiomeData {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            cavern: 0,
            forest: 0,
            haunted: 0,
            icy: 0,
        });
        __this
    }

}
