//! Ported from de.hysky.skyblocker.config.configs.OtherLocationsConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;

/// java: public class OtherLocationsConfig
#[derive(Clone)]
pub struct OtherLocationsConfig {
    /// java: public Barn barn = new Barn();
    pub barn: Barn,
    /// java: public Rift rift = new Rift();
    pub rift: Rift,
    /// java: public TheEnd end = new TheEnd();
    pub end: TheEnd,
    /// java: public SpidersDen spidersDen = new SpidersDen();
    pub spiders_den: SpidersDen,
}

impl OtherLocationsConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            barn: Barn::new(),
            rift: Rift::new(),
            end: TheEnd::new(),
            spiders_den: SpidersDen::new(),
        };
        this
    }

}

/// java: public static class Barn
#[derive(Clone)]
pub struct Barn {
    /// java: public boolean enableGlowingMushroomHelper = true;
    pub enable_glowing_mushroom_helper: bool,
    /// java: public boolean solveHungryHiker = true;
    pub solve_hungry_hiker: bool,
    /// java: public boolean solveTreasureHunter = true;
    pub solve_treasure_hunter: bool,
    /// java: public boolean enableCallTrevorMessage = true;
    pub enable_call_trevor_message: bool,
    /// java: public boolean enablePeltAnimalHighlighter = true;
    pub enable_pelt_animal_highlighter: bool,
}

impl Barn {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_glowing_mushroom_helper: true,
            solve_hungry_hiker: true,
            solve_treasure_hunter: true,
            enable_call_trevor_message: true,
            enable_pelt_animal_highlighter: true,
        };
        this
    }

}

/// java: public static class Rift
#[derive(Clone)]
pub struct Rift {
    /// java: public boolean mirrorverseWaypoints = true;
    pub mirrorverse_waypoints: bool,
    /// java: public boolean blobbercystGlow = true;
    pub blobbercyst_glow: bool,
    /// java: public boolean enigmaSoulWaypoints = false;
    pub enigma_soul_waypoints: bool,
    /// java: public boolean highlightFoundEnigmaSouls = true;
    pub highlight_found_enigma_souls: bool,
    /// java: public boolean autoDetectMcGrubber = true;
    pub auto_detect_mc_grubber: bool,
    /// java: public int mcGrubberStacks = 0;
    pub mc_grubber_stacks: i32,
}

impl Rift {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mirrorverse_waypoints: true,
            blobbercyst_glow: true,
            enigma_soul_waypoints: false,
            highlight_found_enigma_souls: true,
            auto_detect_mc_grubber: true,
            mc_grubber_stacks: 0,
        };
        this
    }

}

/// java: public static class TheEnd
#[derive(Clone)]
pub struct TheEnd {
    /// java: public boolean enableEnderNodeHelper = true;
    pub enable_ender_node_helper: bool,
    /// java: public Waypoint.Type enderNodeWaypointType = Waypoint.Type.OUTLINED_HIGHLIGHT;
    pub ender_node_waypoint_type: Type,
    /// java: @Deprecated public transient boolean hudEnabled = true;
    pub hud_enabled: bool,
    /// java: @Deprecated public transient boolean zealotKillsEnabled = true;
    pub zealot_kills_enabled: bool,
    /// java: @Deprecated public transient boolean protectorLocationEnabled = true;
    pub protector_location_enabled: bool,
    /// java: public boolean waypoint = true;
    pub waypoint: bool,
    /// java: public boolean muteEndermanSounds = true;
    pub mute_enderman_sounds: bool,
    /// java: @Deprecated public transient int x = 10;
    pub x: i32,
    /// java: @Deprecated public transient int y = 10;
    pub y: i32,
}

impl TheEnd {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_ender_node_helper: true,
            ender_node_waypoint_type: Type::OutlinedHighlight,
            hud_enabled: true,
            zealot_kills_enabled: true,
            protector_location_enabled: true,
            waypoint: true,
            mute_enderman_sounds: true,
            x: 10,
            y: 10,
        };
        this
    }

}

/// java: public static class SpidersDen
#[derive(Clone)]
pub struct SpidersDen {
    /// java: public Relics relics = new Relics();
    pub relics: Relics,
}

impl SpidersDen {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            relics: Relics::new(),
        };
        this
    }

}

/// java: public static class Relics
#[derive(Clone)]
pub struct Relics {
    /// java: public boolean enableRelicsHelper = false;
    pub enable_relics_helper: bool,
    /// java: public boolean highlightFoundRelics = true;
    pub highlight_found_relics: bool,
}

impl Relics {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_relics_helper: false,
            highlight_found_relics: true,
        };
        this
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/other_locations_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// OtherLocationsConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl OtherLocationsConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("barn".to_string(), ((&self.barn)).__gson_write()),
            ("rift".to_string(), ((&self.rift)).__gson_write()),
            ("end".to_string(), ((&self.end)).__gson_write()),
            ("spidersDen".to_string(), ((&self.spiders_den)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<OtherLocationsConfig> {
        let __o = __v.as_object()?;
        let mut __this = OtherLocationsConfig::new();
        if let Some(__v) = __o.get("barn") {
            if let Some(__x) = Barn::__gson_read(__v) { __this.barn = __x; }
        }
        if let Some(__v) = __o.get("rift") {
            if let Some(__x) = Rift::__gson_read(__v) { __this.rift = __x; }
        }
        if let Some(__v) = __o.get("end") {
            if let Some(__x) = TheEnd::__gson_read(__v) { __this.end = __x; }
        }
        if let Some(__v) = __o.get("spidersDen") {
            if let Some(__x) = SpidersDen::__gson_read(__v) { __this.spiders_den = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "barn" => self.barn.__gson_patch(__rest, __value),
                "rift" => self.rift.__gson_patch(__rest, __value),
                "end" => self.end.__gson_patch(__rest, __value),
                "spidersDen" => self.spiders_den.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `OtherLocationsConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<OtherLocationsConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.OtherLocationsConfig",
            new: OtherLocationsConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: OtherLocationsConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Barn's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Barn {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableGlowingMushroomHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_glowing_mushroom_helper))),
            ("solveHungryHiker".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_hungry_hiker))),
            ("solveTreasureHunter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_treasure_hunter))),
            ("enableCallTrevorMessage".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_call_trevor_message))),
            ("enablePeltAnimalHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_pelt_animal_highlighter))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Barn> {
        let __o = __v.as_object()?;
        let mut __this = Barn::new();
        if let Some(__v) = __o.get("enableGlowingMushroomHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_glowing_mushroom_helper = __x; }
        }
        if let Some(__v) = __o.get("solveHungryHiker") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_hungry_hiker = __x; }
        }
        if let Some(__v) = __o.get("solveTreasureHunter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_treasure_hunter = __x; }
        }
        if let Some(__v) = __o.get("enableCallTrevorMessage") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_call_trevor_message = __x; }
        }
        if let Some(__v) = __o.get("enablePeltAnimalHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_pelt_animal_highlighter = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableGlowingMushroomHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_glowing_mushroom_helper = *__b; true } else { false },
                "solveHungryHiker" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_hungry_hiker = *__b; true } else { false },
                "solveTreasureHunter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_treasure_hunter = *__b; true } else { false },
                "enableCallTrevorMessage" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_call_trevor_message = *__b; true } else { false },
                "enablePeltAnimalHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_pelt_animal_highlighter = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Barn.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Barn> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.OtherLocationsConfig$Barn",
            new: Barn::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Barn::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Rift's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Rift {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("mirrorverseWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.mirrorverse_waypoints))),
            ("blobbercystGlow".to_string(), dandelion::compat::GsonValue::Bool(*(&self.blobbercyst_glow))),
            ("enigmaSoulWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enigma_soul_waypoints))),
            ("highlightFoundEnigmaSouls".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_found_enigma_souls))),
            ("autoDetectMcGrubber".to_string(), dandelion::compat::GsonValue::Bool(*(&self.auto_detect_mc_grubber))),
            ("mcGrubberStacks".to_string(), dandelion::compat::GsonValue::int(*(&self.mc_grubber_stacks) as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Rift> {
        let __o = __v.as_object()?;
        let mut __this = Rift::new();
        if let Some(__v) = __o.get("mirrorverseWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.mirrorverse_waypoints = __x; }
        }
        if let Some(__v) = __o.get("blobbercystGlow") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.blobbercyst_glow = __x; }
        }
        if let Some(__v) = __o.get("enigmaSoulWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enigma_soul_waypoints = __x; }
        }
        if let Some(__v) = __o.get("highlightFoundEnigmaSouls") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_found_enigma_souls = __x; }
        }
        if let Some(__v) = __o.get("autoDetectMcGrubber") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.auto_detect_mc_grubber = __x; }
        }
        if let Some(__v) = __o.get("mcGrubberStacks") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.mc_grubber_stacks = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "mirrorverseWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.mirrorverse_waypoints = *__b; true } else { false },
                "blobbercystGlow" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.blobbercyst_glow = *__b; true } else { false },
                "enigmaSoulWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enigma_soul_waypoints = *__b; true } else { false },
                "highlightFoundEnigmaSouls" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_found_enigma_souls = *__b; true } else { false },
                "autoDetectMcGrubber" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.auto_detect_mc_grubber = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Rift.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Rift> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.OtherLocationsConfig$Rift",
            new: Rift::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Rift::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// TheEnd's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TheEnd {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableEnderNodeHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_ender_node_helper))),
            ("enderNodeWaypointType".to_string(), dandelion::compat::GsonValue::str(((&self.ender_node_waypoint_type)).name())),
            ("waypoint".to_string(), dandelion::compat::GsonValue::Bool(*(&self.waypoint))),
            ("muteEndermanSounds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.mute_enderman_sounds))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<TheEnd> {
        let __o = __v.as_object()?;
        let mut __this = TheEnd::new();
        if let Some(__v) = __o.get("enableEnderNodeHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_ender_node_helper = __x; }
        }
        if let Some(__v) = __o.get("enderNodeWaypointType") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Type::values().into_iter().find(|__c| __c.name() == __s)) { __this.ender_node_waypoint_type = __x; }
        }
        if let Some(__v) = __o.get("waypoint") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.waypoint = __x; }
        }
        if let Some(__v) = __o.get("muteEndermanSounds") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.mute_enderman_sounds = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableEnderNodeHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_ender_node_helper = *__b; true } else { false },
                "enderNodeWaypointType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Type::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.ender_node_waypoint_type = __c; true } None => false } } else { false },
                "waypoint" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.waypoint = *__b; true } else { false },
                "muteEndermanSounds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.mute_enderman_sounds = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `TheEnd.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<TheEnd> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.OtherLocationsConfig$TheEnd",
            new: TheEnd::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: TheEnd::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SpidersDen's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SpidersDen {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("relics".to_string(), ((&self.relics)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SpidersDen> {
        let __o = __v.as_object()?;
        let mut __this = SpidersDen::new();
        if let Some(__v) = __o.get("relics") {
            if let Some(__x) = Relics::__gson_read(__v) { __this.relics = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "relics" => self.relics.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `SpidersDen.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SpidersDen> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.OtherLocationsConfig$SpidersDen",
            new: SpidersDen::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SpidersDen::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Relics's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Relics {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableRelicsHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_relics_helper))),
            ("highlightFoundRelics".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_found_relics))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Relics> {
        let __o = __v.as_object()?;
        let mut __this = Relics::new();
        if let Some(__v) = __o.get("enableRelicsHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_relics_helper = __x; }
        }
        if let Some(__v) = __o.get("highlightFoundRelics") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_found_relics = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableRelicsHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_relics_helper = *__b; true } else { false },
                "highlightFoundRelics" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_found_relics = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Relics.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Relics> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.OtherLocationsConfig$Relics",
            new: Relics::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Relics::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
