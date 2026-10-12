//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.PlayerData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_calculator::LevelCalculator;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::skill;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::skill::Skill;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: public class PlayerData
#[derive(Clone)]
pub struct PlayerData {
    /// java: @SerializedName("visited_zones") public Set<String> visitedZones = Set.of();
    pub visited_zones: std::collections::HashSet<String>,
    /// java: @SerializedName("last_death") public long lastDeath;
    pub last_death: i64,
    /// java: @SerializedName("perks") public Map<String, Integer> essencePerks = Map.of();
    pub essence_perks: std::collections::HashMap<String, i32>,
    /// java: @SerializedName("garden_chips") public Map<String, Integer> gardenChips = Map.of();
    pub garden_chips: std::collections::HashMap<String, i32>,
    /// java: @SerializedName("active_effects") public List<Effect> activeEffects = List.of();
    pub active_effects: Vec<Effect>,
    /// java: @SerializedName("reaper_peppers_eaten") public int reaperPeppersEaten;
    pub reaper_peppers_eaten: i32,
    /// java: @SerializedName("isopod_husks_eaten") public int isopodHusksEaten;
    pub isopod_husks_eaten: i32,
    /// java: @SerializedName("bee_saliva_eaten") public int beeSalivaEaten;
    pub bee_saliva_eaten: i32,
    /// java: @SerializedName("death_count") public int deathCount;
    pub death_count: i32,
    /// java: @SerializedName("disabled_potion_effects") public Set<String> disabledPotionEffects = Set.of();
    pub disabled_potion_effects: std::collections::HashSet<String>,
    /// java: @SerializedName("achievement_spawned_island_types") public Set<String> spawnedIslandTypes = Set.of();
    pub spawned_island_types: std::collections::HashSet<String>,
    /// java: @SerializedName("visited_modes") public Set<String> visitedIslandModes = Set.of();
    pub visited_island_modes: std::collections::HashSet<String>,
    /// java: @SerializedName("unlocked_coll_tiers") public Set<String> unlockedCollectionTiers = Set.of();
    pub unlocked_collection_tiers: std::collections::HashSet<String>,
    /// java: @SerializedName("crafted_generators") public Set<String> craftedMinions = Set.of();
    pub crafted_minions: std::collections::HashSet<String>,
    /// java: @SerializedName("fishing_treasure_caught") public int fishingTreasuresCaught;
    pub fishing_treasures_caught: i32,
    /// java: public Map<String, Double> experience = Map.of();
    pub experience: std::collections::HashMap<String, f64>,
}

impl PlayerData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            visited_zones: std::collections::HashSet::from([]),
            last_death: 0,
            essence_perks: std::collections::HashMap::from([]),
            garden_chips: std::collections::HashMap::from([]),
            active_effects: vec![],
            reaper_peppers_eaten: 0,
            isopod_husks_eaten: 0,
            bee_saliva_eaten: 0,
            death_count: 0,
            disabled_potion_effects: std::collections::HashSet::from([]),
            spawned_island_types: std::collections::HashSet::from([]),
            visited_island_modes: std::collections::HashSet::from([]),
            unlocked_collection_tiers: std::collections::HashSet::from([]),
            crafted_minions: std::collections::HashSet::from([]),
            fishing_treasures_caught: 0,
            experience: std::collections::HashMap::from([]),
        };
        this
    }

    /// java: public static boolean hasCraftedMinionTier(List<String> craftedMinions, String minionType, int tier)
    pub fn has_crafted_minion_tier(crafted_minions: &Vec<String>, minion_type: &str, mut tier: i32) -> bool {
        return crafted_minions.iter().any(|e| *e == format!("{}_{}", minion_type, tier));
    }

    /// java: public static int getMinionTier(List<String> craftedMinions, String resource)
    pub fn get_minion_tier(crafted_minions: &Vec<String>, resource: &str) -> i32 {
        let mut i: i32 = 1;
        {
            while i < 15 {
                if !(PlayerData::has_crafted_minion_tier(crafted_minions, resource, i)) {
                    break;
                }
                i += 1;
            }
        }
        return i.wrapping_sub(1);
    }

    /// java: public double getSkillExperience(Skill skill)
    pub fn get_skill_experience(&self, mut skill: Skill) -> f64 {
        return self.experience.get(&format!("SKILL_{}", skill.name())).cloned().unwrap_or_else(|| 0.0);
    }

    /// java: public LevelInfo getSkillLevel(Skill skill, LoadingInformation info)
    pub fn get_skill_level(&self, mut skill: Skill, info: &LoadingInformation) -> LevelInfo {
        return LevelCalculator::get_skill_level(((self.get_skill_experience(skill)) as i64), skill, info);
    }

    /// java: public float getSkillAverage(LoadingInformation info)
    pub fn get_skill_average(&self, info: &LoadingInformation) -> f32 {
        let mut average: f32 = ((0) as f32) as f32;
        for skill in skill::CONVENTIONAL_SKILLS.iter().copied() {
            average = (average + ((self.get_skill_level(skill, info)).level() as f32));
        }
        average = (average / ((skill::CONVENTIONAL_SKILLS.len() as i32) as f32));
        return average;
    }

}

/// java: public static class Effect
#[derive(Clone)]
pub struct Effect {
    /// java: @SerializedName("effect") public String effectId = "";
    pub effect_id: String,
    /// java: public int level = 1;
    pub level: i32,
    /// java: @SerializedName("ticks_remaining") public long ticksRemaining;
    pub ticks_remaining: i64,
    /// java: public boolean infinite;
    pub infinite: bool,
    /// java: public int flags;
    pub flags: i32,
}

impl Effect {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            effect_id: "".to_string(),
            level: 1,
            ticks_remaining: 0,
            infinite: false,
            flags: 0,
        };
        this
    }

}
