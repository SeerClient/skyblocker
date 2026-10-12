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
