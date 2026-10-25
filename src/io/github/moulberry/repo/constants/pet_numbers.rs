//! Ported from io.github.moulberry.repo.constants.PetNumbers (neurepoparser 1.12.0).

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class PetNumbers
#[derive(Debug, Default)]
pub struct PetNumbers {
    /// java: @SerializedName("1") Stats statsAtLowLevel;
    pub(crate) stats_at_low_level: Stats,
    /// java: @SerializedName("100") Stats statsAtHighLevel;
    pub(crate) stats_at_high_level: Stats,
    /// java: @SerializedName("stats_levelling_curve") @ApiStatus.Obsolete String levelingCurve;
    pub(crate) leveling_curve: Option<String>,
    /// java: private transient int minLevel; - memoised by `getLowLevel` (atomic: the object is shared between threads)
    min_level: AtomicI32,
    /// java: private transient int maxLevel; - memoised by `getHighLevel`
    max_level: AtomicI32,
}

impl Clone for PetNumbers {
    fn clone(&self) -> Self {
        PetNumbers {
            stats_at_low_level: self.stats_at_low_level.clone(),
            stats_at_high_level: self.stats_at_high_level.clone(),
            leveling_curve: self.leveling_curve.clone(),
            min_level: AtomicI32::new(self.min_level.load(Ordering::Relaxed)),
            max_level: AtomicI32::new(self.max_level.load(Ordering::Relaxed)),
        }
    }
}

/// java: @Getter @AllArgsConstructor @NoArgsConstructor @ToString public static class Stats
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stats {
    /// java: @SerializedName("otherNums") List<Double> otherNumbers = new ArrayList<>();
    pub(crate) other_numbers: Vec<f64>,
    /// java: @SerializedName("statNums") Map<String, Double> statNumbers = new HashMap<>();
    pub(crate) stat_numbers: HashMap<String, f64>,
}

impl Stats {
    /// java: public Stats(final List<Double> otherNumbers, final Map<String, Double> statNumbers)
    pub fn new(other_numbers: Vec<f64>, stat_numbers: HashMap<String, f64>) -> Self {
        Stats { other_numbers, stat_numbers }
    }

    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        Stats {
            other_numbers: gson::member(json, "otherNums").and_then(|v| gson::list(v, gson::as_f64)).unwrap_or_default(),
            stat_numbers: gson::member(json, "statNums").and_then(|v| gson::map(v, gson::string_key, gson::as_f64)).unwrap_or_default(),
        }
    }

    /// java: public List<Double> getOtherNumbers()
    pub fn get_other_numbers(&self) -> &Vec<f64> {
        &self.other_numbers
    }

    /// java: public Map<String, Double> getStatNumbers()
    pub fn get_stat_numbers(&self) -> &HashMap<String, f64> {
        &self.stat_numbers
    }

    /// `getOtherNumbers().add(..)` - Java mutates through the getter.
    pub fn get_other_numbers_mut(&mut self) -> &mut Vec<f64> {
        &mut self.other_numbers
    }

    /// `getStatNumbers().put(..)` - Java mutates through the getter.
    pub fn get_stat_numbers_mut(&mut self) -> &mut HashMap<String, f64> {
        &mut self.stat_numbers
    }
}

impl std::fmt::Display for Stats {
    /// java: public String toString()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PetNumbers.Stats(otherNumbers={:?}, statNumbers={:?})", self.other_numbers, self.stat_numbers)
    }
}

/// `levelingCurve.split(":")` and the field the caller asks for (`Integer.parseInt`).
fn curve_part(leveling_curve: &str, index: usize) -> i32 {
    let split: Vec<&str> = leveling_curve.split(':').collect();
    // java: assert split.length == 3; assert Objects.equals(split[2], "1"); (assertions are off at run time)
    let part = split.get(index).copied().unwrap_or_else(|| panic!("ArrayIndexOutOfBoundsException: {}", index));
    part.parse::<i32>().unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{}\"", part))
}

impl PetNumbers {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        PetNumbers {
            stats_at_low_level: gson::member(json, "1").map(Stats::from_json).unwrap_or_default(),
            stats_at_high_level: gson::member(json, "100").map(Stats::from_json).unwrap_or_default(),
            leveling_curve: gson::string(json, "stats_levelling_curve"),
            min_level: AtomicI32::new(0),
            max_level: AtomicI32::new(0),
        }
    }

    /// java: public int getLowLevel()
    pub fn get_low_level(&self) -> i32 {
        let min_level = self.min_level.load(Ordering::Relaxed);
        if min_level > 0 {
            return min_level;
        }
        let Some(leveling_curve) = &self.leveling_curve else {
            return 1;
        };
        // java: return minLevel = Integer.parseInt(split[0]);
        let value = curve_part(leveling_curve, 0);
        self.min_level.store(value, Ordering::Relaxed);
        value
    }

    /// java: public int getHighLevel()
    pub fn get_high_level(&self) -> i32 {
        let max_level = self.max_level.load(Ordering::Relaxed);
        if max_level > 0 {
            return max_level;
        }
        let Some(leveling_curve) = &self.leveling_curve else {
            return 100;
        };
        // java: return maxLevel = Integer.parseInt(split[1]);
        let value = curve_part(leveling_curve, 1);
        self.max_level.store(value, Ordering::Relaxed);
        value
    }

    /// java: public @Nullable Stats interpolatedStatsAtLevel(int level)
    pub fn interpolated_stats_at_level(&self, level: i32) -> Option<Stats> {
        if level < self.get_low_level() {
            return None;
        }
        if level > self.get_high_level() {
            return None;
        }
        // java: double progress = (level - getLowLevel()) / (double) (getHighLevel() - getLowLevel());
        let progress: f64 = (level - self.get_low_level()) as f64 / (self.get_high_level() - self.get_low_level()) as f64;
        let mut stats = Stats::default();
        // java: for (String stat : getStatsAtLowLevel().statNumbers.keySet())
        for (stat, low_stat) in self.get_stats_at_low_level().get_stat_numbers() {
            // java: Double highStat = getStatsAtHighLevel().getStatNumbers().get(stat); - unboxing a missing one is an NPE
            let high_stat: f64 = *self
                .get_stats_at_high_level()
                .get_stat_numbers()
                .get(stat)
                .unwrap_or_else(|| panic!("java.lang.NullPointerException: no level-100 value for pet stat {}", stat));
            stats.get_stat_numbers_mut().insert(stat.clone(), low_stat + (high_stat - low_stat) * progress);
        }
        // java: while (lowStatIt.hasNext() && highStatIt.hasNext()) stats.getOtherNumbers().add(lowStat + (highStat - lowStat) * progress);
        for (low_stat, high_stat) in self.get_stats_at_low_level().get_other_numbers().iter().zip(self.get_stats_at_high_level().get_other_numbers()) {
            stats.get_other_numbers_mut().push(low_stat + (high_stat - low_stat) * progress);
        }
        Some(stats)
    }

    /// java: public Stats getStatsAtLowLevel()
    pub fn get_stats_at_low_level(&self) -> &Stats {
        &self.stats_at_low_level
    }

    /// java: public Stats getStatsAtHighLevel()
    pub fn get_stats_at_high_level(&self) -> &Stats {
        &self.stats_at_high_level
    }

    /// java: public String getLevelingCurve()
    pub fn get_leveling_curve(&self) -> Option<&str> {
        self.leveling_curve.as_deref()
    }

    /// java: public int getMinLevel()
    pub fn get_min_level(&self) -> i32 {
        self.min_level.load(Ordering::Relaxed)
    }

    /// java: public int getMaxLevel()
    pub fn get_max_level(&self) -> i32 {
        self.max_level.load(Ordering::Relaxed)
    }
}
