//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Foraging by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Foraging
#[derive(Clone)]
pub struct Foraging {
    /// java: @SerializedName("fish_family") public List<String> fishFamily = List.of();
    pub fish_family: Vec<String>,
    /// java: public Starlyn starlyn = new Starlyn();
    pub starlyn: Starlyn,
    /// java: public Hina hina = new Hina();
    pub hina: Hina,
    /// java: @SerializedName("tree_gifts") public TreeGifts treeGifts = new TreeGifts();
    pub tree_gifts: TreeGifts,
    /// java: public Honey honey = new Honey();
    pub honey: Honey,
}

impl Foraging {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fish_family: vec![],
            starlyn: Starlyn::new(),
            hina: Hina::new(),
            tree_gifts: TreeGifts::new(),
            honey: Honey::new(),
        };
        this
    }

}

/// java: public static class Starlyn
#[derive(Clone)]
pub struct Starlyn {
    /// java: @SerializedName("personal_bests") public PersonalBests personalBests = new PersonalBests();
    pub personal_bests: PersonalBests,
}

impl Starlyn {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            personal_bests: PersonalBests::new(),
        };
        this
    }

}

/// java: public static class PersonalBests
#[derive(Clone)]
pub struct PersonalBests {
    /// java: public int agatha;
    pub agatha: i32,
    /// java: @SerializedName("FIG_LOG") public int figLogs;
    pub fig_logs: i32,
    /// java: @SerializedName("MANGROVE_LOG") public int mangroveLogs;
    pub mangrove_logs: i32,
    /// java: public int miria;
    pub miria: i32,
    /// java: @SerializedName("HELIX_LOG") public int helixLogs;
    pub helix_logs: i32,
}

impl PersonalBests {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            agatha: 0,
            fig_logs: 0,
            mangrove_logs: 0,
            miria: 0,
            helix_logs: 0,
        };
        this
    }

}

/// java: public static class Hina
#[derive(Clone)]
pub struct Hina {
    /// java: public Tasks tasks = new Tasks();
    pub tasks: Tasks,
}

impl Hina {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            tasks: Tasks::new(),
        };
        this
    }

}

/// java: public static class Tasks
#[derive(Clone)]
pub struct Tasks {
    /// java: @SerializedName("completed_tasks") public List<String> completedTasks = List.of();
    pub completed_tasks: Vec<String>,
    /// java: @SerializedName("task_progress") public Map<String, Integer> taskProgress = Map.of();
    pub task_progress: std::collections::HashMap<String, i32>,
    /// java: @SerializedName("claimed_rewards") public List<String> claimedRewards = List.of();
    pub claimed_rewards: Vec<String>,
    /// java: @SerializedName("tier_claimed") public int tierClaimed;
    pub tier_claimed: i32,
}

impl Tasks {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            completed_tasks: vec![],
            task_progress: std::collections::HashMap::from([]),
            claimed_rewards: vec![],
            tier_claimed: 0,
        };
        this
    }

}

/// java: public static class TreeGifts
#[derive(Clone)]
pub struct TreeGifts {
    /// java: @SerializedName("FIG") public int fig;
    pub fig: i32,
    /// java: @SerializedName("MANGROVE") public int mangrove;
    pub mangrove: i32,
    /// java: @SerializedName("HELIX") public int helix;
    pub helix: i32,
    /// java: @SerializedName("milestone_tier_claimed") public MilestoneClaimed milestoneClaimed = new MilestoneClaimed();
    pub milestone_claimed: MilestoneClaimed,
}

impl TreeGifts {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fig: 0,
            mangrove: 0,
            helix: 0,
            milestone_claimed: MilestoneClaimed::new(),
        };
        this
    }

}

/// java: public static class MilestoneClaimed
#[derive(Clone)]
pub struct MilestoneClaimed {
    /// java: @SerializedName("FIG") public int fig;
    pub fig: i32,
    /// java: @SerializedName("MANGROVE") public int mangrove;
    pub mangrove: i32,
    /// java: @SerializedName("HELIX") public int helix;
    pub helix: i32,
}

impl MilestoneClaimed {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fig: 0,
            mangrove: 0,
            helix: 0,
        };
        this
    }

}

/// java: public static class Honey
#[derive(Clone)]
pub struct Honey {
    /// java: @SerializedName("refill_times") public RefillTimes honeyhiveRefillTimes = new RefillTimes();
    pub honeyhive_refill_times: RefillTimes,
    /// java: @SerializedName("smeared_trees") public SmearedTrees smearedTrees = new SmearedTrees();
    pub smeared_trees: SmearedTrees,
}

impl Honey {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            honeyhive_refill_times: RefillTimes::new(),
            smeared_trees: SmearedTrees::new(),
        };
        this
    }

}

/// java: public static class RefillTimes
#[derive(Clone)]
pub struct RefillTimes {
    /// java: @SerializedName("hive_1") public long hive1;
    pub hive1: i64,
    /// java: @SerializedName("hive_2") public long hive2;
    pub hive2: i64,
    /// java: @SerializedName("hive_3") public long hive3;
    pub hive3: i64,
    /// java: @SerializedName("hive_4") public long hive4;
    pub hive4: i64,
    /// java: @SerializedName("hive_5") public long hive5;
    pub hive5: i64,
    /// java: @SerializedName("hive_6") public long hive6;
    pub hive6: i64,
    /// java: @SerializedName("hive_7") public long hive7;
    pub hive7: i64,
    /// java: @SerializedName("hive_8") public long hive8;
    pub hive8: i64,
    /// java: @SerializedName("hive_9") public long hive9;
    pub hive9: i64,
    /// java: @SerializedName("hive_10") public long hive10;
    pub hive10: i64,
    /// java: @SerializedName("hive_11") public long hive11;
    pub hive11: i64,
    /// java: @SerializedName("hive_12") public long hive12;
    pub hive12: i64,
    /// java: @SerializedName("hive_13") public long hive13;
    pub hive13: i64,
    /// java: @SerializedName("hive_14") public long hive14;
    pub hive14: i64,
    /// java: @SerializedName("hive_15") public long hive15;
    pub hive15: i64,
    /// java: @SerializedName("hive_16") public long hive16;
    pub hive16: i64,
    /// java: @SerializedName("hive_17") public long hive17;
    pub hive17: i64,
    /// java: @SerializedName("hive_18") public long hive18;
    pub hive18: i64,
    /// java: @SerializedName("hive_19") public long hive19;
    pub hive19: i64,
}

impl RefillTimes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            hive1: 0,
            hive2: 0,
            hive3: 0,
            hive4: 0,
            hive5: 0,
            hive6: 0,
            hive7: 0,
            hive8: 0,
            hive9: 0,
            hive10: 0,
            hive11: 0,
            hive12: 0,
            hive13: 0,
            hive14: 0,
            hive15: 0,
            hive16: 0,
            hive17: 0,
            hive18: 0,
            hive19: 0,
        };
        this
    }

}

/// java: public static class SmearedTrees
#[derive(Clone)]
pub struct SmearedTrees {
    /// java: @SerializedName("fig_1") public long fig1;
    pub fig1: i64,
    /// java: @SerializedName("fig_2") public long fig2;
    pub fig2: i64,
    /// java: @SerializedName("fig_3") public long fig3;
    pub fig3: i64,
    /// java: @SerializedName("mangrove_1") public long mangrove1;
    pub mangrove1: i64,
    /// java: @SerializedName("mangrove_2") public long mangrove2;
    pub mangrove2: i64,
    /// java: @SerializedName("mangrove_3") public long mangrove3;
    pub mangrove3: i64,
    /// java: @SerializedName("helix_1") public long helix1;
    pub helix1: i64,
    /// java: @SerializedName("helix_2") public long helix2;
    pub helix2: i64,
    /// java: @SerializedName("helix_3") public long helix3;
    pub helix3: i64,
    /// java: @SerializedName("helix_mega") public long helixMega;
    pub helix_mega: i64,
}

impl SmearedTrees {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fig1: 0,
            fig2: 0,
            fig3: 0,
            mangrove1: 0,
            mangrove2: 0,
            mangrove3: 0,
            helix1: 0,
            helix2: 0,
            helix3: 0,
            helix_mega: 0,
        };
        this
    }

}

impl Foraging {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "fish_family") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.fish_family = __x;
            }
        }
        if let Some(__v) = gson::member(json, "starlyn") {
            if let Some(__x) = Some(Starlyn::from_json(__v)) {
                self.starlyn = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hina") {
            if let Some(__x) = Some(Hina::from_json(__v)) {
                self.hina = __x;
            }
        }
        if let Some(__v) = gson::member(json, "tree_gifts") {
            if let Some(__x) = Some(TreeGifts::from_json(__v)) {
                self.tree_gifts = __x;
            }
        }
        if let Some(__v) = gson::member(json, "honey") {
            if let Some(__x) = Some(Honey::from_json(__v)) {
                self.honey = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Foraging.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Starlyn {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "personal_bests") {
            if let Some(__x) = Some(PersonalBests::from_json(__v)) {
                self.personal_bests = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Starlyn.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl PersonalBests {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "agatha") {
            if let Some(__x) = gson::as_i32(__v) {
                self.agatha = __x;
            }
        }
        if let Some(__v) = gson::member(json, "FIG_LOG") {
            if let Some(__x) = gson::as_i32(__v) {
                self.fig_logs = __x;
            }
        }
        if let Some(__v) = gson::member(json, "MANGROVE_LOG") {
            if let Some(__x) = gson::as_i32(__v) {
                self.mangrove_logs = __x;
            }
        }
        if let Some(__v) = gson::member(json, "miria") {
            if let Some(__x) = gson::as_i32(__v) {
                self.miria = __x;
            }
        }
        if let Some(__v) = gson::member(json, "HELIX_LOG") {
            if let Some(__x) = gson::as_i32(__v) {
                self.helix_logs = __x;
            }
        }
    }

    /// `GSON.fromJson(json, PersonalBests.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Hina {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "tasks") {
            if let Some(__x) = Some(Tasks::from_json(__v)) {
                self.tasks = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Hina.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Tasks {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "completed_tasks") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.completed_tasks = __x;
            }
        }
        if let Some(__v) = gson::member(json, "task_progress") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::as_i32(__e0)) {
                self.task_progress = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claimed_rewards") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.claimed_rewards = __x;
            }
        }
        if let Some(__v) = gson::member(json, "tier_claimed") {
            if let Some(__x) = gson::as_i32(__v) {
                self.tier_claimed = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Tasks.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl TreeGifts {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "FIG") {
            if let Some(__x) = gson::as_i32(__v) {
                self.fig = __x;
            }
        }
        if let Some(__v) = gson::member(json, "MANGROVE") {
            if let Some(__x) = gson::as_i32(__v) {
                self.mangrove = __x;
            }
        }
        if let Some(__v) = gson::member(json, "HELIX") {
            if let Some(__x) = gson::as_i32(__v) {
                self.helix = __x;
            }
        }
        if let Some(__v) = gson::member(json, "milestone_tier_claimed") {
            if let Some(__x) = Some(MilestoneClaimed::from_json(__v)) {
                self.milestone_claimed = __x;
            }
        }
    }

    /// `GSON.fromJson(json, TreeGifts.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl MilestoneClaimed {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "FIG") {
            if let Some(__x) = gson::as_i32(__v) {
                self.fig = __x;
            }
        }
        if let Some(__v) = gson::member(json, "MANGROVE") {
            if let Some(__x) = gson::as_i32(__v) {
                self.mangrove = __x;
            }
        }
        if let Some(__v) = gson::member(json, "HELIX") {
            if let Some(__x) = gson::as_i32(__v) {
                self.helix = __x;
            }
        }
    }

    /// `GSON.fromJson(json, MilestoneClaimed.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Honey {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "refill_times") {
            if let Some(__x) = Some(RefillTimes::from_json(__v)) {
                self.honeyhive_refill_times = __x;
            }
        }
        if let Some(__v) = gson::member(json, "smeared_trees") {
            if let Some(__x) = Some(SmearedTrees::from_json(__v)) {
                self.smeared_trees = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Honey.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl RefillTimes {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "hive_1") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_4") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_5") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive5 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_6") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive6 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_7") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive7 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_8") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive8 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_9") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive9 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_10") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive10 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_11") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive11 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_12") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive12 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_13") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive13 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_14") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive14 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_15") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive15 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_16") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive16 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_17") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive17 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_18") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive18 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hive_19") {
            if let Some(__x) = gson::as_i64(__v) {
                self.hive19 = __x;
            }
        }
    }

    /// `GSON.fromJson(json, RefillTimes.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl SmearedTrees {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "fig_1") {
            if let Some(__x) = gson::as_i64(__v) {
                self.fig1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fig_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.fig2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fig_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.fig3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "mangrove_1") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mangrove1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "mangrove_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mangrove2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "mangrove_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.mangrove3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "helix_1") {
            if let Some(__x) = gson::as_i64(__v) {
                self.helix1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "helix_2") {
            if let Some(__x) = gson::as_i64(__v) {
                self.helix2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "helix_3") {
            if let Some(__x) = gson::as_i64(__v) {
                self.helix3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "helix_mega") {
            if let Some(__x) = gson::as_i64(__v) {
                self.helix_mega = __x;
            }
        }
    }

    /// `GSON.fromJson(json, SmearedTrees.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
