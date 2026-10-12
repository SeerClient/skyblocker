//! Ported from de.hysky.skyblocker.utils.RegexListUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public final class RegexListUtils
#[derive(Clone)]
pub struct RegexListUtils {}

/// `Function.identity()` for a `List<String>`.
fn identity() -> Arc<dyn Fn(String) -> String> {
    Arc::new(|s: String| s)
}

impl RegexListUtils {
    /// java: private RegexListUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int indexOfInList(List<String> list, Pattern pattern)
    pub fn index_of_in_list(list: &Vec<String>, pattern: &compat::regex::Pattern) -> i32 {
        // java: return indexOfInList(list, Function.identity(), pattern);
        return RegexListUtils::index_of_in_list_3(list, identity(), pattern);
    }

    /// java: public static <T> int indexOfInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern)
    pub fn index_of_in_list_3<T: Clone + 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern) -> i32 {
        return RegexListUtils::index_of_in_list_4(list, to_string_function.clone(), pattern, 0);
    }

    /// java: public static <T> int indexOfInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern, int startIndex)
    pub fn index_of_in_list_4<T: Clone + 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern, mut start_index: i32) -> i32 {
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
        // java: return findInList(list, Function.identity(), pattern);
        return RegexListUtils::find_in_list_3(list, identity(), pattern);
    }

    /// java: public static <T> @Nullable Matcher findInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern)
    pub fn find_in_list_3<T: Clone + 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern) -> Option<compat::regex::Matcher> {
        return RegexListUtils::find_in_list_4(list, to_string_function.clone(), pattern, 0);
    }

    /// java: public static <T> @Nullable Matcher findInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern, int startIndex)
    pub fn find_in_list_4<T: Clone + 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern, mut start_index: i32) -> Option<compat::regex::Matcher> {
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
                    return Some(matcher);
                }
                i += 1;
            }
        }
        return None;
    }

    /// java: public static @Nullable Matcher matchInList(List<String> list, Pattern pattern)
    pub fn match_in_list(list: &Vec<String>, pattern: &compat::regex::Pattern) -> Option<compat::regex::Matcher> {
        // java: return matchInList(list, Function.identity(), pattern);
        return RegexListUtils::match_in_list_list_function_pattern(list, identity(), pattern);
    }

    /// java: public static <T> @Nullable Matcher matchInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern)
    pub fn match_in_list_list_function_pattern<T: Clone + 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern) -> Option<compat::regex::Matcher> {
        return RegexListUtils::match_in_list_list_function_pattern_int(list, to_string_function.clone(), pattern, 0);
    }

    /// java: public static <T> @Nullable Matcher matchInList(List<T> list, Function<T, String> toStringFunction, Pattern pattern, int startIndex)
    pub fn match_in_list_list_function_pattern_int<T: Clone + 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, pattern: &compat::regex::Pattern, mut start_index: i32) -> Option<compat::regex::Matcher> {
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
                    return Some(matcher);
                }
                i += 1;
            }
        }
        return None;
    }

    /// java: public static List<Matcher> matchInList(List<String> list, Pattern... patterns)
    pub fn match_in_list_list_pattern(list: &Vec<String>, patterns: &Vec<compat::regex::Pattern>) -> Vec<compat::regex::Matcher> {
        // java: return matchInList(list, Function.identity(), patterns);
        return RegexListUtils::match_in_list_list_function_patterns(list, identity(), patterns);
    }

    /// java: public static <T> List<Matcher> matchInList(List<T> list, Function<T, String> toStringFunction, Pattern... patterns)
    /// (named apart from the single-pattern overload, which Rust cannot overload)
    pub fn match_in_list_list_function_patterns<T: Clone + 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, patterns: &Vec<compat::regex::Pattern>) -> Vec<compat::regex::Matcher> {
        return RegexListUtils::match_in_list_list_function_int_pattern(list, to_string_function.clone(), 0, patterns);
    }

    /// java: public static <T> List<Matcher> matchInList(List<T> list, Function<T, String> toStringFunction, int startIndex, Pattern... patterns)
    pub fn match_in_list_list_function_int_pattern<T: Clone + 'static>(list: &Vec<T>, to_string_function: Arc<dyn Fn(T) -> String>, mut start_index: i32, patterns: &Vec<compat::regex::Pattern>) -> Vec<compat::regex::Matcher> {
        if list.is_empty() {
            return vec![];
        }
        let mut matchers: Vec<compat::regex::Matcher> = patterns.iter().map(|p| p.matcher("")).collect::<Vec<_>>();
        // `Matcher.hasMatch()`: whether each matcher's last match operation succeeded (a fresh matcher has none)
        let mut has_match: Vec<bool> = vec![false; matchers.len()];
        {
            let mut i: i32 = start_index;
            let mut list_size: i32 = (list.len() as i32);
            while i < list_size {
                let mut line: String = (to_string_function)(list[(i) as usize].clone());
                let mut all_matched: bool = true;
                for (k, matcher) in matchers.iter_mut().enumerate() {
                    // java: if (!matcher.hasMatch() && !matcher.reset(line).matches()) allMatched = false;
                    if !has_match[k] {
                        has_match[k] = matcher.reset_with(&line).matches();
                        if !has_match[k] {
                            all_matched = false;
                        }
                    }
                }
                if all_matched {
                    return matchers;
                }
                i += 1;
            }
        }
        return vec![];
    }

}
