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
        // TODO(fabric2seer): Map.subMap(..)
        //   java: return map.subMap(prefix + "_", prefix + ("_" + 1));
        todo!("fabric2seer: the value of this function is in a TODO above")
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
