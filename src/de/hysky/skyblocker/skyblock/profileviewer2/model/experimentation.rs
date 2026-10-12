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
