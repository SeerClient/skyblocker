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

impl CommunityUpgrades {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "currently_upgrading") {
            if let Some(__x) = (Some(CurrentlyUpgrading::from_json(__v))).map(Some) {
                self.currently_upgrading = __x;
            }
        }
        if let Some(__v) = gson::member(json, "upgrade_states") {
            if let Some(__x) = gson::list(__v, |__e0| Some(UpgradeState::from_json(__e0))) {
                self.upgrade_states = __x;
            }
        }
    }

    /// `GSON.fromJson(json, CommunityUpgrades.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl CurrentlyUpgrading {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "upgrade") {
            if let Some(__x) = gson::as_string(__v) {
                self.upgrade_id = __x;
            }
        }
        if let Some(__v) = gson::member(json, "new_tier") {
            if let Some(__x) = gson::as_i32(__v) {
                self.new_tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "start_ms") {
            if let Some(__x) = gson::as_i64(__v) {
                self.started_at = __x;
            }
        }
        if let Some(__v) = gson::member(json, "who_started") {
            if let Some(__x) = crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid(__v) {
                self.started_by = __x;
            }
        }
    }

    /// `GSON.fromJson(json, CurrentlyUpgrading.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl UpgradeState {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "upgrade") {
            if let Some(__x) = gson::as_string(__v) {
                self.upgrade_id = __x;
            }
        }
        if let Some(__v) = gson::member(json, "tier") {
            if let Some(__x) = gson::as_i32(__v) {
                self.tier = __x;
            }
        }
        if let Some(__v) = gson::member(json, "started_ms") {
            if let Some(__x) = gson::as_i64(__v) {
                self.started_at = __x;
            }
        }
        if let Some(__v) = gson::member(json, "started_by") {
            if let Some(__x) = crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid(__v) {
                self.started_by = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claimed_by") {
            if let Some(__x) = crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer::json_uuid(__v) {
                self.claimed_by = __x;
            }
        }
    }

    /// `GSON.fromJson(json, UpgradeState.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
