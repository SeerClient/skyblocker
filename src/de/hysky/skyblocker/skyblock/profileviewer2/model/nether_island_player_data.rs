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
