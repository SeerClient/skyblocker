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
            // java: return LocalDate.ofEpochDay(this.currentDayStamp); - day 0 is 1970-01-01
            return Some(LocalDate::of(1970, 1, 1).expect("1970-01-01").plus_days(self.current_day_stamp as i64));
        }
    }

    /// java: public int getDailyRunsCompleted()
    pub fn get_daily_runs_completed(&self) -> i32 {
        let mut last_day: LocalDate = self.get_last_daily_run_date().unwrap();
        // java: LocalDate currentDay = LocalDate.now(TimeFormatUtils.GMT);
        let current_day: LocalDate = local_date_now(*crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::time_format_utils::GMT.get());
        return (if current_day == last_day { self.completed_runs_count.min(DAILY_RUNS_MAX_DAILIES) } else { 0 });
    }

    /// java: public Duration timeUntilReset()
    pub fn time_until_reset(&self) -> std::time::Duration {
        let mut current_time: std::time::SystemTime = std::time::SystemTime::now();
        // java: Instant nextResetTime = LocalDate.now(TimeFormatUtils.GMT).plusDays(1L).atStartOfDay(TimeFormatUtils.GMT).toInstant();
        let gmt = *crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::time_format_utils::GMT.get();
        let next_day = local_date_now(gmt).plus_days(1);
        let midnight_local = mcrust::java::time::zone_id::epoch_second_from_civil(mcrust::java::time::zone_id::Civil { year: next_day.get_year(), month: next_day.get_month_value(), day: next_day.get_day_of_month(), hour: 0, minute: 0, second: 0 });
        let midnight = midnight_local - gmt.offset_seconds(mcrust::java::time::instant::Instant::of_epoch_second(midnight_local)) as i64;
        let next_reset_time: std::time::SystemTime = std::time::UNIX_EPOCH + std::time::Duration::from_secs(midnight.max(0) as u64);
        // java: return Duration.between(currentTime, nextResetTime); (non-negative: the reset is always ahead)
        next_reset_time.duration_since(current_time).unwrap_or_default()
    }

}

/// java: `LocalDate.now(ZoneId zone)` - today's date in `zone`
fn local_date_now(zone: mcrust::java::time::zone_id::ZoneId) -> LocalDate {
    let civil = zone.local_civil(mcrust::java::time::instant::Instant::now());
    LocalDate::of(civil.year, civil.month, civil.day).expect("the clock names a real date")
}

impl Dungeons {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "last_dungeon_run") {
            if let Some(__x) = gson::as_string(__v) {
                self.last_dungeon_run = __x;
            }
        }
        if let Some(__v) = gson::member(json, "secrets") {
            if let Some(__x) = gson::as_i32(__v) {
                self.secrets = __x;
            }
        }
        if let Some(__v) = gson::member(json, "selected_dungeon_class") {
            if let Some(__x) = gson::as_string(__v) {
                self.selected_dungeon_class = __x;
            }
        }
        if let Some(__v) = gson::member(json, "daily_runs") {
            if let Some(__x) = Some(DailyRuns::from_json(__v)) {
                self.daily_runs = __x;
            }
        }
        if let Some(__v) = gson::member(json, "treasures") {
            if let Some(__x) = Some(Treasures::from_json(__v)) {
                self.treasures = __x;
            }
        }
        if let Some(__v) = gson::member(json, "player_classes") {
            if let Some(__x) = gson::map(__v, gson::string_key, |__e0| Some(ClassStats::from_json(__e0))) {
                self.class_stats = __x;
            }
        }
        if let Some(__v) = gson::member(json, "dungeon_types") {
            if let Some(__x) = Some(PerDungeonType::from_json(__v)) {
                self.dungeon_types = __x;
            }
        }
    }

    /// `GSON.fromJson(json, Dungeons.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl PerDungeonType {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "master_catacombs") {
            if let Some(__x) = Some(GenericCatacombsBase::from_json(__v) as compat::Shared<dyn GenericCatacombs>) {
                self.master_mode_catacombs = __x;
            }
        }
        if let Some(__v) = gson::member(json, "catacombs") {
            if let Some(__x) = Some(DefaultCatacombs::from_json(__v)) {
                self.catacombs = __x;
            }
        }
    }

    /// `GSON.fromJson(json, PerDungeonType.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl ClassStats {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "experience") {
            if let Some(__x) = gson::as_f64(__v) {
                self.experience = __x;
            }
        }
    }

    /// `GSON.fromJson(json, ClassStats.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}

impl DailyRuns {
    /// Gson's reflective binding of the class's own fields (`ReflectiveTypeAdapterFactory`): each member the
    /// JSON has is read over the field it names (`@SerializedName`, else the field's name) with the field's declared
    /// type; an absent member (or a `null` one, or one Gson would reject) leaves the constructor's value.
    pub fn read_json_fields(&mut self, json: &serde_json::Value) {
        #[allow(unused_imports)]
        use crate::io::github::moulberry::repo::neu_repository::gson;
        if let Some(__v) = gson::member(json, "current_day_stamp") {
            if let Some(__x) = gson::as_i32(__v) {
                self.current_day_stamp = __x;
            }
        }
        if let Some(__v) = gson::member(json, "completed_runs_count") {
            if let Some(__x) = gson::as_i32(__v) {
                self.completed_runs_count = __x;
            }
        }
    }

    /// `GSON.fromJson(json, DailyRuns.class)`: the no-argument constructor, then the fields.
    pub fn from_json(json: &serde_json::Value) -> Self {
        let mut __this = Self::new();
        __this.read_json_fields(json);
        __this
    }
}
