//! Ported from io.github.moulberry.repo.constants.Misc (neurepoparser 1.12.0).

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::io::github::moulberry::repo::constants::rank_data::RankData;
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class Misc
///
/// Every member is in the repository's `constants/misc.json`; one absent there reads as empty (Java: `null`).
#[derive(Clone, Debug, Default)]
pub struct Misc {
    /// java: @SerializedName("slayer_cost") List<String> slayerCost;
    pub(crate) slayer_cost: Vec<String>,
    /// java: @SerializedName("area_names") Map<String, String> areaNames;
    pub(crate) area_names: HashMap<String, String>,
    /// java: Map<String, RankData> ranks;
    pub(crate) ranks: HashMap<String, RankData>,
    /// java: @SerializedName("special_bois") List<String> rainbowNames;
    pub(crate) rainbow_names: Vec<String>,
    /// java: @SerializedName("minions") Map<String, Integer> maxMinionLevel;
    pub(crate) max_minion_level: HashMap<String, i32>,
    /// java: NEUItem credits;
    pub(crate) credits: Option<Arc<NEUItem>>,
    /// java: @SerializedName("talisman_upgrades") Map<String, List<String>> talismanUpgrades;
    pub(crate) talisman_upgrades: HashMap<String, Vec<String>>,
    /// java: @SerializedName("ignored_talisman") List<String> ignoredTalismans;
    pub(crate) ignored_talismans: Vec<String>,
}

impl Misc {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        Misc {
            slayer_cost: gson::string_list(json, "slayer_cost").unwrap_or_default(),
            area_names: gson::member(json, "area_names").and_then(|v| gson::map(v, gson::string_key, gson::as_string)).unwrap_or_default(),
            ranks: gson::member(json, "ranks").and_then(|v| gson::map(v, gson::string_key, |r| Some(RankData::from_json(r)))).unwrap_or_default(),
            rainbow_names: gson::string_list(json, "special_bois").unwrap_or_default(),
            max_minion_level: gson::member(json, "minions").and_then(|v| gson::map(v, gson::string_key, gson::as_i32)).unwrap_or_default(),
            credits: gson::member(json, "credits").filter(|v| v.is_object()).map(NEUItem::from_json),
            talisman_upgrades: gson::member(json, "talisman_upgrades")
                .and_then(|v| gson::map(v, gson::string_key, |l| gson::list(l, gson::as_string)))
                .unwrap_or_default(),
            ignored_talismans: gson::string_list(json, "ignored_talisman").unwrap_or_default(),
        }
    }

    /// java: public List<String> getSlayerCost()
    pub fn get_slayer_cost(&self) -> &[String] {
        &self.slayer_cost
    }

    /// java: public Map<String, String> getAreaNames()
    pub fn get_area_names(&self) -> &HashMap<String, String> {
        &self.area_names
    }

    /// java: public Map<String, RankData> getRanks()
    pub fn get_ranks(&self) -> &HashMap<String, RankData> {
        &self.ranks
    }

    /// java: public List<String> getRainbowNames()
    pub fn get_rainbow_names(&self) -> &[String] {
        &self.rainbow_names
    }

    /// java: public Map<String, Integer> getMaxMinionLevel()
    pub fn get_max_minion_level(&self) -> &HashMap<String, i32> {
        &self.max_minion_level
    }

    /// java: public NEUItem getCredits()
    pub fn get_credits(&self) -> Option<&Arc<NEUItem>> {
        self.credits.as_ref()
    }

    /// java: public Map<String, List<String>> getTalismanUpgrades()
    pub fn get_talisman_upgrades(&self) -> &HashMap<String, Vec<String>> {
        &self.talisman_upgrades
    }

    /// java: public List<String> getIgnoredTalismans()
    pub fn get_ignored_talismans(&self) -> &[String] {
        &self.ignored_talismans
    }
}
