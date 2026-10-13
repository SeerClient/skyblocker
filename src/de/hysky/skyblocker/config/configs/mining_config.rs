//! Ported from de.hysky.skyblocker.config.configs.MiningConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::awt::color::Color;

/// java: public class MiningConfig
#[derive(Clone)]
pub struct MiningConfig {
    /// java: public boolean enableDrillFuel = true;
    pub enable_drill_fuel: bool,
    /// java: public boolean commissionHighlight = true;
    pub commission_highlight: bool,
    /// java: public boolean callMismyla = true;
    pub call_mismyla: bool,
    /// java: public boolean redialOnBadSignal = true;
    pub redial_on_bad_signal: bool,
    /// java: public PickobulusHelper pickobulusHelper = new PickobulusHelper();
    pub pickobulus_helper: PickobulusHelper,
    /// java: public DwarvenMines dwarvenMines = new DwarvenMines();
    pub dwarven_mines: DwarvenMines,
    /// java: @Deprecated public transient DwarvenHud dwarvenHud = new DwarvenHud();
    pub dwarven_hud: DwarvenHud,
    /// java: public CrystalHollows crystalHollows = new CrystalHollows();
    pub crystal_hollows: CrystalHollows,
    /// java: public CrystalsHud crystalsHud = new CrystalsHud();
    pub crystals_hud: CrystalsHud,
    /// java: public CrystalsWaypoints crystalsWaypoints = new CrystalsWaypoints();
    pub crystals_waypoints: CrystalsWaypoints,
    /// java: public CommissionWaypoints commissionWaypoints = new CommissionWaypoints();
    pub commission_waypoints: CommissionWaypoints,
    /// java: public Glacite glacite = new Glacite();
    pub glacite: Glacite,
    /// java: public BlockBreakPrediction blockBreakPrediction = new BlockBreakPrediction();
    pub block_break_prediction: BlockBreakPrediction,
}

impl MiningConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_drill_fuel: true,
            commission_highlight: true,
            call_mismyla: true,
            redial_on_bad_signal: true,
            pickobulus_helper: PickobulusHelper::new(),
            dwarven_mines: DwarvenMines::new(),
            dwarven_hud: DwarvenHud::new(),
            crystal_hollows: CrystalHollows::new(),
            crystals_hud: CrystalsHud::new(),
            crystals_waypoints: CrystalsWaypoints::new(),
            commission_waypoints: CommissionWaypoints::new(),
            glacite: Glacite::new(),
            block_break_prediction: BlockBreakPrediction::new(),
        };
        this
    }

}

/// java: public static class PickobulusHelper
#[derive(Clone)]
pub struct PickobulusHelper {
    /// java: public boolean enablePickobulusHelper = true;
    pub enable_pickobulus_helper: bool,
    /// java: @Deprecated public transient boolean enablePickobulusHud = true;
    pub enable_pickobulus_hud: bool,
    /// java: public boolean hideHudOnCooldown = false;
    pub hide_hud_on_cooldown: bool,
}

impl PickobulusHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_pickobulus_helper: true,
            enable_pickobulus_hud: true,
            hide_hud_on_cooldown: false,
        };
        this
    }

}

/// java: public static class DwarvenMines
#[derive(Clone)]
pub struct DwarvenMines {
    /// java: public boolean solveFetchur = true;
    pub solve_fetchur: bool,
    /// java: public boolean solvePuzzler = true;
    pub solve_puzzler: bool,
    /// java: public boolean enableCarpetHighlighter = true;
    pub enable_carpet_highlighter: bool,
    /// java: public Color carpetHighlightColor = new Color(255, 0, 0, 76);
    pub carpet_highlight_color: Color,
}

impl DwarvenMines {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            solve_fetchur: true,
            solve_puzzler: true,
            enable_carpet_highlighter: true,
            carpet_highlight_color: Color::new_with_alpha(255, 0, 0, 76),
        };
        this
    }

}

/// java: @Deprecated public static class DwarvenHud
#[derive(Clone)]
pub struct DwarvenHud {
    /// java: @Deprecated public boolean enabledCommissions = true;
    pub enabled_commissions: bool,
    /// java: @Deprecated public boolean enabledPowder = true;
    pub enabled_powder: bool,
    /// java: @Deprecated public DwarvenHudStyle style = DwarvenHudStyle.SIMPLE;
    pub style: DwarvenHudStyle,
    /// java: @Deprecated public int commissionsX = 10;
    pub commissions_x: i32,
    /// java: @Deprecated public int commissionsY = 10;
    pub commissions_y: i32,
    /// java: @Deprecated public int powderX = 10;
    pub powder_x: i32,
    /// java: @Deprecated public int powderY = 70;
    pub powder_y: i32,
}

impl DwarvenHud {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled_commissions: true,
            enabled_powder: true,
            style: DwarvenHudStyle::Simple,
            commissions_x: 10,
            commissions_y: 10,
            powder_x: 10,
            powder_y: 70,
        };
        this
    }

}

/// java: public static class CrystalHollows
#[derive(Clone)]
pub struct CrystalHollows {
    /// java: public boolean metalDetectorHelper = true;
    pub metal_detector_helper: bool,
    /// java: public boolean nucleusWaypoints = false;
    pub nucleus_waypoints: bool,
    /// java: public boolean chestHighlighter = true;
    pub chest_highlighter: bool,
    /// java: public Color chestHighlightColor = new Color(0, 0, 255, 128);
    pub chest_highlight_color: Color,
    /// java: @Deprecated public transient boolean enablePowderTracker = true;
    pub enable_powder_tracker: bool,
    /// java: public boolean countNaturalChestsInTracker = true;
    pub count_natural_chests_in_tracker: bool,
    /// java: public List<String> powderTrackerFilter = new ArrayList<>();
    pub powder_tracker_filter: Vec<String>,
}

impl CrystalHollows {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            metal_detector_helper: true,
            nucleus_waypoints: false,
            chest_highlighter: true,
            chest_highlight_color: Color::new_with_alpha(0, 0, 255, 128),
            enable_powder_tracker: true,
            count_natural_chests_in_tracker: true,
            powder_tracker_filter: Vec::new(),
        };
        this
    }

}

/// java: public static class CrystalsHud
#[derive(Clone)]
pub struct CrystalsHud {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean showLocations = true;
    pub show_locations: bool,
    /// java: public int locationSize = 8;
    pub location_size: i32,
    /// java: public int x = 10;
    pub x: i32,
    /// java: public int y = 130;
    pub y: i32,
    /// java: public float mapScaling = 1f;
    pub map_scaling: f32,
}

impl CrystalsHud {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            show_locations: true,
            location_size: 8,
            x: 10,
            y: 130,
            map_scaling: 1.0f32,
        };
        this
    }

}

/// java: public static class CrystalsWaypoints
#[derive(Clone)]
pub struct CrystalsWaypoints {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: @Deprecated public transient float textScale = 1;
    pub text_scale: f32,
    /// java: public boolean findInChat = true;
    pub find_in_chat: bool,
    /// java: public boolean wishingCompassSolver = true;
    pub wishing_compass_solver: bool,
    /// java: public boolean shareFairyGrotto = true;
    pub share_fairy_grotto: bool,
}

impl CrystalsWaypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            text_scale: ((1) as f32) as f32,
            find_in_chat: true,
            wishing_compass_solver: true,
            share_fairy_grotto: true,
        };
        this
    }

}

/// java: public static class CommissionWaypoints
#[derive(Clone)]
pub struct CommissionWaypoints {
    /// java: public CommissionWaypointMode mode = CommissionWaypointMode.BOTH;
    pub mode: CommissionWaypointMode,
    /// java: @Deprecated public transient float textScale = 1;
    pub text_scale: f32,
    /// java: public boolean useColor = true;
    pub use_color: bool,
    /// java: public boolean showBaseCamp = false;
    pub show_base_camp: bool,
    /// java: public boolean showEmissary = true;
    pub show_emissary: bool,
    /// java: public boolean hideEmissaryOnPigeon = true;
    pub hide_emissary_on_pigeon: bool,
}

impl CommissionWaypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            mode: CommissionWaypointMode::Both,
            text_scale: ((1) as f32) as f32,
            use_color: true,
            show_base_camp: false,
            show_emissary: true,
            hide_emissary_on_pigeon: true,
        };
        this
    }

}

/// java: public enum CommissionWaypointMode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommissionWaypointMode {
    Off,
    Dwarven,
    Glacite,
    Both,
}

impl CommissionWaypointMode {
    pub const VALUES: [CommissionWaypointMode; 4] = [CommissionWaypointMode::Off, CommissionWaypointMode::Dwarven, CommissionWaypointMode::Glacite, CommissionWaypointMode::Both];

    /// java: values()
    pub fn values() -> Vec<CommissionWaypointMode> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            CommissionWaypointMode::Off => "OFF",
            CommissionWaypointMode::Dwarven => "DWARVEN",
            CommissionWaypointMode::Glacite => "GLACITE",
            CommissionWaypointMode::Both => "BOTH",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> CommissionWaypointMode {
        match name {
            "OFF" => CommissionWaypointMode::Off,
            "DWARVEN" => CommissionWaypointMode::Dwarven,
            "GLACITE" => CommissionWaypointMode::Glacite,
            "BOTH" => CommissionWaypointMode::Both,
            _ => panic!("IllegalArgumentException: No enum constant CommissionWaypointMode.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.mining.commissionWaypoints.mode.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for CommissionWaypointMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CommissionWaypointMode::to_string(self))
    }
}

/// java: public static class Glacite
#[derive(Clone)]
pub struct Glacite {
    /// java: public boolean coldOverlay = true;
    pub cold_overlay: bool,
    /// java: public boolean fossilSolver = true;
    pub fossil_solver: bool,
    /// java: public boolean solveFossilMuncher = true;
    pub solve_fossil_muncher: bool,
    /// java: public boolean enableCorpseFinder = true;
    pub enable_corpse_finder: bool,
    /// java: public boolean enableParsingChatCorpseFinder = true;
    pub enable_parsing_chat_corpse_finder: bool,
    /// java: public boolean autoShareCorpses = false;
    pub auto_share_corpses: bool,
    /// java: public boolean enableCorpseProfitTracker = true;
    pub enable_corpse_profit_tracker: bool,
    /// java: public boolean forceEnglishCorpseProfitTracker = true;
    pub force_english_corpse_profit_tracker: bool,
}

impl Glacite {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            cold_overlay: true,
            fossil_solver: true,
            solve_fossil_muncher: true,
            enable_corpse_finder: true,
            enable_parsing_chat_corpse_finder: true,
            auto_share_corpses: false,
            enable_corpse_profit_tracker: true,
            force_english_corpse_profit_tracker: true,
        };
        this
    }

}

/// java: public static class BlockBreakPrediction
#[derive(Clone)]
pub struct BlockBreakPrediction {
    /// java: public boolean enabled = false;
    pub enabled: bool,
    /// java: public boolean playSound = false;
    pub play_sound: bool,
}

impl BlockBreakPrediction {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: false,
            play_sound: false,
        };
        this
    }

}

/// java: @Deprecated public enum DwarvenHudStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DwarvenHudStyle {
    Simple,
    Fancy,
    Classic,
}

impl DwarvenHudStyle {
    pub const VALUES: [DwarvenHudStyle; 3] = [DwarvenHudStyle::Simple, DwarvenHudStyle::Fancy, DwarvenHudStyle::Classic];

    /// java: values()
    pub fn values() -> Vec<DwarvenHudStyle> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            DwarvenHudStyle::Simple => "SIMPLE",
            DwarvenHudStyle::Fancy => "FANCY",
            DwarvenHudStyle::Classic => "CLASSIC",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> DwarvenHudStyle {
        match name {
            "SIMPLE" => DwarvenHudStyle::Simple,
            "FANCY" => DwarvenHudStyle::Fancy,
            "CLASSIC" => DwarvenHudStyle::Classic,
            _ => panic!("IllegalArgumentException: No enum constant DwarvenHudStyle.{name}"),
        }
    }

    /// java: @Deprecated @Override public String toString()
    pub fn to_string(&self) -> String {
        return match *self {
            DwarvenHudStyle::Simple => "Simple".to_string(),
            DwarvenHudStyle::Fancy => "Fancy".to_string(),
            DwarvenHudStyle::Classic => "Classic".to_string(),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

}

impl std::fmt::Display for DwarvenHudStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DwarvenHudStyle::to_string(self))
    }
}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/mining_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// MiningConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl MiningConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableDrillFuel".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_drill_fuel))),
            ("commissionHighlight".to_string(), dandelion::compat::GsonValue::Bool(*(&self.commission_highlight))),
            ("callMismyla".to_string(), dandelion::compat::GsonValue::Bool(*(&self.call_mismyla))),
            ("redialOnBadSignal".to_string(), dandelion::compat::GsonValue::Bool(*(&self.redial_on_bad_signal))),
            ("pickobulusHelper".to_string(), ((&self.pickobulus_helper)).__gson_write()),
            ("dwarvenMines".to_string(), ((&self.dwarven_mines)).__gson_write()),
            ("crystalHollows".to_string(), ((&self.crystal_hollows)).__gson_write()),
            ("crystalsHud".to_string(), ((&self.crystals_hud)).__gson_write()),
            ("crystalsWaypoints".to_string(), ((&self.crystals_waypoints)).__gson_write()),
            ("commissionWaypoints".to_string(), ((&self.commission_waypoints)).__gson_write()),
            ("glacite".to_string(), ((&self.glacite)).__gson_write()),
            ("blockBreakPrediction".to_string(), ((&self.block_break_prediction)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<MiningConfig> {
        let __o = __v.as_object()?;
        let mut __this = MiningConfig::new();
        if let Some(__v) = __o.get("enableDrillFuel") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_drill_fuel = __x; }
        }
        if let Some(__v) = __o.get("commissionHighlight") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.commission_highlight = __x; }
        }
        if let Some(__v) = __o.get("callMismyla") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.call_mismyla = __x; }
        }
        if let Some(__v) = __o.get("redialOnBadSignal") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.redial_on_bad_signal = __x; }
        }
        if let Some(__v) = __o.get("pickobulusHelper") {
            if let Some(__x) = PickobulusHelper::__gson_read(__v) { __this.pickobulus_helper = __x; }
        }
        if let Some(__v) = __o.get("dwarvenMines") {
            if let Some(__x) = DwarvenMines::__gson_read(__v) { __this.dwarven_mines = __x; }
        }
        if let Some(__v) = __o.get("crystalHollows") {
            if let Some(__x) = CrystalHollows::__gson_read(__v) { __this.crystal_hollows = __x; }
        }
        if let Some(__v) = __o.get("crystalsHud") {
            if let Some(__x) = CrystalsHud::__gson_read(__v) { __this.crystals_hud = __x; }
        }
        if let Some(__v) = __o.get("crystalsWaypoints") {
            if let Some(__x) = CrystalsWaypoints::__gson_read(__v) { __this.crystals_waypoints = __x; }
        }
        if let Some(__v) = __o.get("commissionWaypoints") {
            if let Some(__x) = CommissionWaypoints::__gson_read(__v) { __this.commission_waypoints = __x; }
        }
        if let Some(__v) = __o.get("glacite") {
            if let Some(__x) = Glacite::__gson_read(__v) { __this.glacite = __x; }
        }
        if let Some(__v) = __o.get("blockBreakPrediction") {
            if let Some(__x) = BlockBreakPrediction::__gson_read(__v) { __this.block_break_prediction = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableDrillFuel" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_drill_fuel = *__b; true } else { false },
                "commissionHighlight" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.commission_highlight = *__b; true } else { false },
                "callMismyla" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.call_mismyla = *__b; true } else { false },
                "redialOnBadSignal" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.redial_on_bad_signal = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "pickobulusHelper" => self.pickobulus_helper.__gson_patch(__rest, __value),
                "dwarvenMines" => self.dwarven_mines.__gson_patch(__rest, __value),
                "crystalHollows" => self.crystal_hollows.__gson_patch(__rest, __value),
                "crystalsHud" => self.crystals_hud.__gson_patch(__rest, __value),
                "crystalsWaypoints" => self.crystals_waypoints.__gson_patch(__rest, __value),
                "commissionWaypoints" => self.commission_waypoints.__gson_patch(__rest, __value),
                "glacite" => self.glacite.__gson_patch(__rest, __value),
                "blockBreakPrediction" => self.block_break_prediction.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `MiningConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<MiningConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig",
            new: MiningConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: MiningConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// PickobulusHelper's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl PickobulusHelper {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enablePickobulusHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_pickobulus_helper))),
            ("hideHudOnCooldown".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_hud_on_cooldown))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<PickobulusHelper> {
        let __o = __v.as_object()?;
        let mut __this = PickobulusHelper::new();
        if let Some(__v) = __o.get("enablePickobulusHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_pickobulus_helper = __x; }
        }
        if let Some(__v) = __o.get("hideHudOnCooldown") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_hud_on_cooldown = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enablePickobulusHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_pickobulus_helper = *__b; true } else { false },
                "hideHudOnCooldown" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_hud_on_cooldown = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `PickobulusHelper.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<PickobulusHelper> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$PickobulusHelper",
            new: PickobulusHelper::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: PickobulusHelper::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// DwarvenMines's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl DwarvenMines {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("solveFetchur".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_fetchur))),
            ("solvePuzzler".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_puzzler))),
            ("enableCarpetHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_carpet_highlighter))),
            ("carpetHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.carpet_highlight_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<DwarvenMines> {
        let __o = __v.as_object()?;
        let mut __this = DwarvenMines::new();
        if let Some(__v) = __o.get("solveFetchur") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_fetchur = __x; }
        }
        if let Some(__v) = __o.get("solvePuzzler") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_puzzler = __x; }
        }
        if let Some(__v) = __o.get("enableCarpetHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_carpet_highlighter = __x; }
        }
        if let Some(__v) = __o.get("carpetHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.carpet_highlight_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "solveFetchur" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_fetchur = *__b; true } else { false },
                "solvePuzzler" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_puzzler = *__b; true } else { false },
                "enableCarpetHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_carpet_highlighter = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `DwarvenMines.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<DwarvenMines> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$DwarvenMines",
            new: DwarvenMines::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: DwarvenMines::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// CrystalHollows's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CrystalHollows {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("metalDetectorHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.metal_detector_helper))),
            ("nucleusWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.nucleus_waypoints))),
            ("chestHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.chest_highlighter))),
            ("chestHighlightColor".to_string(), dandelion::compat::GsonValue::int(((&self.chest_highlight_color)).get_rgb() as i64)),
            ("countNaturalChestsInTracker".to_string(), dandelion::compat::GsonValue::Bool(*(&self.count_natural_chests_in_tracker))),
            ("powderTrackerFilter".to_string(), dandelion::compat::GsonValue::Arr(((&self.powder_tracker_filter)).iter().map(|__x0| dandelion::compat::GsonValue::str(&__x0)).collect())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CrystalHollows> {
        let __o = __v.as_object()?;
        let mut __this = CrystalHollows::new();
        if let Some(__v) = __o.get("metalDetectorHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.metal_detector_helper = __x; }
        }
        if let Some(__v) = __o.get("nucleusWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.nucleus_waypoints = __x; }
        }
        if let Some(__v) = __o.get("chestHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.chest_highlighter = __x; }
        }
        if let Some(__v) = __o.get("chestHighlightColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.chest_highlight_color = __x; }
        }
        if let Some(__v) = __o.get("countNaturalChestsInTracker") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.count_natural_chests_in_tracker = __x; }
        }
        if let Some(__v) = __o.get("powderTrackerFilter") {
            if let Some(__x) = (__v).as_array().map(|__a| __a.iter().filter_map(|__y0| dandelion::compat::gson_string(__y0)).collect()) { __this.powder_tracker_filter = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "metalDetectorHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.metal_detector_helper = *__b; true } else { false },
                "nucleusWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.nucleus_waypoints = *__b; true } else { false },
                "chestHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.chest_highlighter = *__b; true } else { false },
                "countNaturalChestsInTracker" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.count_natural_chests_in_tracker = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `CrystalHollows.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CrystalHollows> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$CrystalHollows",
            new: CrystalHollows::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CrystalHollows::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// CrystalsHud's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CrystalsHud {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("showLocations".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_locations))),
            ("locationSize".to_string(), dandelion::compat::GsonValue::int(*(&self.location_size) as i64)),
            ("x".to_string(), dandelion::compat::GsonValue::int(*(&self.x) as i64)),
            ("y".to_string(), dandelion::compat::GsonValue::int(*(&self.y) as i64)),
            ("mapScaling".to_string(), dandelion::compat::GsonValue::float(*(&self.map_scaling))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CrystalsHud> {
        let __o = __v.as_object()?;
        let mut __this = CrystalsHud::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("showLocations") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_locations = __x; }
        }
        if let Some(__v) = __o.get("locationSize") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.location_size = __x; }
        }
        if let Some(__v) = __o.get("x") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.x = __x; }
        }
        if let Some(__v) = __o.get("y") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.y = __x; }
        }
        if let Some(__v) = __o.get("mapScaling") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.map_scaling = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "showLocations" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_locations = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `CrystalsHud.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CrystalsHud> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$CrystalsHud",
            new: CrystalsHud::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CrystalsHud::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// CrystalsWaypoints's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CrystalsWaypoints {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("findInChat".to_string(), dandelion::compat::GsonValue::Bool(*(&self.find_in_chat))),
            ("wishingCompassSolver".to_string(), dandelion::compat::GsonValue::Bool(*(&self.wishing_compass_solver))),
            ("shareFairyGrotto".to_string(), dandelion::compat::GsonValue::Bool(*(&self.share_fairy_grotto))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CrystalsWaypoints> {
        let __o = __v.as_object()?;
        let mut __this = CrystalsWaypoints::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("findInChat") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.find_in_chat = __x; }
        }
        if let Some(__v) = __o.get("wishingCompassSolver") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.wishing_compass_solver = __x; }
        }
        if let Some(__v) = __o.get("shareFairyGrotto") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.share_fairy_grotto = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "findInChat" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.find_in_chat = *__b; true } else { false },
                "wishingCompassSolver" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.wishing_compass_solver = *__b; true } else { false },
                "shareFairyGrotto" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.share_fairy_grotto = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `CrystalsWaypoints.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CrystalsWaypoints> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$CrystalsWaypoints",
            new: CrystalsWaypoints::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CrystalsWaypoints::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// CommissionWaypoints's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CommissionWaypoints {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("mode".to_string(), dandelion::compat::GsonValue::str(((&self.mode)).name())),
            ("useColor".to_string(), dandelion::compat::GsonValue::Bool(*(&self.use_color))),
            ("showBaseCamp".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_base_camp))),
            ("showEmissary".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_emissary))),
            ("hideEmissaryOnPigeon".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_emissary_on_pigeon))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CommissionWaypoints> {
        let __o = __v.as_object()?;
        let mut __this = CommissionWaypoints::new();
        if let Some(__v) = __o.get("mode") {
            if let Some(__x) = (__v).as_str().and_then(|__s| CommissionWaypointMode::values().into_iter().find(|__c| __c.name() == __s)) { __this.mode = __x; }
        }
        if let Some(__v) = __o.get("useColor") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.use_color = __x; }
        }
        if let Some(__v) = __o.get("showBaseCamp") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_base_camp = __x; }
        }
        if let Some(__v) = __o.get("showEmissary") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_emissary = __x; }
        }
        if let Some(__v) = __o.get("hideEmissaryOnPigeon") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_emissary_on_pigeon = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "mode" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match CommissionWaypointMode::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.mode = __c; true } None => false } } else { false },
                "useColor" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.use_color = *__b; true } else { false },
                "showBaseCamp" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_base_camp = *__b; true } else { false },
                "showEmissary" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_emissary = *__b; true } else { false },
                "hideEmissaryOnPigeon" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_emissary_on_pigeon = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `CommissionWaypoints.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CommissionWaypoints> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$CommissionWaypoints",
            new: CommissionWaypoints::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CommissionWaypoints::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// CommissionWaypointMode's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CommissionWaypointMode {
    pub fn __class_of() -> dandelion::compat::ClassOf<CommissionWaypointMode> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$CommissionWaypointMode",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| CommissionWaypointMode::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || CommissionWaypointMode::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// Glacite's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Glacite {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("coldOverlay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.cold_overlay))),
            ("fossilSolver".to_string(), dandelion::compat::GsonValue::Bool(*(&self.fossil_solver))),
            ("solveFossilMuncher".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_fossil_muncher))),
            ("enableCorpseFinder".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_corpse_finder))),
            ("enableParsingChatCorpseFinder".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_parsing_chat_corpse_finder))),
            ("autoShareCorpses".to_string(), dandelion::compat::GsonValue::Bool(*(&self.auto_share_corpses))),
            ("enableCorpseProfitTracker".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_corpse_profit_tracker))),
            ("forceEnglishCorpseProfitTracker".to_string(), dandelion::compat::GsonValue::Bool(*(&self.force_english_corpse_profit_tracker))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Glacite> {
        let __o = __v.as_object()?;
        let mut __this = Glacite::new();
        if let Some(__v) = __o.get("coldOverlay") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.cold_overlay = __x; }
        }
        if let Some(__v) = __o.get("fossilSolver") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.fossil_solver = __x; }
        }
        if let Some(__v) = __o.get("solveFossilMuncher") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_fossil_muncher = __x; }
        }
        if let Some(__v) = __o.get("enableCorpseFinder") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_corpse_finder = __x; }
        }
        if let Some(__v) = __o.get("enableParsingChatCorpseFinder") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_parsing_chat_corpse_finder = __x; }
        }
        if let Some(__v) = __o.get("autoShareCorpses") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.auto_share_corpses = __x; }
        }
        if let Some(__v) = __o.get("enableCorpseProfitTracker") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_corpse_profit_tracker = __x; }
        }
        if let Some(__v) = __o.get("forceEnglishCorpseProfitTracker") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.force_english_corpse_profit_tracker = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "coldOverlay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.cold_overlay = *__b; true } else { false },
                "fossilSolver" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.fossil_solver = *__b; true } else { false },
                "solveFossilMuncher" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_fossil_muncher = *__b; true } else { false },
                "enableCorpseFinder" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_corpse_finder = *__b; true } else { false },
                "enableParsingChatCorpseFinder" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_parsing_chat_corpse_finder = *__b; true } else { false },
                "autoShareCorpses" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.auto_share_corpses = *__b; true } else { false },
                "enableCorpseProfitTracker" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_corpse_profit_tracker = *__b; true } else { false },
                "forceEnglishCorpseProfitTracker" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.force_english_corpse_profit_tracker = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Glacite.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Glacite> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$Glacite",
            new: Glacite::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Glacite::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// BlockBreakPrediction's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl BlockBreakPrediction {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("playSound".to_string(), dandelion::compat::GsonValue::Bool(*(&self.play_sound))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<BlockBreakPrediction> {
        let __o = __v.as_object()?;
        let mut __this = BlockBreakPrediction::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("playSound") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.play_sound = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "playSound" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.play_sound = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `BlockBreakPrediction.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<BlockBreakPrediction> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.MiningConfig$BlockBreakPrediction",
            new: BlockBreakPrediction::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: BlockBreakPrediction::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
