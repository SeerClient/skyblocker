//! Ported from de.hysky.skyblocker.utils.time.SkyblockTimeField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::time::skyblock_time;
use crate::de::hysky::skyblocker::utils::time::skyblock_time_unit::{ChronoField, ChronoUnit, SkyblockTimeUnit, Temporal, TemporalAccessor, TemporalUnit, ValueRange};

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
            // java: HOUR_OF_DAY(SkyblockTimeUnit.HOURS, SkyblockTimeUnit.DAYS, ValueRange.of(0, 23)),
            SkyblockTimeFieldFields::new(TemporalUnit::Skyblock(SkyblockTimeUnit::Hours), TemporalUnit::Skyblock(SkyblockTimeUnit::Days), ValueRange::of(0, 23)),
            // java: DAY_OF_MONTH(SkyblockTimeUnit.DAYS, SkyblockTimeUnit.MONTHS, ValueRange.of(1, 31)),
            SkyblockTimeFieldFields::new(TemporalUnit::Skyblock(SkyblockTimeUnit::Days), TemporalUnit::Skyblock(SkyblockTimeUnit::Months), ValueRange::of(1, 31)),
            // java: DAY_OF_YEAR(SkyblockTimeUnit.DAYS, SkyblockTimeUnit.YEARS, ValueRange.of(1, 372)),
            SkyblockTimeFieldFields::new(TemporalUnit::Skyblock(SkyblockTimeUnit::Days), TemporalUnit::Skyblock(SkyblockTimeUnit::Years), ValueRange::of(1, 372)),
            // java: MONTH_OF_YEAR(SkyblockTimeUnit.MONTHS, SkyblockTimeUnit.YEARS, ValueRange.of(1, 12)),
            SkyblockTimeFieldFields::new(TemporalUnit::Skyblock(SkyblockTimeUnit::Months), TemporalUnit::Skyblock(SkyblockTimeUnit::Years), ValueRange::of(1, 12)),
            // java: SEASON_OF_YEAR(SkyblockTimeUnit.SEASONS, SkyblockTimeUnit.YEARS, ValueRange.of(1, 4)),
            SkyblockTimeFieldFields::new(TemporalUnit::Skyblock(SkyblockTimeUnit::Seasons), TemporalUnit::Skyblock(SkyblockTimeUnit::Years), ValueRange::of(1, 4)),
            // java: YEAR(SkyblockTimeUnit.YEARS, ChronoUnit.FOREVER, ValueRange.of(1, Long.MAX_VALUE));
            SkyblockTimeFieldFields::new(TemporalUnit::Skyblock(SkyblockTimeUnit::Years), TemporalUnit::Chrono(ChronoUnit::Forever), ValueRange::of(1, i64::MAX)),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public TemporalUnit getBaseUnit()
    pub fn get_base_unit(&self) -> &'static TemporalUnit {
        return &self.fields().base_unit;
    }

    /// java: @Override public TemporalUnit getRangeUnit()
    pub fn get_range_unit(&self) -> &'static TemporalUnit {
        return &self.fields().range_unit;
    }

    /// java: @Override public ValueRange range()
    pub fn range(&self) -> &'static ValueRange {
        return &self.fields().range;
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
    pub fn is_supported_by(&self, temporal: &dyn TemporalAccessor) -> bool {
        // epoch day will have EXTREME rounding issues
        //   java: return temporal.isSupported(ChronoField.INSTANT_SECONDS) || temporal.isSupported(ChronoField.EPOCH_DAY);
        return temporal.is_supported_field(ChronoField::InstantSeconds) || temporal.is_supported_field(ChronoField::EpochDay);
    }

    /// java: @Override public ValueRange rangeRefinedBy(TemporalAccessor temporal)
    pub fn range_refined_by(&self, temporal: &dyn TemporalAccessor) -> &'static ValueRange {
        let _ = temporal;
        return &self.fields().range;
    }

    /// java: @Override public long getFrom(TemporalAccessor temporal)
    pub fn get_from(&self, temporal: &dyn TemporalAccessor) -> i64 {
        let mut seconds: i64;
        //   java: if (temporal.isSupported(ChronoField.INSTANT_SECONDS)) {
        //   java: 			seconds = temporal.getLong(ChronoField.INSTANT_SECONDS);
        //   java: 		} else if (temporal.isSupported(ChronoField.EPOCH_DAY)) {
        //   java: 			seconds = temporal.getLong(ChronoField.EPOCH_DAY) * ChronoUnit.DAYS.getDuration().getSeconds();
        //   java: 		} else throw new UnsupportedTemporalTypeException("Temporal must support epoch seconds or days.");
        if temporal.is_supported_field(ChronoField::InstantSeconds) {
            seconds = temporal.get_long_chrono(ChronoField::InstantSeconds);
        } else if temporal.is_supported_field(ChronoField::EpochDay) {
            seconds = temporal.get_long_chrono(ChronoField::EpochDay).wrapping_mul(ChronoUnit::Days.get_duration_seconds());
        } else {
            panic!("java.time.temporal.UnsupportedTemporalTypeException: Temporal must support epoch seconds or days.");
        }
        //   java: seconds -= SkyblockTime.SKYBLOCK_EPOCH.getEpochSecond();
        // (`SKYBLOCK_EPOCH` is `Instant.ofEpochMilli(1560275700000L)`: read as the constant, the static being a
        // render-thread one and this method a pure function of its argument)
        seconds = seconds.wrapping_sub(mcrust::java::time::instant::Instant::of_epoch_milli(1560275700000).get_epoch_second());
        let mut value: i64;
        if seconds < (0 as i64) {
            // probably not worth throwing a tantrum
            value = ((0) as i64) as i64;
        } else {
            //   java: seconds %= rangeUnit.getDuration().toSeconds();
            seconds %= self.fields().range_unit.get_duration_seconds();
            //   java: value = seconds / baseUnit.getDuration().getSeconds();
            value = seconds / self.fields().base_unit.get_duration_seconds();
        }
        // Only one that starts at 0
        if *self != SkyblockTimeField::HourOfDay {
            value += 1;
        }
        return value;
    }

    /// java: @SuppressWarnings("unchecked") @Override public <R extends Temporal> R adjustInto(R temporal, long newValue)
    pub fn adjust_into<R: Temporal>(&self, temporal: &R, new_value: i64) -> R {
        //   java: range.checkValidValue(newValue, this);
        self.fields().range.check_valid_value(new_value, self);
        //   java: long currentValue = temporal.getLong(this);
        let current_value: i64 = temporal.get_long(*self);
        let offset: i64 = new_value.wrapping_sub(current_value);
        //   java: return (R) temporal.plus(offset, baseUnit);
        return temporal.plus(offset, self.fields().base_unit);
    }

}

impl std::fmt::Display for SkyblockTimeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}


/// the constructor data of each SkyblockTimeField constant
pub struct SkyblockTimeFieldFields {
    pub base_unit: TemporalUnit,
    pub range_unit: TemporalUnit,
    pub range: ValueRange,
}

impl SkyblockTimeFieldFields {
    fn new(base_unit: TemporalUnit, range_unit: TemporalUnit, range: ValueRange) -> Self {
        let this = Self {
            base_unit,
            range_unit,
            range,
        };
        this
    }

}
