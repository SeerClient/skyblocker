//! Ported from de.hysky.skyblocker.config.configs.FarmingConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: public static final Codec<Type> CODEC = StringRepresentable.fromEnum(Type::values);
pub static TYPE_CODEC: compat::MainThread<CodecRef<Type>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(Type::values(), |e: &Type| e.get_serialized_name()));

/// java: public class FarmingConfig
#[derive(Clone)]
pub struct FarmingConfig {
    /// java: @Deprecated public transient FarmingHud farmingHud = new FarmingHud();
    pub farming_hud: FarmingHud,
    /// java: public PestHighlighter pestHighlighter = new PestHighlighter();
    pub pest_highlighter: PestHighlighter,
    /// java: public MouseLock mouseLock = new MouseLock();
    pub mouse_lock: MouseLock,
    /// java: public PlotsWidget plotsWidget = new PlotsWidget();
    pub plots_widget: PlotsWidget,
    /// java: public VisitorHelper visitorHelper = new VisitorHelper();
    pub visitor_helper: VisitorHelper,
    /// java: public Greenhouse greenhouse = new Greenhouse();
    pub greenhouse: Greenhouse,
}

impl FarmingConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            farming_hud: FarmingHud::new(),
            pest_highlighter: PestHighlighter::new(),
            mouse_lock: MouseLock::new(),
            plots_widget: PlotsWidget::new(),
            visitor_helper: VisitorHelper::new(),
            greenhouse: Greenhouse::new(),
        };
        this
    }

}

/// java: public static class Greenhouse
#[derive(Clone)]
pub struct Greenhouse {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean showMutationSlot = true;
    pub show_mutation_slot: bool,
}

impl Greenhouse {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            show_mutation_slot: true,
        };
        this
    }

}

/// java: public static class PestHighlighter
#[derive(Clone)]
pub struct PestHighlighter {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean vinylHighlighter = true;
    pub vinyl_highlighter: bool,
    /// java: public boolean contestHighlighter = true;
    pub contest_highlighter: bool,
    /// java: public boolean enableStereoHarmonyHelperForContest = true;
    pub enable_stereo_harmony_helper_for_contest: bool,
}

impl PestHighlighter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            vinyl_highlighter: true,
            contest_highlighter: true,
            enable_stereo_harmony_helper_for_contest: true,
        };
        this
    }

}

/// java: public static class MouseLock
#[derive(Clone)]
pub struct MouseLock {
    /// java: public boolean lockMouseTool = false;
    pub lock_mouse_tool: bool,
    /// java: public boolean lockMouseGroundOnly = false;
    pub lock_mouse_ground_only: bool,
}

impl MouseLock {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            lock_mouse_tool: false,
            lock_mouse_ground_only: false,
        };
        this
    }

}

/// java: public static class PlotsWidget
#[derive(Clone)]
pub struct PlotsWidget {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean closeScreenOnPlotClick = false;
    pub close_screen_on_plot_click: bool,
    /// java: public int x = 0;
    pub x: i32,
    /// java: public int y = 0;
    pub y: i32,
}

impl PlotsWidget {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            close_screen_on_plot_click: false,
            x: 0,
            y: 0,
        };
        this
    }

}

/// java: public static class VisitorHelper
#[derive(Clone)]
pub struct VisitorHelper {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean showInGardenOnly = true;
    pub show_in_garden_only: bool,
    /// java: public boolean showInStacks = false;
    pub show_in_stacks: bool,
}

impl VisitorHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            show_in_garden_only: true,
            show_in_stacks: false,
        };
        this
    }

}

/// java: public static class FarmingHud
#[derive(Clone)]
pub struct FarmingHud {
    /// java: @Deprecated public transient boolean enabled = true;
    pub enabled: bool,
    /// java: @Deprecated public transient boolean counter = true;
    pub counter: bool,
    /// java: @Deprecated public transient boolean coins = true;
    pub coins: bool,
    /// java: @Deprecated public transient Type type = Type.BOTH;
    pub r#type: Type,
    /// java: @Deprecated public transient boolean includeSeedsPrice = true;
    pub include_seeds_price: bool,
    /// java: @Deprecated public transient boolean experience = true;
    pub experience: bool,
}

impl FarmingHud {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            counter: true,
            coins: true,
            r#type: Type::Both,
            include_seeds_price: true,
            experience: true,
        };
        this
    }

}

/// java: public enum Type implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Both,
    Npc,
    Bazaar,
}

impl Type {
    pub const VALUES: [Type; 3] = [Type::Both, Type::Npc, Type::Bazaar];

    /// java: values()
    pub fn values() -> Vec<Type> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Type::Both => "BOTH",
            Type::Npc => "NPC",
            Type::Bazaar => "BAZAAR",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Type {
        match name {
            "BOTH" => Type::Both,
            "NPC" => Type::Npc,
            "BAZAAR" => Type::Bazaar,
            _ => panic!("IllegalArgumentException: No enum constant Type.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.name().to_lowercase();
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.farming.farmingHud.type.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Type::to_string(self))
    }
}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/farming_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// FarmingConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl FarmingConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("pestHighlighter".to_string(), ((&self.pest_highlighter)).__gson_write()),
            ("mouseLock".to_string(), ((&self.mouse_lock)).__gson_write()),
            ("plotsWidget".to_string(), ((&self.plots_widget)).__gson_write()),
            ("visitorHelper".to_string(), ((&self.visitor_helper)).__gson_write()),
            ("greenhouse".to_string(), ((&self.greenhouse)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<FarmingConfig> {
        let __o = __v.as_object()?;
        let mut __this = FarmingConfig::new();
        if let Some(__v) = __o.get("pestHighlighter") {
            if let Some(__x) = PestHighlighter::__gson_read(__v) { __this.pest_highlighter = __x; }
        }
        if let Some(__v) = __o.get("mouseLock") {
            if let Some(__x) = MouseLock::__gson_read(__v) { __this.mouse_lock = __x; }
        }
        if let Some(__v) = __o.get("plotsWidget") {
            if let Some(__x) = PlotsWidget::__gson_read(__v) { __this.plots_widget = __x; }
        }
        if let Some(__v) = __o.get("visitorHelper") {
            if let Some(__x) = VisitorHelper::__gson_read(__v) { __this.visitor_helper = __x; }
        }
        if let Some(__v) = __o.get("greenhouse") {
            if let Some(__x) = Greenhouse::__gson_read(__v) { __this.greenhouse = __x; }
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
                "pestHighlighter" => self.pest_highlighter.__gson_patch(__rest, __value),
                "mouseLock" => self.mouse_lock.__gson_patch(__rest, __value),
                "plotsWidget" => self.plots_widget.__gson_patch(__rest, __value),
                "visitorHelper" => self.visitor_helper.__gson_patch(__rest, __value),
                "greenhouse" => self.greenhouse.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `FarmingConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<FarmingConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.FarmingConfig",
            new: FarmingConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: FarmingConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Greenhouse's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Greenhouse {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("showMutationSlot".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_mutation_slot))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Greenhouse> {
        let __o = __v.as_object()?;
        let mut __this = Greenhouse::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("showMutationSlot") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_mutation_slot = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "showMutationSlot" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_mutation_slot = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Greenhouse.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Greenhouse> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.FarmingConfig$Greenhouse",
            new: Greenhouse::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Greenhouse::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// PestHighlighter's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl PestHighlighter {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("vinylHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.vinyl_highlighter))),
            ("contestHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.contest_highlighter))),
            ("enableStereoHarmonyHelperForContest".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_stereo_harmony_helper_for_contest))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<PestHighlighter> {
        let __o = __v.as_object()?;
        let mut __this = PestHighlighter::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("vinylHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.vinyl_highlighter = __x; }
        }
        if let Some(__v) = __o.get("contestHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.contest_highlighter = __x; }
        }
        if let Some(__v) = __o.get("enableStereoHarmonyHelperForContest") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_stereo_harmony_helper_for_contest = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "vinylHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.vinyl_highlighter = *__b; true } else { false },
                "contestHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.contest_highlighter = *__b; true } else { false },
                "enableStereoHarmonyHelperForContest" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_stereo_harmony_helper_for_contest = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `PestHighlighter.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<PestHighlighter> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.FarmingConfig$PestHighlighter",
            new: PestHighlighter::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: PestHighlighter::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// MouseLock's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl MouseLock {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("lockMouseTool".to_string(), dandelion::compat::GsonValue::Bool(*(&self.lock_mouse_tool))),
            ("lockMouseGroundOnly".to_string(), dandelion::compat::GsonValue::Bool(*(&self.lock_mouse_ground_only))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<MouseLock> {
        let __o = __v.as_object()?;
        let mut __this = MouseLock::new();
        if let Some(__v) = __o.get("lockMouseTool") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.lock_mouse_tool = __x; }
        }
        if let Some(__v) = __o.get("lockMouseGroundOnly") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.lock_mouse_ground_only = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "lockMouseTool" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.lock_mouse_tool = *__b; true } else { false },
                "lockMouseGroundOnly" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.lock_mouse_ground_only = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `MouseLock.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<MouseLock> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.FarmingConfig$MouseLock",
            new: MouseLock::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: MouseLock::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// PlotsWidget's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl PlotsWidget {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("closeScreenOnPlotClick".to_string(), dandelion::compat::GsonValue::Bool(*(&self.close_screen_on_plot_click))),
            ("x".to_string(), dandelion::compat::GsonValue::int(*(&self.x) as i64)),
            ("y".to_string(), dandelion::compat::GsonValue::int(*(&self.y) as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<PlotsWidget> {
        let __o = __v.as_object()?;
        let mut __this = PlotsWidget::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("closeScreenOnPlotClick") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.close_screen_on_plot_click = __x; }
        }
        if let Some(__v) = __o.get("x") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.x = __x; }
        }
        if let Some(__v) = __o.get("y") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.y = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "closeScreenOnPlotClick" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.close_screen_on_plot_click = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `PlotsWidget.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<PlotsWidget> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.FarmingConfig$PlotsWidget",
            new: PlotsWidget::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: PlotsWidget::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// VisitorHelper's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl VisitorHelper {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("showInGardenOnly".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_in_garden_only))),
            ("showInStacks".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_in_stacks))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<VisitorHelper> {
        let __o = __v.as_object()?;
        let mut __this = VisitorHelper::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("showInGardenOnly") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_in_garden_only = __x; }
        }
        if let Some(__v) = __o.get("showInStacks") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_in_stacks = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "showInGardenOnly" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_in_garden_only = *__b; true } else { false },
                "showInStacks" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_in_stacks = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `VisitorHelper.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<VisitorHelper> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.FarmingConfig$VisitorHelper",
            new: VisitorHelper::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: VisitorHelper::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
