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
