//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.NetherIslandPlayerData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class NetherIslandPlayerData
#[derive(Clone)]
pub struct NetherIslandPlayerData {
    /// java: @SerializedName("selected_faction") public String selectedFaction = "";
    pub selected_faction: String,
    /// java: @SerializedName("mages_reputation") public int magesReputation;
    pub mages_reputation: i32,
    /// java: @SerializedName("barbarians_reputation") public int barbariansReputation;
    pub barbarians_reputation: i32,
    /// java: public Matriarch matriarch = new Matriarch();
    pub matriarch: Matriarch,
    /// java: @SerializedName("last_minibosses_killed") public List<String> lastMinibossesKilled = List.of();
    pub last_minibosses_killed: Vec<String>,
    /// java: public Dojo dojo = new Dojo();
    pub dojo: Dojo,
    /// java: @SerializedName("kuudra_completed_tiers") public KuudraCompletedTiers kuudraCompletedTiers = new KuudraCompletedTiers();
    pub kuudra_completed_tiers: KuudraCompletedTiers,
    /// java: @SerializedName("kuudra_party_finder") public KuudraPartyFinder kuudraPartyFinder = new KuudraPartyFinder();
    pub kuudra_party_finder: KuudraPartyFinder,
    /// java: public Abiphone abiphone = new Abiphone();
    pub abiphone: Abiphone,
    /// java: public Quests quests = new Quests();
    pub quests: Quests,
}

impl NetherIslandPlayerData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            selected_faction: "".to_string(),
            mages_reputation: 0,
            barbarians_reputation: 0,
            matriarch: Matriarch::new(),
            last_minibosses_killed: vec![],
            dojo: Dojo::new(),
            kuudra_completed_tiers: KuudraCompletedTiers::new(),
            kuudra_party_finder: KuudraPartyFinder::new(),
            abiphone: Abiphone::new(),
            quests: Quests::new(),
        };
        this
    }

}

/// java: public static class Matriarch
#[derive(Clone)]
pub struct Matriarch {
    /// java: @SerializedName("pearls_collected") public int pearlsCollected;
    pub pearls_collected: i32,
    /// java: @SerializedName("last_attempt") public long lastAttempt;
    pub last_attempt: i64,
    /// java: @SerializedName("recent_refreshes") public List<Long> recentRefreshes = List.of();
    pub recent_refreshes: Vec<i64>,
}

impl Matriarch {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            pearls_collected: 0,
            last_attempt: 0,
            recent_refreshes: vec![],
        };
        this
    }

}

/// java: public static class Dojo
#[derive(Clone)]
pub struct Dojo {
    /// java: @SerializedName("dojo_points_mob_kb") public int testOfForce;
    pub test_of_force: i32,
    /// java: @SerializedName("dojo_points_wall_jump") public int testOfStamina;
    pub test_of_stamina: i32,
    /// java: @SerializedName("dojo_points_archer") public int testOfMastery;
    pub test_of_mastery: i32,
    /// java: @SerializedName("dojo_points_sword_swap") public int testOfDiscipline;
    pub test_of_discipline: i32,
    /// java: @SerializedName("dojo_points_snake") public int testOfSwiftness;
    pub test_of_swiftness: i32,
    /// java: @SerializedName("dojo_points_lock_head") public int testOfControl;
    pub test_of_control: i32,
    /// java: @SerializedName("dojo_points_fireball") public int testOfTenacity;
    pub test_of_tenacity: i32,
    /// java: @SerializedName("dojo_time_mob_kb") public int testOfForceTime;
    pub test_of_force_time: i32,
    /// java: @SerializedName("dojo_time_wall_jump") public int testOfStaminaTime;
    pub test_of_stamina_time: i32,
    /// java: @SerializedName("dojo_time_archer") public int testOfMasteryTime;
    pub test_of_mastery_time: i32,
    /// java: @SerializedName("dojo_time_sword_swap") public int testOfDisciplineTime;
    pub test_of_discipline_time: i32,
    /// java: @SerializedName("dojo_time_snake") public int testOfSwiftnessTime;
    pub test_of_swiftness_time: i32,
    /// java: @SerializedName("dojo_time_lock_head") public int testOfControlTime;
    pub test_of_control_time: i32,
    /// java: @SerializedName("dojo_time_fireball") public int testOfTenacityTime;
    pub test_of_tenacity_time: i32,
}

impl Dojo {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            test_of_force: 0,
            test_of_stamina: 0,
            test_of_mastery: 0,
            test_of_discipline: 0,
            test_of_swiftness: 0,
            test_of_control: 0,
            test_of_tenacity: 0,
            test_of_force_time: 0,
            test_of_stamina_time: 0,
            test_of_mastery_time: 0,
            test_of_discipline_time: 0,
            test_of_swiftness_time: 0,
            test_of_control_time: 0,
            test_of_tenacity_time: 0,
        };
        this
    }

}

/// java: public static class KuudraCompletedTiers
#[derive(Clone)]
pub struct KuudraCompletedTiers {
    /// java: @SerializedName("none") public int basicTier;
    pub basic_tier: i32,
    /// java: @SerializedName("hot") public int hotTier;
    pub hot_tier: i32,
    /// java: @SerializedName("burning") public int burningTier;
    pub burning_tier: i32,
    /// java: @SerializedName("fiery") public int fieryTier;
    pub fiery_tier: i32,
    /// java: @SerializedName("infernal") public int infernalTier;
    pub infernal_tier: i32,
    /// java: @SerializedName("highest_wave_none") public int highestBasicWave;
    pub highest_basic_wave: i32,
    /// java: @SerializedName("highest_wave_hot") public int highestHotWave;
    pub highest_hot_wave: i32,
    /// java: @SerializedName("highest_wave_burning") public int highestBurningWave;
    pub highest_burning_wave: i32,
    /// java: @SerializedName("highest_wave_fiery") public int highestFieryWave;
    pub highest_fiery_wave: i32,
    /// java: @SerializedName("highest_wave_infernal") public int highestInfernalWave;
    pub highest_infernal_wave: i32,
}

impl KuudraCompletedTiers {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            basic_tier: 0,
            hot_tier: 0,
            burning_tier: 0,
            fiery_tier: 0,
            infernal_tier: 0,
            highest_basic_wave: 0,
            highest_hot_wave: 0,
            highest_burning_wave: 0,
            highest_fiery_wave: 0,
            highest_infernal_wave: 0,
        };
        this
    }

}

/// java: public static class KuudraPartyFinder
#[derive(Clone)]
pub struct KuudraPartyFinder {
    /// java: @SerializedName("search_settings") public SearchSettings searchSettings = new SearchSettings();
    pub search_settings: SearchSettings,
    /// java: @SerializedName("group_builder") public GroupBuilder groupBuilder = new GroupBuilder();
    pub group_builder: GroupBuilder,
}

impl KuudraPartyFinder {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            search_settings: SearchSettings::new(),
            group_builder: GroupBuilder::new(),
        };
        this
    }

}

/// java: public static class SearchSettings
#[derive(Clone)]
pub struct SearchSettings {
    /// java: public String search = "";
    pub search: String,
}

impl SearchSettings {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            search: "".to_string(),
        };
        this
    }

}

/// java: public static class GroupBuilder
#[derive(Clone)]
pub struct GroupBuilder {
    /// java: public String tier = "";
    pub tier: String,
    /// java: public String note = "";
    pub note: String,
    /// java: @SerializedName("combat_level_required") public int combatLevelRequired;
    pub combat_level_required: i32,
}

impl GroupBuilder {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            tier: "".to_string(),
            note: "".to_string(),
            combat_level_required: 0,
        };
        this
    }

}

/// java: public static class Abiphone
#[derive(Clone)]
pub struct Abiphone {
    /// java: @SerializedName("last_dye_called_year") public int lastDyeCalledYear;
    pub last_dye_called_year: i32,
    /// java: @SerializedName("has_used_sirius_personal_phone_number_item") public boolean hasSiriusContactedUnlocked;
    pub has_sirius_contacted_unlocked: bool,
    /// java: @SerializedName("selected_sort") public String selectedSort = "";
    pub selected_sort: String,
    /// java: @SerializedName("selected_ringtone") public String selectedRingtone = "";
    pub selected_ringtone: String,
    /// java: @SerializedName("trio_contact_addons") public int contactTrio;
    pub contact_trio: i32,
    /// java: @SerializedName("active_contacts") public List<String> activeContacts = List.of();
    pub active_contacts: Vec<String>,
    /// java: @SerializedName("operator_chip") public OperatorChip operatorChip = new OperatorChip();
    pub operator_chip: OperatorChip,
    /// java: public Games games = new Games();
    pub games: Games,
    /// java: @SerializedName("contact_data") public Map<String, ContactData> contactData = Map.of();
    pub contact_data: std::collections::HashMap<String, ContactData>,
}

impl Abiphone {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            last_dye_called_year: 0,
            has_sirius_contacted_unlocked: false,
            selected_sort: "".to_string(),
            selected_ringtone: "".to_string(),
            contact_trio: 0,
            active_contacts: vec![],
            operator_chip: OperatorChip::new(),
            games: Games::new(),
            contact_data: std::collections::HashMap::from([]),
        };
        this
    }

}

/// java: public static class OperatorChip
#[derive(Clone)]
pub struct OperatorChip {
    /// java: @SerializedName("repaired_index") public int repairedIndex;
    pub repaired_index: i32,
}

impl OperatorChip {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            repaired_index: 0,
        };
        this
    }

}

/// java: public static class Games
#[derive(Clone)]
pub struct Games {
    /// java: @SerializedName("tic_tac_toe_draws") public int ticTacToeDraws;
    pub tic_tac_toe_draws: i32,
    /// java: @SerializedName("tic_tac_toe_losses") public int ticTacToeLoses;
    pub tic_tac_toe_loses: i32,
    /// java: @SerializedName("snake_best_score") public int snakeBestScore;
    pub snake_best_score: i32,
}

impl Games {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            tic_tac_toe_draws: 0,
            tic_tac_toe_loses: 0,
            snake_best_score: 0,
        };
        this
    }

}

/// java: public static class ContactData
#[derive(Clone)]
pub struct ContactData {
    /// java: @SerializedName("talked_to") public @Nullable Boolean talkedTo;
    pub talked_to: Option<bool>,
    /// java: @SerializedName("completed_quest") public @Nullable Boolean completedQuest;
    pub completed_quest: Option<bool>,
    /// java: @SerializedName("last_call") public @Nullable Long lastCall;
    pub last_call: Option<i64>,
    /// java: @SerializedName("dnd_enabled") public @Nullable Boolean dndEnabled;
    pub dnd_enabled: Option<bool>,
    /// java: @SerializedName("incoming_calls_count") public @Nullable Integer incomingCallsCount;
    pub incoming_calls_count: Option<i32>,
    /// java: @SerializedName("last_call_incoming") public @Nullable Long lastCallIncoming;
    pub last_call_incoming: Option<i64>,
    /// java: public @Nullable SpecificContactData specific;
    pub specific: Option<SpecificContactData>,
}

impl ContactData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            talked_to: None,
            completed_quest: None,
            last_call: None,
            dnd_enabled: None,
            incoming_calls_count: None,
            last_call_incoming: None,
            specific: None,
        };
        this
    }

}

/// java: public static class SpecificContactData
#[derive(Clone)]
pub struct SpecificContactData {
    /// java: @SerializedName("unlocked_target_practice_iv") public @Nullable Boolean unlockedTargetPracticeIv;
    pub unlocked_target_practice_iv: Option<bool>,
    /// java: @SerializedName("last_reward_year") public @Nullable Integer lastRewardYear;
    pub last_reward_year: Option<i32>,
    /// java: @SerializedName("last_mistake") public @Nullable Long lastMistake;
    pub last_mistake: Option<i64>,
    /// java: @SerializedName("color_index_given") public @Nullable Integer colorIndexGiven;
    pub color_index_given: Option<i32>,
    /// java: @SerializedName("gave_saving_grace") public @Nullable Boolean gaveSavingGrace;
    pub gave_saving_grace: Option<bool>,
}

impl SpecificContactData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            unlocked_target_practice_iv: None,
            last_reward_year: None,
            last_mistake: None,
            color_index_given: None,
            gave_saving_grace: None,
        };
        this
    }

}

/// java: public static class Quests
#[derive(Clone)]
pub struct Quests {
    /// java: @SerializedName("found_kuudra_helmet") public boolean foundKuudraHelmet;
    pub found_kuudra_helmet: bool,
    /// java: @SerializedName("found_kuudra_chestplate") public boolean foundKuudraChestplate;
    pub found_kuudra_chestplate: bool,
    /// java: @SerializedName("found_kuudra_leggings") public boolean foundKuudraLeggings;
    pub found_kuudra_leggings: bool,
    /// java: @SerializedName("found_kuudra_boots") public boolean foundKuudraBoots;
    pub found_kuudra_boots: bool,
    /// java: @SerializedName("weird_sailor") public boolean weirdSailor;
    pub weird_sailor: bool,
    /// java: @SerializedName("fished_wet_napkin") public boolean fishedWetNapkin;
    pub fished_wet_napkin: bool,
    /// java: @SerializedName("cavity_rarity") public String cavityRarity = "";
    pub cavity_rarity: String,
}

impl Quests {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            found_kuudra_helmet: false,
            found_kuudra_chestplate: false,
            found_kuudra_leggings: false,
            found_kuudra_boots: false,
            weird_sailor: false,
            fished_wet_napkin: false,
            cavity_rarity: "".to_string(),
        };
        this
    }

}

impl NetherIslandPlayerData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "selected_faction") {
            if let Some(__x) = gson::as_string(__v) {
                self.selected_faction = __x;
            }
        }
        if let Some(__v) = gson::member(json, "mages_reputation") {
            if let Some(__x) = gson::as_i32(__v) {
                self.mages_reputation = __x;
            }
        }
        if let Some(__v) = gson::member(json, "barbarians_reputation") {
            if let Some(__x) = gson::as_i32(__v) {
                self.barbarians_reputation = __x;
            }
        }
        if let Some(__v) = gson::member(json, "matriarch") {
            if let Some(__x) = Some(Matriarch::from_json(__v)) {
                self.matriarch = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_minibosses_killed") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.last_minibosses_killed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo") {
            if let Some(__x) = Some(Dojo::from_json(__v)) {
                self.dojo = __x;
            }
        }
        if let Some(__v) = gson::member(json, "kuudra_completed_tiers") {
            if let Some(__x) = Some(KuudraCompletedTiers::from_json(__v)) {
                self.kuudra_completed_tiers = __x;
            }
        }
        if let Some(__v) = gson::member(json, "kuudra_party_finder") {
            if let Some(__x) = Some(KuudraPartyFinder::from_json(__v)) {
                self.kuudra_party_finder = __x;
            }
        }
        if let Some(__v) = gson::member(json, "abiphone") {
            if let Some(__x) = Some(Abiphone::from_json(__v)) {
                self.abiphone = __x;
            }
        }
        if let Some(__v) = gson::member(json, "quests") {
            if let Some(__x) = Some(Quests::from_json(__v)) {
                self.quests = __x;
            }
        }
    }

    /// `GSON.fromJson(json, NetherIslandPlayerData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Matriarch {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "pearls_collected") {
            if let Some(__x) = gson::as_i32(__v) {
                self.pearls_collected = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_attempt") {
            if let Some(__x) = gson::as_i64(__v) {
                self.last_attempt = __x;
            }
        }
        if let Some(__v) = gson::member(json, "recent_refreshes") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_i64(__e0)) {
                self.recent_refreshes = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Matriarch.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Dojo {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "dojo_points_mob_kb") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_force = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_points_wall_jump") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_stamina = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_points_archer") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_mastery = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_points_sword_swap") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_discipline = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_points_snake") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_swiftness = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_points_lock_head") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_control = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_points_fireball") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_tenacity = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_time_mob_kb") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_force_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_time_wall_jump") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_stamina_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_time_archer") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_mastery_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_time_sword_swap") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_discipline_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_time_snake") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_swiftness_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_time_lock_head") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_control_time = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dojo_time_fireball") {
            if let Some(__x) = gson::as_i32(__v) {
                self.test_of_tenacity_time = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Dojo.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl KuudraCompletedTiers {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "none") {
            if let Some(__x) = gson::as_i32(__v) {
                self.basic_tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "hot") {
            if let Some(__x) = gson::as_i32(__v) {
                self.hot_tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "burning") {
            if let Some(__x) = gson::as_i32(__v) {
                self.burning_tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fiery") {
            if let Some(__x) = gson::as_i32(__v) {
                self.fiery_tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "infernal") {
            if let Some(__x) = gson::as_i32(__v) {
                self.infernal_tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "highest_wave_none") {
            if let Some(__x) = gson::as_i32(__v) {
                self.highest_basic_wave = __x;
            }
        }
        if let Some(__v) = gson::member(json, "highest_wave_hot") {
            if let Some(__x) = gson::as_i32(__v) {
                self.highest_hot_wave = __x;
            }
        }
        if let Some(__v) = gson::member(json, "highest_wave_burning") {
            if let Some(__x) = gson::as_i32(__v) {
                self.highest_burning_wave = __x;
            }
        }
        if let Some(__v) = gson::member(json, "highest_wave_fiery") {
            if let Some(__x) = gson::as_i32(__v) {
                self.highest_fiery_wave = __x;
            }
        }
        if let Some(__v) = gson::member(json, "highest_wave_infernal") {
            if let Some(__x) = gson::as_i32(__v) {
                self.highest_infernal_wave = __x;
            }
        }
    }

    /// `GSON.fromJson(json, KuudraCompletedTiers.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl KuudraPartyFinder {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "search_settings") {
            if let Some(__x) = Some(SearchSettings::from_json(__v)) {
                self.search_settings = __x;
            }
        }
        if let Some(__v) = gson::member(json, "group_builder") {
            if let Some(__x) = Some(GroupBuilder::from_json(__v)) {
                self.group_builder = __x;
            }
        }
    }

    /// `GSON.fromJson(json, KuudraPartyFinder.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl SearchSettings {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "search") {
            if let Some(__x) = gson::as_string(__v) {
                self.search = __x;
            }
        }
    }

    /// `GSON.fromJson(json, SearchSettings.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl GroupBuilder {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "tier") {
            if let Some(__x) = gson::as_string(__v) {
                self.tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "note") {
            if let Some(__x) = gson::as_string(__v) {
                self.note = __x;
            }
        }
        if let Some(__v) = gson::member(json, "combat_level_required") {
            if let Some(__x) = gson::as_i32(__v) {
                self.combat_level_required = __x;
            }
        }
    }

    /// `GSON.fromJson(json, GroupBuilder.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Abiphone {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "last_dye_called_year") {
            if let Some(__x) = gson::as_i32(__v) {
                self.last_dye_called_year = __x;
            }
        }
        if let Some(__v) = gson::member(json, "has_used_sirius_personal_phone_number_item") {
            if let Some(__x) = gson::as_bool(__v) {
                self.has_sirius_contacted_unlocked = __x;
            }
        }
        if let Some(__v) = gson::member(json, "selected_sort") {
            if let Some(__x) = gson::as_string(__v) {
                self.selected_sort = __x;
            }
        }
        if let Some(__v) = gson::member(json, "selected_ringtone") {
            if let Some(__x) = gson::as_string(__v) {
                self.selected_ringtone = __x;
            }
        }
        if let Some(__v) = gson::member(json, "trio_contact_addons") {
            if let Some(__x) = gson::as_i32(__v) {
                self.contact_trio = __x;
            }
        }
        if let Some(__v) = gson::member(json, "active_contacts") {
            if let Some(__x) = gson::list(__v, |__e0| gson::as_string(__e0)) {
                self.active_contacts = __x;
            }
        }
        if let Some(__v) = gson::member(json, "operator_chip") {
            if let Some(__x) = Some(OperatorChip::from_json(__v)) {
                self.operator_chip = __x;
            }
        }
        if let Some(__v) = gson::member(json, "games") {
            if let Some(__x) = Some(Games::from_json(__v)) {
                self.games = __x;
            }
        }
        if let Some(__v) = gson::member(json, "contact_data") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(ContactData::from_json(__e0))) {
                self.contact_data = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Abiphone.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl OperatorChip {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "repaired_index") {
            if let Some(__x) = gson::as_i32(__v) {
                self.repaired_index = __x;
            }
        }
    }

    /// `GSON.fromJson(json, OperatorChip.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Games {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "tic_tac_toe_draws") {
            if let Some(__x) = gson::as_i32(__v) {
                self.tic_tac_toe_draws = __x;
            }
        }
        if let Some(__v) = gson::member(json, "tic_tac_toe_losses") {
            if let Some(__x) = gson::as_i32(__v) {
                self.tic_tac_toe_loses = __x;
            }
        }
        if let Some(__v) = gson::member(json, "snake_best_score") {
            if let Some(__x) = gson::as_i32(__v) {
                self.snake_best_score = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Games.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl ContactData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "talked_to") {
            if let Some(__x) = (gson::as_bool(__v)).map(Some) {
                self.talked_to = __x;
            }
        }
        if let Some(__v) = gson::member(json, "completed_quest") {
            if let Some(__x) = (gson::as_bool(__v)).map(Some) {
                self.completed_quest = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_call") {
            if let Some(__x) = (gson::as_i64(__v)).map(Some) {
                self.last_call = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dnd_enabled") {
            if let Some(__x) = (gson::as_bool(__v)).map(Some) {
                self.dnd_enabled = __x;
            }
        }
        if let Some(__v) = gson::member(json, "incoming_calls_count") {
            if let Some(__x) = (gson::as_i32(__v)).map(Some) {
                self.incoming_calls_count = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_call_incoming") {
            if let Some(__x) = (gson::as_i64(__v)).map(Some) {
                self.last_call_incoming = __x;
            }
        }
        if let Some(__v) = gson::member(json, "specific") {
            if let Some(__x) = (Some(SpecificContactData::from_json(__v))).map(Some) {
                self.specific = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ContactData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl SpecificContactData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "unlocked_target_practice_iv") {
            if let Some(__x) = (gson::as_bool(__v)).map(Some) {
                self.unlocked_target_practice_iv = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_reward_year") {
            if let Some(__x) = (gson::as_i32(__v)).map(Some) {
                self.last_reward_year = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_mistake") {
            if let Some(__x) = (gson::as_i64(__v)).map(Some) {
                self.last_mistake = __x;
            }
        }
        if let Some(__v) = gson::member(json, "color_index_given") {
            if let Some(__x) = (gson::as_i32(__v)).map(Some) {
                self.color_index_given = __x;
            }
        }
        if let Some(__v) = gson::member(json, "gave_saving_grace") {
            if let Some(__x) = (gson::as_bool(__v)).map(Some) {
                self.gave_saving_grace = __x;
            }
        }
    }

    /// `GSON.fromJson(json, SpecificContactData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Quests {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "found_kuudra_helmet") {
            if let Some(__x) = gson::as_bool(__v) {
                self.found_kuudra_helmet = __x;
            }
        }
        if let Some(__v) = gson::member(json, "found_kuudra_chestplate") {
            if let Some(__x) = gson::as_bool(__v) {
                self.found_kuudra_chestplate = __x;
            }
        }
        if let Some(__v) = gson::member(json, "found_kuudra_leggings") {
            if let Some(__x) = gson::as_bool(__v) {
                self.found_kuudra_leggings = __x;
            }
        }
        if let Some(__v) = gson::member(json, "found_kuudra_boots") {
            if let Some(__x) = gson::as_bool(__v) {
                self.found_kuudra_boots = __x;
            }
        }
        if let Some(__v) = gson::member(json, "weird_sailor") {
            if let Some(__x) = gson::as_bool(__v) {
                self.weird_sailor = __x;
            }
        }
        if let Some(__v) = gson::member(json, "fished_wet_napkin") {
            if let Some(__x) = gson::as_bool(__v) {
                self.fished_wet_napkin = __x;
            }
        }
        if let Some(__v) = gson::member(json, "cavity_rarity") {
            if let Some(__x) = gson::as_string(__v) {
                self.cavity_rarity = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Quests.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
