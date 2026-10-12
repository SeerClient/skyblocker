//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.SlayerData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::slayer_boss::SlayerBoss;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_calculator::LevelCalculator;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::slayers::slayer_type::SlayerType;
use mcrust::java::text::decimal_format_symbols::Locale;

/// java: public class SlayerData
#[derive(Clone)]
pub struct SlayerData {
    /// java: @SerializedName("slayer_bosses") public Map<String, SlayerBoss> slayerBosses = Map.of();
    pub slayer_bosses: std::collections::HashMap<String, SlayerBoss>,
}

impl SlayerData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            slayer_bosses: std::collections::HashMap::from([]),
        };
        this
    }

    /// java: public SlayerBoss getSlayerData(SlayerType slayer)
    pub fn get_slayer_data(&self, mut slayer: SlayerType) -> SlayerBoss {
        return self.slayer_bosses.get(&slayer.fields().friendly_name.to_lowercase()).cloned().unwrap_or_else(|| SlayerBoss::new());
    }

    /// java: public double getSlayerExperience(SlayerType slayer)
    pub fn get_slayer_experience(&self, mut slayer: SlayerType) -> f64 {
        return (self.get_slayer_data(slayer).xp) as f64;
    }

    /// java: public LevelInfo getSkillLevel(SlayerType slayer)
    pub fn get_skill_level(&self, mut slayer: SlayerType) -> LevelInfo {
        return LevelCalculator::get_slayer_level(((self.get_slayer_experience(slayer)) as i64), slayer);
    }

}
