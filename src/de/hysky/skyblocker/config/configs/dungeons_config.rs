//! Ported from de.hysky.skyblocker.config.configs.DungeonsConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map_labels::RoomLabelType;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;

/// java: public class DungeonsConfig
#[derive(Clone)]
pub struct DungeonsConfig {
    /// java: public boolean fancyPartyFinder = false;
    pub fancy_party_finder: bool,
    /// java: public boolean croesusHelper = true;
    pub croesus_helper: bool,
    /// java: public boolean salvageHelper = true;
    pub salvage_helper: bool,
    /// java: public boolean onlyHighlightDonatedItems = false;
    pub only_highlight_donated_items: bool,
    /// java: public boolean sellableItemsHighlighter = true;
    pub sellable_items_highlighter: bool,
    /// java: public boolean bloodCampHelper = false;
    pub blood_camp_helper: bool,
    /// java: public boolean playerSecretsTracker = false;
    pub player_secrets_tracker: bool,
    /// java: public boolean classBasedPlayerGlow = true;
    pub class_based_player_glow: bool,
    /// java: public boolean starredMobGlow = true;
    pub starred_mob_glow: bool,
    /// java: public boolean starredMobBoundingBoxes = false;
    pub starred_mob_bounding_boxes: bool,
    /// java: public boolean highlightDoorKeys = true;
    pub highlight_door_keys: bool,
    /// java: public boolean allowDroppingProtectedItems = false;
    pub allow_dropping_protected_items: bool,
    /// java: @Deprecated public transient boolean dungeonSplits = false;
    pub dungeon_splits: bool,
    /// java: public boolean hideSoulweaverSkulls = false;
    pub hide_soulweaver_skulls: bool,
    /// java: public DungeonMap dungeonMap = new DungeonMap();
    pub dungeon_map: DungeonMap,
    /// java: public SpiritLeapOverlay leapOverlay = new SpiritLeapOverlay();
    pub leap_overlay: compat::Shared<SpiritLeapOverlay>,
    /// java: public PuzzleSolvers puzzleSolvers = new PuzzleSolvers();
    pub puzzle_solvers: PuzzleSolvers,
    /// java: public TheProfessor theProfessor = new TheProfessor();
    pub the_professor: TheProfessor,
    /// java: public Livid livid = new Livid();
    pub livid: compat::Shared<Livid>,
    /// java: public Terminals terminals = new Terminals();
    pub terminals: Terminals,
    /// java: public Devices devices = new Devices();
    pub devices: Devices,
    /// java: public Goldor goldor = new Goldor();
    pub goldor: Goldor,
    /// java: public TerminalHud terminalHud = new TerminalHud();
    pub terminal_hud: compat::Shared<TerminalHud>,
    /// java: public SecretWaypoints secretWaypoints = new SecretWaypoints();
    pub secret_waypoints: compat::Shared<SecretWaypoints>,
    /// java: public SecretSync secretSync = new SecretSync();
    pub secret_sync: compat::Shared<SecretSync>,
    /// java: public RareRoomAlert rareRoomAlert = new RareRoomAlert();
    pub rare_room_alert: compat::Shared<RareRoomAlert>,
    /// java: public MimicMessage mimicMessage = new MimicMessage();
    pub mimic_message: compat::Shared<MimicMessage>,
    /// java: public PrinceMessage princeMessage = new PrinceMessage();
    pub prince_message: compat::Shared<PrinceMessage>,
    /// java: public BatMessage batMessage = new BatMessage();
    pub bat_message: compat::Shared<BatMessage>,
    /// java: public DoorHighlight doorHighlight = new DoorHighlight();
    pub door_highlight: DoorHighlight,
    /// java: public DungeonScore dungeonScore = new DungeonScore();
    pub dungeon_score: compat::Shared<DungeonScore>,
    /// java: public DungeonChestProfit dungeonChestProfit = new DungeonChestProfit();
    pub dungeon_chest_profit: DungeonChestProfit,
}

impl DungeonsConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fancy_party_finder: false,
            croesus_helper: true,
            salvage_helper: true,
            only_highlight_donated_items: false,
            sellable_items_highlighter: true,
            blood_camp_helper: false,
            player_secrets_tracker: false,
            class_based_player_glow: true,
            starred_mob_glow: true,
            starred_mob_bounding_boxes: false,
            highlight_door_keys: true,
            allow_dropping_protected_items: false,
            dungeon_splits: false,
            hide_soulweaver_skulls: false,
            dungeon_map: DungeonMap::new(),
            leap_overlay: SpiritLeapOverlay::new(),
            puzzle_solvers: PuzzleSolvers::new(),
            the_professor: TheProfessor::new(),
            livid: Livid::new(),
            terminals: Terminals::new(),
            devices: Devices::new(),
            goldor: Goldor::new(),
            terminal_hud: TerminalHud::new(),
            secret_waypoints: SecretWaypoints::new(),
            secret_sync: SecretSync::new(),
            rare_room_alert: RareRoomAlert::new(),
            mimic_message: MimicMessage::new(),
            prince_message: PrinceMessage::new(),
            bat_message: BatMessage::new(),
            door_highlight: DoorHighlight::new(),
            dungeon_score: DungeonScore::new(),
            dungeon_chest_profit: DungeonChestProfit::new(),
        };
        this
    }

}

/// java: public static class DungeonMap
#[derive(Clone)]
pub struct DungeonMap {
    /// java: public boolean enableMap = true;
    pub enable_map: bool,
    /// java: public boolean fancyMap = true;
    pub fancy_map: bool,
    /// java: public boolean showSelfHead = true;
    pub show_self_head: bool,
    /// java: public boolean showRoomLabels = true;
    pub show_room_labels: bool,
    /// java: public DungeonMapLabels.RoomLabelType roomLabelType = DungeonMapLabels.RoomLabelType.ROOM_NAME_AND_SECRETS_FOUND;
    pub room_label_type: RoomLabelType,
    /// java: public boolean hideCheckmarks = true;
    pub hide_checkmarks: bool,
    /// java: public boolean showOutline = true;
    pub show_outline: bool,
    /// java: public boolean backgroundBlur = false;
    pub background_blur: bool,
    /// java: public float mapScaling = 1f;
    pub map_scaling: f32,
    /// java: public int mapX = 2;
    pub map_x: i32,
    /// java: public int mapY = 2;
    pub map_y: i32,
}

impl DungeonMap {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_map: true,
            fancy_map: true,
            show_self_head: true,
            show_room_labels: true,
            room_label_type: RoomLabelType::RoomNameAndSecretsFound,
            hide_checkmarks: true,
            show_outline: true,
            background_blur: false,
            map_scaling: 1.0f32,
            map_x: 2,
            map_y: 2,
        };
        this
    }

}

/// java: public static class SpiritLeapOverlay
#[derive(Clone)]
pub struct SpiritLeapOverlay {
    /// java: public boolean enableLeapOverlay = true;
    pub enable_leap_overlay: bool,
    /// java: public boolean leapKeybinds = true;
    pub leap_keybinds: bool,
    /// java: public boolean showMap = true;
    pub show_map: bool,
    /// java: public float scale = 1.2f;
    pub scale: f32,
    /// java: public boolean enableLeapMessage = false;
    pub enable_leap_message: bool,
    /// java: public String leapMessage = "Leaped to [name]!";
    pub leap_message: String,
}

impl SpiritLeapOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            enable_leap_overlay: true,
            leap_keybinds: true,
            show_map: true,
            scale: 1.2f32,
            enable_leap_message: false,
            leap_message: "Leaped to [name]!".to_string(),
        });
        __this
    }

}

/// java: public static class PuzzleSolvers
#[derive(Clone)]
pub struct PuzzleSolvers {
    /// java: public boolean solveTicTacToe = true;
    pub solve_tic_tac_toe: bool,
    /// java: public boolean solveThreeWeirdos = true;
    pub solve_three_weirdos: bool,
    /// java: public boolean creeperSolver = true;
    pub creeper_solver: bool,
    /// java: @Deprecated public transient boolean solveWaterboard = true;
    pub solve_waterboard: bool,
    /// java: public boolean waterboardOneFlow = true;
    pub waterboard_one_flow: bool,
    /// java: public boolean previewWaterPath = true;
    pub preview_water_path: bool,
    /// java: public boolean previewLeverEffects = true;
    pub preview_lever_effects: bool,
    /// java: public boolean blazeSolver = true;
    pub blaze_solver: bool,
    /// java: public boolean solveBoulder = true;
    pub solve_boulder: bool,
    /// java: public boolean solveIceFill = true;
    pub solve_ice_fill: bool,
    /// java: public boolean solveSilverfish = true;
    pub solve_silverfish: bool,
    /// java: public boolean solveTrivia = true;
    pub solve_trivia: bool,
    /// java: public boolean solveTeleportMaze = true;
    pub solve_teleport_maze: bool,
}

impl PuzzleSolvers {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            solve_tic_tac_toe: true,
            solve_three_weirdos: true,
            creeper_solver: true,
            solve_waterboard: true,
            waterboard_one_flow: true,
            preview_water_path: true,
            preview_lever_effects: true,
            blaze_solver: true,
            solve_boulder: true,
            solve_ice_fill: true,
            solve_silverfish: true,
            solve_trivia: true,
            solve_teleport_maze: true,
        };
        this
    }

}

/// java: public static class TheProfessor
#[derive(Clone)]
pub struct TheProfessor {
    /// java: public boolean fireFreezeStaffTimer = true;
    pub fire_freeze_staff_timer: bool,
    /// java: public boolean floor3GuardianHealthDisplay = true;
    pub floor3_guardian_health_display: bool,
}

impl TheProfessor {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            fire_freeze_staff_timer: true,
            floor3_guardian_health_display: true,
        };
        this
    }

}

/// java: public static class Livid
#[derive(Clone)]
pub struct Livid {
    /// java: public boolean enableSolidColor = false;
    pub enable_solid_color: bool,
    /// java: public Color customColor = Color.RED;
    pub custom_color: Color,
    /// java: public boolean enableLividColorGlow = false;
    pub enable_livid_color_glow: bool,
    /// java: public boolean enableLividColorBoundingBox = true;
    pub enable_livid_color_bounding_box: bool,
    /// java: public boolean enableLividColorText = false;
    pub enable_livid_color_text: bool,
    /// java: public boolean enableLividColorTitle = false;
    pub enable_livid_color_title: bool,
    /// java: public String lividColorText = "The livid color is [color]";
    pub livid_color_text: String,
}

impl Livid {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            enable_solid_color: false,
            custom_color: mcrust::java::awt::color::red(),
            enable_livid_color_glow: false,
            enable_livid_color_bounding_box: true,
            enable_livid_color_text: false,
            enable_livid_color_title: false,
            livid_color_text: "The livid color is [color]".to_string(),
        });
        __this
    }

}

/// java: public static class Terminals
#[derive(Clone)]
pub struct Terminals {
    /// java: public boolean solveColor = true;
    pub solve_color: bool,
    /// java: public boolean solveSameColor = true;
    pub solve_same_color: bool,
    /// java: public boolean solveOrder = true;
    pub solve_order: bool,
    /// java: public boolean solveStartsWith = true;
    pub solve_starts_with: bool,
    /// java: public boolean blockIncorrectClicks = false;
    pub block_incorrect_clicks: bool,
}

impl Terminals {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            solve_color: true,
            solve_same_color: true,
            solve_order: true,
            solve_starts_with: true,
            block_incorrect_clicks: false,
        };
        this
    }

}

/// java: public static class Devices
#[derive(Clone)]
pub struct Devices {
    /// java: public boolean solveSimonSays = true;
    pub solve_simon_says: bool,
    /// java: public boolean solveLightsOn = true;
    pub solve_lights_on: bool,
    /// java: public boolean solveArrowAlign = true;
    pub solve_arrow_align: bool,
    /// java: public boolean solveTargetPractice = true;
    pub solve_target_practice: bool,
}

impl Devices {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            solve_simon_says: true,
            solve_lights_on: true,
            solve_arrow_align: true,
            solve_target_practice: true,
        };
        this
    }

}

/// java: public static class Goldor
#[derive(Clone)]
pub struct Goldor {
    /// java: public boolean enableGoldorWaypoints = true;
    pub enable_goldor_waypoints: bool,
    /// java: public Waypoint.Type waypointType = Waypoint.Type.WAYPOINT;
    pub waypoint_type: crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type,
}

impl Goldor {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_goldor_waypoints: true,
            waypoint_type: crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type::Waypoint,
        };
        this
    }

}

/// java: public static class TerminalHud
#[derive(Clone)]
pub struct TerminalHud {
    /// java: public boolean enableTerminalHud = false;
    pub enable_terminal_hud: bool,
    /// java: public boolean showTerminalStatus = true;
    pub show_terminal_status: bool,
    /// java: public boolean showPlayerAtTerminal = true;
    pub show_player_at_terminal: bool,
    /// java: public boolean showTerminals = true;
    pub show_terminals: bool,
    /// java: public boolean showDevice = true;
    pub show_device: bool,
    /// java: public boolean showLevers = true;
    pub show_levers: bool,
    /// java: public boolean showGate = true;
    pub show_gate: bool,
}

impl TerminalHud {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            enable_terminal_hud: false,
            show_terminal_status: true,
            show_player_at_terminal: true,
            show_terminals: true,
            show_device: true,
            show_levers: true,
            show_gate: true,
        });
        __this
    }

}

/// java: public static class SecretWaypoints
#[derive(Clone)]
pub struct SecretWaypoints {
    /// java: @Deprecated public transient boolean enableRoomMatching = true;
    pub enable_room_matching: bool,
    /// java: public boolean enableSecretWaypoints = true;
    pub enable_secret_waypoints: bool,
    /// java: public Waypoint.Type waypointType = Waypoint.Type.WAYPOINT;
    pub waypoint_type: crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type,
    /// java: public boolean adaptiveBoxSize = false;
    pub adaptive_box_size: bool,
    /// java: public boolean showSecretText = true;
    pub show_secret_text: bool,
    /// java: public boolean enableEntranceWaypoints = true;
    pub enable_entrance_waypoints: bool,
    /// java: public boolean enableSuperboomWaypoints = true;
    pub enable_superboom_waypoints: bool,
    /// java: public boolean enableChestWaypoints = true;
    pub enable_chest_waypoints: bool,
    /// java: public boolean enableItemWaypoints = true;
    pub enable_item_waypoints: bool,
    /// java: public boolean enableBatWaypoints = true;
    pub enable_bat_waypoints: bool,
    /// java: public boolean enableWitherWaypoints = true;
    pub enable_wither_waypoints: bool,
    /// java: public boolean enableRedstoneKeyWaypoints = true;
    pub enable_redstone_key_waypoints: bool,
    /// java: public boolean enableLeverWaypoints = true;
    pub enable_lever_waypoints: bool,
    /// java: public boolean enableFairySoulWaypoints = true;
    pub enable_fairy_soul_waypoints: bool,
    /// java: public boolean enableStonkWaypoints = true;
    pub enable_stonk_waypoints: bool,
    /// java: public boolean enableAotvWaypoints = true;
    pub enable_aotv_waypoints: bool,
    /// java: public boolean enablePearlWaypoints = true;
    pub enable_pearl_waypoints: bool,
    /// java: public boolean enablePrinceWaypoints = true;
    pub enable_prince_waypoints: bool,
    /// java: public boolean enableDefaultWaypoints = true;
    pub enable_default_waypoints: bool,
}

impl SecretWaypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            enable_room_matching: true,
            enable_secret_waypoints: true,
            waypoint_type: crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type::Waypoint,
            adaptive_box_size: false,
            show_secret_text: true,
            enable_entrance_waypoints: true,
            enable_superboom_waypoints: true,
            enable_chest_waypoints: true,
            enable_item_waypoints: true,
            enable_bat_waypoints: true,
            enable_wither_waypoints: true,
            enable_redstone_key_waypoints: true,
            enable_lever_waypoints: true,
            enable_fairy_soul_waypoints: true,
            enable_stonk_waypoints: true,
            enable_aotv_waypoints: true,
            enable_pearl_waypoints: true,
            enable_prince_waypoints: true,
            enable_default_waypoints: true,
        });
        __this
    }

}

/// java: public static class SecretSync
#[derive(Clone)]
pub struct SecretSync {
    /// java: public boolean receiveRoomMatch = true;
    pub receive_room_match: bool,
    /// java: public boolean receiveRoomSecretCount = true;
    pub receive_room_secret_count: bool,
    /// java: public boolean hideReceivedWaypoints = true;
    pub hide_received_waypoints: bool,
}

impl SecretSync {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            receive_room_match: true,
            receive_room_secret_count: true,
            hide_received_waypoints: true,
        });
        __this
    }

}

/// java: public static class RareRoomAlert
#[derive(Clone)]
pub struct RareRoomAlert {
    /// java: public boolean enabled = false;
    pub enabled: bool,
    /// java: public boolean showForTrinity = true;
    pub show_for_trinity: bool,
    /// java: public boolean showForTomioka = false;
    pub show_for_tomioka: bool,
    /// java: public boolean showForDuncan = false;
    pub show_for_duncan: bool,
}

impl RareRoomAlert {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            enabled: false,
            show_for_trinity: true,
            show_for_tomioka: false,
            show_for_duncan: false,
        });
        __this
    }

}

/// java: public static class MimicMessage
#[derive(Clone)]
pub struct MimicMessage {
    /// java: public boolean sendMimicMessage = true;
    pub send_mimic_message: bool,
    /// java: @Deprecated public transient String mimicMessage = "Mimic dead!";
    pub mimic_message: String,
}

impl MimicMessage {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            send_mimic_message: true,
            mimic_message: "Mimic dead!".to_string(),
        });
        __this
    }

}

/// java: public static class PrinceMessage
#[derive(Clone)]
pub struct PrinceMessage {
    /// java: public boolean sendPrinceMessage = true;
    pub send_prince_message: bool,
    /// java: @Deprecated public transient String princeMessage = "Prince dead!";
    pub prince_message: String,
}

impl PrinceMessage {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            send_prince_message: true,
            prince_message: "Prince dead!".to_string(),
        });
        __this
    }

}

/// java: public static class BatMessage
#[derive(Clone)]
pub struct BatMessage {
    /// java: public boolean sendBatMessage = true;
    pub send_bat_message: bool,
}

impl BatMessage {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            send_bat_message: true,
        });
        __this
    }

}

/// java: public static class DoorHighlight
#[derive(Clone)]
pub struct DoorHighlight {
    /// java: public boolean enableDoorHighlight = true;
    pub enable_door_highlight: bool,
    /// java: public Type doorHighlightType = Type.OUTLINED_HIGHLIGHT;
    pub door_highlight_type: Type,
}

impl DoorHighlight {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_door_highlight: true,
            door_highlight_type: Type::OutlinedHighlight,
        };
        this
    }

}

/// java: public enum Type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Highlight,
    OutlinedHighlight,
    Outline,
}

impl Type {
    pub const VALUES: [Type; 3] = [Type::Highlight, Type::OutlinedHighlight, Type::Outline];

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
            Type::Highlight => "HIGHLIGHT",
            Type::OutlinedHighlight => "OUTLINED_HIGHLIGHT",
            Type::Outline => "OUTLINE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Type {
        match name {
            "HIGHLIGHT" => Type::Highlight,
            "OUTLINED_HIGHLIGHT" => Type::OutlinedHighlight,
            "OUTLINE" => Type::Outline,
            _ => panic!("IllegalArgumentException: No enum constant Type.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.dungeons.doorHighlight.doorHighlightType.type.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Type::to_string(self))
    }
}

/// java: public static class DungeonScore
#[derive(Clone)]
pub struct DungeonScore {
    /// java: public boolean enableDungeonScore270Message = false;
    pub enable_dungeon_score270_message: bool,
    /// java: public boolean enableDungeonScore270Title = false;
    pub enable_dungeon_score270_title: bool,
    /// java: public boolean enableDungeonScore270Sound = false;
    pub enable_dungeon_score270_sound: bool,
    /// java: public String dungeonScore270Message = "270 Score Reached!";
    pub dungeon_score270_message: String,
    /// java: public boolean enableDungeonScore300Message = true;
    pub enable_dungeon_score300_message: bool,
    /// java: public boolean enableDungeonScore300Title = true;
    pub enable_dungeon_score300_title: bool,
    /// java: public boolean enableDungeonScore300Sound = true;
    pub enable_dungeon_score300_sound: bool,
    /// java: public String dungeonScore300Message = "300 Score Reached!";
    pub dungeon_score300_message: String,
    /// java: public boolean enableDungeonCryptsMessage = true;
    pub enable_dungeon_crypts_message: bool,
    /// java: public int dungeonCryptsMessageThreshold = 250;
    pub dungeon_crypts_message_threshold: i32,
    /// java: public String dungeonCryptsMessage = "Crypts: [crypts]/5";
    pub dungeon_crypts_message: String,
    /// java: public boolean enableScoreHUD = true;
    pub enable_score_hud: bool,
    /// java: public int scoreX = 29;
    pub score_x: i32,
    /// java: public int scoreY = 134;
    pub score_y: i32,
    /// java: public float scoreScaling = 1f;
    pub score_scaling: f32,
}

impl DungeonScore {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            enable_dungeon_score270_message: false,
            enable_dungeon_score270_title: false,
            enable_dungeon_score270_sound: false,
            dungeon_score270_message: "270 Score Reached!".to_string(),
            enable_dungeon_score300_message: true,
            enable_dungeon_score300_title: true,
            enable_dungeon_score300_sound: true,
            dungeon_score300_message: "300 Score Reached!".to_string(),
            enable_dungeon_crypts_message: true,
            dungeon_crypts_message_threshold: 250,
            dungeon_crypts_message: "Crypts: [crypts]/5".to_string(),
            enable_score_hud: true,
            score_x: 29,
            score_y: 134,
            score_scaling: 1.0f32,
        });
        __this
    }

}

/// java: public static class DungeonChestProfit
#[derive(Clone)]
pub struct DungeonChestProfit {
    /// java: public boolean enableProfitCalculator = true;
    pub enable_profit_calculator: bool,
    /// java: public boolean includeKismet = false;
    pub include_kismet: bool,
    /// java: public boolean includeEssence = true;
    pub include_essence: bool,
    /// java: public boolean croesusProfit = true;
    pub croesus_profit: bool,
    /// java: public int neutralThreshold = 1000;
    pub neutral_threshold: i32,
    /// java: public ChatFormatting neutralColor = ChatFormatting.DARK_GRAY;
    pub neutral_color: ChatFormatting,
    /// java: public ChatFormatting profitColor = ChatFormatting.DARK_GREEN;
    pub profit_color: ChatFormatting,
    /// java: public ChatFormatting lossColor = ChatFormatting.RED;
    pub loss_color: ChatFormatting,
    /// java: public ChatFormatting incompleteColor = ChatFormatting.BLUE;
    pub incomplete_color: ChatFormatting,
}

impl DungeonChestProfit {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_profit_calculator: true,
            include_kismet: false,
            include_essence: true,
            croesus_profit: true,
            neutral_threshold: 1000,
            neutral_color: ChatFormatting::DarkGray,
            profit_color: ChatFormatting::DarkGreen,
            loss_color: ChatFormatting::Red,
            incomplete_color: ChatFormatting::Blue,
        };
        this
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/dungeons_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// DungeonsConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl DungeonsConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("fancyPartyFinder".to_string(), dandelion::compat::GsonValue::Bool(*(&self.fancy_party_finder))),
            ("croesusHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.croesus_helper))),
            ("salvageHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.salvage_helper))),
            ("onlyHighlightDonatedItems".to_string(), dandelion::compat::GsonValue::Bool(*(&self.only_highlight_donated_items))),
            ("sellableItemsHighlighter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.sellable_items_highlighter))),
            ("bloodCampHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.blood_camp_helper))),
            ("playerSecretsTracker".to_string(), dandelion::compat::GsonValue::Bool(*(&self.player_secrets_tracker))),
            ("classBasedPlayerGlow".to_string(), dandelion::compat::GsonValue::Bool(*(&self.class_based_player_glow))),
            ("starredMobGlow".to_string(), dandelion::compat::GsonValue::Bool(*(&self.starred_mob_glow))),
            ("starredMobBoundingBoxes".to_string(), dandelion::compat::GsonValue::Bool(*(&self.starred_mob_bounding_boxes))),
            ("highlightDoorKeys".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_door_keys))),
            ("allowDroppingProtectedItems".to_string(), dandelion::compat::GsonValue::Bool(*(&self.allow_dropping_protected_items))),
            ("hideSoulweaverSkulls".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_soulweaver_skulls))),
            ("dungeonMap".to_string(), ((&self.dungeon_map)).__gson_write()),
            ("leapOverlay".to_string(), compat::read((&self.leap_overlay), |__t| __t.__gson_write())),
            ("puzzleSolvers".to_string(), ((&self.puzzle_solvers)).__gson_write()),
            ("theProfessor".to_string(), ((&self.the_professor)).__gson_write()),
            ("livid".to_string(), compat::read((&self.livid), |__t| __t.__gson_write())),
            ("terminals".to_string(), ((&self.terminals)).__gson_write()),
            ("devices".to_string(), ((&self.devices)).__gson_write()),
            ("goldor".to_string(), ((&self.goldor)).__gson_write()),
            ("terminalHud".to_string(), compat::read((&self.terminal_hud), |__t| __t.__gson_write())),
            ("secretWaypoints".to_string(), compat::read((&self.secret_waypoints), |__t| __t.__gson_write())),
            ("secretSync".to_string(), compat::read((&self.secret_sync), |__t| __t.__gson_write())),
            ("rareRoomAlert".to_string(), compat::read((&self.rare_room_alert), |__t| __t.__gson_write())),
            ("mimicMessage".to_string(), compat::read((&self.mimic_message), |__t| __t.__gson_write())),
            ("princeMessage".to_string(), compat::read((&self.prince_message), |__t| __t.__gson_write())),
            ("batMessage".to_string(), compat::read((&self.bat_message), |__t| __t.__gson_write())),
            ("doorHighlight".to_string(), ((&self.door_highlight)).__gson_write()),
            ("dungeonScore".to_string(), compat::read((&self.dungeon_score), |__t| __t.__gson_write())),
            ("dungeonChestProfit".to_string(), ((&self.dungeon_chest_profit)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<DungeonsConfig> {
        let __o = __v.as_object()?;
        let mut __this = DungeonsConfig::new();
        if let Some(__v) = __o.get("fancyPartyFinder") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.fancy_party_finder = __x; }
        }
        if let Some(__v) = __o.get("croesusHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.croesus_helper = __x; }
        }
        if let Some(__v) = __o.get("salvageHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.salvage_helper = __x; }
        }
        if let Some(__v) = __o.get("onlyHighlightDonatedItems") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.only_highlight_donated_items = __x; }
        }
        if let Some(__v) = __o.get("sellableItemsHighlighter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.sellable_items_highlighter = __x; }
        }
        if let Some(__v) = __o.get("bloodCampHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.blood_camp_helper = __x; }
        }
        if let Some(__v) = __o.get("playerSecretsTracker") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.player_secrets_tracker = __x; }
        }
        if let Some(__v) = __o.get("classBasedPlayerGlow") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.class_based_player_glow = __x; }
        }
        if let Some(__v) = __o.get("starredMobGlow") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.starred_mob_glow = __x; }
        }
        if let Some(__v) = __o.get("starredMobBoundingBoxes") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.starred_mob_bounding_boxes = __x; }
        }
        if let Some(__v) = __o.get("highlightDoorKeys") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_door_keys = __x; }
        }
        if let Some(__v) = __o.get("allowDroppingProtectedItems") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.allow_dropping_protected_items = __x; }
        }
        if let Some(__v) = __o.get("hideSoulweaverSkulls") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_soulweaver_skulls = __x; }
        }
        if let Some(__v) = __o.get("dungeonMap") {
            if let Some(__x) = DungeonMap::__gson_read(__v) { __this.dungeon_map = __x; }
        }
        if let Some(__v) = __o.get("leapOverlay") {
            if let Some(__x) = SpiritLeapOverlay::__gson_read(__v) { __this.leap_overlay = __x; }
        }
        if let Some(__v) = __o.get("puzzleSolvers") {
            if let Some(__x) = PuzzleSolvers::__gson_read(__v) { __this.puzzle_solvers = __x; }
        }
        if let Some(__v) = __o.get("theProfessor") {
            if let Some(__x) = TheProfessor::__gson_read(__v) { __this.the_professor = __x; }
        }
        if let Some(__v) = __o.get("livid") {
            if let Some(__x) = Livid::__gson_read(__v) { __this.livid = __x; }
        }
        if let Some(__v) = __o.get("terminals") {
            if let Some(__x) = Terminals::__gson_read(__v) { __this.terminals = __x; }
        }
        if let Some(__v) = __o.get("devices") {
            if let Some(__x) = Devices::__gson_read(__v) { __this.devices = __x; }
        }
        if let Some(__v) = __o.get("goldor") {
            if let Some(__x) = Goldor::__gson_read(__v) { __this.goldor = __x; }
        }
        if let Some(__v) = __o.get("terminalHud") {
            if let Some(__x) = TerminalHud::__gson_read(__v) { __this.terminal_hud = __x; }
        }
        if let Some(__v) = __o.get("secretWaypoints") {
            if let Some(__x) = SecretWaypoints::__gson_read(__v) { __this.secret_waypoints = __x; }
        }
        if let Some(__v) = __o.get("secretSync") {
            if let Some(__x) = SecretSync::__gson_read(__v) { __this.secret_sync = __x; }
        }
        if let Some(__v) = __o.get("rareRoomAlert") {
            if let Some(__x) = RareRoomAlert::__gson_read(__v) { __this.rare_room_alert = __x; }
        }
        if let Some(__v) = __o.get("mimicMessage") {
            if let Some(__x) = MimicMessage::__gson_read(__v) { __this.mimic_message = __x; }
        }
        if let Some(__v) = __o.get("princeMessage") {
            if let Some(__x) = PrinceMessage::__gson_read(__v) { __this.prince_message = __x; }
        }
        if let Some(__v) = __o.get("batMessage") {
            if let Some(__x) = BatMessage::__gson_read(__v) { __this.bat_message = __x; }
        }
        if let Some(__v) = __o.get("doorHighlight") {
            if let Some(__x) = DoorHighlight::__gson_read(__v) { __this.door_highlight = __x; }
        }
        if let Some(__v) = __o.get("dungeonScore") {
            if let Some(__x) = DungeonScore::__gson_read(__v) { __this.dungeon_score = __x; }
        }
        if let Some(__v) = __o.get("dungeonChestProfit") {
            if let Some(__x) = DungeonChestProfit::__gson_read(__v) { __this.dungeon_chest_profit = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "fancyPartyFinder" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.fancy_party_finder = *__b; true } else { false },
                "croesusHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.croesus_helper = *__b; true } else { false },
                "salvageHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.salvage_helper = *__b; true } else { false },
                "onlyHighlightDonatedItems" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.only_highlight_donated_items = *__b; true } else { false },
                "sellableItemsHighlighter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.sellable_items_highlighter = *__b; true } else { false },
                "bloodCampHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.blood_camp_helper = *__b; true } else { false },
                "playerSecretsTracker" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.player_secrets_tracker = *__b; true } else { false },
                "classBasedPlayerGlow" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.class_based_player_glow = *__b; true } else { false },
                "starredMobGlow" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.starred_mob_glow = *__b; true } else { false },
                "starredMobBoundingBoxes" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.starred_mob_bounding_boxes = *__b; true } else { false },
                "highlightDoorKeys" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_door_keys = *__b; true } else { false },
                "allowDroppingProtectedItems" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.allow_dropping_protected_items = *__b; true } else { false },
                "hideSoulweaverSkulls" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_soulweaver_skulls = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "dungeonMap" => self.dungeon_map.__gson_patch(__rest, __value),
                "leapOverlay" => compat::write(&self.leap_overlay, |__t| __t.__gson_patch(__rest, __value)),
                "puzzleSolvers" => self.puzzle_solvers.__gson_patch(__rest, __value),
                "theProfessor" => self.the_professor.__gson_patch(__rest, __value),
                "livid" => compat::write(&self.livid, |__t| __t.__gson_patch(__rest, __value)),
                "terminals" => self.terminals.__gson_patch(__rest, __value),
                "devices" => self.devices.__gson_patch(__rest, __value),
                "goldor" => self.goldor.__gson_patch(__rest, __value),
                "terminalHud" => compat::write(&self.terminal_hud, |__t| __t.__gson_patch(__rest, __value)),
                "secretWaypoints" => compat::write(&self.secret_waypoints, |__t| __t.__gson_patch(__rest, __value)),
                "secretSync" => compat::write(&self.secret_sync, |__t| __t.__gson_patch(__rest, __value)),
                "rareRoomAlert" => compat::write(&self.rare_room_alert, |__t| __t.__gson_patch(__rest, __value)),
                "mimicMessage" => compat::write(&self.mimic_message, |__t| __t.__gson_patch(__rest, __value)),
                "princeMessage" => compat::write(&self.prince_message, |__t| __t.__gson_patch(__rest, __value)),
                "batMessage" => compat::write(&self.bat_message, |__t| __t.__gson_patch(__rest, __value)),
                "doorHighlight" => self.door_highlight.__gson_patch(__rest, __value),
                "dungeonScore" => compat::write(&self.dungeon_score, |__t| __t.__gson_patch(__rest, __value)),
                "dungeonChestProfit" => self.dungeon_chest_profit.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `DungeonsConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<DungeonsConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig",
            new: DungeonsConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: DungeonsConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// DungeonMap's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl DungeonMap {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableMap".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_map))),
            ("fancyMap".to_string(), dandelion::compat::GsonValue::Bool(*(&self.fancy_map))),
            ("showSelfHead".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_self_head))),
            ("showRoomLabels".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_room_labels))),
            ("roomLabelType".to_string(), dandelion::compat::GsonValue::str(((&self.room_label_type)).name())),
            ("hideCheckmarks".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_checkmarks))),
            ("showOutline".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_outline))),
            ("backgroundBlur".to_string(), dandelion::compat::GsonValue::Bool(*(&self.background_blur))),
            ("mapScaling".to_string(), dandelion::compat::GsonValue::float(*(&self.map_scaling))),
            ("mapX".to_string(), dandelion::compat::GsonValue::int(*(&self.map_x) as i64)),
            ("mapY".to_string(), dandelion::compat::GsonValue::int(*(&self.map_y) as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<DungeonMap> {
        let __o = __v.as_object()?;
        let mut __this = DungeonMap::new();
        if let Some(__v) = __o.get("enableMap") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_map = __x; }
        }
        if let Some(__v) = __o.get("fancyMap") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.fancy_map = __x; }
        }
        if let Some(__v) = __o.get("showSelfHead") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_self_head = __x; }
        }
        if let Some(__v) = __o.get("showRoomLabels") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_room_labels = __x; }
        }
        if let Some(__v) = __o.get("roomLabelType") {
            if let Some(__x) = (__v).as_str().and_then(|__s| RoomLabelType::values().into_iter().find(|__c| __c.name() == __s)) { __this.room_label_type = __x; }
        }
        if let Some(__v) = __o.get("hideCheckmarks") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_checkmarks = __x; }
        }
        if let Some(__v) = __o.get("showOutline") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_outline = __x; }
        }
        if let Some(__v) = __o.get("backgroundBlur") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.background_blur = __x; }
        }
        if let Some(__v) = __o.get("mapScaling") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.map_scaling = __x; }
        }
        if let Some(__v) = __o.get("mapX") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.map_x = __x; }
        }
        if let Some(__v) = __o.get("mapY") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.map_y = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableMap" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_map = *__b; true } else { false },
                "fancyMap" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.fancy_map = *__b; true } else { false },
                "showSelfHead" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_self_head = *__b; true } else { false },
                "showRoomLabels" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_room_labels = *__b; true } else { false },
                "roomLabelType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match RoomLabelType::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.room_label_type = __c; true } None => false } } else { false },
                "hideCheckmarks" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_checkmarks = *__b; true } else { false },
                "showOutline" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_outline = *__b; true } else { false },
                "backgroundBlur" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.background_blur = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `DungeonMap.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<DungeonMap> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$DungeonMap",
            new: DungeonMap::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: DungeonMap::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SpiritLeapOverlay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SpiritLeapOverlay {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableLeapOverlay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_leap_overlay))),
            ("leapKeybinds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.leap_keybinds))),
            ("showMap".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_map))),
            ("scale".to_string(), dandelion::compat::GsonValue::float(*(&self.scale))),
            ("enableLeapMessage".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_leap_message))),
            ("leapMessage".to_string(), dandelion::compat::GsonValue::str(&(&self.leap_message))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<SpiritLeapOverlay>> {
        let __o = __v.as_object()?;
        let __h = SpiritLeapOverlay::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("enableLeapOverlay") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_leap_overlay = __x; }
            }
            if let Some(__v) = __o.get("leapKeybinds") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.leap_keybinds = __x; }
            }
            if let Some(__v) = __o.get("showMap") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_map = __x; }
            }
            if let Some(__v) = __o.get("scale") {
                if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.scale = __x; }
            }
            if let Some(__v) = __o.get("enableLeapMessage") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_leap_message = __x; }
            }
            if let Some(__v) = __o.get("leapMessage") {
                if let Some(__x) = dandelion::compat::gson_string(__v) { __this.leap_message = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableLeapOverlay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_leap_overlay = *__b; true } else { false },
                "leapKeybinds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.leap_keybinds = *__b; true } else { false },
                "showMap" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_map = *__b; true } else { false },
                "enableLeapMessage" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_leap_message = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SpiritLeapOverlay.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<SpiritLeapOverlay>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$SpiritLeapOverlay",
            new: SpiritLeapOverlay::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: SpiritLeapOverlay::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// PuzzleSolvers's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl PuzzleSolvers {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("solveTicTacToe".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_tic_tac_toe))),
            ("solveThreeWeirdos".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_three_weirdos))),
            ("creeperSolver".to_string(), dandelion::compat::GsonValue::Bool(*(&self.creeper_solver))),
            ("waterboardOneFlow".to_string(), dandelion::compat::GsonValue::Bool(*(&self.waterboard_one_flow))),
            ("previewWaterPath".to_string(), dandelion::compat::GsonValue::Bool(*(&self.preview_water_path))),
            ("previewLeverEffects".to_string(), dandelion::compat::GsonValue::Bool(*(&self.preview_lever_effects))),
            ("blazeSolver".to_string(), dandelion::compat::GsonValue::Bool(*(&self.blaze_solver))),
            ("solveBoulder".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_boulder))),
            ("solveIceFill".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_ice_fill))),
            ("solveSilverfish".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_silverfish))),
            ("solveTrivia".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_trivia))),
            ("solveTeleportMaze".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_teleport_maze))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<PuzzleSolvers> {
        let __o = __v.as_object()?;
        let mut __this = PuzzleSolvers::new();
        if let Some(__v) = __o.get("solveTicTacToe") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_tic_tac_toe = __x; }
        }
        if let Some(__v) = __o.get("solveThreeWeirdos") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_three_weirdos = __x; }
        }
        if let Some(__v) = __o.get("creeperSolver") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.creeper_solver = __x; }
        }
        if let Some(__v) = __o.get("waterboardOneFlow") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.waterboard_one_flow = __x; }
        }
        if let Some(__v) = __o.get("previewWaterPath") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.preview_water_path = __x; }
        }
        if let Some(__v) = __o.get("previewLeverEffects") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.preview_lever_effects = __x; }
        }
        if let Some(__v) = __o.get("blazeSolver") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.blaze_solver = __x; }
        }
        if let Some(__v) = __o.get("solveBoulder") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_boulder = __x; }
        }
        if let Some(__v) = __o.get("solveIceFill") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_ice_fill = __x; }
        }
        if let Some(__v) = __o.get("solveSilverfish") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_silverfish = __x; }
        }
        if let Some(__v) = __o.get("solveTrivia") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_trivia = __x; }
        }
        if let Some(__v) = __o.get("solveTeleportMaze") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_teleport_maze = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "solveTicTacToe" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_tic_tac_toe = *__b; true } else { false },
                "solveThreeWeirdos" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_three_weirdos = *__b; true } else { false },
                "creeperSolver" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.creeper_solver = *__b; true } else { false },
                "waterboardOneFlow" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.waterboard_one_flow = *__b; true } else { false },
                "previewWaterPath" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.preview_water_path = *__b; true } else { false },
                "previewLeverEffects" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.preview_lever_effects = *__b; true } else { false },
                "blazeSolver" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.blaze_solver = *__b; true } else { false },
                "solveBoulder" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_boulder = *__b; true } else { false },
                "solveIceFill" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_ice_fill = *__b; true } else { false },
                "solveSilverfish" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_silverfish = *__b; true } else { false },
                "solveTrivia" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_trivia = *__b; true } else { false },
                "solveTeleportMaze" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_teleport_maze = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `PuzzleSolvers.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<PuzzleSolvers> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$PuzzleSolvers",
            new: PuzzleSolvers::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: PuzzleSolvers::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// TheProfessor's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TheProfessor {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("fireFreezeStaffTimer".to_string(), dandelion::compat::GsonValue::Bool(*(&self.fire_freeze_staff_timer))),
            ("floor3GuardianHealthDisplay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.floor3_guardian_health_display))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<TheProfessor> {
        let __o = __v.as_object()?;
        let mut __this = TheProfessor::new();
        if let Some(__v) = __o.get("fireFreezeStaffTimer") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.fire_freeze_staff_timer = __x; }
        }
        if let Some(__v) = __o.get("floor3GuardianHealthDisplay") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.floor3_guardian_health_display = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "fireFreezeStaffTimer" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.fire_freeze_staff_timer = *__b; true } else { false },
                "floor3GuardianHealthDisplay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.floor3_guardian_health_display = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `TheProfessor.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<TheProfessor> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$TheProfessor",
            new: TheProfessor::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: TheProfessor::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Livid's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Livid {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableSolidColor".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_solid_color))),
            ("customColor".to_string(), dandelion::compat::GsonValue::int(((&self.custom_color)).get_rgb() as i64)),
            ("enableLividColorGlow".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_livid_color_glow))),
            ("enableLividColorBoundingBox".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_livid_color_bounding_box))),
            ("enableLividColorText".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_livid_color_text))),
            ("enableLividColorTitle".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_livid_color_title))),
            ("lividColorText".to_string(), dandelion::compat::GsonValue::str(&(&self.livid_color_text))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<Livid>> {
        let __o = __v.as_object()?;
        let __h = Livid::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("enableSolidColor") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_solid_color = __x; }
            }
            if let Some(__v) = __o.get("customColor") {
                if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.custom_color = __x; }
            }
            if let Some(__v) = __o.get("enableLividColorGlow") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_livid_color_glow = __x; }
            }
            if let Some(__v) = __o.get("enableLividColorBoundingBox") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_livid_color_bounding_box = __x; }
            }
            if let Some(__v) = __o.get("enableLividColorText") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_livid_color_text = __x; }
            }
            if let Some(__v) = __o.get("enableLividColorTitle") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_livid_color_title = __x; }
            }
            if let Some(__v) = __o.get("lividColorText") {
                if let Some(__x) = dandelion::compat::gson_string(__v) { __this.livid_color_text = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableSolidColor" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_solid_color = *__b; true } else { false },
                "enableLividColorGlow" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_livid_color_glow = *__b; true } else { false },
                "enableLividColorBoundingBox" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_livid_color_bounding_box = *__b; true } else { false },
                "enableLividColorText" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_livid_color_text = *__b; true } else { false },
                "enableLividColorTitle" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_livid_color_title = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Livid.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<Livid>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$Livid",
            new: Livid::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: Livid::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Terminals's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Terminals {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("solveColor".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_color))),
            ("solveSameColor".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_same_color))),
            ("solveOrder".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_order))),
            ("solveStartsWith".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_starts_with))),
            ("blockIncorrectClicks".to_string(), dandelion::compat::GsonValue::Bool(*(&self.block_incorrect_clicks))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Terminals> {
        let __o = __v.as_object()?;
        let mut __this = Terminals::new();
        if let Some(__v) = __o.get("solveColor") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_color = __x; }
        }
        if let Some(__v) = __o.get("solveSameColor") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_same_color = __x; }
        }
        if let Some(__v) = __o.get("solveOrder") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_order = __x; }
        }
        if let Some(__v) = __o.get("solveStartsWith") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_starts_with = __x; }
        }
        if let Some(__v) = __o.get("blockIncorrectClicks") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.block_incorrect_clicks = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "solveColor" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_color = *__b; true } else { false },
                "solveSameColor" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_same_color = *__b; true } else { false },
                "solveOrder" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_order = *__b; true } else { false },
                "solveStartsWith" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_starts_with = *__b; true } else { false },
                "blockIncorrectClicks" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.block_incorrect_clicks = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Terminals.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Terminals> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$Terminals",
            new: Terminals::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Terminals::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Devices's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Devices {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("solveSimonSays".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_simon_says))),
            ("solveLightsOn".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_lights_on))),
            ("solveArrowAlign".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_arrow_align))),
            ("solveTargetPractice".to_string(), dandelion::compat::GsonValue::Bool(*(&self.solve_target_practice))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Devices> {
        let __o = __v.as_object()?;
        let mut __this = Devices::new();
        if let Some(__v) = __o.get("solveSimonSays") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_simon_says = __x; }
        }
        if let Some(__v) = __o.get("solveLightsOn") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_lights_on = __x; }
        }
        if let Some(__v) = __o.get("solveArrowAlign") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_arrow_align = __x; }
        }
        if let Some(__v) = __o.get("solveTargetPractice") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.solve_target_practice = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "solveSimonSays" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_simon_says = *__b; true } else { false },
                "solveLightsOn" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_lights_on = *__b; true } else { false },
                "solveArrowAlign" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_arrow_align = *__b; true } else { false },
                "solveTargetPractice" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.solve_target_practice = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Devices.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Devices> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$Devices",
            new: Devices::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Devices::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Goldor's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Goldor {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableGoldorWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_goldor_waypoints))),
            ("waypointType".to_string(), dandelion::compat::GsonValue::str(((&self.waypoint_type)).name())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Goldor> {
        let __o = __v.as_object()?;
        let mut __this = Goldor::new();
        if let Some(__v) = __o.get("enableGoldorWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_goldor_waypoints = __x; }
        }
        if let Some(__v) = __o.get("waypointType") {
            if let Some(__x) = (__v).as_str().and_then(|__s| crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type::values().into_iter().find(|__c| __c.name() == __s)) { __this.waypoint_type = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableGoldorWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_goldor_waypoints = *__b; true } else { false },
                "waypointType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.waypoint_type = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Goldor.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Goldor> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$Goldor",
            new: Goldor::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Goldor::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// TerminalHud's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TerminalHud {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableTerminalHud".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_terminal_hud))),
            ("showTerminalStatus".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_terminal_status))),
            ("showPlayerAtTerminal".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_player_at_terminal))),
            ("showTerminals".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_terminals))),
            ("showDevice".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_device))),
            ("showLevers".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_levers))),
            ("showGate".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_gate))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<TerminalHud>> {
        let __o = __v.as_object()?;
        let __h = TerminalHud::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("enableTerminalHud") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_terminal_hud = __x; }
            }
            if let Some(__v) = __o.get("showTerminalStatus") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_terminal_status = __x; }
            }
            if let Some(__v) = __o.get("showPlayerAtTerminal") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_player_at_terminal = __x; }
            }
            if let Some(__v) = __o.get("showTerminals") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_terminals = __x; }
            }
            if let Some(__v) = __o.get("showDevice") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_device = __x; }
            }
            if let Some(__v) = __o.get("showLevers") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_levers = __x; }
            }
            if let Some(__v) = __o.get("showGate") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_gate = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableTerminalHud" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_terminal_hud = *__b; true } else { false },
                "showTerminalStatus" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_terminal_status = *__b; true } else { false },
                "showPlayerAtTerminal" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_player_at_terminal = *__b; true } else { false },
                "showTerminals" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_terminals = *__b; true } else { false },
                "showDevice" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_device = *__b; true } else { false },
                "showLevers" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_levers = *__b; true } else { false },
                "showGate" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_gate = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `TerminalHud.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<TerminalHud>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$TerminalHud",
            new: TerminalHud::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: TerminalHud::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SecretWaypoints's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SecretWaypoints {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableSecretWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_secret_waypoints))),
            ("waypointType".to_string(), dandelion::compat::GsonValue::str(((&self.waypoint_type)).name())),
            ("adaptiveBoxSize".to_string(), dandelion::compat::GsonValue::Bool(*(&self.adaptive_box_size))),
            ("showSecretText".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_secret_text))),
            ("enableEntranceWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_entrance_waypoints))),
            ("enableSuperboomWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_superboom_waypoints))),
            ("enableChestWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_chest_waypoints))),
            ("enableItemWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_item_waypoints))),
            ("enableBatWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_bat_waypoints))),
            ("enableWitherWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_wither_waypoints))),
            ("enableRedstoneKeyWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_redstone_key_waypoints))),
            ("enableLeverWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_lever_waypoints))),
            ("enableFairySoulWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_fairy_soul_waypoints))),
            ("enableStonkWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_stonk_waypoints))),
            ("enableAotvWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_aotv_waypoints))),
            ("enablePearlWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_pearl_waypoints))),
            ("enablePrinceWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_prince_waypoints))),
            ("enableDefaultWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_default_waypoints))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<SecretWaypoints>> {
        let __o = __v.as_object()?;
        let __h = SecretWaypoints::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("enableSecretWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_secret_waypoints = __x; }
            }
            if let Some(__v) = __o.get("waypointType") {
                if let Some(__x) = (__v).as_str().and_then(|__s| crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type::values().into_iter().find(|__c| __c.name() == __s)) { __this.waypoint_type = __x; }
            }
            if let Some(__v) = __o.get("adaptiveBoxSize") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.adaptive_box_size = __x; }
            }
            if let Some(__v) = __o.get("showSecretText") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_secret_text = __x; }
            }
            if let Some(__v) = __o.get("enableEntranceWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_entrance_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableSuperboomWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_superboom_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableChestWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_chest_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableItemWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_item_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableBatWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_bat_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableWitherWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_wither_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableRedstoneKeyWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_redstone_key_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableLeverWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_lever_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableFairySoulWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_fairy_soul_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableStonkWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_stonk_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableAotvWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_aotv_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enablePearlWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_pearl_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enablePrinceWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_prince_waypoints = __x; }
            }
            if let Some(__v) = __o.get("enableDefaultWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_default_waypoints = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableSecretWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_secret_waypoints = *__b; true } else { false },
                "waypointType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.waypoint_type = __c; true } None => false } } else { false },
                "adaptiveBoxSize" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.adaptive_box_size = *__b; true } else { false },
                "showSecretText" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_secret_text = *__b; true } else { false },
                "enableEntranceWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_entrance_waypoints = *__b; true } else { false },
                "enableSuperboomWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_superboom_waypoints = *__b; true } else { false },
                "enableChestWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_chest_waypoints = *__b; true } else { false },
                "enableItemWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_item_waypoints = *__b; true } else { false },
                "enableBatWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_bat_waypoints = *__b; true } else { false },
                "enableWitherWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_wither_waypoints = *__b; true } else { false },
                "enableRedstoneKeyWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_redstone_key_waypoints = *__b; true } else { false },
                "enableLeverWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_lever_waypoints = *__b; true } else { false },
                "enableFairySoulWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_fairy_soul_waypoints = *__b; true } else { false },
                "enableStonkWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_stonk_waypoints = *__b; true } else { false },
                "enableAotvWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_aotv_waypoints = *__b; true } else { false },
                "enablePearlWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_pearl_waypoints = *__b; true } else { false },
                "enablePrinceWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_prince_waypoints = *__b; true } else { false },
                "enableDefaultWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_default_waypoints = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SecretWaypoints.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<SecretWaypoints>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$SecretWaypoints",
            new: SecretWaypoints::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: SecretWaypoints::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SecretSync's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SecretSync {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("receiveRoomMatch".to_string(), dandelion::compat::GsonValue::Bool(*(&self.receive_room_match))),
            ("receiveRoomSecretCount".to_string(), dandelion::compat::GsonValue::Bool(*(&self.receive_room_secret_count))),
            ("hideReceivedWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_received_waypoints))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<SecretSync>> {
        let __o = __v.as_object()?;
        let __h = SecretSync::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("receiveRoomMatch") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.receive_room_match = __x; }
            }
            if let Some(__v) = __o.get("receiveRoomSecretCount") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.receive_room_secret_count = __x; }
            }
            if let Some(__v) = __o.get("hideReceivedWaypoints") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_received_waypoints = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "receiveRoomMatch" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.receive_room_match = *__b; true } else { false },
                "receiveRoomSecretCount" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.receive_room_secret_count = *__b; true } else { false },
                "hideReceivedWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_received_waypoints = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SecretSync.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<SecretSync>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$SecretSync",
            new: SecretSync::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: SecretSync::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// RareRoomAlert's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl RareRoomAlert {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("showForTrinity".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_for_trinity))),
            ("showForTomioka".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_for_tomioka))),
            ("showForDuncan".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_for_duncan))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<RareRoomAlert>> {
        let __o = __v.as_object()?;
        let __h = RareRoomAlert::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("enabled") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
            }
            if let Some(__v) = __o.get("showForTrinity") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_for_trinity = __x; }
            }
            if let Some(__v) = __o.get("showForTomioka") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_for_tomioka = __x; }
            }
            if let Some(__v) = __o.get("showForDuncan") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_for_duncan = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "showForTrinity" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_for_trinity = *__b; true } else { false },
                "showForTomioka" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_for_tomioka = *__b; true } else { false },
                "showForDuncan" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_for_duncan = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `RareRoomAlert.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<RareRoomAlert>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$RareRoomAlert",
            new: RareRoomAlert::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: RareRoomAlert::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// MimicMessage's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl MimicMessage {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("sendMimicMessage".to_string(), dandelion::compat::GsonValue::Bool(*(&self.send_mimic_message))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<MimicMessage>> {
        let __o = __v.as_object()?;
        let __h = MimicMessage::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("sendMimicMessage") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.send_mimic_message = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "sendMimicMessage" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.send_mimic_message = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `MimicMessage.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<MimicMessage>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$MimicMessage",
            new: MimicMessage::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: MimicMessage::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// PrinceMessage's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl PrinceMessage {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("sendPrinceMessage".to_string(), dandelion::compat::GsonValue::Bool(*(&self.send_prince_message))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<PrinceMessage>> {
        let __o = __v.as_object()?;
        let __h = PrinceMessage::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("sendPrinceMessage") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.send_prince_message = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "sendPrinceMessage" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.send_prince_message = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `PrinceMessage.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<PrinceMessage>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$PrinceMessage",
            new: PrinceMessage::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: PrinceMessage::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// BatMessage's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl BatMessage {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("sendBatMessage".to_string(), dandelion::compat::GsonValue::Bool(*(&self.send_bat_message))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<BatMessage>> {
        let __o = __v.as_object()?;
        let __h = BatMessage::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("sendBatMessage") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.send_bat_message = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "sendBatMessage" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.send_bat_message = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `BatMessage.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<BatMessage>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$BatMessage",
            new: BatMessage::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: BatMessage::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// DoorHighlight's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl DoorHighlight {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableDoorHighlight".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_door_highlight))),
            ("doorHighlightType".to_string(), dandelion::compat::GsonValue::str(((&self.door_highlight_type)).name())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<DoorHighlight> {
        let __o = __v.as_object()?;
        let mut __this = DoorHighlight::new();
        if let Some(__v) = __o.get("enableDoorHighlight") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_door_highlight = __x; }
        }
        if let Some(__v) = __o.get("doorHighlightType") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Type::values().into_iter().find(|__c| __c.name() == __s)) { __this.door_highlight_type = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableDoorHighlight" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_door_highlight = *__b; true } else { false },
                "doorHighlightType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Type::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.door_highlight_type = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `DoorHighlight.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<DoorHighlight> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$DoorHighlight",
            new: DoorHighlight::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: DoorHighlight::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Type's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Type {
    pub fn __class_of() -> dandelion::compat::ClassOf<Type> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$DoorHighlight$Type",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| Type::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || Type::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// DungeonScore's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl DungeonScore {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableDungeonScore270Message".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_dungeon_score270_message))),
            ("enableDungeonScore270Title".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_dungeon_score270_title))),
            ("enableDungeonScore270Sound".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_dungeon_score270_sound))),
            ("dungeonScore270Message".to_string(), dandelion::compat::GsonValue::str(&(&self.dungeon_score270_message))),
            ("enableDungeonScore300Message".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_dungeon_score300_message))),
            ("enableDungeonScore300Title".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_dungeon_score300_title))),
            ("enableDungeonScore300Sound".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_dungeon_score300_sound))),
            ("dungeonScore300Message".to_string(), dandelion::compat::GsonValue::str(&(&self.dungeon_score300_message))),
            ("enableDungeonCryptsMessage".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_dungeon_crypts_message))),
            ("dungeonCryptsMessageThreshold".to_string(), dandelion::compat::GsonValue::int(*(&self.dungeon_crypts_message_threshold) as i64)),
            ("dungeonCryptsMessage".to_string(), dandelion::compat::GsonValue::str(&(&self.dungeon_crypts_message))),
            ("enableScoreHUD".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_score_hud))),
            ("scoreX".to_string(), dandelion::compat::GsonValue::int(*(&self.score_x) as i64)),
            ("scoreY".to_string(), dandelion::compat::GsonValue::int(*(&self.score_y) as i64)),
            ("scoreScaling".to_string(), dandelion::compat::GsonValue::float(*(&self.score_scaling))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<DungeonScore>> {
        let __o = __v.as_object()?;
        let __h = DungeonScore::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("enableDungeonScore270Message") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_dungeon_score270_message = __x; }
            }
            if let Some(__v) = __o.get("enableDungeonScore270Title") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_dungeon_score270_title = __x; }
            }
            if let Some(__v) = __o.get("enableDungeonScore270Sound") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_dungeon_score270_sound = __x; }
            }
            if let Some(__v) = __o.get("dungeonScore270Message") {
                if let Some(__x) = dandelion::compat::gson_string(__v) { __this.dungeon_score270_message = __x; }
            }
            if let Some(__v) = __o.get("enableDungeonScore300Message") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_dungeon_score300_message = __x; }
            }
            if let Some(__v) = __o.get("enableDungeonScore300Title") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_dungeon_score300_title = __x; }
            }
            if let Some(__v) = __o.get("enableDungeonScore300Sound") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_dungeon_score300_sound = __x; }
            }
            if let Some(__v) = __o.get("dungeonScore300Message") {
                if let Some(__x) = dandelion::compat::gson_string(__v) { __this.dungeon_score300_message = __x; }
            }
            if let Some(__v) = __o.get("enableDungeonCryptsMessage") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_dungeon_crypts_message = __x; }
            }
            if let Some(__v) = __o.get("dungeonCryptsMessageThreshold") {
                if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.dungeon_crypts_message_threshold = __x; }
            }
            if let Some(__v) = __o.get("dungeonCryptsMessage") {
                if let Some(__x) = dandelion::compat::gson_string(__v) { __this.dungeon_crypts_message = __x; }
            }
            if let Some(__v) = __o.get("enableScoreHUD") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_score_hud = __x; }
            }
            if let Some(__v) = __o.get("scoreX") {
                if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.score_x = __x; }
            }
            if let Some(__v) = __o.get("scoreY") {
                if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.score_y = __x; }
            }
            if let Some(__v) = __o.get("scoreScaling") {
                if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.score_scaling = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableDungeonScore270Message" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_dungeon_score270_message = *__b; true } else { false },
                "enableDungeonScore270Title" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_dungeon_score270_title = *__b; true } else { false },
                "enableDungeonScore270Sound" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_dungeon_score270_sound = *__b; true } else { false },
                "enableDungeonScore300Message" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_dungeon_score300_message = *__b; true } else { false },
                "enableDungeonScore300Title" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_dungeon_score300_title = *__b; true } else { false },
                "enableDungeonScore300Sound" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_dungeon_score300_sound = *__b; true } else { false },
                "enableDungeonCryptsMessage" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_dungeon_crypts_message = *__b; true } else { false },
                "enableScoreHUD" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_score_hud = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `DungeonScore.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<DungeonScore>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$DungeonScore",
            new: DungeonScore::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: DungeonScore::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// DungeonChestProfit's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl DungeonChestProfit {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableProfitCalculator".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_profit_calculator))),
            ("includeKismet".to_string(), dandelion::compat::GsonValue::Bool(*(&self.include_kismet))),
            ("includeEssence".to_string(), dandelion::compat::GsonValue::Bool(*(&self.include_essence))),
            ("croesusProfit".to_string(), dandelion::compat::GsonValue::Bool(*(&self.croesus_profit))),
            ("neutralThreshold".to_string(), dandelion::compat::GsonValue::int(*(&self.neutral_threshold) as i64)),
            ("neutralColor".to_string(), dandelion::compat::GsonValue::str((match (&self.neutral_color) { ChatFormatting::Black => "BLACK", ChatFormatting::DarkBlue => "DARK_BLUE", ChatFormatting::DarkGreen => "DARK_GREEN", ChatFormatting::DarkAqua => "DARK_AQUA", ChatFormatting::DarkRed => "DARK_RED", ChatFormatting::DarkPurple => "DARK_PURPLE", ChatFormatting::Gold => "GOLD", ChatFormatting::Gray => "GRAY", ChatFormatting::DarkGray => "DARK_GRAY", ChatFormatting::Blue => "BLUE", ChatFormatting::Green => "GREEN", ChatFormatting::Aqua => "AQUA", ChatFormatting::Red => "RED", ChatFormatting::LightPurple => "LIGHT_PURPLE", ChatFormatting::Yellow => "YELLOW", ChatFormatting::White => "WHITE", ChatFormatting::Obfuscated => "OBFUSCATED", ChatFormatting::Bold => "BOLD", ChatFormatting::Strikethrough => "STRIKETHROUGH", ChatFormatting::Underline => "UNDERLINE", ChatFormatting::Italic => "ITALIC", ChatFormatting::Reset => "RESET", #[allow(unreachable_patterns)] _ => "" }))),
            ("profitColor".to_string(), dandelion::compat::GsonValue::str((match (&self.profit_color) { ChatFormatting::Black => "BLACK", ChatFormatting::DarkBlue => "DARK_BLUE", ChatFormatting::DarkGreen => "DARK_GREEN", ChatFormatting::DarkAqua => "DARK_AQUA", ChatFormatting::DarkRed => "DARK_RED", ChatFormatting::DarkPurple => "DARK_PURPLE", ChatFormatting::Gold => "GOLD", ChatFormatting::Gray => "GRAY", ChatFormatting::DarkGray => "DARK_GRAY", ChatFormatting::Blue => "BLUE", ChatFormatting::Green => "GREEN", ChatFormatting::Aqua => "AQUA", ChatFormatting::Red => "RED", ChatFormatting::LightPurple => "LIGHT_PURPLE", ChatFormatting::Yellow => "YELLOW", ChatFormatting::White => "WHITE", ChatFormatting::Obfuscated => "OBFUSCATED", ChatFormatting::Bold => "BOLD", ChatFormatting::Strikethrough => "STRIKETHROUGH", ChatFormatting::Underline => "UNDERLINE", ChatFormatting::Italic => "ITALIC", ChatFormatting::Reset => "RESET", #[allow(unreachable_patterns)] _ => "" }))),
            ("lossColor".to_string(), dandelion::compat::GsonValue::str((match (&self.loss_color) { ChatFormatting::Black => "BLACK", ChatFormatting::DarkBlue => "DARK_BLUE", ChatFormatting::DarkGreen => "DARK_GREEN", ChatFormatting::DarkAqua => "DARK_AQUA", ChatFormatting::DarkRed => "DARK_RED", ChatFormatting::DarkPurple => "DARK_PURPLE", ChatFormatting::Gold => "GOLD", ChatFormatting::Gray => "GRAY", ChatFormatting::DarkGray => "DARK_GRAY", ChatFormatting::Blue => "BLUE", ChatFormatting::Green => "GREEN", ChatFormatting::Aqua => "AQUA", ChatFormatting::Red => "RED", ChatFormatting::LightPurple => "LIGHT_PURPLE", ChatFormatting::Yellow => "YELLOW", ChatFormatting::White => "WHITE", ChatFormatting::Obfuscated => "OBFUSCATED", ChatFormatting::Bold => "BOLD", ChatFormatting::Strikethrough => "STRIKETHROUGH", ChatFormatting::Underline => "UNDERLINE", ChatFormatting::Italic => "ITALIC", ChatFormatting::Reset => "RESET", #[allow(unreachable_patterns)] _ => "" }))),
            ("incompleteColor".to_string(), dandelion::compat::GsonValue::str((match (&self.incomplete_color) { ChatFormatting::Black => "BLACK", ChatFormatting::DarkBlue => "DARK_BLUE", ChatFormatting::DarkGreen => "DARK_GREEN", ChatFormatting::DarkAqua => "DARK_AQUA", ChatFormatting::DarkRed => "DARK_RED", ChatFormatting::DarkPurple => "DARK_PURPLE", ChatFormatting::Gold => "GOLD", ChatFormatting::Gray => "GRAY", ChatFormatting::DarkGray => "DARK_GRAY", ChatFormatting::Blue => "BLUE", ChatFormatting::Green => "GREEN", ChatFormatting::Aqua => "AQUA", ChatFormatting::Red => "RED", ChatFormatting::LightPurple => "LIGHT_PURPLE", ChatFormatting::Yellow => "YELLOW", ChatFormatting::White => "WHITE", ChatFormatting::Obfuscated => "OBFUSCATED", ChatFormatting::Bold => "BOLD", ChatFormatting::Strikethrough => "STRIKETHROUGH", ChatFormatting::Underline => "UNDERLINE", ChatFormatting::Italic => "ITALIC", ChatFormatting::Reset => "RESET", #[allow(unreachable_patterns)] _ => "" }))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<DungeonChestProfit> {
        let __o = __v.as_object()?;
        let mut __this = DungeonChestProfit::new();
        if let Some(__v) = __o.get("enableProfitCalculator") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_profit_calculator = __x; }
        }
        if let Some(__v) = __o.get("includeKismet") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.include_kismet = __x; }
        }
        if let Some(__v) = __o.get("includeEssence") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.include_essence = __x; }
        }
        if let Some(__v) = __o.get("croesusProfit") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.croesus_profit = __x; }
        }
        if let Some(__v) = __o.get("neutralThreshold") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.neutral_threshold = __x; }
        }
        if let Some(__v) = __o.get("neutralColor") {
            if let Some(__x) = (__v).as_str().and_then(|__s| (match __s { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None })) { __this.neutral_color = __x; }
        }
        if let Some(__v) = __o.get("profitColor") {
            if let Some(__x) = (__v).as_str().and_then(|__s| (match __s { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None })) { __this.profit_color = __x; }
        }
        if let Some(__v) = __o.get("lossColor") {
            if let Some(__x) = (__v).as_str().and_then(|__s| (match __s { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None })) { __this.loss_color = __x; }
        }
        if let Some(__v) = __o.get("incompleteColor") {
            if let Some(__x) = (__v).as_str().and_then(|__s| (match __s { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None })) { __this.incomplete_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableProfitCalculator" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_profit_calculator = *__b; true } else { false },
                "includeKismet" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.include_kismet = *__b; true } else { false },
                "includeEssence" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.include_essence = *__b; true } else { false },
                "croesusProfit" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.croesus_profit = *__b; true } else { false },
                "neutralColor" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match (match __s.as_str() { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None }) { Some(__c) => { self.neutral_color = __c; true } None => false } } else { false },
                "profitColor" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match (match __s.as_str() { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None }) { Some(__c) => { self.profit_color = __c; true } None => false } } else { false },
                "lossColor" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match (match __s.as_str() { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None }) { Some(__c) => { self.loss_color = __c; true } None => false } } else { false },
                "incompleteColor" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match (match __s.as_str() { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None }) { Some(__c) => { self.incomplete_color = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `DungeonChestProfit.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<DungeonChestProfit> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.DungeonsConfig$DungeonChestProfit",
            new: DungeonChestProfit::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: DungeonChestProfit::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
