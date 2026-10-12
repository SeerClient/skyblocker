//! Ported from de.hysky.skyblocker.utils.time.SkyblockTimeField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::time::skyblock_time;
use crate::de::hysky::skyblocker::utils::time::skyblock_time_unit::SkyblockTimeUnit;

/// java: public enum SkyblockTimeField implements TemporalField
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SkyblockTimeField {
    HourOfDay,
    DayOfMonth,
    DayOfYear,
    MonthOfYear,
    SeasonOfYear,
    Year,
}

impl SkyblockTimeField {
    pub const VALUES: [SkyblockTimeField; 6] = [SkyblockTimeField::HourOfDay, SkyblockTimeField::DayOfMonth, SkyblockTimeField::DayOfYear, SkyblockTimeField::MonthOfYear, SkyblockTimeField::SeasonOfYear, SkyblockTimeField::Year];

    /// java: values()
    pub fn values() -> Vec<SkyblockTimeField> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SkyblockTimeField::HourOfDay => "HOUR_OF_DAY",
            SkyblockTimeField::DayOfMonth => "DAY_OF_MONTH",
            SkyblockTimeField::DayOfYear => "DAY_OF_YEAR",
            SkyblockTimeField::MonthOfYear => "MONTH_OF_YEAR",
            SkyblockTimeField::SeasonOfYear => "SEASON_OF_YEAR",
            SkyblockTimeField::Year => "YEAR",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SkyblockTimeField {
        match name {
            "HOUR_OF_DAY" => SkyblockTimeField::HourOfDay,
            "DAY_OF_MONTH" => SkyblockTimeField::DayOfMonth,
            "DAY_OF_YEAR" => SkyblockTimeField::DayOfYear,
            "MONTH_OF_YEAR" => SkyblockTimeField::MonthOfYear,
            "SEASON_OF_YEAR" => SkyblockTimeField::SeasonOfYear,
            "YEAR" => SkyblockTimeField::Year,
            _ => panic!("IllegalArgumentException: No enum constant SkyblockTimeField.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SkyblockTimeFieldFields {
        static FIELDS: LazyLock<Vec<SkyblockTimeFieldFields>> = LazyLock::new(|| vec![
            todo!("HOUR_OF_DAY"),
            todo!("DAY_OF_MONTH"),
            todo!("DAY_OF_YEAR"),
            todo!("MONTH_OF_YEAR"),
            todo!("SEASON_OF_YEAR"),
            todo!("YEAR"),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public TemporalUnit getBaseUnit()
    pub fn get_base_unit(&self) -> crate::unresolved::TemporalUnit {
        return self.fields().base_unit.clone();
    }

    /// java: @Override public TemporalUnit getRangeUnit()
    pub fn get_range_unit(&self) -> crate::unresolved::TemporalUnit {
        return self.fields().range_unit.clone();
    }

    /// java: @Override public ValueRange range()
    pub fn range(&self) -> crate::unresolved::ValueRange {
        return self.fields().range.clone();
    }

    /// java: @Override public boolean isDateBased()
    pub fn is_date_based(&self) -> bool {
        return false;
    }

    /// java: @Override public boolean isTimeBased()
    pub fn is_time_based(&self) -> bool {
        return false;
    }

    /// java: @Override public boolean isSupportedBy(TemporalAccessor temporal)
    pub fn is_supported_by(&self, temporal: &crate::unresolved::TemporalAccessor) -> bool {
        // TODO(fabric2seer): `.isSupported(..)` on a TemporalAccessor
        //   java: return temporal.isSupported(ChronoField.INSTANT_SECONDS) || temporal.isSupported(ChronoField.EPOCH_DAY);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ValueRange rangeRefinedBy(TemporalAccessor temporal)
    pub fn range_refined_by(&self, temporal: &crate::unresolved::TemporalAccessor) -> crate::unresolved::ValueRange {
        return self.fields().range.clone();
    }

    /// java: @Override public long getFrom(TemporalAccessor temporal)
    pub fn get_from(&self, temporal: &crate::unresolved::TemporalAccessor) -> i64 {
        let mut seconds: i64;
        // TODO(fabric2seer): `.isSupported(..)` on a TemporalAccessor
        //   java: if (temporal.isSupported(ChronoField.INSTANT_SECONDS)) {
        //   java: 			seconds = temporal.getLong(ChronoField.INSTANT_SECONDS);
        //   java: 		} else if (temporal.isSupported(ChronoField.EPOCH_DAY)) {
        //   java: 			seconds = temporal.getLong(ChronoField.EPOCH_DAY) * ChronoUnit.DAYS.getDuration().getSeconds();
        //   java: 		} else throw new UnsupportedTemporalTypeException("Temporal must support epoch seconds or days.");
        // TODO(fabric2seer): `.getEpochSecond(..)` on a Instant
        //   java: seconds -= SkyblockTime.SKYBLOCK_EPOCH.getEpochSecond();
        let mut value: i64;
        if seconds < (0 as i64) {
            value = ((0) as i64) as i64;
        } else {
            // TODO(fabric2seer): `.getDuration(..)` on a TemporalUnit
            //   java: seconds %= rangeUnit.getDuration().toSeconds();
            // TODO(fabric2seer): `.getDuration(..)` on a TemporalUnit
            //   java: value = seconds / baseUnit.getDuration().getSeconds();
        }
        if *self != HOUR_OF_DAY {
            value += 1;
        }
        return value;
    }

    /// java: @SuppressWarnings("unchecked") @Override public <R extends Temporal> R adjustInto(R temporal, long newValue)
    pub fn adjust_into<R: 'static>(&self, temporal: &R, mut new_value: i64) -> R {
        // TODO(fabric2seer): `.checkValidValue(..)` on a ValueRange
        //   java: range.checkValidValue(newValue, this);
        // TODO(fabric2seer): `.getLong(..)` on a R
        //   java: long currentValue = temporal.getLong(this);
        let mut current_value: i64 = todo!("fabric2seer: the initialiser of `currentValue` (TODO above)");
        let mut offset: i64 = new_value.wrapping_sub(current_value);
        // TODO(fabric2seer): `.plus(..)` on a R
        //   java: return (R) temporal.plus(offset, baseUnit);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for SkyblockTimeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each SkyblockTimeField constant
pub struct SkyblockTimeFieldFields {
    pub base_unit: crate::unresolved::TemporalUnit,
    pub range_unit: crate::unresolved::TemporalUnit,
    pub range: crate::unresolved::ValueRange,
}

impl SkyblockTimeFieldFields {
    fn new(mut base_unit: crate::unresolved::TemporalUnit, mut range_unit: crate::unresolved::TemporalUnit, mut range: crate::unresolved::ValueRange) -> Self {
        let mut this = Self {
            base_unit: Default::default(),
            range_unit: Default::default(),
            range: Default::default(),
        };
        this.base_unit = base_unit.clone();
        this.range_unit = range_unit.clone();
        this.range = range.clone();
        this
    }

}
