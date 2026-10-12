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
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.mining.commissionWaypoints.mode." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for CommissionWaypointMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CommissionWaypointMode::to_string(*self))
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
        write!(f, "{}", DwarvenHudStyle::to_string(*self))
    }
}
