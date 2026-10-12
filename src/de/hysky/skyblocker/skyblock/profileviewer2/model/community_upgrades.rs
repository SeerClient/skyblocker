//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.CommunityUpgrades by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::util::uuid::Uuid;

/// java: public class CommunityUpgrades
#[derive(Clone)]
pub struct CommunityUpgrades {
    /// java: @SerializedName("currently_upgrading") public @Nullable CurrentlyUpgrading currentlyUpgrading;
    pub currently_upgrading: Option<CurrentlyUpgrading>,
    /// java: @SerializedName("upgrade_states") public List<UpgradeState> upgradeStates = List.of();
    pub upgrade_states: Vec<UpgradeState>,
}

impl CommunityUpgrades {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            currently_upgrading: None,
            upgrade_states: vec![],
        };
        this
    }

    /// java: public int getUpgradeTier(String upgradeId)
    pub fn get_upgrade_tier(&self, upgrade_id: &str) -> i32 {
        let mut tier: i32 = 0;
        for upgrade_state in self.upgrade_states.iter() {
            if !(upgrade_id == upgrade_state.upgrade_id) {
                continue;
            }
            tier = tier.max(upgrade_state.tier);
        }
        return tier;
    }

}

/// java: public static class CurrentlyUpgrading
#[derive(Clone)]
pub struct CurrentlyUpgrading {
    /// java: @SerializedName("upgrade") public String upgradeId = "";
    pub upgrade_id: String,
    /// java: @SerializedName("new_tier") public int newTier;
    pub new_tier: i32,
    /// java: @SerializedName("start_ms") public long startedAt;
    pub started_at: i64,
    /// java: @SerializedName("who_started") public UUID startedBy = UUID.randomUUID();
    pub started_by: Uuid,
}

impl CurrentlyUpgrading {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            upgrade_id: "".to_string(),
            new_tier: 0,
            started_at: 0,
            started_by: Uuid::random_uuid(),
        };
        this
    }

}

/// java: public static class UpgradeState
#[derive(Clone)]
pub struct UpgradeState {
    /// java: @SerializedName("upgrade") public String upgradeId = "";
    pub upgrade_id: String,
    /// java: public int tier;
    pub tier: i32,
    /// java: @SerializedName("started_ms") public long startedAt;
    pub started_at: i64,
    /// java: @SerializedName("started_by") public UUID startedBy = UUID.randomUUID();
    pub started_by: Uuid,
    /// java: @SerializedName("claimed_by") public UUID claimedBy = UUID.randomUUID();
    pub claimed_by: Uuid,
}

impl UpgradeState {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            upgrade_id: "".to_string(),
            tier: 0,
            started_at: 0,
            started_by: Uuid::random_uuid(),
            claimed_by: Uuid::random_uuid(),
        };
        this
    }

}
