//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.model.Dungeons by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_class::DungeonClass;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::default_catacombs::DefaultCatacombs;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::AggregateStat;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::GenericCatacombs;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::generic_catacombs::GenericCatacombsBase;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::treasures::Treasures;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_calculator::LevelCalculator;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::level_info::LevelInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::skill::Skill;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::time_format_utils;
use mcrust::java::time::local_date::LocalDate;

/// java: public static final int MAX_DAILIES = 5;
pub const DAILY_RUNS_MAX_DAILIES: i32 = 5;

/// java: public class Dungeons
#[derive(Clone)]
pub struct Dungeons {
    /// java: @SerializedName("last_dungeon_run") public String lastDungeonRun = "";
    pub last_dungeon_run: String,
    /// java: public int secrets;
    pub secrets: i32,
    /// java: @SerializedName("selected_dungeon_class") public String selectedDungeonClass = "";
    pub selected_dungeon_class: String,
    /// java: @SerializedName("daily_runs") public DailyRuns dailyRuns = new DailyRuns();
    pub daily_runs: DailyRuns,
    /// java: public Treasures treasures = new Treasures();
    pub treasures: Treasures,
    /// java: @SerializedName("player_classes") public Map<String, ClassStats> classStats = Map.of();
    pub class_stats: std::collections::HashMap<String, ClassStats>,
    /// java: @SerializedName("dungeon_types") public PerDungeonType dungeonTypes = new PerDungeonType();
    pub dungeon_types: PerDungeonType,
}

impl Dungeons {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            last_dungeon_run: "".to_string(),
            secrets: 0,
            selected_dungeon_class: "".to_string(),
            daily_runs: DailyRuns::new(),
            treasures: Treasures::new(),
            class_stats: std::collections::HashMap::from([]),
            dungeon_types: PerDungeonType::new(),
        };
        this
    }

    /// java: public ClassStats getClassData(DungeonClass dungeonClass)
    pub fn get_class_data(&self, mut dungeon_class: DungeonClass) -> ClassStats {
        return self.class_stats.get(&dungeon_class.api_name()).cloned().unwrap_or_else(|| ClassStats::new());
    }

    /// java: public LevelInfo getClassAverage(LoadingInformation info)
    pub fn get_class_average(&self, info: &LoadingInformation) -> LevelInfo {
        let mut cumulative_xp: i64 = ((0) as i64) as i64;
        cumulative_xp = cumulative_xp.wrapping_add(self.get_class_data(DungeonClass::Healer).get_level_info(info).xp());
        cumulative_xp = cumulative_xp.wrapping_add(self.get_class_data(DungeonClass::Mage).get_level_info(info).xp());
        cumulative_xp = cumulative_xp.wrapping_add(self.get_class_data(DungeonClass::Berserk).get_level_info(info).xp());
        cumulative_xp = cumulative_xp.wrapping_add(self.get_class_data(DungeonClass::Archer).get_level_info(info).xp());
        cumulative_xp = cumulative_xp.wrapping_add(self.get_class_data(DungeonClass::Tank).get_level_info(info).xp());
        let mut average_xp: i64 = cumulative_xp.wrapping_div(5 as i64);
        return LevelCalculator::get_skill_level(average_xp, Skill::Catacombs, info);
    }

    /// java: public int getTotalDungeonRuns()
    pub fn get_total_dungeon_runs(&self) -> i32 {
        let mut normal_runs: i32 = ((AggregateStat::get_manually_calculated_total(&compat::read(&self.dungeon_types.catacombs, |__t| __t.tier_completions.clone()))) as i32);
        let mut master_runs: i32 = ((AggregateStat::get_manually_calculated_total(&compat::read(&self.dungeon_types.master_mode_catacombs, |__t| __t.generic_catacombs_base().tier_completions.clone()))) as i32);
        return normal_runs.wrapping_add(master_runs);
    }

    /// java: public double getSecretsPerRun()
    pub fn get_secrets_per_run(&self) -> f64 {
        let mut runs: f64 = ((self.get_total_dungeon_runs()) as f64) as f64;
        let mut secrets: f64 = (self.secrets) as f64;
        return (if runs > (0 as f64) { (secrets / runs) } else { ((0) as f64) as f64 });
    }

}

/// java: public static class PerDungeonType
#[derive(Clone)]
pub struct PerDungeonType {
    /// java: @SerializedName("master_catacombs") public GenericCatacombs masterModeCatacombs = new GenericCatacombs();
    pub master_mode_catacombs: compat::Shared<dyn GenericCatacombs>,
    /// java: public DefaultCatacombs catacombs = new DefaultCatacombs();
    pub catacombs: compat::Shared<DefaultCatacombs>,
}

impl PerDungeonType {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            master_mode_catacombs: (GenericCatacombsBase::new() as compat::Shared<dyn GenericCatacombs>),
            catacombs: DefaultCatacombs::new(),
        };
        this
    }

}

/// java: public static class ClassStats
#[derive(Clone)]
pub struct ClassStats {
    /// java: public double experience;
    pub experience: f64,
}

impl ClassStats {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            experience: 0.0,
        };
        this
    }

    /// java: public LevelInfo getLevelInfo(LoadingInformation info)
    pub fn get_level_info(&self, info: &LoadingInformation) -> LevelInfo {
        return LevelCalculator::get_skill_level(((self.experience) as i64), Skill::Catacombs, info);
    }

}

/// java: public static class DailyRuns
#[derive(Clone)]
pub struct DailyRuns {
    /// java: @SerializedName("current_day_stamp") public int currentDayStamp;
    pub current_day_stamp: i32,
    /// java: @SerializedName("completed_runs_count") public int completedRunsCount;
    pub completed_runs_count: i32,
}

impl DailyRuns {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            current_day_stamp: 0,
            completed_runs_count: 0,
        };
        this
    }

    /// java: private @Nullable LocalDate getLastDailyRunDate()
    pub fn get_last_daily_run_date(&self) -> Option<LocalDate> {
        if self.current_day_stamp == 0 {
            return None;
        } else {
            // TODO(fabric2seer): static `LocalDate.ofEpochDay` has no Rust counterpart in the port
            // rust: return Some(LocalDate::of_epoch_day(self.current_day_stamp));
            return todo!("fabric2seer: see the TODO above");
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public int getDailyRunsCompleted()
    pub fn get_daily_runs_completed(&self) -> i32 {
        let mut last_day: LocalDate = self.get_last_daily_run_date().unwrap();
        // TODO(fabric2seer): `now` takes 0 argument(s) in the port (``), Java passes 1
        // rust: let mut current_day: LocalDate = LocalDate::now(time_format_utils::GMT.get_mut().clone());
        let mut current_day: LocalDate = todo!("fabric2seer: the initialiser of `currentDay` (TODO above)");
        return (if current_day == last_day { self.completed_runs_count.min(DAILY_RUNS_MAX_DAILIES) } else { 0 });
    }

    /// java: public Duration timeUntilReset()
    pub fn time_until_reset(&self) -> std::time::Duration {
        let mut current_time: std::time::SystemTime = std::time::SystemTime::now();
        // TODO(fabric2seer): `.plusDays(..)` on a value of unknown type
        //   java: Instant nextResetTime = LocalDate.now(TimeFormatUtils.GMT)
        //   java: 					.plusDays(1L)
        //   java: 					.atStartOfDay(TimeFormatUtils.GMT)
        //   java: 					.toInstant();
        let mut next_reset_time: std::time::SystemTime = todo!("fabric2seer: the initialiser of `nextResetTime` (TODO above)");
        // TODO(fabric2seer): static call Duration.between(..)
        //   java: return Duration.between(currentTime, nextResetTime);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
