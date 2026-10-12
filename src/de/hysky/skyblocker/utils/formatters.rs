//! Ported from de.hysky.skyblocker.utils.Formatters by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::lang::number::Number;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::text::number_format;
use mcrust::java::time::format::date_time_formatter::DateTimeFormatter;
use mcrust::java::time::zone_id::ZoneId;
use mcrust::net::minecraft::util::util;

/// java: public static final NumberFormat INTEGER_NUMBERS = NumberFormat.getIntegerInstance(Locale.US);
pub static INTEGER_NUMBERS: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| number_format::get_integer_instance(Locale::Us));
/// java: public static final NumberFormat TRIPLE_NUMBERS = Util.make(NumberFormat.getInstance(Locale.US), nf -> nf.setMaximumFractionDigits(3));
/// (`NumberFormat.getInstance(Locale.US)` is a `DecimalFormat` over the US number pattern `#,##0.###`; with at most 3
/// fraction digits that is the pattern `#,##0.###`)
pub static TRIPLE_NUMBERS: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| DecimalFormat::new("#,##0.###", mcrust::java::text::decimal_format_symbols::get_instance(Locale::Us)));
/// java: public static final NumberFormat DOUBLE_NUMBERS = Util.make(NumberFormat.getInstance(Locale.US), nf -> nf.setMaximumFractionDigits(2));
/// (`NumberFormat.getInstance(Locale.US)` is a `DecimalFormat` over the US number pattern `#,##0.###`; with at most 2
/// fraction digits that is the pattern `#,##0.##`)
pub static DOUBLE_NUMBERS: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| DecimalFormat::new("#,##0.##", mcrust::java::text::decimal_format_symbols::get_instance(Locale::Us)));
/// java: public static final NumberFormat FLOAT_NUMBERS = Util.make(NumberFormat.getInstance(Locale.US), nf -> nf.setMaximumFractionDigits(1));
/// (`NumberFormat.getInstance(Locale.US)` is a `DecimalFormat` over the US number pattern `#,##0.###`; with at most 1
/// fraction digits that is the pattern `#,##0.#`)
pub static FLOAT_NUMBERS: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| DecimalFormat::new("#,##0.#", mcrust::java::text::decimal_format_symbols::get_instance(Locale::Us)));
/// java: public static final NumberFormat SHORT_INTEGER_NUMBERS = NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT);
pub static SHORT_INTEGER_NUMBERS: compat::MainThread<CompactNumberFormat> = compat::MainThread::new(|| CompactNumberFormat::us_short());
/// java: public static final NumberFormat SHORT_FLOAT_NUMBERS = Util.make(NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT), nf -> nf.setMinimumFractionDigits(1));
pub static SHORT_FLOAT_NUMBERS: compat::MainThread<CompactNumberFormat> = compat::MainThread::new(|| {
    let mut nf = CompactNumberFormat::us_short();
    nf.set_minimum_fraction_digits(1);
    nf
});
/// java: public static final NumberFormat SHORT_DOUBLE_NUMBERS = Util.make(NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT), nf -> nf.setMinimumFractionDigits(2));
pub static SHORT_DOUBLE_NUMBERS: compat::MainThread<CompactNumberFormat> = compat::MainThread::new(|| {
    let mut nf = CompactNumberFormat::us_short();
    nf.set_minimum_fraction_digits(2);
    nf
});
/// java: public static final NumberFormat DIFF_NUMBERS = Util.make((DecimalFormat) NumberFormat.getNumberInstance(Locale.US), nf -> nf.setPositivePrefix("+"));
/// (`getNumberInstance(Locale.US)` is the US number pattern `#,##0.###`)
pub static DIFF_NUMBERS: compat::MainThread<PositivePrefixDecimalFormat> = compat::MainThread::new(|| PositivePrefixDecimalFormat {
    format: DecimalFormat::new("#,##0.###", mcrust::java::text::decimal_format_symbols::get_instance(Locale::Us)),
    positive_prefix: "+".to_string(),
});

/// `java.text.CompactNumberFormat` for `Locale.US` and `NumberFormat.Style.SHORT` - the JDK class the three
/// `SHORT_*_NUMBERS` instances are, which the port's `java.text` does not carry (GAPS_L.md). US SHORT's patterns
/// (CLDR) are `0K 00K 000K 0M 00M 000M 0B 00B 000B 0T 00T 000T` for 10^3 .. 10^14, the same for every plural
/// count; below 1000 there is no pattern and the number is formatted plainly. The instance starts as
/// `CompactNumberFormat`'s constructor leaves it: no grouping, 0 fraction digits, rounding HALF_EVEN.
#[derive(Clone, Debug)]
pub struct CompactNumberFormat {
    minimum_fraction_digits: usize,
    maximum_fraction_digits: usize,
}

/// The US SHORT suffixes, by the power of ten a pattern starts at (index 3 = `0K`).
const US_SHORT_SUFFIXES: [&str; 15] = ["", "", "", "K", "K", "K", "M", "M", "M", "B", "B", "B", "T", "T", "T"];

impl CompactNumberFormat {
    /// `NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT)`.
    pub fn us_short() -> CompactNumberFormat {
        CompactNumberFormat { minimum_fraction_digits: 0, maximum_fraction_digits: 0 }
    }

    /// `NumberFormat.setMinimumFractionDigits`: the maximum is raised with it.
    pub fn set_minimum_fraction_digits(&mut self, new_value: usize) {
        self.minimum_fraction_digits = new_value;
        if self.maximum_fraction_digits < new_value {
            self.maximum_fraction_digits = new_value;
        }
    }

    /// The digits of `value` (non-negative) with HALF_EVEN rounding on the exact value (as the port's
    /// `DecimalFormat` rounds) to between `min` and `max` fraction digits, no grouping.
    fn subformat(value: f64, min: usize, max: usize) -> String {
        let rounded = format!("{:.*}", max, value);
        let (integer, fraction) = match rounded.split_once('.') {
            Some((i, f)) => (i.to_string(), f.to_string()),
            None => (rounded, String::new()),
        };
        let mut fraction: Vec<char> = fraction.chars().collect();
        while fraction.len() > min && fraction.last() == Some(&'0') {
            fraction.pop();
        }
        if fraction.is_empty() { integer } else { format!("{}.{}", integer, fraction.into_iter().collect::<String>()) }
    }

    /// `CompactNumberFormat.format(double)`.
    pub fn format(&self, number: f64) -> String {
        if number.is_nan() {
            return "NaN".to_string();
        }
        let is_negative = number < 0.0 || (number == 0.0 && (1.0 / number) < 0.0);
        let sign = if is_negative { "-" } else { "" };
        if number.is_infinite() {
            return format!("{}\u{221E}", sign);
        }
        let number = number.abs();
        // The double is first rounded with the MINIMUM fraction digits; its integer part picks the pattern
        // ("999.6 should round up to 1000 and output 1K").
        let rounded_number: f64 = format!("{:.*}", self.minimum_fraction_digits, number).parse().unwrap_or(number);
        let integer_part = rounded_number as i64;
        // `selectCompactPattern`: the largest power of ten not above the number, capped at the last pattern.
        let mut index: Option<usize> = None;
        let mut power: i64 = 1;
        for i in 0..US_SHORT_SUFFIXES.len() {
            if integer_part >= power {
                index = Some(i);
            }
            power = power.saturating_mul(10);
        }
        if let Some(i) = index
            && !US_SHORT_SUFFIXES[i].is_empty()
        {
            // The divisor is 10^(i - (zeros - 1)): every K pattern divides by 1000, and so on.
            let divisor = 10f64.powi((i - i % 3) as i32);
            let scaled = rounded_number / divisor;
            return format!("{}{}{}", sign, CompactNumberFormat::subformat(scaled, self.minimum_fraction_digits, self.maximum_fraction_digits), US_SHORT_SUFFIXES[i]);
        }
        // No pattern: the default decimal format, with the same fraction digits.
        format!("{}{}", sign, CompactNumberFormat::subformat(number, self.minimum_fraction_digits, self.maximum_fraction_digits))
    }

    /// `NumberFormat.parse(String)`: the longest number at the start (`Err` - Java's `ParseException` - when
    /// there is none), multiplied by the divisor of a compact suffix that follows it. As `CompactNumberFormat`
    /// answers, an integral result is a `Long` and anything else a `Double`.
    pub fn parse(&self, source: &str) -> Result<Number, String> {
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0usize;
        let negative = chars.first() == Some(&'-');
        if negative {
            i += 1;
        }
        let start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        if i < chars.len() && chars[i] == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit() {
            i += 1;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
        }
        if i == start {
            return Err(format!("Unparseable number: \"{}\"", source));
        }
        let digits: String = chars[start..i].iter().collect();
        let mut value: f64 = digits.parse().map_err(|_| format!("Unparseable number: \"{}\"", source))?;
        let multiplier = match chars.get(i) {
            Some('K') => 1e3,
            Some('M') => 1e6,
            Some('B') => 1e9,
            Some('T') => 1e12,
            _ => 1.0,
        };
        value *= multiplier;
        if negative {
            value = -value;
        }
        if value.fract() == 0.0 && value.abs() < 9.2e18 && !(value == 0.0 && negative) {
            Ok(Number::Long(value as i64))
        } else {
            Ok(Number::Double(value))
        }
    }
}

/// `DecimalFormat` after `setPositivePrefix(..)` (the port's `DecimalFormat` has no prefixes): the positive prefix
/// goes before every number that is not negative; the negative prefix stays `-`.
#[derive(Clone, Debug)]
pub struct PositivePrefixDecimalFormat {
    format: DecimalFormat,
    positive_prefix: String,
}

impl PositivePrefixDecimalFormat {
    /// `NumberFormat.format(double)`.
    pub fn format(&self, number: f64) -> String {
        let formatted = self.format.format(number);
        if number.is_nan() || number.is_sign_negative() { formatted } else { format!("{}{}", self.positive_prefix, formatted) }
    }
}
//   java: public static final DateTimeFormatter DATE_FORMATTER = DateTimeFormatter.ofPattern("E MMM d yyyy " + getTimeFormat(), Locale.US).withZone(getTimeZone());
/// java: public static final DateTimeFormatter DATE_FORMATTER = DateTimeFormatter.ofPattern("E MMM d yyyy " + getTimeFormat(), Locale.US).withZone(getTimeZone());
/// The port's `DateTimeFormatter` is a pattern only (English names, no zone - `Locale.US`'s): `withZone(getTimeZone())`
/// is applied where it formats (`ZonedDateTime::of_instant(.., Formatters::get_time_zone())`) - the convention of
/// `ObtainedDateTooltip.OBTAINED_DATE_FORMATTER` (GAPS_batchB.md).
pub static DATE_FORMATTER: compat::MainThread<DateTimeFormatter> = compat::MainThread::new(|| DateTimeFormatter::of_pattern(&format!("E MMM d yyyy {}", Formatters::get_time_format())));

/// java: public class Formatters
#[derive(Clone)]
pub struct Formatters {}

impl Formatters {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Number parseNumber(String number) throws NumberFormatException
    pub fn parse_number(number: &str) -> Number {
        // java: try { return SHORT_INTEGER_NUMBERS.parse(number.replace(",", "")); }
        //       catch (ParseException _) { throw new NumberFormatException("For input string: \"" + number + "\""); }
        match SHORT_INTEGER_NUMBERS.get().parse(&number.replace(",", "")) {
            Ok(value) => return value,
            Err(_) => {
                panic!("NumberFormatException: {}", format!("For input string: \"{}\"", number));
            }
        }
    }

    /// java: private static String getTimeFormat()
    pub fn get_time_format() -> String {
        return (if Formatters::is12_hour_clock() || crate::de::hysky::skyblocker::debug::debug::Debug::is_test_environment() { "h:mm:ss a" } else { "HH:mm:ss" }).to_string();
    }

    /// java: private static ZoneId getTimeZone()
    pub fn get_time_zone() -> ZoneId {
        // java: return Debug.isTestEnvironment() ? ZoneId.of("UTC") : ZoneId.systemDefault();
        // rust: port gap - the port's `ZoneId` is the system default only (`ZoneId.of` is not ported), so the test
        //   environment's UTC zone cannot be named (GAPS_batchE.md)
        return ZoneId::system_default();
    }

    /// java: private static boolean is12HourClock()
    pub fn is12_hour_clock() -> bool {
        return match mcrust::net::minecraft::util::util::get_platform() {
            mcrust::net::minecraft::util::util::OS::Osx => crate::de::hysky::skyblocker::compatibility::mac_compatibility::MacCompatibility::is12_hour_clock(),
            mcrust::net::minecraft::util::util::OS::Windows => crate::de::hysky::skyblocker::compatibility::windows_compatibility::WindowsCompatibility::is12_hour_clock(),
            // The j formatting template returns the preferred formatting for the time
            // If the format contains a (am/pm pattern) then the preference is to use the 12 hour clock, otherwise its the 24 hour clock
            // java: default -> DateTimePatternGenerator.getInstance(Locale.getDefault()).getBestPattern("j").contains("a");
            // rust: port gap - ICU's `DateTimePatternGenerator` (and a locale database) is not in the port: the default
            //   locale's pattern cannot be asked, and the 24 hour clock is taken (GAPS_batchE.md)
            _ => false,
        };
    }

}
