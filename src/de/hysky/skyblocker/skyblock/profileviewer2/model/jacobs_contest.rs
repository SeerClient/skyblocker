//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.JacobsContest by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class JacobsContest
#[derive(Clone)]
pub struct JacobsContest {
    /// java: public boolean talked;
    pub talked: bool,
    /// java: public boolean migration;
    pub migration: bool,
    /// java: public Perks perks = new Perks();
    pub perks: Perks,
    /// java: @SerializedName("medals_inv") public MedalsInventory medalsInventory = new MedalsInventory();
    pub medals_inventory: MedalsInventory,
    /// java: @SerializedName("personal_bests") public Map<String, Long> personalBests = new HashMap<>();
    pub personal_bests: std::collections::HashMap<String, i64>,
    /// java: @SerializedName("unique_brackets") public Map<String, List<String>> uniqueBrackets = new HashMap<>();
    pub unique_brackets: std::collections::HashMap<String, Vec<String>>,
    /// java: public Map<String, Contest> contests = new HashMap<>();
    pub contests: std::collections::HashMap<String, Contest>,
}

impl JacobsContest {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            talked: false,
            migration: false,
            perks: Perks::new(),
            medals_inventory: MedalsInventory::new(),
            personal_bests: std::collections::HashMap::new(),
            unique_brackets: std::collections::HashMap::new(),
            contests: std::collections::HashMap::new(),
        };
        this
    }

}

/// java: public static class Perks
#[derive(Clone)]
pub struct Perks {
    /// java: @SerializedName("farming_level_cap") public int farmingLevelCap;
    pub farming_level_cap: i32,
    /// java: @SerializedName("double_drops") public int doubleDrops;
    pub double_drops: i32,
    /// java: @SerializedName("personal_bests") public boolean personalBests;
    pub personal_bests: bool,
}

impl Perks {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            farming_level_cap: 0,
            double_drops: 0,
            personal_bests: false,
        };
        this
    }

}

/// java: public static class MedalsInventory
#[derive(Clone)]
pub struct MedalsInventory {
    /// java: public int bronze;
    pub bronze: i32,
    /// java: public int silver;
    pub silver: i32,
    /// java: public int gold;
    pub gold: i32,
}

impl MedalsInventory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            bronze: 0,
            silver: 0,
            gold: 0,
        };
        this
    }

    /// java: public int getTotalMedals()
    pub fn get_total_medals(&self) -> i32 {
        return (self.bronze.wrapping_add(self.silver)).wrapping_add(self.gold);
    }

}

/// java: public static class Contest
#[derive(Clone)]
pub struct Contest {
    /// java: @SerializedName("claimed_rewards") public boolean claimedRewards;
    pub claimed_rewards: bool,
    /// java: @SerializedName("claimed_position") public int claimedPosition;
    pub claimed_position: i32,
    /// java: @SerializedName("claimed_participants") public int claimedParticipants;
    pub claimed_participants: i32,
    /// java: public int collected;
    pub collected: i32,
}

impl Contest {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            claimed_rewards: false,
            claimed_position: 0,
            claimed_participants: 0,
            collected: 0,
        };
        this
    }

}
