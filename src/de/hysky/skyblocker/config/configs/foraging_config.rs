//! Ported from de.hysky.skyblocker.config.configs.ForagingConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::awt::color::Color;

/// java: public class ForagingConfig
#[derive(Clone)]
pub struct ForagingConfig {
    /// java: public boolean enableTreeFelledNotification = false;
    pub enable_tree_felled_notification: bool,
    /// java: public MoongladeMarsh moongladeMarsh = new MoongladeMarsh();
    pub moonglade_marsh: MoongladeMarsh,
    /// java: public TorrhusCanyon torrhusCanyon = new TorrhusCanyon();
    pub torrhus_canyon: TorrhusCanyon,
    /// java: public SweepOverlay sweepOverlay = new SweepOverlay();
    pub sweep_overlay: SweepOverlay,
}

impl ForagingConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_tree_felled_notification: false,
            moonglade_marsh: MoongladeMarsh::new(),
            torrhus_canyon: TorrhusCanyon::new(),
            sweep_overlay: SweepOverlay::new(),
        };
        this
    }

}

/// java: public static class MoongladeMarsh
#[derive(Clone)]
pub struct MoongladeMarsh {
    /// java: @Deprecated public transient boolean enableForestNodeHelper = true;
    pub enable_forest_node_helper: bool,
    /// java: public boolean solveForestTemplePuzzle = true;
    pub solve_forest_temple_puzzle: bool,
    /// java: public boolean enableLushlilacHighlighter = true;
    pub enable_lushlilac_highlighter: bool,
    /// java: public boolean enableSeaLumiesHighlighter = true;
    pub enable_sea_lumies_highlighter: bool,
    /// java: public int seaLumiesMinimumCount = 3;
    pub sea_lumies_minimum_count: i32,
    /// java: @Deprecated public transient boolean enableTreeBreakProgress = true;
    pub enable_tree_break_progress: bool,
    /// java: public boolean enableTunerSolver = true;
    pub enable_tuner_solver: bool,
    /// java: @Deprecated public transient boolean enableSweepDetailsWidget;
    pub enable_sweep_details_widget: bool,
}

impl MoongladeMarsh {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_forest_node_helper: true,
            solve_forest_temple_puzzle: true,
            enable_lushlilac_highlighter: true,
            enable_sea_lumies_highlighter: true,
            sea_lumies_minimum_count: 3,
            enable_tree_break_progress: true,
            enable_tuner_solver: true,
            enable_sweep_details_widget: false,
        };
        this
    }

}

/// java: public static class TorrhusCanyon
#[derive(Clone)]
pub struct TorrhusCanyon {
    /// java: public boolean solveDesertTemplePuzzles = true;
    pub solve_desert_temple_puzzles: bool,
    /// java: public boolean enableRubyVeilshroomHighlighter = true;
    pub enable_ruby_veilshroom_highlighter: bool,
    /// java: public boolean enableHoneyhiveHighlighter = true;
    pub enable_honeyhive_highlighter: bool,
}

impl TorrhusCanyon {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            solve_desert_temple_puzzles: true,
            enable_ruby_veilshroom_highlighter: true,
            enable_honeyhive_highlighter: true,
        };
        this
    }

}

/// java: public static class SweepOverlay
#[derive(Clone)]
pub struct SweepOverlay {
    /// java: public boolean enableSweepOverlay = true;
    pub enable_sweep_overlay: bool,
    /// java: public boolean enableThrownAbilityOverlay = true;
    pub enable_thrown_ability_overlay: bool,
    /// java: public Color sweepOverlayColor = new Color(0x40FF9600, true);
    pub sweep_overlay_color: Color,
}

impl SweepOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_sweep_overlay: true,
            enable_thrown_ability_overlay: true,
            sweep_overlay_color: Color::from_rgba(0x40FF9600, true),
        };
        this
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/foraging_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// ForagingConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ForagingConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableTreeFelledNotification".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_tree_felled_notification))),
            ("moongladeMarsh".to_string(), ((&self.moonglade_marsh)).__gson_write()),
            ("torrhusCanyon".to_string(), ((&self.torrhus_canyon)).__gson_write()),
            ("sweepOverlay".to_string(), ((&self.sweep_overlay)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ForagingConfig> {
        let __o = __v.as_object()?;
        let mut __this = ForagingConfig::new();
        if let Some(__v) = __o.get("enableTreeFelledNotification") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_tree_felled_notification = __x; }
        }
        if let Some(__v) = __o.get("moongladeMarsh") {
            if let Some(__x) = MoongladeMarsh::__gson_read(__v) { __this.moonglade_marsh = __x; }
        }
        if let Some(__v) = __o.get("torrhusCanyon") {
            if let Some(__x) = TorrhusCanyon::__gson_read(__v) { __this.torrhus_canyon = __x; }
        }
        if let Some(__v) = __o.get("sweepOverlay") {
            if let Some(__x) = SweepOverlay::__gson_read(__v) { __this.sweep_overlay = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableTreeFelledNotification" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_tree_felled_notification = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "moongladeMarsh" => self.moonglade_marsh.__gson_patch(__rest, __value),
                "torrhusCanyon" => self.torrhus_canyon.__gson_patch(__rest, __value),
                "sweepOverlay" => self.sweep_overlay.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `ForagingConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ForagingConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.ForagingConfig",
            new: ForagingConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ForagingConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// MoongladeMarsh's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl MoongladeMarsh {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("solveForestTemplePuzzle".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_forest_temple_puzzle))),
            ("enableLushlilacHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_lushlilac_highlighter))),
            ("enableSeaLumiesHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_sea_lumies_highlighter))),
            ("seaLumiesMinimumCount".to_string(), dandelion::compat::GsonValue::int(*(&self.sea_lumies_minimum_count) as i64)),
            ("enableTunerSolver".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_tuner_solver))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<MoongladeMarsh> {
        let __o = __v.as_object()?;
        let mut __this = MoongladeMarsh::new();
        if let Some(__v) = __o.get("solveForestTemplePuzzle") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_forest_temple_puzzle = __x; }
        }
        if let Some(__v) = __o.get("enableLushlilacHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_lushlilac_highlighter = __x; }
        }
        if let Some(__v) = __o.get("enableSeaLumiesHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_sea_lumies_highlighter = __x; }
        }
        if let Some(__v) = __o.get("seaLumiesMinimumCount") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.sea_lumies_minimum_count = __x; }
        }
        if let Some(__v) = __o.get("enableTunerSolver") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_tuner_solver = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "solveForestTemplePuzzle" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_forest_temple_puzzle = *__b; true } else { false },
                "enableLushlilacHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_lushlilac_highlighter = *__b; true } else { false },
                "enableSeaLumiesHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_sea_lumies_highlighter = *__b; true } else { false },
                "enableTunerSolver" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_tuner_solver = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `MoongladeMarsh.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<MoongladeMarsh> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.ForagingConfig$MoongladeMarsh",
            new: MoongladeMarsh::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: MoongladeMarsh::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// TorrhusCanyon's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TorrhusCanyon {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("solveDesertTemplePuzzles".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_desert_temple_puzzles))),
            ("enableRubyVeilshroomHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_ruby_veilshroom_highlighter))),
            ("enableHoneyhiveHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_honeyhive_highlighter))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<TorrhusCanyon> {
        let __o = __v.as_object()?;
        let mut __this = TorrhusCanyon::new();
        if let Some(__v) = __o.get("solveDesertTemplePuzzles") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_desert_temple_puzzles = __x; }
        }
        if let Some(__v) = __o.get("enableRubyVeilshroomHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_ruby_veilshroom_highlighter = __x; }
        }
        if let Some(__v) = __o.get("enableHoneyhiveHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_honeyhive_highlighter = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "solveDesertTemplePuzzles" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_desert_temple_puzzles = *__b; true } else { false },
                "enableRubyVeilshroomHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_ruby_veilshroom_highlighter = *__b; true } else { false },
                "enableHoneyhiveHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_honeyhive_highlighter = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `TorrhusCanyon.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<TorrhusCanyon> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.ForagingConfig$TorrhusCanyon",
            new: TorrhusCanyon::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: TorrhusCanyon::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SweepOverlay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SweepOverlay {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableSweepOverlay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_sweep_overlay))),
            ("enableThrownAbilityOverlay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_thrown_ability_overlay))),
            ("sweepOverlayColor".to_string(), dandelion::compat::GsonValue::int(((&self.sweep_overlay_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SweepOverlay> {
        let __o = __v.as_object()?;
        let mut __this = SweepOverlay::new();
        if let Some(__v) = __o.get("enableSweepOverlay") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_sweep_overlay = __x; }
        }
        if let Some(__v) = __o.get("enableThrownAbilityOverlay") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_thrown_ability_overlay = __x; }
        }
        if let Some(__v) = __o.get("sweepOverlayColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.sweep_overlay_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableSweepOverlay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_sweep_overlay = *__b; true } else { false },
                "enableThrownAbilityOverlay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_thrown_ability_overlay = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SweepOverlay.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SweepOverlay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.ForagingConfig$SweepOverlay",
            new: SweepOverlay::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SweepOverlay::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
