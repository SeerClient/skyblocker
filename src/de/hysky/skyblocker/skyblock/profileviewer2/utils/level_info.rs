//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.LevelInfo by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public record LevelInfo(long xp, int level, Cap cap, Optional<Progress> progress)
#[derive(Clone, Debug, PartialEq)]
pub struct LevelInfo {
    /// java: long xp
    pub xp: i64,
    /// java: int level
    pub level: i32,
    /// java: Cap cap
    pub cap: Cap,
    /// java: Optional<Progress> progress
    pub progress: Option<Progress>,
}

impl LevelInfo {
    /// java: public LevelInfo(long xp, int level, Cap cap, @Nullable Progress progress)
    pub fn new(mut xp: i64, mut level: i32, mut cap: Cap, mut progress: Option<Progress>) -> Self {
        // TODO(fabric2seer): this(..) to an unknown constructor
        //   java: this(xp, level, cap, Optional.ofNullable(progress));
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: the record accessor `xp()`
    pub fn xp(&self) -> i64 {
        self.xp
    }

    /// java: the record accessor `level()`
    pub fn level(&self) -> i32 {
        self.level
    }

    /// java: the record accessor `cap()`
    pub fn cap(&self) -> Cap {
        self.cap.clone()
    }

    /// java: the record accessor `progress()`
    pub fn progress(&self) -> Option<Progress> {
        self.progress.clone()
    }

    /// java: public double levelWithProgress()
    pub fn level_with_progress(&self) -> f64 {
        if self.progress.is_some() {
            let mut progress: Progress = self.progress.clone().unwrap();
            return ((self.level as f64) + progress.percentage_to_next_level());
        }
        return (self.level) as f64;
    }

    /// java: public boolean isLevelNotAtAnyMaximum()
    pub fn is_level_not_at_any_maximum(&self) -> bool {
        return (self.level != self.cap.reachable_max_level() && self.level != self.cap.absolute_max_level());
    }

    /// java: public boolean isLevelCapped()
    pub fn is_level_capped(&self) -> bool {
        return (self.level == self.cap.reachable_max_level() && self.level != self.cap.absolute_max_level());
    }

    /// java: public boolean isLevelAbsolutelyMaxed()
    pub fn is_level_absolutely_maxed(&self) -> bool {
        return (self.level >= self.cap.absolute_max_level());
    }

}

/// java: public record Cap(int reachableMaxLevel, int absoluteMaxLevel)
#[derive(Clone, Debug, PartialEq)]
pub struct Cap {
    /// java: int reachableMaxLevel
    pub reachable_max_level: i32,
    /// java: int absoluteMaxLevel
    pub absolute_max_level: i32,
}

impl Cap {
    /// java: the canonical record constructor
    pub fn new(mut reachable_max_level: i32, mut absolute_max_level: i32) -> Self {
        Self { reachable_max_level, absolute_max_level }
    }

    /// java: the record accessor `reachableMaxLevel()`
    pub fn reachable_max_level(&self) -> i32 {
        self.reachable_max_level
    }

    /// java: the record accessor `absoluteMaxLevel()`
    pub fn absolute_max_level(&self) -> i32 {
        self.absolute_max_level
    }

}

/// java: public record Progress(long xpProgress, long xpNeeded)
#[derive(Clone, Debug, PartialEq)]
pub struct Progress {
    /// java: long xpProgress
    pub xp_progress: i64,
    /// java: long xpNeeded
    pub xp_needed: i64,
}

impl Progress {
    /// java: the canonical record constructor
    pub fn new(mut xp_progress: i64, mut xp_needed: i64) -> Self {
        Self { xp_progress, xp_needed }
    }

    /// java: the record accessor `xpProgress()`
    pub fn xp_progress(&self) -> i64 {
        self.xp_progress
    }

    /// java: the record accessor `xpNeeded()`
    pub fn xp_needed(&self) -> i64 {
        self.xp_needed
    }

    /// java: public double percentageToNextLevel()
    pub fn percentage_to_next_level(&self) -> f64 {
        return (((self.xp_progress()) as f64) / ((self.xp_needed()) as f64));
    }

}
