//! Ported from de.hysky.skyblocker.utils.time.SkyblockTimeUnit by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::time::skyblock_time_field::SkyblockTimeField;

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
    pub fn add_to<R: Temporal>(&self, temporal: &R, amount: i64) -> R {
        let duration = self.get_duration();
        //   java: if (temporal.isSupported(ChronoUnit.SECONDS)) {
        //   java: 			return (R) temporal.plus(duration.toSeconds() * amount, ChronoUnit.SECONDS);
        if temporal.is_supported_unit(ChronoUnit::Seconds) {
            return temporal.plus_chrono((duration.as_secs() as i64).wrapping_mul(amount), ChronoUnit::Seconds);
        //   java: 		} else if (temporal.isSupported(ChronoUnit.DAYS)) {
        //   java: 			return (R) temporal.plus(duration.toDays() * amount, ChronoUnit.DAYS);
        } else if temporal.is_supported_unit(ChronoUnit::Days) {
            return temporal.plus_chrono(((duration.as_secs() / 86400) as i64).wrapping_mul(amount), ChronoUnit::Days);
        }
        panic!("UnsupportedTemporalTypeException: {}", "Temporal must support seconds or days.");
    }

    /// java: @Override public long between(Temporal temporal1Inclusive, Temporal temporal2Exclusive)
    pub fn between<T: Temporal>(&self, temporal1_inclusive: &T, temporal2_exclusive: &T) -> i64 {
        let duration_seconds: i64 = self.get_duration().as_secs() as i64;
        //   java: if (temporal1Inclusive.isSupported(ChronoUnit.SECONDS)) {
        //   java: 			long seconds = temporal1Inclusive.until(temporal2Exclusive, ChronoUnit.SECONDS);
        //   java: 			return seconds / duration.toSeconds();
        if temporal1_inclusive.is_supported_unit(ChronoUnit::Seconds) {
            let seconds: i64 = temporal1_inclusive.until_chrono(temporal2_exclusive, ChronoUnit::Seconds);
            return seconds / duration_seconds;
        //   java: 		} else if (temporal1Inclusive.isSupported(ChronoUnit.DAYS)) {
        //   java: 			long days = temporal1Inclusive.until(temporal2Exclusive, ChronoUnit.DAYS);
        //   java: 			return days * ChronoUnit.DAYS.getDuration().toSeconds() / duration.toSeconds();
        } else if temporal1_inclusive.is_supported_unit(ChronoUnit::Days) {
            let days: i64 = temporal1_inclusive.until_chrono(temporal2_exclusive, ChronoUnit::Days);
            return days.wrapping_mul(ChronoUnit::Days.get_duration_seconds()) / duration_seconds;
        }
        panic!("UnsupportedTemporalTypeException: {}", "Temporal must support seconds or days.");
    }

}

// ---- java.time.temporal, as SkyblockTimeUnit, SkyblockTimeField and SkyblockQueries use it ----
// The game's `java::time` has `Instant` and `LocalDate` but no `java.time.temporal` interfaces: the members of
// `ChronoField`, `ChronoUnit`, `TemporalUnit`, `ValueRange`, `TemporalAccessor`, `Temporal` and `TemporalQuery` these
// three classes reach are below, with the JDK's two temporals the mod uses (`Instant`, `LocalDate`) implementing them.

/// java.time.temporal.ChronoField - the two constants `SkyblockTimeField` asks a temporal about
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChronoField {
    InstantSeconds,
    EpochDay,
}

/// java.time.temporal.ChronoUnit - the constants these classes name
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChronoUnit {
    Seconds,
    Days,
    Forever,
}

impl ChronoUnit {
    /// `getDuration().getSeconds()` (`FOREVER` is `Duration.ofSeconds(Long.MAX_VALUE, 999_999_999)`)
    pub fn get_duration_seconds(&self) -> i64 {
        match self {
            ChronoUnit::Seconds => 1,
            ChronoUnit::Days => 86400,
            ChronoUnit::Forever => i64::MAX,
        }
    }
}

/// java.time.temporal.TemporalUnit, as `SkyblockTimeField` holds one (a Skyblock unit, or `ChronoUnit.FOREVER`)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemporalUnit {
    Skyblock(SkyblockTimeUnit),
    Chrono(ChronoUnit),
}

impl TemporalUnit {
    /// `getDuration().getSeconds()` / `toSeconds()` - equal for every unit here (none has a nano part but
    /// FOREVER, whose `toSeconds()` is its `getSeconds()`)
    pub fn get_duration_seconds(&self) -> i64 {
        match self {
            TemporalUnit::Skyblock(unit) => unit.get_duration().as_secs() as i64,
            TemporalUnit::Chrono(unit) => unit.get_duration_seconds(),
        }
    }
}

/// java.time.temporal.ValueRange - a fixed range (`ValueRange.of(min, max)`)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueRange {
    pub min: i64,
    pub max: i64,
}

impl ValueRange {
    /// `ValueRange.of(min, max)`
    pub const fn of(min: i64, max: i64) -> ValueRange {
        ValueRange { min, max }
    }

    /// `isIntValue()`
    pub fn is_int_value(&self) -> bool {
        self.min >= i32::MIN as i64 && self.max <= i32::MAX as i64
    }

    /// `isValidValue(value)`
    pub fn is_valid_value(&self, value: i64) -> bool {
        value >= self.min && value <= self.max
    }

    /// `isValidIntValue(value)`
    pub fn is_valid_int_value(&self, value: i64) -> bool {
        self.is_int_value() && self.is_valid_value(value)
    }

    /// `checkValidValue(value, field)` - a `DateTimeException` carrying `genInvalidFieldMessage`
    pub fn check_valid_value(&self, value: i64, field: &dyn std::fmt::Display) -> i64 {
        if !self.is_valid_value(value) {
            panic!("DateTimeException: Invalid value for {} (valid values {}): {}", field, self, value);
        }
        value
    }

    /// `checkValidIntValue(value, field)`
    pub fn check_valid_int_value(&self, value: i64, field: &dyn std::fmt::Display) -> i32 {
        if !self.is_valid_int_value(value) {
            panic!("DateTimeException: Invalid value for {} (valid values {}): {}", field, self, value);
        }
        value as i32
    }
}

impl std::fmt::Display for ValueRange {
    /// `ValueRange.toString()` of a fixed range: `min - max`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.min, self.max)
    }
}

/// java.time.temporal.TemporalAccessor - its reads of a `ChronoField`; the `TemporalField` overloads (a
/// `SkyblockTimeField`) are the inherent methods on `dyn TemporalAccessor`, as the JDK's default bodies route them.
pub trait TemporalAccessor {
    /// `isSupported(ChronoField)`
    fn is_supported_field(&self, field: ChronoField) -> bool;
    /// `getLong(ChronoField)` - `UnsupportedTemporalTypeException` for a field `isSupported` refuses
    fn get_long_chrono(&self, field: ChronoField) -> i64;
}

impl dyn TemporalAccessor + '_ {
    /// `isSupported(TemporalField)` for a field that is not a `ChronoField`: `field.isSupportedBy(this)`
    pub fn is_supported(&self, field: SkyblockTimeField) -> bool {
        field.is_supported_by(self)
    }

    /// `getLong(TemporalField)` for a field that is not a `ChronoField`: `field.getFrom(this)`
    pub fn get_long(&self, field: SkyblockTimeField) -> i64 {
        field.get_from(self)
    }

    /// `get(TemporalField)` (`Instant.get`): `range(field).checkValidIntValue(field.getFrom(this), field)`, `range(field)`
    /// being `field.rangeRefinedBy(this)` for a field that is not a `ChronoField`
    pub fn get(&self, field: SkyblockTimeField) -> i32 {
        field.range_refined_by(self).check_valid_int_value(field.get_from(self), &field)
    }

    /// `query(TemporalQuery)` - the default body, `query.queryFrom(this)`
    pub fn query<R>(&self, query: &TemporalQuery<R>) -> R {
        (query.0)(self)
    }
}

/// java.time.temporal.Temporal - the arithmetic `SkyblockTimeUnit.addTo` / `between` and
/// `SkyblockTimeField.adjustInto` use.
pub trait Temporal: TemporalAccessor + Sized {
    /// `isSupported(ChronoUnit)`
    fn is_supported_unit(&self, unit: ChronoUnit) -> bool;
    /// `plus(amount, ChronoUnit)`
    fn plus_chrono(&self, amount: i64, unit: ChronoUnit) -> Self;
    /// `until(end, ChronoUnit)`
    fn until_chrono(&self, end: &Self, unit: ChronoUnit) -> i64;

    /// `plus(amount, TemporalUnit)`: a `ChronoUnit` is the temporal's own; any other unit is `unit.addTo(this, amount)`
    fn plus(&self, amount: i64, unit: TemporalUnit) -> Self {
        match unit {
            TemporalUnit::Chrono(unit) => self.plus_chrono(amount, unit),
            TemporalUnit::Skyblock(unit) => unit.add_to(self, amount),
        }
    }

    /// `getLong(TemporalField)` for a field that is not a `ChronoField`: `field.getFrom(this)`
    fn get_long(&self, field: SkyblockTimeField) -> i64 {
        field.get_from(self)
    }

    /// `with(TemporalField, newValue)` for a field that is not a `ChronoField`: `field.adjustInto(this, newValue)`
    fn with(&self, field: SkyblockTimeField, new_value: i64) -> Self {
        field.adjust_into(self, new_value)
    }
}

/// java.time.temporal.TemporalQuery<R> - `queryFrom(TemporalAccessor)`
pub struct TemporalQuery<R>(pub fn(&dyn TemporalAccessor) -> R);

impl TemporalAccessor for mcrust::java::time::instant::Instant {
    /// `Instant.isSupported(TemporalField)`: INSTANT_SECONDS (with the nano / milli fields), not EPOCH_DAY
    fn is_supported_field(&self, field: ChronoField) -> bool {
        field == ChronoField::InstantSeconds
    }

    fn get_long_chrono(&self, field: ChronoField) -> i64 {
        match field {
            ChronoField::InstantSeconds => self.get_epoch_second(),
            ChronoField::EpochDay => panic!("UnsupportedTemporalTypeException: Unsupported field: EpochDay"),
        }
    }
}

impl Temporal for mcrust::java::time::instant::Instant {
    /// `Instant.isSupported(TemporalUnit)`: the time-based units and DAYS
    fn is_supported_unit(&self, unit: ChronoUnit) -> bool {
        unit != ChronoUnit::Forever
    }

    /// `Instant.plus(amount, unit)`: SECONDS is `plusSeconds`, DAYS `plusSeconds(multiplyExact(amount, SECONDS_PER_DAY))`
    fn plus_chrono(&self, amount: i64, unit: ChronoUnit) -> Self {
        match unit {
            ChronoUnit::Seconds => self.plus_seconds(amount),
            ChronoUnit::Days => self.plus_seconds(amount.checked_mul(86400).unwrap_or_else(|| panic!("ArithmeticException: long overflow"))),
            ChronoUnit::Forever => panic!("UnsupportedTemporalTypeException: Unsupported unit: Forever"),
        }
    }

    /// `Instant.until(end, unit)`: SECONDS is `secondsUntil` (the nanos round the difference toward zero), DAYS is
    /// `secondsUntil(end) / SECONDS_PER_DAY`
    fn until_chrono(&self, end: &Self, unit: ChronoUnit) -> i64 {
        let mut secs_diff: i64 = end.get_epoch_second() - self.get_epoch_second();
        let nanos_diff: i64 = end.get_nano() as i64 - self.get_nano() as i64;
        if secs_diff > 0 && nanos_diff < 0 {
            secs_diff -= 1;
        } else if secs_diff < 0 && nanos_diff > 0 {
            secs_diff += 1;
        }
        match unit {
            ChronoUnit::Seconds => secs_diff,
            ChronoUnit::Days => secs_diff / 86400,
            ChronoUnit::Forever => panic!("UnsupportedTemporalTypeException: Unsupported unit: Forever"),
        }
    }
}

/// `LocalDate.toEpochDay()` (proleptic Gregorian, 1970-01-01 is day 0)
fn local_date_epoch_day(date: &mcrust::java::time::local_date::LocalDate) -> i64 {
    let (year, month, day) = (date.get_year() as i64, date.get_month_value() as i64, date.get_day_of_month() as i64);
    let mut total: i64 = 365 * year;
    if year >= 0 {
        total += (year + 3) / 4 - (year + 99) / 100 + (year + 399) / 400;
    } else {
        total -= year / -4 - year / -100 + year / -400;
    }
    total += (367 * month - 362) / 12;
    total += day - 1;
    if month > 2 {
        total -= 1;
        let leap = (year & 3) == 0 && (year % 100 != 0 || year % 400 == 0);
        if !leap {
            total -= 1;
        }
    }
    total - 719528
}

impl TemporalAccessor for mcrust::java::time::local_date::LocalDate {
    /// `LocalDate.isSupported(TemporalField)`: the date-based fields - EPOCH_DAY, not INSTANT_SECONDS
    fn is_supported_field(&self, field: ChronoField) -> bool {
        field == ChronoField::EpochDay
    }

    fn get_long_chrono(&self, field: ChronoField) -> i64 {
        match field {
            ChronoField::EpochDay => local_date_epoch_day(self),
            ChronoField::InstantSeconds => panic!("UnsupportedTemporalTypeException: Unsupported field: InstantSeconds"),
        }
    }
}

impl Temporal for mcrust::java::time::local_date::LocalDate {
    /// `LocalDate.isSupported(TemporalUnit)`: the date-based units - DAYS, not SECONDS
    fn is_supported_unit(&self, unit: ChronoUnit) -> bool {
        unit == ChronoUnit::Days
    }

    /// `LocalDate.plus(amount, DAYS)` is `plusDays`
    fn plus_chrono(&self, amount: i64, unit: ChronoUnit) -> Self {
        match unit {
            ChronoUnit::Days => self.plus_days(amount),
            _ => panic!("UnsupportedTemporalTypeException: Unsupported unit: {:?}", unit),
        }
    }

    /// `LocalDate.until(end, DAYS)` is `daysUntil`: `end.toEpochDay() - toEpochDay()`
    fn until_chrono(&self, end: &Self, unit: ChronoUnit) -> i64 {
        match unit {
            ChronoUnit::Days => local_date_epoch_day(end) - local_date_epoch_day(self),
            _ => panic!("UnsupportedTemporalTypeException: Unsupported unit: {:?}", unit),
        }
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
