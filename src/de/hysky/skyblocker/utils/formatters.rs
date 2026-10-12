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
// TODO(fabric2seer): static `NumberFormat.Style` has no Rust counterpart in the port's NumberFormat file
//   java: public static final NumberFormat SHORT_INTEGER_NUMBERS = NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT);
/// java: public static final NumberFormat SHORT_INTEGER_NUMBERS = NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT);
pub static SHORT_INTEGER_NUMBERS: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `SHORT_INTEGER_NUMBERS` - see the TODO above"));
// TODO(fabric2seer): static `NumberFormat.Style` has no Rust counterpart in the port's NumberFormat file
//   java: public static final NumberFormat SHORT_FLOAT_NUMBERS = Util.make(NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT), nf -> nf.setMinimumFractionDigits(1));
/// java: public static final NumberFormat SHORT_FLOAT_NUMBERS = Util.make(NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT), nf -> nf.setMinimumFractionDigits(1));
pub static SHORT_FLOAT_NUMBERS: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `SHORT_FLOAT_NUMBERS` - see the TODO above"));
// TODO(fabric2seer): static `NumberFormat.Style` has no Rust counterpart in the port's NumberFormat file
//   java: public static final NumberFormat SHORT_DOUBLE_NUMBERS = Util.make(NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT), nf -> nf.setMinimumFractionDigits(2));
/// java: public static final NumberFormat SHORT_DOUBLE_NUMBERS = Util.make(NumberFormat.getCompactNumberInstance(Locale.US, NumberFormat.Style.SHORT), nf -> nf.setMinimumFractionDigits(2));
pub static SHORT_DOUBLE_NUMBERS: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `SHORT_DOUBLE_NUMBERS` - see the TODO above"));
// TODO(fabric2seer): static `NumberFormat.getNumberInstance` has no Rust counterpart in the port
//   java: public static final NumberFormat DIFF_NUMBERS = Util.make((DecimalFormat) NumberFormat.getNumberInstance(Locale.US), nf -> nf.setPositivePrefix("+"));
/// java: public static final NumberFormat DIFF_NUMBERS = Util.make((DecimalFormat) NumberFormat.getNumberInstance(Locale.US), nf -> nf.setPositivePrefix("+"));
pub static DIFF_NUMBERS: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `DIFF_NUMBERS` - see the TODO above"));
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
        // java: try {..} catch (ParseException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `DecimalFormat.parse` has no Rust method in the port (looked for `parse` in DecimalFormat and JavaSrc)
            // rust: return SHORT_INTEGER_NUMBERS.get_mut().parse(&number.replace(",", ""));
            return todo!("fabric2seer: see the TODO above");
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                panic!("NumberFormatException: {}", format!("For input string: \"{}\"", number).to_string());
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
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
