//! Ported from de.hysky.skyblocker.utils.RegexUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::roman_numerals::RomanNumerals;

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
    pub fn find_long_from_matcher(matcher: &compat::regex::Matcher) -> Option<i64> {
        // TODO(fabric2seer): Matcher.hasMatch
        //   java: return findLongFromMatcher(matcher, matcher.hasMatch() ? matcher.end() : 0);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static OptionalLong findLongFromMatcher(Matcher matcher, int startingIndex)
    pub fn find_long_from_matcher_2(matcher: &compat::regex::Matcher, mut starting_index: i32) -> Option<i64> {
        if !(matcher.find_from(starting_index)) {
            // TODO(fabric2seer): static call OptionalLong.empty(..)
            //   java: return OptionalLong.empty();
        }
        // TODO(fabric2seer): static call OptionalLong.of(..)
        //   java: return OptionalLong.of(Long.parseLong(matcher.group(1).replace(",", "")));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static OptionalInt findIntFromMatcher(Matcher matcher)
    pub fn find_int_from_matcher(matcher: &compat::regex::Matcher) -> Option<i32> {
        // TODO(fabric2seer): Matcher.hasMatch
        //   java: return findIntFromMatcher(matcher, matcher.hasMatch() ? matcher.end() : 0);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static OptionalInt findIntFromMatcher(Matcher matcher, int startingIndex)
    pub fn find_int_from_matcher_2(matcher: &compat::regex::Matcher, mut starting_index: i32) -> Option<i32> {
        if !(matcher.find_from(starting_index)) {
            return None;
        }
        return Some(RegexUtils::parse_int_from_matcher(matcher, 1));
    }

    /// java: public static OptionalInt findRomanNumeralFromMatcher(Matcher matcher)
    pub fn find_roman_numeral_from_matcher(matcher: &compat::regex::Matcher) -> Option<i32> {
        // TODO(fabric2seer): Matcher.hasMatch
        //   java: return findRomanNumeralFromMatcher(matcher, matcher.hasMatch() ? matcher.end() : 0);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static OptionalInt findRomanNumeralFromMatcher(Matcher matcher, int startingIndex)
    pub fn find_roman_numeral_from_matcher_2(matcher: &compat::regex::Matcher, mut starting_index: i32) -> Option<i32> {
        if !(matcher.find_from(starting_index)) {
            return None;
        }
        let mut result: String = matcher.group(1).unwrap();
        // TODO(fabric2seer): the condition: static call StringUtils.isEmpty(..)
        //   java: StringUtils.isEmpty(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            return Some(0);
        }
        if !(RomanNumerals::is_valid_roman_numeral(Some(result.clone()))) {
            return None;
        }
        let mut result_int: i32 = RomanNumerals::roman_to_decimal(Some(result.clone()));
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
        return Some((s.replace(",", "")).trim().parse::<i32>().expect("NumberFormatException"));
    }

    /// java: public static OptionalInt parseOptionalIntFromMatcher(MatchResult matcher, String group)
    pub fn parse_optional_int_from_matcher_match_result_string(matcher: &crate::unresolved::MatchResult, group: &str) -> Option<i32> {
        // TODO(fabric2seer): `.group(..)` on a MatchResult
        //   java: String s = matcher.group(group);
        let mut s: String = todo!("fabric2seer: the initialiser of `s` (TODO above)");
        if false /* java: s == null: never null here */ {
            return None;
        }
        return Some((s.replace(",", "")).trim().parse::<i32>().expect("NumberFormatException"));
    }

    /// java: public static int parseIntFromMatcher(Matcher matcher, int group)
    pub fn parse_int_from_matcher(matcher: &compat::regex::Matcher, mut group: i32) -> i32 {
        return (matcher.group(group).unwrap().replace(",", "")).trim().parse::<i32>().expect("NumberFormatException");
    }

    /// java: public static int parseIntFromMatcher(Matcher matcher, String group)
    pub fn parse_int_from_matcher_matcher_string(matcher: &compat::regex::Matcher, group: &str) -> i32 {
        return (matcher.group_named(group).unwrap().replace(",", "")).trim().parse::<i32>().expect("NumberFormatException");
    }

    /// java: public static OptionalDouble findDoubleFromMatcher(Matcher matcher)
    pub fn find_double_from_matcher(matcher: &compat::regex::Matcher) -> Option<f64> {
        // TODO(fabric2seer): Matcher.hasMatch
        //   java: return findDoubleFromMatcher(matcher, matcher.hasMatch() ? matcher.end() : 0);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static OptionalDouble findDoubleFromMatcher(Matcher matcher, int startingIndex)
    pub fn find_double_from_matcher_2(matcher: &compat::regex::Matcher, mut starting_index: i32) -> Option<f64> {
        if !(matcher.find_from(starting_index)) {
            return None;
        }
        return Some((matcher.group(1).unwrap().replace(",", "")).trim().parse::<f64>().expect("NumberFormatException"));
    }

}
