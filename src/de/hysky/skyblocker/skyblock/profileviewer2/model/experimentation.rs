//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Experimentation by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class Experimentation
#[derive(Clone)]
pub struct Experimentation {
    /// java: @SerializedName("claims_resets") public int claimsReset;
    pub claims_reset: i32,
    /// java: @SerializedName("claims_resets_timestamp") public long claimsResetTimestamp;
    pub claims_reset_timestamp: i64,
    /// java: @SerializedName("serums_drank") public int serumsDrank;
    pub serums_drank: i32,
    /// java: @SerializedName("claimed_retroactive_rng") public boolean claimedRetroactiveRng;
    pub claimed_retroactive_rng: bool,
    /// java: @SerializedName("charge_track_timestamp") public long chargeTrackTimestamp;
    pub charge_track_timestamp: i64,
    /// java: public ExperimentationStats numbers = new ExperimentationStats();
    pub numbers: compat::Shared<ExperimentationStats>,
    /// java: public ExperimentationStats pairings = new ExperimentationStats();
    pub pairings: compat::Shared<ExperimentationStats>,
    /// java: public ExperimentationStats simon = new ExperimentationStats();
    pub simon: compat::Shared<ExperimentationStats>,
}

impl Experimentation {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            claims_reset: 0,
            claims_reset_timestamp: 0,
            serums_drank: 0,
            claimed_retroactive_rng: false,
            charge_track_timestamp: 0,
            numbers: ExperimentationStats::new(),
            pairings: ExperimentationStats::new(),
            simon: ExperimentationStats::new(),
        };
        this
    }

}

/// java: public static class ExperimentationStats
#[derive(Clone)]
pub struct ExperimentationStats {
    /// java: @SerializedName("last_attempt") public int lastAttempt;
    pub last_attempt: i32,
    /// java: @SerializedName("attempts_1") public int attempts1;
    pub attempts1: i32,
    /// java: @SerializedName("claims_1") public int claims1;
    pub claims1: i32,
    /// java: @SerializedName("best_score_1") public int bestScore1;
    pub best_score1: i32,
    /// java: @SerializedName("attempts_2") public int attempts2;
    pub attempts2: i32,
    /// java: @SerializedName("claims_2") public int claims2;
    pub claims2: i32,
    /// java: @SerializedName("best_score_2") public int bestScore2;
    pub best_score2: i32,
    /// java: @SerializedName("attempts_3") public int attempts3;
    pub attempts3: i32,
    /// java: @SerializedName("claims_3") public int claims3;
    pub claims3: i32,
    /// java: @SerializedName("best_score_3") public int bestScore3;
    pub best_score3: i32,
    /// java: @SerializedName("bonus_clicks") public int bonusClicks;
    pub bonus_clicks: i32,
    /// java: @SerializedName("last_claimed") public long lastClaimed;
    pub last_claimed: i64,
    /// java: public boolean claimed;
    pub claimed: bool,
}

impl ExperimentationStats {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            last_attempt: 0,
            attempts1: 0,
            claims1: 0,
            best_score1: 0,
            attempts2: 0,
            claims2: 0,
            best_score2: 0,
            attempts3: 0,
            claims3: 0,
            best_score3: 0,
            bonus_clicks: 0,
            last_claimed: 0,
            claimed: false,
        });
        __this
    }

}

impl Experimentation {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "claims_resets") {
            if let Some(__x) = gson::as_i32(__v) {
                self.claims_reset = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claims_resets_timestamp") {
            if let Some(__x) = gson::as_i64(__v) {
                self.claims_reset_timestamp = __x;
            }
        }
        if let Some(__v) = gson::member(json, "serums_drank") {
            if let Some(__x) = gson::as_i32(__v) {
                self.serums_drank = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claimed_retroactive_rng") {
            if let Some(__x) = gson::as_bool(__v) {
                self.claimed_retroactive_rng = __x;
            }
        }
        if let Some(__v) = gson::member(json, "charge_track_timestamp") {
            if let Some(__x) = gson::as_i64(__v) {
                self.charge_track_timestamp = __x;
            }
        }
        if let Some(__v) = gson::member(json, "numbers") {
            if let Some(__x) = Some(ExperimentationStats::from_json(__v)) {
                self.numbers = __x;
            }
        }
        if let Some(__v) = gson::member(json, "pairings") {
            if let Some(__x) = Some(ExperimentationStats::from_json(__v)) {
                self.pairings = __x;
            }
        }
        if let Some(__v) = gson::member(json, "simon") {
            if let Some(__x) = Some(ExperimentationStats::from_json(__v)) {
                self.simon = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Experimentation.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl ExperimentationStats {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "last_attempt") {
            if let Some(__x) = gson::as_i32(__v) {
                self.last_attempt = __x;
            }
        }
        if let Some(__v) = gson::member(json, "attempts_1") {
            if let Some(__x) = gson::as_i32(__v) {
                self.attempts1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claims_1") {
            if let Some(__x) = gson::as_i32(__v) {
                self.claims1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "best_score_1") {
            if let Some(__x) = gson::as_i32(__v) {
                self.best_score1 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "attempts_2") {
            if let Some(__x) = gson::as_i32(__v) {
                self.attempts2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claims_2") {
            if let Some(__x) = gson::as_i32(__v) {
                self.claims2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "best_score_2") {
            if let Some(__x) = gson::as_i32(__v) {
                self.best_score2 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "attempts_3") {
            if let Some(__x) = gson::as_i32(__v) {
                self.attempts3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claims_3") {
            if let Some(__x) = gson::as_i32(__v) {
                self.claims3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "best_score_3") {
            if let Some(__x) = gson::as_i32(__v) {
                self.best_score3 = __x;
            }
        }
        if let Some(__v) = gson::member(json, "bonus_clicks") {
            if let Some(__x) = gson::as_i32(__v) {
                self.bonus_clicks = __x;
            }
        }
        if let Some(__v) = gson::member(json, "last_claimed") {
            if let Some(__x) = gson::as_i64(__v) {
                self.last_claimed = __x;
            }
        }
        if let Some(__v) = gson::member(json, "claimed") {
            if let Some(__x) = gson::as_bool(__v) {
                self.claimed = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ExperimentationStats.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> compat::Shared<Self> {
        let __this = Self::new();
        compat::write(&__this, |__t| __t.read_json_fields(json));
        __this
    }
}
