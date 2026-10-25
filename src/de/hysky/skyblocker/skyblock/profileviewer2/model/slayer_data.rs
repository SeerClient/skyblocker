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

impl SlayerData {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "slayer_bosses") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(SlayerBoss::from_json(__e0))) {
                self.slayer_bosses = __x;
            }
        }
    }

    /// `GSON.fromJson(json, SlayerData.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
