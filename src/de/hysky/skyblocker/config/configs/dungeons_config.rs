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
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.dungeons.doorHighlight.doorHighlightType.type." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Type::to_string(*self))
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
