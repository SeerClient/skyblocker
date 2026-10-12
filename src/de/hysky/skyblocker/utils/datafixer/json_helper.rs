//! Ported from de.hysky.skyblocker.utils.datafixer.JsonHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class JsonHelper
#[derive(Clone)]
pub struct JsonHelper {}

impl JsonHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static OptionalInt getInt(JsonObject root, String path)
    pub fn get_int(root: &serde_json::Map<String, serde_json::Value>, path: &str) -> Option<i32> {
        if false /* java: root == null: never null here */ {
            return None;
        }
        if !(path.contains(".")) {
            return (if compat::json::has(&root, path) && !(compat::json::is_null(&root.get(path).cloned().unwrap())) { Some(compat::json::get_as_int(&root.get(path).cloned().unwrap())) } else { None });
        }
        let mut split: Vec<String> = compat::regex::split(&path, "\\.");
        let mut property_name: String = split[((split.len() as i32).wrapping_sub(1)) as usize].clone();
        let mut objects2_traverse: Vec<String> = (0..((split.len() as i32).wrapping_sub(1)) as usize).map(|_| Default::default()).collect::<Vec<_>>();
        objects2_traverse[(0) as usize..((0) + ((split.len() as i32).wrapping_sub(1))) as usize].clone_from_slice(&split[(0) as usize..((0) + ((split.len() as i32).wrapping_sub(1))) as usize]);
        let mut current_level: serde_json::Map<String, serde_json::Value> = root.clone();
        for object_name in objects2_traverse.iter() {
            if compat::json::has(&current_level, object_name) && !(compat::json::is_null(&current_level.get(object_name).cloned().unwrap())) {
                current_level = compat::json::get_as_json_object_of(&current_level, object_name);
            } else {
                return None;
            }
        }
        return (if compat::json::has(&current_level, &property_name) && !(compat::json::is_null(&current_level.get(&property_name).cloned().unwrap())) { Some(compat::json::get_as_int(&current_level.get(&property_name).cloned().unwrap())) } else { None });
    }

    /// java: public static Optional<Boolean> getBoolean(JsonObject root, String path)
    pub fn get_boolean(root: &serde_json::Map<String, serde_json::Value>, path: &str) -> Option<bool> {
        if false /* java: root == null: never null here */ {
            return None;
        }
        if !(path.contains(".")) {
            return (if compat::json::has(&root, path) && !(compat::json::is_null(&root.get(path).cloned().unwrap())) { Some(compat::json::get_as_boolean(&root.get(path).cloned().unwrap())) } else { None });
        }
        let mut split: Vec<String> = compat::regex::split(&path, "\\.");
        let mut property_name: String = split[((split.len() as i32).wrapping_sub(1)) as usize].clone();
        let mut objects2_traverse: Vec<String> = (0..((split.len() as i32).wrapping_sub(1)) as usize).map(|_| Default::default()).collect::<Vec<_>>();
        objects2_traverse[(0) as usize..((0) + ((split.len() as i32).wrapping_sub(1))) as usize].clone_from_slice(&split[(0) as usize..((0) + ((split.len() as i32).wrapping_sub(1))) as usize]);
        let mut current_level: serde_json::Map<String, serde_json::Value> = root.clone();
        for object_name in objects2_traverse.iter() {
            if compat::json::has(&current_level, object_name) && !(compat::json::is_null(&current_level.get(object_name).cloned().unwrap())) {
                current_level = compat::json::get_as_json_object_of(&current_level, object_name);
            } else {
                return None;
            }
        }
        return (if compat::json::has(&current_level, &property_name) && !(compat::json::is_null(&current_level.get(&property_name).cloned().unwrap())) { Some(compat::json::get_as_boolean(&current_level.get(&property_name).cloned().unwrap())) } else { None });
    }

    /// java: public static Optional<String> getString(JsonObject root, String path)
    pub fn get_string(root: &serde_json::Map<String, serde_json::Value>, path: &str) -> Option<String> {
        if false /* java: root == null: never null here */ {
            return None;
        }
        if !(path.contains(".")) {
            return (if compat::json::has(&root, path) && !(compat::json::is_null(&root.get(path).cloned().unwrap())) { Some(compat::json::get_as_string(&root.get(path).cloned().unwrap())) } else { None });
        }
        let mut split: Vec<String> = compat::regex::split(&path, "\\.");
        let mut property_name: String = split[((split.len() as i32).wrapping_sub(1)) as usize].clone();
        let mut objects2_traverse: Vec<String> = (0..((split.len() as i32).wrapping_sub(1)) as usize).map(|_| Default::default()).collect::<Vec<_>>();
        objects2_traverse[(0) as usize..((0) + ((split.len() as i32).wrapping_sub(1))) as usize].clone_from_slice(&split[(0) as usize..((0) + ((split.len() as i32).wrapping_sub(1))) as usize]);
        let mut current_level: serde_json::Map<String, serde_json::Value> = root.clone();
        for object_name in objects2_traverse.iter() {
            if compat::json::has(&current_level, object_name) && !(compat::json::is_null(&current_level.get(object_name).cloned().unwrap())) {
                current_level = compat::json::get_as_json_object_of(&current_level, object_name);
            } else {
                return None;
            }
        }
        return (if compat::json::has(&current_level, &property_name) && !(compat::json::is_null(&current_level.get(&property_name).cloned().unwrap())) { Some(compat::json::get_as_string(&current_level.get(&property_name).cloned().unwrap())) } else { None });
    }

}
