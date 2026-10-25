//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.SlayerBoss by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class SlayerBoss
#[derive(Clone)]
pub struct SlayerBoss {
    /// java: @SerializedName("claimed_levels") public Map<String, Boolean> claimedLevels = Map.of();
    pub claimed_levels: std::collections::HashMap<String, bool>,
    /// java: @SerializedName("boss_kills_tier_0") int bossKills0;
    pub boss_kills0: i32,
    /// java: @SerializedName("boss_kills_tier_1") int bossKills1;
    pub boss_kills1: i32,
    /// java: @SerializedName("boss_kills_tier_2") int bossKills2;
    pub boss_kills2: i32,
    /// java: @SerializedName("boss_kills_tier_3") int bossKills3;
    pub boss_kills3: i32,
    /// java: @SerializedName("boss_kills_tier_4") int bossKills4;
    pub boss_kills4: i32,
    /// java: public int xp;
    pub xp: i32,
}

impl SlayerBoss {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            claimed_levels: std::collections::HashMap::from([]),
            boss_kills0: 0,
            boss_kills1: 0,
            boss_kills2: 0,
            boss_kills3: 0,
            boss_kills4: 0,
            xp: 0,
        };
        this
    }

    /// java: public int getBossKillsByZeroIndexedTier(int tier)
    pub fn get_boss_kills_by_zero_indexed_tier(&self, mut tier: i32) -> i32 {
        return match tier {
            0 => self.boss_kills0,
            1 => self.boss_kills1,
            2 => self.boss_kills2,
            3 => self.boss_kills3,
            4 => self.boss_kills4,
            _ => 0,
        };
    }

    /// java: public int getTotalBossKills()
    pub fn get_total_boss_kills(&self) -> i32 {
        let mut total: i32 = 0;
        for i in 0..=4 {
            total = total.wrapping_add(self.get_boss_kills_by_zero_indexed_tier(i));
        }
        return total;
    }

    /// java: public int getTierWithMostKills()
    pub fn get_tier_with_most_kills(&self) -> i32 {
        {
            let mut i: i32 = 4;
            while i >= 0 {
                if self.get_boss_kills_by_zero_indexed_tier(i) > 1 {
                    return i;
                }
                i -= 1;
            }
        }
        return -1;
    }

}

impl SlayerBoss {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "claimed_levels") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| gson::as_bool(__e0)) {
                self.claimed_levels = __x;
            }
        }
        if let Some(__v) = gson::member(json, "boss_kills_tier_0") {
            if let Some(__x) = gson::as_i32(__v) {
                self.boss_kills0 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "boss_kills_tier_1") {
            if let Some(__x) = gson::as_i32(__v) {
                self.boss_kills1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "boss_kills_tier_2") {
            if let Some(__x) = gson::as_i32(__v) {
                self.boss_kills2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "boss_kills_tier_3") {
            if let Some(__x) = gson::as_i32(__v) {
                self.boss_kills3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "boss_kills_tier_4") {
            if let Some(__x) = gson::as_i32(__v) {
                self.boss_kills4 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "xp") {
            if let Some(__x) = gson::as_i32(__v) {
                self.xp = __x;
            }
        }
    }

    /// `GSON.fromJson(json, SlayerBoss.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
