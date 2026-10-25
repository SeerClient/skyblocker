//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Bestiary by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Bestiary
#[derive(Clone)]
pub struct Bestiary {
    /// java: public NavigableMap<String, Object> kills = new TreeMap<>();
    pub kills: std::collections::BTreeMap<String, compat::Object>,
    /// java: public NavigableMap<String, Object> deaths = new TreeMap<>();
    pub deaths: std::collections::BTreeMap<String, compat::Object>,
    /// java: public boolean migrated_stats = true;
    pub migrated_stats: bool,
    /// java: public Milestone milestone = new Milestone();
    pub milestone: Milestone,
}

impl Milestone {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`); an absent member leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "last_claimed_milestone") {
            if let Some(__x) = gson::as_i32(__v) {
                self.last_claimed_milestone = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Milestone.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl Bestiary {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            kills: std::collections::BTreeMap::new(),
            deaths: std::collections::BTreeMap::new(),
            migrated_stats: true,
            milestone: Milestone::new(),
        };
        this
    }

    /// java: private static Map<String, Object> getPrefixMap(NavigableMap<String, Object> map, String prefix)
    pub fn get_prefix_map(map: &std::collections::BTreeMap<String, compat::Object>, prefix: &str) -> std::collections::HashMap<String, compat::Object> {
        // java: return map.subMap(prefix + "_", prefix + ("_" + 1)); - the keys from `prefix_` (inclusive) to `prefix_1`
        // (exclusive)
        let from = format!("{prefix}_");
        let to = format!("{prefix}_1");
        map.range(from..to).map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// java: public Map<String, Object> getAllKills(String mobKind)
    pub fn get_all_kills(&self, mob_kind: &str) -> std::collections::HashMap<String, compat::Object> {
        return Bestiary::get_prefix_map(&self.kills, mob_kind);
    }

    /// java: public Map<String, Object> getAllDeaths(String mobKind)
    pub fn get_all_deaths(&self, mob_kind: &str) -> std::collections::HashMap<String, compat::Object> {
        return Bestiary::get_prefix_map(&self.deaths, mob_kind);
    }

}

/// java: public static class Milestone
#[derive(Clone)]
pub struct Milestone {
    /// java: @SerializedName("last_claimed_milestone") public int lastClaimedMilestone;
    pub last_claimed_milestone: i32,
}

impl Milestone {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            last_claimed_milestone: 0,
        };
        this
    }

}

impl Bestiary {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "kills") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_object(__e0))).map(|__m| __m.into_iter().collect()) {
                self.kills = __x;
            }
        }
        if let Some(__v) = gson::member(json, "deaths") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_object(__e0))).map(|__m| __m.into_iter().collect()) {
                self.deaths = __x;
            }
        }
        if let Some(__v) = gson::member(json, "migrated_stats") {
            if let Some(__x) = gson::as_bool(__v) {
                self.migrated_stats = __x;
            }
        }
        if let Some(__v) = gson::member(json, "milestone") {
            if let Some(__x) = Some(Milestone::from_json(__v)) {
                self.milestone = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Bestiary.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
