//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Treasures by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::util::uuid::Uuid;

/// java: public class Treasures
#[derive(Clone)]
pub struct Treasures {
    /// java: public List<Run> runs = List.of();
    pub runs: Vec<Run>,
    /// java: public List<Chest> chests = List.of();
    pub chests: Vec<Chest>,
}

impl Treasures {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            runs: vec![],
            chests: vec![],
        };
        this
    }

    /// java: public Map<UUID, List<Chest>> chestsByRun()
    pub fn chests_by_run(&self) -> std::collections::HashMap<Uuid, Vec<Chest>> {
        let mut chests_by_run: std::collections::HashMap<Uuid, Vec<Chest>> = compat::grouping_by(self.chests.iter().cloned(), |chest| chest.run_id.clone());
        return chests_by_run;
    }

    /// java: public int totalRuns()
    pub fn total_runs(&self) -> i32 {
        return (self.chests_by_run().len() as i32);
    }

    /// java: public int runsWithUnclaimedChests()
    pub fn runs_with_unclaimed_chests(&self) -> i32 {
        // TODO(fabric2seer): not a lambda
        //   java: return (int) this.chestsByRun().entrySet().stream()
        //   java: 				// Check paid for dungeons and isOpened for kuudra
        //   java: 				.filter(Predicate.not(entry -> entry.getValue().stream().anyMatch(chest -> chest.paid || chest.isOpened)))
        //   java: 				.count();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: public static class Chest
#[derive(Clone)]
pub struct Chest {
    /// java: public String type = "";
    pub r#type: String,
    /// java: @SerializedName("run_id") public UUID runId = UUID.randomUUID();
    pub run_id: Uuid,
    /// java: @SerializedName("chest_id") public UUID chestId = UUID.randomUUID();
    pub chest_id: Uuid,
    /// java: @SerializedName("treasure_type") public String treasureType = "";
    pub treasure_type: String,
    /// java: public Rewards rewards = new Rewards();
    pub rewards: Rewards,
    /// java: public int quality;
    pub quality: i32,
    /// java: @SerializedName("shiny_eligible") public boolean shinyEligible;
    pub shiny_eligible: bool,
    /// java: public boolean paid;
    pub paid: bool,
    /// java: public int rerolls;
    pub rerolls: i32,
    /// java: @SerializedName("chest_type") public String chestType = "";
    pub chest_type: String,
    /// java: @SerializedName("run_tier") public String kuudraTier = "";
    pub kuudra_tier: String,
    /// java: @SerializedName("loot_table_id") public String lootTableId = "";
    pub loot_table_id: String,
    /// java: @SerializedName("secondary_slots") public int secondarySlots;
    pub secondary_slots: i32,
    /// java: @SerializedName("percentage_completed") public int percentageCompleted;
    pub percentage_completed: i32,
    /// java: @SerializedName("is_opened") public boolean isOpened;
    pub is_opened: bool,
    /// java: @SerializedName("is_rerolled") public boolean isRerolled;
    pub is_rerolled: bool,
    /// java: @SerializedName("is_attribute_rerolled") public boolean isAttributeRerolled;
    pub is_attribute_rerolled: bool,
}

impl Chest {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            r#type: "".to_string(),
            run_id: Uuid::random_uuid(),
            chest_id: Uuid::random_uuid(),
            treasure_type: "".to_string(),
            rewards: Rewards::new(),
            quality: 0,
            shiny_eligible: false,
            paid: false,
            rerolls: 0,
            chest_type: "".to_string(),
            kuudra_tier: "".to_string(),
            loot_table_id: "".to_string(),
            secondary_slots: 0,
            percentage_completed: 0,
            is_opened: false,
            is_rerolled: false,
            is_attribute_rerolled: false,
        };
        this
    }

}

/// java: public static class Rewards
#[derive(Clone)]
pub struct Rewards {
    /// java: public List<String> rewards = List.of();
    pub rewards: Vec<String>,
    /// java: @SerializedName("rolled_rng_meter_randomly") public boolean rolledRngMeterRandomly;
    pub rolled_rng_meter_randomly: bool,
}

impl Rewards {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            rewards: vec![],
            rolled_rng_meter_randomly: false,
        };
        this
    }

}

/// java: public static class Run
#[derive(Clone)]
pub struct Run {
    /// java: public String type = "";
    pub r#type: String,
    /// java: @SerializedName("run_id") public UUID runId = UUID.randomUUID();
    pub run_id: Uuid,
    /// java: @SerializedName("completion_ts") public long completionTimestamp;
    pub completion_timestamp: i64,
    /// java: @SerializedName("dungeon_type") public String dungeonType = "";
    pub dungeon_type: String,
    /// java: @SerializedName("dungeon_tier") public int dungeonTier;
    pub dungeon_tier: i32,
    /// java: @SerializedName("tier_id") public String kuudraTier = "";
    pub kuudra_tier: String,
    /// java: public List<Participant> participants = List.of();
    pub participants: Vec<Participant>,
}

impl Run {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            r#type: "".to_string(),
            run_id: Uuid::random_uuid(),
            completion_timestamp: 0,
            dungeon_type: "".to_string(),
            dungeon_tier: 0,
            kuudra_tier: "".to_string(),
            participants: vec![],
        };
        this
    }

}

/// java: public static class Participant
#[derive(Clone)]
pub struct Participant {
    /// java: @SerializedName("player_uuid") public UUID playerUuid;
    pub player_uuid: Option<Uuid>,
    /// java: @SerializedName("display_name") public String displayName;
    pub display_name: String,
    /// java: @SerializedName("class_milestone") public int classMilestone;
    pub class_milestone: i32,
}

impl Participant {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            player_uuid: None,
            display_name: Default::default(),
            class_milestone: 0,
        };
        this
    }

}
