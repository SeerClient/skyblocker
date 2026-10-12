//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Levelling by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Levelling
#[derive(Clone)]
pub struct Levelling {
    /// java: public int experience;
    pub experience: i32,
    /// java: @SerializedName("highest_pet_score") public int highestPetScore;
    pub highest_pet_score: i32,
    /// java: @SerializedName("mining_fiesta_ores_mined") public long miningFiestaOresMined;
    pub mining_fiesta_ores_mined: i64,
    /// java: @SerializedName("fishing_festival_sharks_killed") public int fishingFestivalSharksKilled;
    pub fishing_festival_sharks_killed: i32,
    /// java: public boolean migrated;
    pub migrated: bool,
    /// java: @SerializedName("migrated_completions_2") public boolean migratedCompletions2;
    pub migrated_completions2: bool,
    /// java: @SerializedName("category_expanded") public boolean categoryExpanded;
    pub category_expanded: bool,
    /// java: @SerializedName("claimed_talisman") public boolean claimedTalisman;
    pub claimed_talisman: bool,
    /// java: public Map<String, Integer> completions = Map.of();
    pub completions: std::collections::HashMap<String, i32>,
    /// java: @SerializedName("bop_bonus") public String bookOfProgressionBonus = "";
    pub book_of_progression_bonus: String,
    /// java: @SerializedName("selected_symbol") public String selectedEmblem = "";
    pub selected_emblem: String,
    /// java: @SerializedName("emblem_unlocks") public List<String> emblemUnlocks = List.of();
    pub emblem_unlocks: Vec<String>,
    /// java: @SerializedName("last_viewed_tasks") public List<String> lastViewedTasks = List.of();
    pub last_viewed_tasks: Vec<String>,
    /// java: @SerializedName("completed_tasks") public List<String> completedTasks = List.of();
    pub completed_tasks: Vec<String>,
    /// java: @SerializedName("task_sort") public String taskSort = "";
    pub task_sort: String,
}

impl Levelling {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            experience: 0,
            highest_pet_score: 0,
            mining_fiesta_ores_mined: 0,
            fishing_festival_sharks_killed: 0,
            migrated: false,
            migrated_completions2: false,
            category_expanded: false,
            claimed_talisman: false,
            completions: std::collections::HashMap::from([]),
            book_of_progression_bonus: "".to_string(),
            selected_emblem: "".to_string(),
            emblem_unlocks: vec![],
            last_viewed_tasks: vec![],
            completed_tasks: vec![],
            task_sort: "".to_string(),
        };
        this
    }

}
