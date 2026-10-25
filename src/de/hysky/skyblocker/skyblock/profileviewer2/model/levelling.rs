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

impl Levelling {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "experience") {
            if let Some(__x) = gson::as_i32(__v) {
                self.experience = __x;
            }
        }
        if let Some(__v) = gson::member(json, "highest_pet_score") {
            if let Some(__x) = gson::as_i32(__v) {
                self.highest_pet_score = __x;
            }
        }
        if let Some(__v) = gson::member(json, "mining_fiesta_ores_mined") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mining_fiesta_ores_mined = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fishing_festival_sharks_killed") {
            if let Some(__x) = gson::as_i32(__v) {
                self.fishing_festival_sharks_killed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "migrated") {
            if let Some(__x) = gson::as_bool(__v) {
                self.migrated = __x;
            }
        }
        if let Some(__v) = gson::member(json, "migrated_completions_2") {
            if let Some(__x) = gson::as_bool(__v) {
                self.migrated_completions2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "category_expanded") {
            if let Some(__x) = gson::as_bool(__v) {
                self.category_expanded = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claimed_talisman") {
            if let Some(__x) = gson::as_bool(__v) {
                self.claimed_talisman = __x;
            }
        }
        if let Some(__v) = gson::member(json, "completions") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::as_i32(__e0)) {
                self.completions = __x;
            }
        }
        if let Some(__v) = gson::member(json, "bop_bonus") {
            if let Some(__x) = gson::as_string(__v) {
                self.book_of_progression_bonus = __x;
            }
        }
        if let Some(__v) = gson::member(json, "selected_symbol") {
            if let Some(__x) = gson::as_string(__v) {
                self.selected_emblem = __x;
            }
        }
        if let Some(__v) = gson::member(json, "emblem_unlocks") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.emblem_unlocks = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_viewed_tasks") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.last_viewed_tasks = __x;
            }
        }
        if let Some(__v) = gson::member(json, "completed_tasks") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.completed_tasks = __x;
            }
        }
        if let Some(__v) = gson::member(json, "task_sort") {
            if let Some(__x) = gson::as_string(__v) {
                self.task_sort = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Levelling.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
