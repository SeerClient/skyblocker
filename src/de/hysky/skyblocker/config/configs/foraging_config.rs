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
        // TODO(fabric2seer): the initialiser of `sweepOverlayColor`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 2
        //   java: public Color sweepOverlayColor = new Color(0x40FF9600, true);
        let mut this = Self {
            enable_sweep_overlay: true,
            enable_thrown_ability_overlay: true,
            sweep_overlay_color: Default::default(),
        };
        this
    }

}
