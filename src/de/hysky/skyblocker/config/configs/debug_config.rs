//! Ported from de.hysky.skyblocker.config.configs.DebugConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::debug::debug::DumpFormat;

/// java: public class DebugConfig
#[derive(Clone)]
pub struct DebugConfig {
    /// java: public int dumpRange = 5;
    pub dump_range: i32,
    /// java: public Debug.DumpFormat dumpFormat = Debug.DumpFormat.SNBT;
    pub dump_format: DumpFormat,
    /// java: public boolean showInvisibleArmorStands = false;
    pub show_invisible_armor_stands: bool,
    /// java: public boolean webSocketDebug = false;
    pub web_socket_debug: bool,
    /// java: public boolean corpseFinderDebug = false;
    pub corpse_finder_debug: bool,
    /// java: public boolean enableRepoDev = false;
    pub enable_repo_dev: bool,
    /// java: public SkyBlockResourcePack skyblockResourcePack = new SkyBlockResourcePack();
    pub skyblock_resource_pack: SkyBlockResourcePack,
}

impl DebugConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            dump_range: 5,
            dump_format: DumpFormat::Snbt,
            show_invisible_armor_stands: false,
            web_socket_debug: false,
            corpse_finder_debug: false,
            enable_repo_dev: false,
            skyblock_resource_pack: SkyBlockResourcePack::new(),
        };
        this
    }

}

/// java: public static class SkyBlockResourcePack
#[derive(Clone)]
pub struct SkyBlockResourcePack {
    /// java: public boolean downloadResourcePack = false;
    pub download_resource_pack: bool,
}

impl SkyBlockResourcePack {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            download_resource_pack: false,
        };
        this
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/debug_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// DebugConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl DebugConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("dumpRange".to_string(), dandelion::compat::GsonValue::int(*(&self.dump_range) as i64)),
            ("dumpFormat".to_string(), dandelion::compat::GsonValue::str(((&self.dump_format)).name())),
            ("showInvisibleArmorStands".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_invisible_armor_stands))),
            ("webSocketDebug".to_string(), dandelion::compat::GsonValue::Bool(*(&self.web_socket_debug))),
            ("corpseFinderDebug".to_string(), dandelion::compat::GsonValue::Bool(*(&self.corpse_finder_debug))),
            ("enableRepoDev".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_repo_dev))),
            ("skyblockResourcePack".to_string(), ((&self.skyblock_resource_pack)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<DebugConfig> {
        let __o = __v.as_object()?;
        let mut __this = DebugConfig::new();
        if let Some(__v) = __o.get("dumpRange") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.dump_range = __x; }
        }
        if let Some(__v) = __o.get("dumpFormat") {
            if let Some(__x) = (__v).as_str().and_then(|__s| DumpFormat::values().into_iter().find(|__c| __c.name() == __s)) { __this.dump_format = __x; }
        }
        if let Some(__v) = __o.get("showInvisibleArmorStands") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_invisible_armor_stands = __x; }
        }
        if let Some(__v) = __o.get("webSocketDebug") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.web_socket_debug = __x; }
        }
        if let Some(__v) = __o.get("corpseFinderDebug") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.corpse_finder_debug = __x; }
        }
        if let Some(__v) = __o.get("enableRepoDev") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_repo_dev = __x; }
        }
        if let Some(__v) = __o.get("skyblockResourcePack") {
            if let Some(__x) = SkyBlockResourcePack::__gson_read(__v) { __this.skyblock_resource_pack = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "dumpFormat" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match DumpFormat::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.dump_format = __c; true } None => false } } else { false },
                "showInvisibleArmorStands" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_invisible_armor_stands = *__b; true } else { false },
                "webSocketDebug" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.web_socket_debug = *__b; true } else { false },
                "corpseFinderDebug" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.corpse_finder_debug = *__b; true } else { false },
                "enableRepoDev" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_repo_dev = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "skyblockResourcePack" => self.skyblock_resource_pack.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `DebugConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<DebugConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DebugConfig",
            new: DebugConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: DebugConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SkyBlockResourcePack's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SkyBlockResourcePack {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("downloadResourcePack".to_string(), dandelion::compat::GsonValue::Bool(*(&self.download_resource_pack))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SkyBlockResourcePack> {
        let __o = __v.as_object()?;
        let mut __this = SkyBlockResourcePack::new();
        if let Some(__v) = __o.get("downloadResourcePack") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.download_resource_pack = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "downloadResourcePack" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.download_resource_pack = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SkyBlockResourcePack.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SkyBlockResourcePack> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DebugConfig$SkyBlockResourcePack",
            new: SkyBlockResourcePack::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SkyBlockResourcePack::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
