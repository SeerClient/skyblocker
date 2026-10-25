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

impl JacobsContest {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "talked") {
            if let Some(__x) = gson::as_bool(__v) {
                self.talked = __x;
            }
        }
        if let Some(__v) = gson::member(json, "migration") {
            if let Some(__x) = gson::as_bool(__v) {
                self.migration = __x;
            }
        }
        if let Some(__v) = gson::member(json, "perks") {
            if let Some(__x) = Some(Perks::from_json(__v)) {
                self.perks = __x;
            }
        }
        if let Some(__v) = gson::member(json, "medals_inv") {
            if let Some(__x) = Some(MedalsInventory::from_json(__v)) {
                self.medals_inventory = __x;
            }
        }
        if let Some(__v) = gson::member(json, "personal_bests") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::as_i64(__e0)) {
                self.personal_bests = __x;
            }
        }
        if let Some(__v) = gson::member(json, "unique_brackets") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::list(__e0, |__e1| gson::as_string(__e1))) {
                self.unique_brackets = __x;
            }
        }
        if let Some(__v) = gson::member(json, "contests") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(Contest::from_json(__e0))) {
                self.contests = __x;
            }
        }
    }

    /// `GSON.fromJson(json, JacobsContest.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Perks {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "farming_level_cap") {
            if let Some(__x) = gson::as_i32(__v) {
                self.farming_level_cap = __x;
            }
        }
        if let Some(__v) = gson::member(json, "double_drops") {
            if let Some(__x) = gson::as_i32(__v) {
                self.double_drops = __x;
            }
        }
        if let Some(__v) = gson::member(json, "personal_bests") {
            if let Some(__x) = gson::as_bool(__v) {
                self.personal_bests = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Perks.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl MedalsInventory {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "bronze") {
            if let Some(__x) = gson::as_i32(__v) {
                self.bronze = __x;
            }
        }
        if let Some(__v) = gson::member(json, "silver") {
            if let Some(__x) = gson::as_i32(__v) {
                self.silver = __x;
            }
        }
        if let Some(__v) = gson::member(json, "gold") {
            if let Some(__x) = gson::as_i32(__v) {
                self.gold = __x;
            }
        }
    }

    /// `GSON.fromJson(json, MedalsInventory.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Contest {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "claimed_rewards") {
            if let Some(__x) = gson::as_bool(__v) {
                self.claimed_rewards = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claimed_position") {
            if let Some(__x) = gson::as_i32(__v) {
                self.claimed_position = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claimed_participants") {
            if let Some(__x) = gson::as_i32(__v) {
                self.claimed_participants = __x;
            }
        }
        if let Some(__v) = gson::member(json, "collected") {
            if let Some(__x) = gson::as_i32(__v) {
                self.collected = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Contest.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
