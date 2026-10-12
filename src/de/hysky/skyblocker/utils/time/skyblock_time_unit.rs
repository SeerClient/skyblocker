//! Ported from de.hysky.skyblocker.utils.time.SkyblockTimeUnit by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public enum SkyblockTimeUnit implements TemporalUnit
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SkyblockTimeUnit {
    Hours,
    Days,
    Months,
    Seasons,
    Years,
    Centuries,
}

impl SkyblockTimeUnit {
    pub const VALUES: [SkyblockTimeUnit; 6] = [SkyblockTimeUnit::Hours, SkyblockTimeUnit::Days, SkyblockTimeUnit::Months, SkyblockTimeUnit::Seasons, SkyblockTimeUnit::Years, SkyblockTimeUnit::Centuries];

    /// java: values()
    pub fn values() -> Vec<SkyblockTimeUnit> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SkyblockTimeUnit::Hours => "HOURS",
            SkyblockTimeUnit::Days => "DAYS",
            SkyblockTimeUnit::Months => "MONTHS",
            SkyblockTimeUnit::Seasons => "SEASONS",
            SkyblockTimeUnit::Years => "YEARS",
            SkyblockTimeUnit::Centuries => "CENTURIES",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SkyblockTimeUnit {
        match name {
            "HOURS" => SkyblockTimeUnit::Hours,
            "DAYS" => SkyblockTimeUnit::Days,
            "MONTHS" => SkyblockTimeUnit::Months,
            "SEASONS" => SkyblockTimeUnit::Seasons,
            "YEARS" => SkyblockTimeUnit::Years,
            "CENTURIES" => SkyblockTimeUnit::Centuries,
            _ => panic!("IllegalArgumentException: No enum constant SkyblockTimeUnit.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SkyblockTimeUnitFields {
        static FIELDS: LazyLock<Vec<SkyblockTimeUnitFields>> = LazyLock::new(|| vec![
            SkyblockTimeUnitFields::new(std::time::Duration::from_secs((50) as u64)),
            SkyblockTimeUnitFields::new(std::time::Duration::from_secs(((20) as u64) * 60)),
            SkyblockTimeUnitFields::new(std::time::Duration::from_secs(((20i32.wrapping_mul(31)) as u64) * 60)),
            SkyblockTimeUnitFields::new(std::time::Duration::from_secs(((31) as u64) * 3600)),
            SkyblockTimeUnitFields::new(std::time::Duration::from_secs(((124) as u64) * 3600)),
            SkyblockTimeUnitFields::new(std::time::Duration::from_secs(((124i32.wrapping_mul(100)) as u64) * 3600)),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public Duration getDuration()
    pub fn get_duration(&self) -> std::time::Duration {
        return self.fields().duration.clone();
    }

    /// java: @Override public boolean isDurationEstimated()
    pub fn is_duration_estimated(&self) -> bool {
        return false;
    }

    /// java: @Override public boolean isDateBased()
    pub fn is_date_based(&self) -> bool {
        return false;
    }

    /// java: @Override public boolean isTimeBased()
    pub fn is_time_based(&self) -> bool {
        return false;
    }

    /// java: @SuppressWarnings("unchecked") @Override public <R extends Temporal> R addTo(R temporal, long amount)
    pub fn add_to<R: 'static>(&self, temporal: &R, mut amount: i64) -> R {
        // TODO(fabric2seer): `.isSupported(..)` on a R
        //   java: if (temporal.isSupported(ChronoUnit.SECONDS)) {
        //   java: 			return (R) temporal.plus(duration.toSeconds() * amount, ChronoUnit.SECONDS);
        //   java: 		} else if (temporal.isSupported(ChronoUnit.DAYS)) {
        //   java: 			return (R) temporal.plus(duration.toDays() * amount, ChronoUnit.DAYS);
        //   java: 		}
        panic!("UnsupportedTemporalTypeException: {}", "Temporal must support seconds or days.");
    }

    /// java: @Override public long between(Temporal temporal1Inclusive, Temporal temporal2Exclusive)
    pub fn between(&self, temporal1_inclusive: &crate::unresolved::Temporal, temporal2_exclusive: &crate::unresolved::Temporal) -> i64 {
        // TODO(fabric2seer): `.isSupported(..)` on a Temporal
        //   java: if (temporal1Inclusive.isSupported(ChronoUnit.SECONDS)) {
        //   java: 			long seconds = temporal1Inclusive.until(temporal2Exclusive, ChronoUnit.SECONDS);
        //   java: 			return seconds / duration.toSeconds();
        //   java: 		} else if (temporal1Inclusive.isSupported(ChronoUnit.DAYS)) {
        //   java: 			long days = temporal1Inclusive.until(temporal2Exclusive, ChronoUnit.DAYS);
        //   java: 			return days * ChronoUnit.DAYS.getDuration().toSeconds() / duration.toSeconds();
        //   java: 		}
        panic!("UnsupportedTemporalTypeException: {}", "Temporal must support seconds or days.");
    }

}

impl std::fmt::Display for SkyblockTimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each SkyblockTimeUnit constant
pub struct SkyblockTimeUnitFields {
    pub duration: std::time::Duration,
}

impl SkyblockTimeUnitFields {
    fn new(mut duration: std::time::Duration) -> Self {
        let mut this = Self {
            duration: Default::default(),
        };
        this.duration = duration.clone();
        this
    }

}
