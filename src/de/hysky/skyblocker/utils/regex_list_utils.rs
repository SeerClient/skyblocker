//! Ported from de.hysky.skyblocker.utils.RegexListUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public final class RegexListUtils
#[derive(Clone)]
pub struct RegexListUtils {}

impl RegexListUtils {
    /// java: private RegexListUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int indexOfInList(List<String> list, Pattern pattern)
    pub fn index_of_in_list(list: &Vec<String>, pattern: &compat::regex::Pattern) -> i32 {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return indexOfInList(list, Function.identity(), pattern);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <T> int indexOfInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern)
    pub fn index_of_in_list_3<T: 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern) -> i32 {
        return RegexListUtils::index_of_in_list_4(list, to_string_function.clone(), pattern, 0);
    }

    /// java: public static <T> int indexOfInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern, int startIndex)
    pub fn index_of_in_list_4<T: 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern, mut start_index: i32) -> i32 {
        if start_index >= (list.len() as i32) {
            return -1;
        }
        let mut matcher: compat::regex::Matcher = pattern.matcher("");
        {
            let mut i: i32 = start_index;
            let mut list_size: i32 = (list.len() as i32);
            while i < list_size {
                let mut string: String = (to_string_function)(list[(i) as usize].clone());
                if matcher.reset_with(&string).matches() {
                    return i;
                }
                i += 1;
            }
        }
        return -1;
    }

    /// java: public static @Nullable Matcher findInList(List<String> list, Pattern pattern)
    pub fn find_in_list(list: &Vec<String>, pattern: &compat::regex::Pattern) -> Option<compat::regex::Matcher> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return findInList(list, Function.identity(), pattern);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <T> @Nullable Matcher findInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern)
    pub fn find_in_list_3<T: 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern) -> compat::regex::Matcher {
        return RegexListUtils::find_in_list_4(list, to_string_function.clone(), pattern, 0).unwrap();
    }

    /// java: public static <T> @Nullable Matcher findInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern, int startIndex)
    pub fn find_in_list_4<T: 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern, mut start_index: i32) -> Option<compat::regex::Matcher> {
        if start_index >= (list.len() as i32) {
            return None;
        }
        let mut matcher: compat::regex::Matcher = pattern.matcher("");
        {
            let mut i: i32 = start_index;
            let mut list_size: i32 = (list.len() as i32);
            while i < list_size {
                let mut string: String = (to_string_function)(list[(i) as usize].clone());
                if matcher.reset_with(&string).find() {
                    return Some(matcher.clone());
                }
                i += 1;
            }
        }
        return None;
    }

    /// java: public static @Nullable Matcher matchInList(List<String> list, Pattern pattern)
    pub fn match_in_list(list: &Vec<String>, pattern: &compat::regex::Pattern) -> Option<compat::regex::Matcher> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return matchInList(list, Function.identity(), pattern);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <T> @Nullable Matcher matchInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern)
    pub fn match_in_list_list_function_pattern<T: 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern) -> compat::regex::Matcher {
        return RegexListUtils::match_in_list_list_function_pattern_int(list, to_string_function.clone(), pattern, 0).unwrap();
    }

    /// java: public static <T> @Nullable Matcher matchInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern, int startIndex)
    pub fn match_in_list_list_function_pattern_int<T: 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern, mut start_index: i32) -> Option<compat::regex::Matcher> {
        if start_index >= (list.len() as i32) {
            return None;
        }
        let mut matcher: compat::regex::Matcher = pattern.matcher("");
        {
            let mut i: i32 = start_index;
            let mut list_size: i32 = (list.len() as i32);
            while i < list_size {
                let mut string: String = (to_string_function)(list[(i) as usize].clone());
                if matcher.reset_with(&string).matches() {
                    return Some(matcher.clone());
                }
                i += 1;
            }
        }
        return None;
    }

    /// java: public static List<Matcher> matchInList(List<String> list, Pattern... patterns)
    pub fn match_in_list_list_pattern(list: &Vec<String>, patterns: &Vec<compat::regex::Pattern>) -> Vec<compat::regex::Matcher> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return matchInList(list, Function.identity(), patterns);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <T> List<Matcher> matchInList(List<T> list, Function<T, String> toStringFunction, Pattern... patterns)
    pub fn match_in_list_list_function_pattern<T: 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, patterns: &Vec<compat::regex::Pattern>) -> Vec<compat::regex::Matcher> {
        return RegexListUtils::match_in_list_list_function_int_pattern(list, to_string_function.clone(), 0, patterns);
    }

    /// java: public static <T> List<Matcher> matchInList(List<T> list, Function<T, String> toStringFunction, int startIndex, Pattern... patterns)
    pub fn match_in_list_list_function_int_pattern<T: 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, mut start_index: i32, patterns: &Vec<compat::regex::Pattern>) -> Vec<compat::regex::Matcher> {
        if list.is_empty() {
            return vec![];
        }
        let mut matchers: Vec<compat::regex::Matcher> = patterns.iter().cloned().map(|p| p.matcher("")).collect::<Vec<_>>();
        {
            let mut i: i32 = start_index;
            let mut list_size: i32 = (list.len() as i32);
            while i < list_size {
                let mut line: String = (to_string_function)(list[(i) as usize].clone());
                let mut all_matched: bool = true;
                for matcher in matchers.iter_mut() {
                    // TODO(fabric2seer): the condition: Matcher.hasMatch
                    //   java: !matcher.hasMatch() && !matcher.reset(line).matches()
                    if todo!("fabric2seer: the condition (TODO above)") {
                        all_matched = false;
                    }
                }
                if all_matched {
                    return matchers.clone();
                }
                i += 1;
            }
        }
        return vec![];
    }

}
