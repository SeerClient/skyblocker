//! Ported from de.hysky.skyblocker.config.configs.UIAndVisualsConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::gyro_overlay::Mode;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_mode::SlotTextMode;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::positioner::Positioner;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::multiplayer::player_info::PlayerInfo;
use mcrust::net::minecraft::network::chat::component::Component as _;

// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<NameSorting> CODEC = StringRepresentable.fromEnum(NameSorting::values);
/// java: public static final Codec<NameSorting> CODEC = StringRepresentable.fromEnum(NameSorting::values);
pub static NAME_SORTING_CODEC: compat::MainThread<CodecRef<NameSorting>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public class UIAndVisualsConfig
#[derive(Clone)]
pub struct UIAndVisualsConfig {
    /// java: public boolean swingOnAbilities = false;
    pub swing_on_abilities: bool,
    /// java: public int nightVisionStrength = 100;
    pub night_vision_strength: i32,
    /// java: public boolean compactorDeletorPreview = true;
    pub compactor_deletor_preview: bool,
    /// java: public boolean dontStripSkinAlphaValues = true;
    pub dont_strip_skin_alpha_values: bool,
    /// java: public boolean backpackPreviewWithoutShift = false;
    pub backpack_preview_without_shift: bool,
    /// java: public boolean hideEmptyTooltips = true;
    pub hide_empty_tooltips: bool,
    /// java: public boolean fancyCraftingTable = true;
    pub fancy_crafting_table: bool,
    /// java: public boolean hideStatusEffectOverlay = true;
    pub hide_status_effect_overlay: bool,
    /// java: public boolean showEquipmentInInventory = true;
    pub show_equipment_in_inventory: bool,
    /// java: public boolean cancelComponentUpdateAnimation = true;
    pub cancel_component_update_animation: bool,
    /// java: public boolean showCustomizeButton = true;
    pub show_customize_button: bool,
    /// java: public boolean showConfigButton = false;
    pub show_config_button: bool,
    /// java: public boolean trueQuiverCount = true;
    pub true_quiver_count: bool,
    /// java: public SkyBlockInventoryScreen skyblockInventoryScreen = new SkyBlockInventoryScreen();
    pub skyblock_inventory_screen: SkyBlockInventoryScreen,
    /// java: public ChestValue chestValue = new ChestValue();
    pub chest_value: ChestValue,
    /// java: public ItemCooldown itemCooldown = new ItemCooldown();
    pub item_cooldown: ItemCooldown,
    /// java: public boolean museumOverlay = true;
    pub museum_overlay: bool,
    /// java: public SlotText slotText = new SlotText();
    pub slot_text: SlotText,
    /// java: public StorageOverlay storageOverlay = new StorageOverlay();
    pub storage_overlay: StorageOverlay,
    /// java: public RadialMenu radialMenu = new RadialMenu();
    pub radial_menu: RadialMenu,
    /// java: public InventorySearchConfig inventorySearch = new InventorySearchConfig();
    pub inventory_search: InventorySearchConfig,
    /// java: public TitleContainer titleContainer = new TitleContainer();
    pub title_container: TitleContainer,
    /// java: public TabHudConf tabHud = new TabHudConf();
    pub tab_hud: TabHudConf,
    /// java: public FancyAuctionHouse fancyAuctionHouse = new FancyAuctionHouse();
    pub fancy_auction_house: FancyAuctionHouse,
    /// java: public Bars bars = new Bars();
    pub bars: Bars,
    /// java: public Waypoints waypoints = new Waypoints();
    pub waypoints: Waypoints,
    /// java: public TeleportOverlay teleportOverlay = new TeleportOverlay();
    pub teleport_overlay: TeleportOverlay,
    /// java: public SmoothAOTE smoothAOTE = new SmoothAOTE();
    pub smooth_aote: SmoothAOTE,
    /// java: public SearchOverlay searchOverlay = new SearchOverlay();
    pub search_overlay: SearchOverlay,
    /// java: public BazaarQuickQuantities bazaarQuickQuantities = new BazaarQuickQuantities();
    pub bazaar_quick_quantities: BazaarQuickQuantities,
    /// java: public InputCalculator inputCalculator = new InputCalculator();
    pub input_calculator: InputCalculator,
    /// java: public FlameOverlay flameOverlay = new FlameOverlay();
    pub flame_overlay: FlameOverlay,
    /// java: public CompactDamage compactDamage = new CompactDamage();
    pub compact_damage: CompactDamage,
    /// java: public HealthBars healthBars = new HealthBars();
    pub health_bars: HealthBars,
    /// java: public GyrokineticWandOverlay gyroOverlay = new GyrokineticWandOverlay();
    pub gyro_overlay: GyrokineticWandOverlay,
    /// java: @Deprecated public transient ItemPickup itemPickup = new ItemPickup();
    pub item_pickup: ItemPickup,
}

impl UIAndVisualsConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            swing_on_abilities: false,
            night_vision_strength: 100,
            compactor_deletor_preview: true,
            dont_strip_skin_alpha_values: true,
            backpack_preview_without_shift: false,
            hide_empty_tooltips: true,
            fancy_crafting_table: true,
            hide_status_effect_overlay: true,
            show_equipment_in_inventory: true,
            cancel_component_update_animation: true,
            show_customize_button: true,
            show_config_button: false,
            true_quiver_count: true,
            skyblock_inventory_screen: SkyBlockInventoryScreen::new(),
            chest_value: ChestValue::new(),
            item_cooldown: ItemCooldown::new(),
            museum_overlay: true,
            slot_text: SlotText::new(),
            storage_overlay: StorageOverlay::new(),
            radial_menu: RadialMenu::new(),
            inventory_search: InventorySearchConfig::new(),
            title_container: TitleContainer::new(),
            tab_hud: TabHudConf::new(),
            fancy_auction_house: FancyAuctionHouse::new(),
            bars: Bars::new(),
            waypoints: Waypoints::new(),
            teleport_overlay: TeleportOverlay::new(),
            smooth_aote: SmoothAOTE::new(),
            search_overlay: SearchOverlay::new(),
            bazaar_quick_quantities: BazaarQuickQuantities::new(),
            input_calculator: InputCalculator::new(),
            flame_overlay: FlameOverlay::new(),
            compact_damage: CompactDamage::new(),
            health_bars: HealthBars::new(),
            gyro_overlay: GyrokineticWandOverlay::new(),
            item_pickup: ItemPickup::new(),
        };
        this
    }

}

/// java: public static class SkyBlockInventoryScreen
#[derive(Clone)]
pub struct SkyBlockInventoryScreen {
    /// java: public boolean openEquipmentToStatsPage = false;
    pub open_equipment_to_stats_page: bool,
}

impl SkyBlockInventoryScreen {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            open_equipment_to_stats_page: false,
        };
        this
    }

}

/// java: public static class ChestValue
#[derive(Clone)]
pub struct ChestValue {
    /// java: public boolean enableChestValue = true;
    pub enable_chest_value: bool,
    /// java: public ChatFormatting color = ChatFormatting.DARK_GREEN;
    pub color: ChatFormatting,
    /// java: public ChatFormatting incompleteColor = ChatFormatting.BLUE;
    pub incomplete_color: ChatFormatting,
}

impl ChestValue {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_chest_value: true,
            color: ChatFormatting::DarkGreen,
            incomplete_color: ChatFormatting::Blue,
        };
        this
    }

}

/// java: public static class ItemCooldown
#[derive(Clone)]
pub struct ItemCooldown {
    /// java: public boolean enableItemCooldowns = true;
    pub enable_item_cooldowns: bool,
}

impl ItemCooldown {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_item_cooldowns: true,
        };
        this
    }

}

/// java: public static class SlotText
#[derive(Clone)]
pub struct SlotText {
    /// java: public SlotTextMode slotTextMode = SlotTextMode.ENABLED;
    pub slot_text_mode: SlotTextMode,
    /// java: public Object2BooleanOpenHashMap<String> textEnabled = new Object2BooleanOpenHashMap<>();
    pub text_enabled: std::collections::HashMap<String, bool>,
    /// java: public boolean slotTextToggled = true;
    pub slot_text_toggled: bool,
}

impl SlotText {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            slot_text_mode: SlotTextMode::Enabled,
            text_enabled: std::collections::HashMap::new(),
            slot_text_toggled: true,
        };
        this
    }

}

/// java: public static class StorageOverlay
#[derive(Clone)]
pub struct StorageOverlay {
    /// java: public boolean enabled = false;
    pub enabled: bool,
    /// java: public int storagesPerRow = 3;
    pub storages_per_row: i32,
    /// java: public int backpackWidth = 9;
    pub backpack_width: i32,
    /// java: public boolean rememberSearch = false;
    pub remember_search: bool,
    /// java: public boolean rememberOpened = false;
    pub remember_opened: bool,
    /// java: public transient boolean doNotResetCursor = true;
    pub do_not_reset_cursor: bool,
}

impl StorageOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: false,
            storages_per_row: 3,
            backpack_width: 9,
            remember_search: false,
            remember_opened: false,
            do_not_reset_cursor: true,
        };
        this
    }

}

/// java: public static class RadialMenu
#[derive(Clone)]
pub struct RadialMenu {
    /// java: public boolean enabled = false;
    pub enabled: bool,
    /// java: public boolean tooltipsWithoutShift = false;
    pub tooltips_without_shift: bool,
    /// java: public Object2BooleanOpenHashMap<String> enabledMenus = new Object2BooleanOpenHashMap<>();
    pub enabled_menus: std::collections::HashMap<String, bool>,
}

impl RadialMenu {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: false,
            tooltips_without_shift: false,
            enabled_menus: std::collections::HashMap::new(),
        };
        this
    }

}

/// java: public static class InventorySearchConfig
#[derive(Clone)]
pub struct InventorySearchConfig {
    /// java: public EnableState enabled = EnableState.SKYBLOCK;
    pub enabled: EnableState,
    /// java: public boolean ctrlK = false;
    pub ctrl_k: bool,
    /// java: public boolean clickableText = false;
    pub clickable_text: bool,
}

impl InventorySearchConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: EnableState::Skyblock,
            ctrl_k: false,
            clickable_text: false,
        };
        this
    }

}

/// java: public enum EnableState
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EnableState {
    Off,
    Skyblock,
    Everywhere,
}

impl EnableState {
    pub const VALUES: [EnableState; 3] = [EnableState::Off, EnableState::Skyblock, EnableState::Everywhere];

    /// java: values()
    pub fn values() -> Vec<EnableState> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            EnableState::Off => "OFF",
            EnableState::Skyblock => "SKYBLOCK",
            EnableState::Everywhere => "EVERYWHERE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> EnableState {
        match name {
            "OFF" => EnableState::Off,
            "SKYBLOCK" => EnableState::Skyblock,
            "EVERYWHERE" => EnableState::Everywhere,
            _ => panic!("IllegalArgumentException: No enum constant EnableState.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.uiAndVisuals.inventorySearch.state." + this.name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public boolean isEnabled()
    pub fn is_enabled(&self) -> bool {
        return match *self {
            EnableState::Off => false,
            EnableState::Skyblock => Utils::is_on_skyblock(),
            EnableState::Everywhere => true,
            _ => unreachable!("switch expression: no case matched"),
        };
    }

}

impl std::fmt::Display for EnableState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", EnableState::to_string(*self))
    }
}

/// java: public static class TitleContainer
#[derive(Clone)]
pub struct TitleContainer {
    /// java: public float titleContainerScale = 100;
    pub title_container_scale: f32,
    /// java: public int x = -1;
    pub x: i32,
    /// java: public int y = -1;
    pub y: i32,
    /// java: public Direction direction = Direction.VERTICAL;
    pub direction: Direction,
    /// java: public Alignment alignment = Alignment.MIDDLE;
    pub alignment: Alignment,
}

impl TitleContainer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            title_container_scale: ((100) as f32) as f32,
            x: -1,
            y: -1,
            direction: Direction::Vertical,
            alignment: Alignment::Middle,
        };
        this
    }

}

/// java: public enum Direction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    pub const VALUES: [Direction; 2] = [Direction::Horizontal, Direction::Vertical];

    /// java: values()
    pub fn values() -> Vec<Direction> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Direction::Horizontal => "HORIZONTAL",
            Direction::Vertical => "VERTICAL",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Direction {
        match name {
            "HORIZONTAL" => Direction::Horizontal,
            "VERTICAL" => Direction::Vertical,
            _ => panic!("IllegalArgumentException: No enum constant Direction.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.uiAndVisuals.titleContainer.direction." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Direction::to_string(*self))
    }
}

/// java: public enum Alignment
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Alignment {
    Left,
    Middle,
    Right,
}

impl Alignment {
    pub const VALUES: [Alignment; 3] = [Alignment::Left, Alignment::Middle, Alignment::Right];

    /// java: values()
    pub fn values() -> Vec<Alignment> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Alignment::Left => "LEFT",
            Alignment::Middle => "MIDDLE",
            Alignment::Right => "RIGHT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Alignment {
        match name {
            "LEFT" => Alignment::Left,
            "MIDDLE" => Alignment::Middle,
            "RIGHT" => Alignment::Right,
            _ => panic!("IllegalArgumentException: No enum constant Alignment.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.uiAndVisuals.titleContainer.alignment." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Alignment::to_string(*self))
    }
}

/// java: public static class TabHudConf
#[derive(Clone)]
pub struct TabHudConf {
    /// java: public boolean tabHudEnabled = true;
    pub tab_hud_enabled: bool,
    /// java: public int tabHudScale = 100;
    pub tab_hud_scale: i32,
    /// java: public boolean showVanillaTabByDefault = true;
    pub show_vanilla_tab_by_default: bool,
    /// java: public TabHudStyle style = TabHudStyle.FANCY;
    pub style: TabHudStyle,
    /// java: public boolean displayIcons = true;
    pub display_icons: bool,
    /// java: public boolean compactWidgets = false;
    pub compact_widgets: bool,
    /// java: public boolean enableHudBackground = true;
    pub enable_hud_background: bool,
    /// java: @Deprecated public transient boolean effectsFromFooter = false;
    pub effects_from_footer: bool,
    /// java: public Positioner defaultPositioning = Positioner.CENTERED;
    pub default_positioning: Positioner,
    /// java: @Deprecated public transient boolean plainPlayerNames = false;
    pub plain_player_names: bool,
    /// java: @Deprecated public transient NameSorting nameSorting = NameSorting.DEFAULT;
    pub name_sorting: NameSorting,
}

impl TabHudConf {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            tab_hud_enabled: true,
            tab_hud_scale: 100,
            show_vanilla_tab_by_default: true,
            style: TabHudStyle::Fancy,
            display_icons: true,
            compact_widgets: false,
            enable_hud_background: true,
            effects_from_footer: false,
            default_positioning: Positioner::Centered,
            plain_player_names: false,
            name_sorting: NameSorting::Default,
        };
        this
    }

}

/// java: public enum TabHudStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TabHudStyle {
    Minimal,
    Simple,
    Classic,
    Fancy,
}

impl TabHudStyle {
    pub const VALUES: [TabHudStyle; 4] = [TabHudStyle::Minimal, TabHudStyle::Simple, TabHudStyle::Classic, TabHudStyle::Fancy];

    /// java: values()
    pub fn values() -> Vec<TabHudStyle> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TabHudStyle::Minimal => "MINIMAL",
            TabHudStyle::Simple => "SIMPLE",
            TabHudStyle::Classic => "CLASSIC",
            TabHudStyle::Fancy => "FANCY",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TabHudStyle {
        match name {
            "MINIMAL" => TabHudStyle::Minimal,
            "SIMPLE" => TabHudStyle::Simple,
            "CLASSIC" => TabHudStyle::Classic,
            "FANCY" => TabHudStyle::Fancy,
            _ => panic!("IllegalArgumentException: No enum constant TabHudStyle.{name}"),
        }
    }

    /// java: public boolean isMinimal()
    pub fn is_minimal(&self) -> bool {
        return (*self == MINIMAL);
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.uiAndVisuals.tabHud.style." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for TabHudStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", TabHudStyle::to_string(*self))
    }
}

/// java: public enum NameSorting implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NameSorting {
    Default,
    Alphabetical,
    SkyblockLevel,
}

impl NameSorting {
    pub const VALUES: [NameSorting; 3] = [NameSorting::Default, NameSorting::Alphabetical, NameSorting::SkyblockLevel];

    /// java: values()
    pub fn values() -> Vec<NameSorting> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            NameSorting::Default => "DEFAULT",
            NameSorting::Alphabetical => "ALPHABETICAL",
            NameSorting::SkyblockLevel => "SKYBLOCK_LEVEL",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> NameSorting {
        match name {
            "DEFAULT" => NameSorting::Default,
            "ALPHABETICAL" => NameSorting::Alphabetical,
            "SKYBLOCK_LEVEL" => NameSorting::SkyblockLevel,
            _ => panic!("IllegalArgumentException: No enum constant NameSorting.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static NameSortingFields {
        static FIELDS: LazyLock<Vec<NameSortingFields>> = LazyLock::new(|| vec![
            NameSortingFields::new(Arc::new(move |_, _| (0).cmp(&0))),
            NameSortingFields::new(Arc::new(move |a, b| (|a, b| a.to_lowercase().cmp(&b.to_lowercase()))(&(|ple| NameSorting::match_player_name(&ple.get_tab_list_display_name().unwrap().get_string(), "name").unwrap_or("".to_string()))(a), &(|ple| NameSorting::match_player_name(&ple.get_tab_list_display_name().unwrap().get_string(), "name").unwrap_or("".to_string()))(b)))),
            todo!("SKYBLOCK_LEVEL"),
        ]);
        &FIELDS[self as usize]
    }

    /// java: private static Optional<String> matchPlayerName(String name, String group)
    pub fn match_player_name(name: &str, group: &str) -> Option<String> {
        let mut matcher: compat::regex::Matcher = player_list_manager::PLAYER_NAME_PATTERN.matcher(name);
        return (if matcher.matches() { matcher.group_named(group) } else { None });
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return match *self {
            NameSorting::Default => "Default".to_string(),
            NameSorting::Alphabetical => "Alphabetical".to_string(),
            NameSorting::SkyblockLevel => "Skyblock Level".to_string(),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name().toLowerCase(Locale.ENGLISH);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for NameSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NameSorting::to_string(*self))
    }
}

/// the constructor data of each NameSorting constant
pub struct NameSortingFields {
    pub comparator: Arc<dyn Fn(&PlayerInfo, &PlayerInfo) -> std::cmp::Ordering>,
}

impl NameSortingFields {
    fn new(comparator: Arc<dyn Fn(&PlayerInfo, &PlayerInfo) -> std::cmp::Ordering>) -> Self {
        let mut this = Self {
            comparator: Default::default(),
        };
        this.comparator = comparator.clone();
        this
    }

}

/// java: public static class FancyAuctionHouse
#[derive(Clone)]
pub struct FancyAuctionHouse {
    /// java: public boolean enabled = false;
    pub enabled: bool,
    /// java: public boolean highlightCheapBIN = true;
    pub highlight_cheap_bin: bool,
}

impl FancyAuctionHouse {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: false,
            highlight_cheap_bin: true,
        };
        this
    }

}

/// java: public static class Bars
#[derive(Clone)]
pub struct Bars {
    /// java: public boolean enableBars = true;
    pub enable_bars: bool,
    /// java: public boolean enableBarsRift = true;
    pub enable_bars_rift: bool,
    /// java: public boolean riftHealthHP = false;
    pub rift_health_hp: bool,
    /// java: public boolean enableVanillaStyleManaBar = false;
    pub enable_vanilla_style_mana_bar: bool,
    /// java: public boolean enableVanillaStyleManaBarBlinking = true;
    pub enable_vanilla_style_mana_bar_blinking: bool,
    /// java: public boolean useHungerBarSprites = false;
    pub use_hunger_bar_sprites: bool,
    /// java: public IntelligenceDisplay intelligenceDisplay = IntelligenceDisplay.ORIGINAL;
    pub intelligence_display: IntelligenceDisplay,
    /// java: public boolean showEstimatedTilde = true;
    pub show_estimated_tilde: bool,
    /// java: public boolean hasSeenVitalityAtLeastOnce = false;
    pub has_seen_vitality_at_least_once: bool,
    /// java: @SuppressWarnings("DeprecatedIsStillUsed") @Deprecated public LegacyBarPositions barPositions = new LegacyBarPositions();
    pub bar_positions: LegacyBarPositions,
}

impl Bars {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_bars: true,
            enable_bars_rift: true,
            rift_health_hp: false,
            enable_vanilla_style_mana_bar: false,
            enable_vanilla_style_mana_bar_blinking: true,
            use_hunger_bar_sprites: false,
            intelligence_display: IntelligenceDisplay::Original,
            show_estimated_tilde: true,
            has_seen_vitality_at_least_once: false,
            bar_positions: LegacyBarPositions::new(),
        };
        this
    }

}

/// java: public enum IntelligenceDisplay
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntelligenceDisplay {
    Original,
    Accurate,
    InFront,
}

impl IntelligenceDisplay {
    pub const VALUES: [IntelligenceDisplay; 3] = [IntelligenceDisplay::Original, IntelligenceDisplay::Accurate, IntelligenceDisplay::InFront];

    /// java: values()
    pub fn values() -> Vec<IntelligenceDisplay> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            IntelligenceDisplay::Original => "ORIGINAL",
            IntelligenceDisplay::Accurate => "ACCURATE",
            IntelligenceDisplay::InFront => "IN_FRONT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> IntelligenceDisplay {
        match name {
            "ORIGINAL" => IntelligenceDisplay::Original,
            "ACCURATE" => IntelligenceDisplay::Accurate,
            "IN_FRONT" => IntelligenceDisplay::InFront,
            _ => panic!("IllegalArgumentException: No enum constant IntelligenceDisplay.{name}"),
        }
    }

}

impl std::fmt::Display for IntelligenceDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: @SuppressWarnings("DeprecatedIsStillUsed") @Deprecated public static class LegacyBarPositions
#[derive(Clone)]
pub struct LegacyBarPositions {
    /// java: @Deprecated public LegacyBarPosition healthBarPosition = LegacyBarPosition.LAYER1;
    pub health_bar_position: LegacyBarPosition,
    /// java: @Deprecated public LegacyBarPosition manaBarPosition = LegacyBarPosition.LAYER1;
    pub mana_bar_position: LegacyBarPosition,
    /// java: @Deprecated public LegacyBarPosition defenceBarPosition = LegacyBarPosition.RIGHT;
    pub defence_bar_position: LegacyBarPosition,
    /// java: @Deprecated public LegacyBarPosition experienceBarPosition = LegacyBarPosition.LAYER2;
    pub experience_bar_position: LegacyBarPosition,
}

impl LegacyBarPositions {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            health_bar_position: LegacyBarPosition::Layer1,
            mana_bar_position: LegacyBarPosition::Layer1,
            defence_bar_position: LegacyBarPosition::Right,
            experience_bar_position: LegacyBarPosition::Layer2,
        };
        this
    }

}

/// java: public enum LegacyBarPosition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LegacyBarPosition {
    Layer1,
    Layer2,
    Right,
    None,
}

impl LegacyBarPosition {
    pub const VALUES: [LegacyBarPosition; 4] = [LegacyBarPosition::Layer1, LegacyBarPosition::Layer2, LegacyBarPosition::Right, LegacyBarPosition::None];

    /// java: values()
    pub fn values() -> Vec<LegacyBarPosition> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            LegacyBarPosition::Layer1 => "LAYER1",
            LegacyBarPosition::Layer2 => "LAYER2",
            LegacyBarPosition::Right => "RIGHT",
            LegacyBarPosition::None => "NONE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> LegacyBarPosition {
        match name {
            "LAYER1" => LegacyBarPosition::Layer1,
            "LAYER2" => LegacyBarPosition::Layer2,
            "RIGHT" => LegacyBarPosition::Right,
            "NONE" => LegacyBarPosition::None,
            _ => panic!("IllegalArgumentException: No enum constant LegacyBarPosition.{name}"),
        }
    }

}

impl std::fmt::Display for LegacyBarPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: public static class Waypoints
#[derive(Clone)]
pub struct Waypoints {
    /// java: public boolean enableWaypoints = true;
    pub enable_waypoints: bool,
    /// java: public Waypoint.Type waypointType = Waypoint.Type.WAYPOINT;
    pub waypoint_type: Type,
    /// java: public boolean renderLine = true;
    pub render_line: bool,
    /// java: public Color lineColor = new Color(0, 255, 0, 255);
    pub line_color: Color,
    /// java: public float lineWidth = 5f;
    pub line_width: f32,
    /// java: public boolean allowSkippingWaypoints = true;
    pub allow_skipping_waypoints: bool,
    /// java: public boolean allowGoingBackwards = true;
    pub allow_going_backwards: bool,
    /// java: public float waypointActivationRadius = 2f;
    pub waypoint_activation_radius: f32,
    /// java: public boolean enableChatWaypoints = true;
    pub enable_chat_waypoints: bool,
}

impl Waypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_waypoints: true,
            waypoint_type: Type::Waypoint,
            render_line: true,
            line_color: Color::new_with_alpha(0, 255, 0, 255),
            line_width: 5.0f32,
            allow_skipping_waypoints: true,
            allow_going_backwards: true,
            waypoint_activation_radius: 2.0f32,
            enable_chat_waypoints: true,
        };
        this
    }

}

/// java: public static class TeleportOverlay
#[derive(Clone)]
pub struct TeleportOverlay {
    /// java: public boolean enableTeleportOverlays = true;
    pub enable_teleport_overlays: bool,
    /// java: public boolean showWhenInAir = false;
    pub show_when_in_air: bool,
    /// java: public Color teleportOverlayColor = new Color(0x7F761594, true);
    pub teleport_overlay_color: Color,
    /// java: public HighlightType highlightType = HighlightType.HIGHLIGHT;
    pub highlight_type: HighlightType,
    /// java: public boolean enableWeirdTransmission = false;
    pub enable_weird_transmission: bool,
    /// java: public boolean enableInstantTransmission = false;
    pub enable_instant_transmission: bool,
    /// java: public boolean enableEtherTransmission = true;
    pub enable_ether_transmission: bool,
    /// java: public boolean enableSinrecallTransmission = false;
    pub enable_sinrecall_transmission: bool,
    /// java: public boolean enableWitherImpact = false;
    pub enable_wither_impact: bool,
}

impl TeleportOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        // TODO(fabric2seer): the initialiser of `teleportOverlayColor`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 2
        //   java: public Color teleportOverlayColor = new Color(0x7F761594, true);
        let mut this = Self {
            enable_teleport_overlays: true,
            show_when_in_air: false,
            teleport_overlay_color: Default::default(),
            highlight_type: HighlightType::Highlight,
            enable_weird_transmission: false,
            enable_instant_transmission: false,
            enable_ether_transmission: true,
            enable_sinrecall_transmission: false,
            enable_wither_impact: false,
        };
        this
    }

}

/// java: public enum HighlightType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HighlightType {
    Highlight,
    OutlinedHighlight,
    Outline,
}

impl HighlightType {
    pub const VALUES: [HighlightType; 3] = [HighlightType::Highlight, HighlightType::OutlinedHighlight, HighlightType::Outline];

    /// java: values()
    pub fn values() -> Vec<HighlightType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            HighlightType::Highlight => "HIGHLIGHT",
            HighlightType::OutlinedHighlight => "OUTLINED_HIGHLIGHT",
            HighlightType::Outline => "OUTLINE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> HighlightType {
        match name {
            "HIGHLIGHT" => HighlightType::Highlight,
            "OUTLINED_HIGHLIGHT" => HighlightType::OutlinedHighlight,
            "OUTLINE" => HighlightType::Outline,
            _ => panic!("IllegalArgumentException: No enum constant HighlightType.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.uiAndVisuals.teleportOverlay.highlightType." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for HighlightType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HighlightType::to_string(*self))
    }
}

/// java: public static class SmoothAOTE
#[derive(Clone)]
pub struct SmoothAOTE {
    /// java: public boolean predictive = false;
    pub predictive: bool,
    /// java: public boolean thirdPerson = true;
    pub third_person: bool,
    /// java: public boolean enableWeirdTransmission = false;
    pub enable_weird_transmission: bool,
    /// java: public boolean enableInstantTransmission = false;
    pub enable_instant_transmission: bool,
    /// java: public boolean enableEtherTransmission = false;
    pub enable_ether_transmission: bool,
    /// java: public boolean enableSinrecallTransmission = false;
    pub enable_sinrecall_transmission: bool,
    /// java: public boolean enableWitherImpact = false;
    pub enable_wither_impact: bool,
    /// java: public int maximumAddedLag = 100;
    pub maximum_added_lag: i32,
}

impl SmoothAOTE {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            predictive: false,
            third_person: true,
            enable_weird_transmission: false,
            enable_instant_transmission: false,
            enable_ether_transmission: false,
            enable_sinrecall_transmission: false,
            enable_wither_impact: false,
            maximum_added_lag: 100,
        };
        this
    }

}

/// java: public static class SearchOverlay
#[derive(Clone)]
pub struct SearchOverlay {
    /// java: public boolean enableBazaar = true;
    pub enable_bazaar: bool,
    /// java: public boolean enableAuctionHouse = true;
    pub enable_auction_house: bool,
    /// java: public boolean enableIronmanAuctionHouse = false;
    pub enable_ironman_auction_house: bool,
    /// java: public boolean enableMuseum = true;
    pub enable_museum: bool,
    /// java: public boolean keepPreviousSearches = false;
    pub keep_previous_searches: bool,
    /// java: public int maxSuggestions = 3;
    pub max_suggestions: i32,
    /// java: public int historyLength = 3;
    pub history_length: i32,
    /// java: public boolean enableCommands = false;
    pub enable_commands: bool,
    /// java: public boolean commandAutocomplete = true;
    pub command_autocomplete: bool,
    /// java: public List<String> bazaarHistory = new ArrayList<>();
    pub bazaar_history: Vec<String>,
    /// java: public List<String> auctionHistory = new ArrayList<>();
    pub auction_history: Vec<String>,
    /// java: public List<String> museumHistory = new ArrayList<>();
    pub museum_history: Vec<String>,
}

impl SearchOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_bazaar: true,
            enable_auction_house: true,
            enable_ironman_auction_house: false,
            enable_museum: true,
            keep_previous_searches: false,
            max_suggestions: 3,
            history_length: 3,
            enable_commands: false,
            command_autocomplete: true,
            bazaar_history: Vec::new(),
            auction_history: Vec::new(),
            museum_history: Vec::new(),
        };
        this
    }

}

/// java: public static class BazaarQuickQuantities
#[derive(Clone)]
pub struct BazaarQuickQuantities {
    /// java: public boolean enabled = false;
    pub enabled: bool,
    /// java: public boolean closeSignOnUse = false;
    pub close_sign_on_use: bool,
    /// java: public int slot1Quantity = 28;
    pub slot1_quantity: i32,
    /// java: public int slot2Quantity = 2240;
    pub slot2_quantity: i32,
    /// java: public int slot3Quantity = 256;
    pub slot3_quantity: i32,
}

impl BazaarQuickQuantities {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: false,
            close_sign_on_use: false,
            slot1_quantity: 28,
            slot2_quantity: 2240,
            slot3_quantity: 256,
        };
        this
    }

}

/// java: public static class InputCalculator
#[derive(Clone)]
pub struct InputCalculator {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean requiresEquals = false;
    pub requires_equals: bool,
    /// java: public boolean closeSignsWithEnter = true;
    pub close_signs_with_enter: bool,
}

impl InputCalculator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            requires_equals: false,
            close_signs_with_enter: true,
        };
        this
    }

}

/// java: public static class FlameOverlay
#[derive(Clone)]
pub struct FlameOverlay {
    /// java: public int flameHeight = 100;
    pub flame_height: i32,
    /// java: public int flameOpacity = 100;
    pub flame_opacity: i32,
}

impl FlameOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            flame_height: 100,
            flame_opacity: 100,
        };
        this
    }

}

/// java: public static class CompactDamage
#[derive(Clone)]
pub struct CompactDamage {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: @Deprecated public transient int precision = 1;
    pub precision: i32,
    /// java: public int maxPrecision = 4;
    pub max_precision: i32,
    /// java: public Color normalDamageColor = new Color(0xFFFFFF);
    pub normal_damage_color: Color,
    /// java: public Color critDamageGradientStart = new Color(0xFFFF55);
    pub crit_damage_gradient_start: Color,
    /// java: public Color critDamageGradientEnd = new Color(0xFF5555);
    pub crit_damage_gradient_end: Color,
}

impl CompactDamage {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        // TODO(fabric2seer): the initialiser of `normalDamageColor`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
        //   java: public Color normalDamageColor = new Color(0xFFFFFF);
        // TODO(fabric2seer): the initialiser of `critDamageGradientStart`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
        //   java: public Color critDamageGradientStart = new Color(0xFFFF55);
        // TODO(fabric2seer): the initialiser of `critDamageGradientEnd`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
        //   java: public Color critDamageGradientEnd = new Color(0xFF5555);
        let mut this = Self {
            enabled: true,
            precision: 1,
            max_precision: 4,
            normal_damage_color: Default::default(),
            crit_damage_gradient_start: Default::default(),
            crit_damage_gradient_end: Default::default(),
        };
        this
    }

}

/// java: public static class HealthBars
#[derive(Clone)]
pub struct HealthBars {
    /// java: public boolean enabled = false;
    pub enabled: bool,
    /// java: public float scale = 1.5f;
    pub scale: f32,
    /// java: public boolean removeHealthFromName = true;
    pub remove_health_from_name: bool,
    /// java: public boolean removeMaxHealthFromName = true;
    pub remove_max_health_from_name: bool,
    /// java: public boolean applyToHealthOnlyMobs = true;
    pub apply_to_health_only_mobs: bool,
    /// java: public boolean hideFullHealth = false;
    pub hide_full_health: bool,
    /// java: public Color fullBarColor = new Color(0x00FF00);
    pub full_bar_color: Color,
    /// java: public Color halfBarColor = new Color(0xFFFF00);
    pub half_bar_color: Color,
    /// java: public Color emptyBarColor = new Color(0xFF0000);
    pub empty_bar_color: Color,
}

impl HealthBars {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        // TODO(fabric2seer): the initialiser of `fullBarColor`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
        //   java: public Color fullBarColor = new Color(0x00FF00);
        // TODO(fabric2seer): the initialiser of `halfBarColor`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
        //   java: public Color halfBarColor = new Color(0xFFFF00);
        // TODO(fabric2seer): the initialiser of `emptyBarColor`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
        //   java: public Color emptyBarColor = new Color(0xFF0000);
        let mut this = Self {
            enabled: false,
            scale: 1.5f32,
            remove_health_from_name: true,
            remove_max_health_from_name: true,
            apply_to_health_only_mobs: true,
            hide_full_health: false,
            full_bar_color: Default::default(),
            half_bar_color: Default::default(),
            empty_bar_color: Default::default(),
        };
        this
    }

}

/// java: public static class GyrokineticWandOverlay
#[derive(Clone)]
pub struct GyrokineticWandOverlay {
    /// java: public GyroOverlay.Mode gyroOverlayMode = GyroOverlay.Mode.OFF;
    pub gyro_overlay_mode: Mode,
    /// java: public Color gyroOverlayColor = new Color(0x7F761594, true);
    pub gyro_overlay_color: Color,
}

impl GyrokineticWandOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        // TODO(fabric2seer): the initialiser of `gyroOverlayColor`: `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 2
        //   java: public Color gyroOverlayColor = new Color(0x7F761594, true);
        let mut this = Self {
            gyro_overlay_mode: Mode::Off,
            gyro_overlay_color: Default::default(),
        };
        this
    }

}

/// java: public static class ItemPickup
#[derive(Clone)]
pub struct ItemPickup {
    /// java: @Deprecated public transient boolean enabled = false;
    pub enabled: bool,
    /// java: @Deprecated public transient boolean sackNotifications = false;
    pub sack_notifications: bool,
    /// java: @Deprecated public transient boolean showItemName = true;
    pub show_item_name: bool,
    /// java: @Deprecated public transient int lifeTime = 3;
    pub life_time: i32,
    /// java: @Deprecated public transient boolean splitNotifications = false;
    pub split_notifications: bool,
}

impl ItemPickup {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: false,
            sack_notifications: false,
            show_item_name: true,
            life_time: 3,
            split_notifications: false,
        };
        this
    }

}
