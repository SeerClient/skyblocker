//! Ported from io.github.moulberry.repo.constants.PetLevelingBehaviourOverride (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::data::rarity::Rarity;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class PetLevelingBehaviourOverride
#[derive(Clone, Debug, Default)]
pub struct PetLevelingBehaviourOverride {
    /// java: @SerializedName("xp_multiplier") @Nullable Double xpMultiplier;
    pub(crate) xp_multiplier: Option<f64>,
    /// java: @SerializedName("pet_rarity_offset") Map<Rarity, Integer> petLevelStartOffset;
    pub(crate) pet_level_start_offset: Option<HashMap<Rarity, i32>>,
    /// java: @SerializedName("type") @ApiStatus.Obsolete Integer rawLevelOverrideType;
    pub(crate) raw_level_override_type: Option<i32>,
    /// java: @SerializedName("max_level") Integer maxLevel;
    pub(crate) max_level: Option<i32>,
    /// java: @SerializedName("pet_levels") List<Integer> petExpCostModifier;
    pub(crate) pet_exp_cost_modifier: Option<Vec<i32>>,
}

/// java: public enum PetExpModifierType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PetExpModifierType {
    /// Append the given levels to the base levels.
    Append,
    /// Replace the base levels with the given levels.
    Replace,
}

impl PetLevelingBehaviourOverride {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        PetLevelingBehaviourOverride {
            xp_multiplier: gson::member(json, "xp_multiplier").and_then(gson::as_f64),
            pet_level_start_offset: gson::member(json, "pet_rarity_offset").and_then(|v| gson::map(v, Rarity::value_of, gson::as_i32)),
            raw_level_override_type: gson::integer(json, "type"),
            max_level: gson::integer(json, "max_level"),
            pet_exp_cost_modifier: gson::member(json, "pet_levels").and_then(|v| gson::list(v, gson::as_i32)),
        }
    }

    /// java: public @Nullable PetExpModifierType getPetExpCostModifierType()
    pub fn get_pet_exp_cost_modifier_type(&self) -> Option<PetExpModifierType> {
        match self.raw_level_override_type {
            None => None,
            Some(1) => Some(PetExpModifierType::Append),
            Some(2) => Some(PetExpModifierType::Replace),
            Some(_) => None,
        }
    }

    /// java: public @Nullable Double getXpMultiplier()
    pub fn get_xp_multiplier(&self) -> Option<f64> {
        self.xp_multiplier
    }

    /// java: public Map<Rarity, Integer> getPetLevelStartOffset()
    pub fn get_pet_level_start_offset(&self) -> Option<&HashMap<Rarity, i32>> {
        self.pet_level_start_offset.as_ref()
    }

    /// java: public Integer getRawLevelOverrideType()
    pub fn get_raw_level_override_type(&self) -> Option<i32> {
        self.raw_level_override_type
    }

    /// java: public Integer getMaxLevel()
    pub fn get_max_level(&self) -> Option<i32> {
        self.max_level
    }

    /// java: public List<Integer> getPetExpCostModifier()
    pub fn get_pet_exp_cost_modifier(&self) -> Option<&Vec<i32>> {
        self.pet_exp_cost_modifier.as_ref()
    }
}
