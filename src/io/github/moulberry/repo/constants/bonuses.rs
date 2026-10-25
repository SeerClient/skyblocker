//! Ported from io.github.moulberry.repo.constants.Bonuses (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::data::rarity::Rarity;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class Bonuses
#[derive(Clone, Debug, Default)]
pub struct Bonuses {
    /// java: @SerializedName("pet_rewards") Map<Integer, Map<String, Float>> petRewards;
    pub(crate) pet_rewards: Option<HashMap<i32, HashMap<String, f32>>>,
    /// java: @SerializedName("pet_value") Map<Rarity, Integer> petValue;
    pub(crate) pet_value: Option<HashMap<Rarity, i32>>,
    /// java: @SerializedName("bonus_stats") Map<String, Map<Integer, Map<String, Float>>> bonusStats;
    pub(crate) bonus_stats: Option<HashMap<String, HashMap<i32, HashMap<String, f32>>>>,
}

fn stat_map(json: &Value) -> Option<HashMap<String, f32>> {
    gson::map(json, gson::string_key, gson::as_f32)
}

impl Bonuses {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        Bonuses {
            pet_rewards: gson::member(json, "pet_rewards").and_then(|v| gson::map(v, gson::int_key, stat_map)),
            pet_value: gson::member(json, "pet_value").and_then(|v| gson::map(v, Rarity::value_of, gson::as_i32)),
            bonus_stats: gson::member(json, "bonus_stats")
                .and_then(|v| gson::map(v, gson::string_key, |tiers| gson::map(tiers, gson::int_key, stat_map))),
        }
    }

    /// java: public int getPetValue(Rarity rarity)
    pub fn get_pet_value_1(&self, rarity: Rarity) -> i32 {
        // java: if (petValue == null) return 0; return petValue.getOrDefault(rarity, 0);
        match &self.pet_value {
            None => 0,
            Some(pet_value) => pet_value.get(&rarity).copied().unwrap_or(0),
        }
    }

    /// java: public Map<String, Float> getPetRewards(int score)
    pub fn get_pet_rewards_1(&self, score: i32) -> HashMap<String, f32> {
        // java: if (petRewards == null) return new HashMap<>();
        let Some(pet_rewards) = &self.pet_rewards else {
            return HashMap::new();
        };
        // java: petRewards.entrySet().stream().sorted(Comparator.comparingInt(Map.Entry::getKey)).filter(it -> it.getKey() <= score)
        //       .map(Map.Entry::getValue).reduce(new HashMap<>(), (a, b) -> { a.putAll(b); return a; });
        let mut entries: Vec<(&i32, &HashMap<String, f32>)> = pet_rewards.iter().collect();
        entries.sort_by_key(|(k, _)| **k);
        let mut a: HashMap<String, f32> = HashMap::new();
        for (_, b) in entries.into_iter().filter(|(k, _)| **k <= score) {
            a.extend(b.iter().map(|(k, v)| (k.clone(), *v)));
        }
        a
    }

    /// java: public Map<String, Float> getAccumulativeLevelingRewards(String name, int level)
    pub fn get_accumulative_leveling_rewards(&self, name: &str, level: i32) -> HashMap<String, f32> {
        // java: if (bonusStats == null) return new HashMap<>();
        let Some(bonus_stats) = &self.bonus_stats else {
            return HashMap::new();
        };
        let empty: HashMap<i32, HashMap<String, f32>> = HashMap::new();
        // java: Map<Integer, Map<String, Float>> rewardTiers = bonusStats.getOrDefault(name, new HashMap<>());
        let reward_tiers = bonus_stats.get(name).unwrap_or(&empty);
        let mut rewards: HashMap<String, f32> = HashMap::new();
        let no_bonuses: HashMap<String, f32> = HashMap::new();
        let mut current_bonuses: &HashMap<String, f32> = &no_bonuses;
        for i in 0..=level {
            // java: currentBonuses = rewardTiers.getOrDefault(i, currentBonuses);
            current_bonuses = reward_tiers.get(&i).unwrap_or(current_bonuses);
            // java: for (reward : currentBonuses.entrySet()) rewards.put(reward.getKey(), rewards.getOrDefault(reward.getKey(), 0.0F) + reward.getValue());
            for (key, value) in current_bonuses {
                let sum = rewards.get(key).copied().unwrap_or(0.0) + value;
                rewards.insert(key.clone(), sum);
            }
        }
        rewards
    }

    /// java: public Map<Integer, Map<String, Float>> getPetRewards()
    pub fn get_pet_rewards(&self) -> Option<&HashMap<i32, HashMap<String, f32>>> {
        self.pet_rewards.as_ref()
    }

    /// java: public Map<Rarity, Integer> getPetValue()
    pub fn get_pet_value(&self) -> Option<&HashMap<Rarity, i32>> {
        self.pet_value.as_ref()
    }

    /// java: public Map<String, Map<Integer, Map<String, Float>>> getBonusStats()
    pub fn get_bonus_stats(&self) -> Option<&HashMap<String, HashMap<i32, HashMap<String, f32>>>> {
        self.bonus_stats.as_ref()
    }
}
