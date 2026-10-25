//! Ported from io.github.moulberry.repo.constants.PetLevelingData (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::constants::pet_leveling_behaviour_override::PetLevelingBehaviourOverride;
use crate::io::github::moulberry::repo::data::rarity::Rarity;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class PetLevelingData
///
/// Every member is in the repository's `constants/pets.json`; one absent there reads as empty (Java: `null`).
#[derive(Clone, Debug, Default)]
pub struct PetLevelingData {
    /// java: @SerializedName("pet_rarity_offset") Map<Rarity, Integer> petLevelStartOffset;
    pub(crate) pet_level_start_offset: HashMap<Rarity, i32>,
    /// java: @SerializedName("pet_levels") List<Integer> petExpCostForLevel;
    pub(crate) pet_exp_cost_for_level: Vec<i32>,
    /// java: @SerializedName("custom_pet_leveling") Map<@PetId String, PetLevelingBehaviourOverride> petLevelingBehaviourOverrides;
    pub(crate) pet_leveling_behaviour_overrides: HashMap<String, PetLevelingBehaviourOverride>,
    /// java: @SerializedName("pet_types") Map<@PetId String, String> petExpTypes;
    pub(crate) pet_exp_types: HashMap<String, String>,
}

impl PetLevelingData {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        PetLevelingData {
            pet_level_start_offset: gson::member(json, "pet_rarity_offset")
                .and_then(|v| gson::map(v, Rarity::value_of, gson::as_i32))
                .unwrap_or_default(),
            pet_exp_cost_for_level: gson::member(json, "pet_levels").and_then(|v| gson::list(v, gson::as_i32)).unwrap_or_default(),
            pet_leveling_behaviour_overrides: gson::member(json, "custom_pet_leveling")
                .and_then(|v| gson::map(v, gson::string_key, |o| Some(PetLevelingBehaviourOverride::from_json(o))))
                .unwrap_or_default(),
            pet_exp_types: gson::member(json, "pet_types").and_then(|v| gson::map(v, gson::string_key, gson::as_string)).unwrap_or_default(),
        }
    }

    /// java: public Map<Rarity, Integer> getPetLevelStartOffset()
    pub fn get_pet_level_start_offset(&self) -> &HashMap<Rarity, i32> {
        &self.pet_level_start_offset
    }

    /// java: public List<Integer> getPetExpCostForLevel()
    pub fn get_pet_exp_cost_for_level(&self) -> &[i32] {
        &self.pet_exp_cost_for_level
    }

    /// java: public Map<@PetId String, PetLevelingBehaviourOverride> getPetLevelingBehaviourOverrides()
    pub fn get_pet_leveling_behaviour_overrides(&self) -> &HashMap<String, PetLevelingBehaviourOverride> {
        &self.pet_leveling_behaviour_overrides
    }

    /// java: public Map<@PetId String, String> getPetExpTypes()
    pub fn get_pet_exp_types(&self) -> &HashMap<String, String> {
        &self.pet_exp_types
    }
}
