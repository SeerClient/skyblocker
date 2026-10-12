//! Ported from de.hysky.skyblocker.utils.RegexUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;

/// `java.util.regex.MatchResult.group(String)`: a `Matcher` is a `MatchResult`, and so is what `Matcher.results()`
/// / `toMatchResult()` hand out.
pub trait MatchResultGroups {
    fn group_named(&self, name: &str) -> Option<String>;
}

impl MatchResultGroups for compat::regex::Matcher {
    fn group_named(&self, name: &str) -> Option<String> {
        compat::regex::Matcher::group_named(self, name)
    }
}

impl MatchResultGroups for compat::regex::MatchResult {
    fn group_named(&self, name: &str) -> Option<String> {
        compat::regex::MatchResult::group_named(self, name)
    }
}

/// `matcher.hasMatch() ? matcher.end() : 0`
fn next_start(matcher: &compat::regex::Matcher) -> i32 {
    if matcher.has_match() { matcher.end(0) } else { 0 }
}

/// java: `Integer.parseInt(s.replace(",", ""))` (no trimming: a space is a NumberFormatException)
fn parse_int(s: &str) -> i32 {
    s.replace(",", "").parse::<i32>().expect("NumberFormatException")
}

/// java: public class RegexUtils
#[derive(Clone)]
pub struct RegexUtils {}

impl RegexUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static OptionalLong findLongFromMatcher(Matcher matcher)
    pub fn find_long_from_matcher(matcher: &mut compat::regex::Matcher) -> Option<i64> {
        // java: return findLongFromMatcher(matcher, matcher.hasMatch() ? matcher.end() : 0);
        let start: i32 = next_start(matcher);
        return RegexUtils::find_long_from_matcher_2(matcher, start);
    }

    /// java: public static OptionalLong findLongFromMatcher(Matcher matcher, int startingIndex)
    pub fn find_long_from_matcher_2(matcher: &mut compat::regex::Matcher, mut starting_index: i32) -> Option<i64> {
        if !(matcher.find_from(starting_index)) {
            return None;
        }
        // java: return OptionalLong.of(Long.parseLong(matcher.group(1).replace(",", "")));
        return Some(matcher.group(1).expect("NullPointerException: group 1").replace(",", "").parse::<i64>().expect("NumberFormatException"));
    }

    /// java: public static OptionalInt findIntFromMatcher(Matcher matcher)
    pub fn find_int_from_matcher(matcher: &mut compat::regex::Matcher) -> Option<i32> {
        // java: return findIntFromMatcher(matcher, matcher.hasMatch() ? matcher.end() : 0);
        let start: i32 = next_start(matcher);
        return RegexUtils::find_int_from_matcher_2(matcher, start);
    }

    /// java: public static OptionalInt findIntFromMatcher(Matcher matcher, int startingIndex)
    pub fn find_int_from_matcher_2(matcher: &mut compat::regex::Matcher, mut starting_index: i32) -> Option<i32> {
        if !(matcher.find_from(starting_index)) {
            return None;
        }
        return Some(RegexUtils::parse_int_from_matcher(matcher, 1));
    }

    /// java: public static OptionalInt findRomanNumeralFromMatcher(Matcher matcher)
    pub fn find_roman_numeral_from_matcher(matcher: &mut compat::regex::Matcher) -> Option<i32> {
        // java: return findRomanNumeralFromMatcher(matcher, matcher.hasMatch() ? matcher.end() : 0);
        let start: i32 = next_start(matcher);
        return RegexUtils::find_roman_numeral_from_matcher_2(matcher, start);
    }

    /// java: public static OptionalInt findRomanNumeralFromMatcher(Matcher matcher, int startingIndex)
    pub fn find_roman_numeral_from_matcher_2(matcher: &mut compat::regex::Matcher, mut starting_index: i32) -> Option<i32> {
        if !(matcher.find_from(starting_index)) {
            return None;
        }
        let mut result: Option<String> = matcher.group(1);
        // java: if (StringUtils.isEmpty(result)) return OptionalInt.of(0); // Special case for level 0
        if result.as_deref().is_none_or(|s| s.is_empty()) {
            return Some(0);
        }
        if !(RomanNumerals::is_valid_roman_numeral(result.clone())) {
            return None;
        }
        let mut result_int: i32 = RomanNumerals::roman_to_decimal(result.clone());
        if result_int <= 0 {
            return None;
        }
        return Some(result_int);
    }

    /// java: public static OptionalInt parseOptionalIntFromMatcher(Matcher matcher, int group)
    pub fn parse_optional_int_from_matcher(matcher: &compat::regex::Matcher, mut group: i32) -> Option<i32> {
        let Some(mut s) = matcher.group(group) else {
            return None;
        };
        return Some(parse_int(&s));
    }

    /// java: public static OptionalInt parseOptionalIntFromMatcher(MatchResult matcher, String group)
    pub fn parse_optional_int_from_matcher_match_result_string(matcher: &impl MatchResultGroups, group: &str) -> Option<i32> {
        // java: String s = matcher.group(group); if (s == null) return OptionalInt.empty();
        let Some(mut s) = matcher.group_named(group) else {
            return None;
        };
        return Some(parse_int(&s));
    }

    /// java: public static int parseIntFromMatcher(Matcher matcher, int group)
    pub fn parse_int_from_matcher(matcher: &compat::regex::Matcher, mut group: i32) -> i32 {
        return parse_int(&matcher.group(group).expect("NullPointerException: group"));
    }

    /// java: public static int parseIntFromMatcher(Matcher matcher, String group)
    pub fn parse_int_from_matcher_matcher_string(matcher: &compat::regex::Matcher, group: &str) -> i32 {
        return parse_int(&matcher.group_named(group).expect("NullPointerException: group"));
    }

    /// java: public static OptionalDouble findDoubleFromMatcher(Matcher matcher)
    pub fn find_double_from_matcher(matcher: &mut compat::regex::Matcher) -> Option<f64> {
        // java: return findDoubleFromMatcher(matcher, matcher.hasMatch() ? matcher.end() : 0);
        let start: i32 = next_start(matcher);
        return RegexUtils::find_double_from_matcher_2(matcher, start);
    }

    /// java: public static OptionalDouble findDoubleFromMatcher(Matcher matcher, int startingIndex)
    pub fn find_double_from_matcher_2(matcher: &mut compat::regex::Matcher, mut starting_index: i32) -> Option<f64> {
        if !(matcher.find_from(starting_index)) {
            return None;
        }
        // java: Double.parseDouble(..) - which does trim (FloatingDecimal.readJavaFormatString)
        return Some((matcher.group(1).expect("NullPointerException: group 1").replace(",", "")).trim().parse::<f64>().expect("NumberFormatException"));
    }

}
