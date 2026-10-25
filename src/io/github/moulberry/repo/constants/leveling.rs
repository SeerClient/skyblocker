//! Ported from io.github.moulberry.repo.constants.Leveling (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class Leveling
///
/// Every member is in the repository's `constants/leveling.json`; one absent there reads as empty (Java: `null`).
#[derive(Clone, Debug, Default)]
pub struct Leveling {
    /// java: @SerializedName("leveling_xp") List<Integer> skillExperienceRequiredPerLevel;
    pub(crate) skill_experience_required_per_level: Vec<i32>,
    /// java: @SerializedName("leveling_caps") Map<String, Integer> maximumLevels;
    pub(crate) maximum_levels: HashMap<String, i32>,
    /// java: @SerializedName("runecrafting_xp") List<Integer> runecraftingExperienceRequiredPerLevel;
    pub(crate) runecrafting_experience_required_per_level: Vec<i32>,
    /// java: @SerializedName("slayer_xp") Map<String, List<Integer>> slayerExperienceRequiredPerLevel;
    pub(crate) slayer_experience_required_per_level: HashMap<String, Vec<i32>>,
    /// java: @SerializedName("slayer_boss_xp") List<Integer> slayerExperiencePerBoss;
    pub(crate) slayer_experience_per_boss: Vec<i32>,
    /// java: @SerializedName("catacombs") List<Integer> catacombsExperienceRequiredPerLevel;
    pub(crate) catacombs_experience_required_per_level: Vec<i32>,
    /// java: @SerializedName("HOTM") List<Integer> hotmExperienceRequiredPerLevel;
    pub(crate) hotm_experience_required_per_level: Vec<i32>,
    /// java: @SerializedName("social") List<Integer> socialExperienceRequiredPerLevel;
    pub(crate) social_experience_required_per_level: Vec<i32>,
}

fn int_list(json: &Value, name: &str) -> Vec<i32> {
    gson::member(json, name).and_then(|v| gson::list(v, gson::as_i32)).unwrap_or_default()
}

impl Leveling {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        Leveling {
            skill_experience_required_per_level: int_list(json, "leveling_xp"),
            maximum_levels: gson::member(json, "leveling_caps").and_then(|v| gson::map(v, gson::string_key, gson::as_i32)).unwrap_or_default(),
            runecrafting_experience_required_per_level: int_list(json, "runecrafting_xp"),
            slayer_experience_required_per_level: gson::member(json, "slayer_xp")
                .and_then(|v| gson::map(v, gson::string_key, |l| gson::list(l, gson::as_i32)))
                .unwrap_or_default(),
            slayer_experience_per_boss: int_list(json, "slayer_boss_xp"),
            catacombs_experience_required_per_level: int_list(json, "catacombs"),
            hotm_experience_required_per_level: int_list(json, "HOTM"),
            social_experience_required_per_level: int_list(json, "social"),
        }
    }

    /// java: public List<Integer> getSkillExperienceRequiredPerLevel()
    pub fn get_skill_experience_required_per_level(&self) -> &[i32] {
        &self.skill_experience_required_per_level
    }

    /// java: public Map<String, Integer> getMaximumLevels()
    pub fn get_maximum_levels(&self) -> &HashMap<String, i32> {
        &self.maximum_levels
    }

    /// java: public List<Integer> getRunecraftingExperienceRequiredPerLevel()
    pub fn get_runecrafting_experience_required_per_level(&self) -> &[i32] {
        &self.runecrafting_experience_required_per_level
    }

    /// java: public Map<String, List<Integer>> getSlayerExperienceRequiredPerLevel()
    pub fn get_slayer_experience_required_per_level(&self) -> &HashMap<String, Vec<i32>> {
        &self.slayer_experience_required_per_level
    }

    /// java: public List<Integer> getSlayerExperiencePerBoss()
    pub fn get_slayer_experience_per_boss(&self) -> &[i32] {
        &self.slayer_experience_per_boss
    }

    /// java: public List<Integer> getCatacombsExperienceRequiredPerLevel()
    pub fn get_catacombs_experience_required_per_level(&self) -> &[i32] {
        &self.catacombs_experience_required_per_level
    }

    /// java: public List<Integer> getHotmExperienceRequiredPerLevel()
    pub fn get_hotm_experience_required_per_level(&self) -> &[i32] {
        &self.hotm_experience_required_per_level
    }

    /// java: public List<Integer> getSocialExperienceRequiredPerLevel()
    pub fn get_social_experience_required_per_level(&self) -> &[i32] {
        &self.social_experience_required_per_level
    }
}
