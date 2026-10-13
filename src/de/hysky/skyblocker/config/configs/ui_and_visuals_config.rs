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

/// java: public static final Codec<NameSorting> CODEC = StringRepresentable.fromEnum(NameSorting::values);
pub static NAME_SORTING_CODEC: compat::MainThread<CodecRef<NameSorting>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(NameSorting::values(), |e: &NameSorting| e.get_serialized_name()));

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
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.uiAndVisuals.inventorySearch.state.{}", self.name()), &[]);
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
        write!(f, "{}", EnableState::to_string(self))
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
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.uiAndVisuals.titleContainer.direction.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Direction::to_string(self))
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
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.uiAndVisuals.titleContainer.alignment.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Alignment::to_string(self))
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
        return (*self == TabHudStyle::Minimal);
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.uiAndVisuals.tabHud.style.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for TabHudStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", TabHudStyle::to_string(self))
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
            // java: DEFAULT((_, _) -> 0)
            NameSortingFields::new(Arc::new(move |_: &PlayerInfo, _: &PlayerInfo| std::cmp::Ordering::Equal)),
            // java: ALPHABETICAL(Comparator.comparing(ple -> matchPlayerName(..., "name").orElse(""), String.CASE_INSENSITIVE_ORDER))
            NameSortingFields::new(Arc::new(move |a: &PlayerInfo, b: &PlayerInfo| {
                let key = |ple: &PlayerInfo| NameSorting::match_player_name(&ple.get_tab_list_display_name().unwrap().get_string(), "name").unwrap_or_else(|| "".to_string());
                NameSorting::case_insensitive_order(&key(a), &key(b))
            })),
            // java: SKYBLOCK_LEVEL(Comparator.<PlayerInfo>comparingInt(ple -> matchPlayerName(..., "level").map(Integer::parseInt).orElse(0)).reversed())
            NameSortingFields::new(Arc::new(move |a: &PlayerInfo, b: &PlayerInfo| {
                let key = |ple: &PlayerInfo| NameSorting::match_player_name(&ple.get_tab_list_display_name().unwrap().get_string(), "level")
                    .map(|level| level.parse::<i32>().expect("NumberFormatException"))
                    .unwrap_or(0);
                key(b).cmp(&key(a))
            })),
        ]);
        &FIELDS[self as usize]
    }

    /// `String.CASE_INSENSITIVE_ORDER`: char by char, each compared upper-cased then lower-cased.
    fn case_insensitive_order(a: &str, b: &str) -> std::cmp::Ordering {
        let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
        for (c1, c2) in a.iter().zip(b.iter()) {
            if c1 != c2 {
                let (u1, u2) = (compat::char_upper(*c1), compat::char_upper(*c2));
                if u1 != u2 {
                    let (l1, l2) = (compat::char_lower(u1), compat::char_lower(u2));
                    if l1 != l2 {
                        return l1.cmp(&l2);
                    }
                }
            }
        }
        a.len().cmp(&b.len())
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
        return self.name().to_lowercase();
    }

}

impl std::fmt::Display for NameSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NameSorting::to_string(self))
    }
}

/// the constructor data of each NameSorting constant
pub struct NameSortingFields {
    pub comparator: Arc<dyn Fn(&PlayerInfo, &PlayerInfo) -> std::cmp::Ordering + Send + Sync>,
}

impl NameSortingFields {
    fn new(comparator: Arc<dyn Fn(&PlayerInfo, &PlayerInfo) -> std::cmp::Ordering + Send + Sync>) -> Self {
        Self { comparator }
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
        let mut this = Self {
            enable_teleport_overlays: true,
            show_when_in_air: false,
            teleport_overlay_color: Color::from_rgba(0x7F761594, true),
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
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.uiAndVisuals.teleportOverlay.highlightType.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for HighlightType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HighlightType::to_string(self))
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
        let mut this = Self {
            enabled: true,
            precision: 1,
            max_precision: 4,
            normal_damage_color: Color::from_rgb(0xFFFFFF),
            crit_damage_gradient_start: Color::from_rgb(0xFFFF55),
            crit_damage_gradient_end: Color::from_rgb(0xFF5555),
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
        let mut this = Self {
            enabled: false,
            scale: 1.5f32,
            remove_health_from_name: true,
            remove_max_health_from_name: true,
            apply_to_health_only_mobs: true,
            hide_full_health: false,
            full_bar_color: Color::from_rgb(0x00FF00),
            half_bar_color: Color::from_rgb(0xFFFF00),
            empty_bar_color: Color::from_rgb(0xFF0000),
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
        let mut this = Self {
            gyro_overlay_mode: Mode::Off,
            gyro_overlay_color: Color::from_rgba(0x7F761594, true),
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

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/ui_and_visuals_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// UIAndVisualsConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl UIAndVisualsConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("swingOnAbilities".to_string(), dandelion::compat::GsonValue::Bool(*(&self.swing_on_abilities))),
            ("nightVisionStrength".to_string(), dandelion::compat::GsonValue::int(*(&self.night_vision_strength) as i64)),
            ("compactorDeletorPreview".to_string(), dandelion::compat::GsonValue::Bool(*(&self.compactor_deletor_preview))),
            ("dontStripSkinAlphaValues".to_string(), dandelion::compat::GsonValue::Bool(*(&self.dont_strip_skin_alpha_values))),
            ("backpackPreviewWithoutShift".to_string(), dandelion::compat::GsonValue::Bool(*(&self.backpack_preview_without_shift))),
            ("hideEmptyTooltips".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_empty_tooltips))),
            ("fancyCraftingTable".to_string(), dandelion::compat::GsonValue::Bool(*(&self.fancy_crafting_table))),
            ("hideStatusEffectOverlay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_status_effect_overlay))),
            ("showEquipmentInInventory".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_equipment_in_inventory))),
            ("cancelComponentUpdateAnimation".to_string(), dandelion::compat::GsonValue::Bool(*(&self.cancel_component_update_animation))),
            ("showCustomizeButton".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_customize_button))),
            ("showConfigButton".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_config_button))),
            ("trueQuiverCount".to_string(), dandelion::compat::GsonValue::Bool(*(&self.true_quiver_count))),
            ("skyblockInventoryScreen".to_string(), ((&self.skyblock_inventory_screen)).__gson_write()),
            ("chestValue".to_string(), ((&self.chest_value)).__gson_write()),
            ("itemCooldown".to_string(), ((&self.item_cooldown)).__gson_write()),
            ("museumOverlay".to_string(), dandelion::compat::GsonValue::Bool(*(&self.museum_overlay))),
            ("slotText".to_string(), ((&self.slot_text)).__gson_write()),
            ("storageOverlay".to_string(), ((&self.storage_overlay)).__gson_write()),
            ("radialMenu".to_string(), ((&self.radial_menu)).__gson_write()),
            ("inventorySearch".to_string(), ((&self.inventory_search)).__gson_write()),
            ("titleContainer".to_string(), ((&self.title_container)).__gson_write()),
            ("tabHud".to_string(), ((&self.tab_hud)).__gson_write()),
            ("fancyAuctionHouse".to_string(), ((&self.fancy_auction_house)).__gson_write()),
            ("bars".to_string(), ((&self.bars)).__gson_write()),
            ("waypoints".to_string(), ((&self.waypoints)).__gson_write()),
            ("teleportOverlay".to_string(), ((&self.teleport_overlay)).__gson_write()),
            ("smoothAOTE".to_string(), ((&self.smooth_aote)).__gson_write()),
            ("searchOverlay".to_string(), ((&self.search_overlay)).__gson_write()),
            ("bazaarQuickQuantities".to_string(), ((&self.bazaar_quick_quantities)).__gson_write()),
            ("inputCalculator".to_string(), ((&self.input_calculator)).__gson_write()),
            ("flameOverlay".to_string(), ((&self.flame_overlay)).__gson_write()),
            ("compactDamage".to_string(), ((&self.compact_damage)).__gson_write()),
            ("healthBars".to_string(), ((&self.health_bars)).__gson_write()),
            ("gyroOverlay".to_string(), ((&self.gyro_overlay)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<UIAndVisualsConfig> {
        let __o = __v.as_object()?;
        let mut __this = UIAndVisualsConfig::new();
        if let Some(__v) = __o.get("swingOnAbilities") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.swing_on_abilities = __x; }
        }
        if let Some(__v) = __o.get("nightVisionStrength") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.night_vision_strength = __x; }
        }
        if let Some(__v) = __o.get("compactorDeletorPreview") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.compactor_deletor_preview = __x; }
        }
        if let Some(__v) = __o.get("dontStripSkinAlphaValues") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.dont_strip_skin_alpha_values = __x; }
        }
        if let Some(__v) = __o.get("backpackPreviewWithoutShift") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.backpack_preview_without_shift = __x; }
        }
        if let Some(__v) = __o.get("hideEmptyTooltips") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_empty_tooltips = __x; }
        }
        if let Some(__v) = __o.get("fancyCraftingTable") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.fancy_crafting_table = __x; }
        }
        if let Some(__v) = __o.get("hideStatusEffectOverlay") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_status_effect_overlay = __x; }
        }
        if let Some(__v) = __o.get("showEquipmentInInventory") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_equipment_in_inventory = __x; }
        }
        if let Some(__v) = __o.get("cancelComponentUpdateAnimation") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.cancel_component_update_animation = __x; }
        }
        if let Some(__v) = __o.get("showCustomizeButton") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_customize_button = __x; }
        }
        if let Some(__v) = __o.get("showConfigButton") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_config_button = __x; }
        }
        if let Some(__v) = __o.get("trueQuiverCount") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.true_quiver_count = __x; }
        }
        if let Some(__v) = __o.get("skyblockInventoryScreen") {
            if let Some(__x) = SkyBlockInventoryScreen::__gson_read(__v) { __this.skyblock_inventory_screen = __x; }
        }
        if let Some(__v) = __o.get("chestValue") {
            if let Some(__x) = ChestValue::__gson_read(__v) { __this.chest_value = __x; }
        }
        if let Some(__v) = __o.get("itemCooldown") {
            if let Some(__x) = ItemCooldown::__gson_read(__v) { __this.item_cooldown = __x; }
        }
        if let Some(__v) = __o.get("museumOverlay") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.museum_overlay = __x; }
        }
        if let Some(__v) = __o.get("slotText") {
            if let Some(__x) = SlotText::__gson_read(__v) { __this.slot_text = __x; }
        }
        if let Some(__v) = __o.get("storageOverlay") {
            if let Some(__x) = StorageOverlay::__gson_read(__v) { __this.storage_overlay = __x; }
        }
        if let Some(__v) = __o.get("radialMenu") {
            if let Some(__x) = RadialMenu::__gson_read(__v) { __this.radial_menu = __x; }
        }
        if let Some(__v) = __o.get("inventorySearch") {
            if let Some(__x) = InventorySearchConfig::__gson_read(__v) { __this.inventory_search = __x; }
        }
        if let Some(__v) = __o.get("titleContainer") {
            if let Some(__x) = TitleContainer::__gson_read(__v) { __this.title_container = __x; }
        }
        if let Some(__v) = __o.get("tabHud") {
            if let Some(__x) = TabHudConf::__gson_read(__v) { __this.tab_hud = __x; }
        }
        if let Some(__v) = __o.get("fancyAuctionHouse") {
            if let Some(__x) = FancyAuctionHouse::__gson_read(__v) { __this.fancy_auction_house = __x; }
        }
        if let Some(__v) = __o.get("bars") {
            if let Some(__x) = Bars::__gson_read(__v) { __this.bars = __x; }
        }
        if let Some(__v) = __o.get("waypoints") {
            if let Some(__x) = Waypoints::__gson_read(__v) { __this.waypoints = __x; }
        }
        if let Some(__v) = __o.get("teleportOverlay") {
            if let Some(__x) = TeleportOverlay::__gson_read(__v) { __this.teleport_overlay = __x; }
        }
        if let Some(__v) = __o.get("smoothAOTE") {
            if let Some(__x) = SmoothAOTE::__gson_read(__v) { __this.smooth_aote = __x; }
        }
        if let Some(__v) = __o.get("searchOverlay") {
            if let Some(__x) = SearchOverlay::__gson_read(__v) { __this.search_overlay = __x; }
        }
        if let Some(__v) = __o.get("bazaarQuickQuantities") {
            if let Some(__x) = BazaarQuickQuantities::__gson_read(__v) { __this.bazaar_quick_quantities = __x; }
        }
        if let Some(__v) = __o.get("inputCalculator") {
            if let Some(__x) = InputCalculator::__gson_read(__v) { __this.input_calculator = __x; }
        }
        if let Some(__v) = __o.get("flameOverlay") {
            if let Some(__x) = FlameOverlay::__gson_read(__v) { __this.flame_overlay = __x; }
        }
        if let Some(__v) = __o.get("compactDamage") {
            if let Some(__x) = CompactDamage::__gson_read(__v) { __this.compact_damage = __x; }
        }
        if let Some(__v) = __o.get("healthBars") {
            if let Some(__x) = HealthBars::__gson_read(__v) { __this.health_bars = __x; }
        }
        if let Some(__v) = __o.get("gyroOverlay") {
            if let Some(__x) = GyrokineticWandOverlay::__gson_read(__v) { __this.gyro_overlay = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "swingOnAbilities" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.swing_on_abilities = *__b; true } else { false },
                "compactorDeletorPreview" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.compactor_deletor_preview = *__b; true } else { false },
                "dontStripSkinAlphaValues" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.dont_strip_skin_alpha_values = *__b; true } else { false },
                "backpackPreviewWithoutShift" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.backpack_preview_without_shift = *__b; true } else { false },
                "hideEmptyTooltips" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_empty_tooltips = *__b; true } else { false },
                "fancyCraftingTable" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.fancy_crafting_table = *__b; true } else { false },
                "hideStatusEffectOverlay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_status_effect_overlay = *__b; true } else { false },
                "showEquipmentInInventory" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_equipment_in_inventory = *__b; true } else { false },
                "cancelComponentUpdateAnimation" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.cancel_component_update_animation = *__b; true } else { false },
                "showCustomizeButton" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_customize_button = *__b; true } else { false },
                "showConfigButton" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_config_button = *__b; true } else { false },
                "trueQuiverCount" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.true_quiver_count = *__b; true } else { false },
                "museumOverlay" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.museum_overlay = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "skyblockInventoryScreen" => self.skyblock_inventory_screen.__gson_patch(__rest, __value),
                "chestValue" => self.chest_value.__gson_patch(__rest, __value),
                "itemCooldown" => self.item_cooldown.__gson_patch(__rest, __value),
                "slotText" => self.slot_text.__gson_patch(__rest, __value),
                "storageOverlay" => self.storage_overlay.__gson_patch(__rest, __value),
                "radialMenu" => self.radial_menu.__gson_patch(__rest, __value),
                "inventorySearch" => self.inventory_search.__gson_patch(__rest, __value),
                "titleContainer" => self.title_container.__gson_patch(__rest, __value),
                "tabHud" => self.tab_hud.__gson_patch(__rest, __value),
                "fancyAuctionHouse" => self.fancy_auction_house.__gson_patch(__rest, __value),
                "bars" => self.bars.__gson_patch(__rest, __value),
                "waypoints" => self.waypoints.__gson_patch(__rest, __value),
                "teleportOverlay" => self.teleport_overlay.__gson_patch(__rest, __value),
                "smoothAOTE" => self.smooth_aote.__gson_patch(__rest, __value),
                "searchOverlay" => self.search_overlay.__gson_patch(__rest, __value),
                "bazaarQuickQuantities" => self.bazaar_quick_quantities.__gson_patch(__rest, __value),
                "inputCalculator" => self.input_calculator.__gson_patch(__rest, __value),
                "flameOverlay" => self.flame_overlay.__gson_patch(__rest, __value),
                "compactDamage" => self.compact_damage.__gson_patch(__rest, __value),
                "healthBars" => self.health_bars.__gson_patch(__rest, __value),
                "gyroOverlay" => self.gyro_overlay.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `UIAndVisualsConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<UIAndVisualsConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig",
            new: UIAndVisualsConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: UIAndVisualsConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SkyBlockInventoryScreen's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SkyBlockInventoryScreen {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("openEquipmentToStatsPage".to_string(), dandelion::compat::GsonValue::Bool(*(&self.open_equipment_to_stats_page))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SkyBlockInventoryScreen> {
        let __o = __v.as_object()?;
        let mut __this = SkyBlockInventoryScreen::new();
        if let Some(__v) = __o.get("openEquipmentToStatsPage") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.open_equipment_to_stats_page = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "openEquipmentToStatsPage" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.open_equipment_to_stats_page = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SkyBlockInventoryScreen.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SkyBlockInventoryScreen> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$SkyBlockInventoryScreen",
            new: SkyBlockInventoryScreen::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SkyBlockInventoryScreen::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ChestValue's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ChestValue {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableChestValue".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_chest_value))),
            ("color".to_string(), dandelion::compat::GsonValue::str((match (&self.color) { ChatFormatting::Black => "BLACK", ChatFormatting::DarkBlue => "DARK_BLUE", ChatFormatting::DarkGreen => "DARK_GREEN", ChatFormatting::DarkAqua => "DARK_AQUA", ChatFormatting::DarkRed => "DARK_RED", ChatFormatting::DarkPurple => "DARK_PURPLE", ChatFormatting::Gold => "GOLD", ChatFormatting::Gray => "GRAY", ChatFormatting::DarkGray => "DARK_GRAY", ChatFormatting::Blue => "BLUE", ChatFormatting::Green => "GREEN", ChatFormatting::Aqua => "AQUA", ChatFormatting::Red => "RED", ChatFormatting::LightPurple => "LIGHT_PURPLE", ChatFormatting::Yellow => "YELLOW", ChatFormatting::White => "WHITE", ChatFormatting::Obfuscated => "OBFUSCATED", ChatFormatting::Bold => "BOLD", ChatFormatting::Strikethrough => "STRIKETHROUGH", ChatFormatting::Underline => "UNDERLINE", ChatFormatting::Italic => "ITALIC", ChatFormatting::Reset => "RESET", #[allow(unreachable_patterns)] _ => "" }))),
            ("incompleteColor".to_string(), dandelion::compat::GsonValue::str((match (&self.incomplete_color) { ChatFormatting::Black => "BLACK", ChatFormatting::DarkBlue => "DARK_BLUE", ChatFormatting::DarkGreen => "DARK_GREEN", ChatFormatting::DarkAqua => "DARK_AQUA", ChatFormatting::DarkRed => "DARK_RED", ChatFormatting::DarkPurple => "DARK_PURPLE", ChatFormatting::Gold => "GOLD", ChatFormatting::Gray => "GRAY", ChatFormatting::DarkGray => "DARK_GRAY", ChatFormatting::Blue => "BLUE", ChatFormatting::Green => "GREEN", ChatFormatting::Aqua => "AQUA", ChatFormatting::Red => "RED", ChatFormatting::LightPurple => "LIGHT_PURPLE", ChatFormatting::Yellow => "YELLOW", ChatFormatting::White => "WHITE", ChatFormatting::Obfuscated => "OBFUSCATED", ChatFormatting::Bold => "BOLD", ChatFormatting::Strikethrough => "STRIKETHROUGH", ChatFormatting::Underline => "UNDERLINE", ChatFormatting::Italic => "ITALIC", ChatFormatting::Reset => "RESET", #[allow(unreachable_patterns)] _ => "" }))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ChestValue> {
        let __o = __v.as_object()?;
        let mut __this = ChestValue::new();
        if let Some(__v) = __o.get("enableChestValue") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_chest_value = __x; }
        }
        if let Some(__v) = __o.get("color") {
            if let Some(__x) = (__v).as_str().and_then(|__s| (match __s { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None })) { __this.color = __x; }
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
                "enableChestValue" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_chest_value = *__b; true } else { false },
                "color" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match (match __s.as_str() { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None }) { Some(__c) => { self.color = __c; true } None => false } } else { false },
                "incompleteColor" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match (match __s.as_str() { "BLACK" => Some(ChatFormatting::Black), "DARK_BLUE" => Some(ChatFormatting::DarkBlue), "DARK_GREEN" => Some(ChatFormatting::DarkGreen), "DARK_AQUA" => Some(ChatFormatting::DarkAqua), "DARK_RED" => Some(ChatFormatting::DarkRed), "DARK_PURPLE" => Some(ChatFormatting::DarkPurple), "GOLD" => Some(ChatFormatting::Gold), "GRAY" => Some(ChatFormatting::Gray), "DARK_GRAY" => Some(ChatFormatting::DarkGray), "BLUE" => Some(ChatFormatting::Blue), "GREEN" => Some(ChatFormatting::Green), "AQUA" => Some(ChatFormatting::Aqua), "RED" => Some(ChatFormatting::Red), "LIGHT_PURPLE" => Some(ChatFormatting::LightPurple), "YELLOW" => Some(ChatFormatting::Yellow), "WHITE" => Some(ChatFormatting::White), "OBFUSCATED" => Some(ChatFormatting::Obfuscated), "BOLD" => Some(ChatFormatting::Bold), "STRIKETHROUGH" => Some(ChatFormatting::Strikethrough), "UNDERLINE" => Some(ChatFormatting::Underline), "ITALIC" => Some(ChatFormatting::Italic), "RESET" => Some(ChatFormatting::Reset), _ => None }) { Some(__c) => { self.incomplete_color = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ChestValue.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ChestValue> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$ChestValue",
            new: ChestValue::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ChestValue::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ItemCooldown's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ItemCooldown {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableItemCooldowns".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_item_cooldowns))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ItemCooldown> {
        let __o = __v.as_object()?;
        let mut __this = ItemCooldown::new();
        if let Some(__v) = __o.get("enableItemCooldowns") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_item_cooldowns = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableItemCooldowns" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_item_cooldowns = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ItemCooldown.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ItemCooldown> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$ItemCooldown",
            new: ItemCooldown::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ItemCooldown::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SlotText's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SlotText {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("slotTextMode".to_string(), dandelion::compat::GsonValue::str(((&self.slot_text_mode)).name())),
            ("textEnabled".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.text_enabled)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::GsonValue::Bool(*__x0))).collect())),
            ("slotTextToggled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.slot_text_toggled))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SlotText> {
        let __o = __v.as_object()?;
        let mut __this = SlotText::new();
        if let Some(__v) = __o.get("slotTextMode") {
            if let Some(__x) = (__v).as_str().and_then(|__s| SlotTextMode::values().into_iter().find(|__c| __c.name() == __s)) { __this.slot_text_mode = __x; }
        }
        if let Some(__v) = __o.get("textEnabled") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_bool(__y0)?))).collect()) { __this.text_enabled = __x; }
        }
        if let Some(__v) = __o.get("slotTextToggled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.slot_text_toggled = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "slotTextMode" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match SlotTextMode::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.slot_text_mode = __c; true } None => false } } else { false },
                "slotTextToggled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.slot_text_toggled = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SlotText.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SlotText> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$SlotText",
            new: SlotText::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SlotText::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// StorageOverlay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl StorageOverlay {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("storagesPerRow".to_string(), dandelion::compat::GsonValue::int(*(&self.storages_per_row) as i64)),
            ("backpackWidth".to_string(), dandelion::compat::GsonValue::int(*(&self.backpack_width) as i64)),
            ("rememberSearch".to_string(), dandelion::compat::GsonValue::Bool(*(&self.remember_search))),
            ("rememberOpened".to_string(), dandelion::compat::GsonValue::Bool(*(&self.remember_opened))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<StorageOverlay> {
        let __o = __v.as_object()?;
        let mut __this = StorageOverlay::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("storagesPerRow") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.storages_per_row = __x; }
        }
        if let Some(__v) = __o.get("backpackWidth") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.backpack_width = __x; }
        }
        if let Some(__v) = __o.get("rememberSearch") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.remember_search = __x; }
        }
        if let Some(__v) = __o.get("rememberOpened") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.remember_opened = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "rememberSearch" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.remember_search = *__b; true } else { false },
                "rememberOpened" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.remember_opened = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `StorageOverlay.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<StorageOverlay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$StorageOverlay",
            new: StorageOverlay::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: StorageOverlay::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// RadialMenu's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl RadialMenu {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("tooltipsWithoutShift".to_string(), dandelion::compat::GsonValue::Bool(*(&self.tooltips_without_shift))),
            ("enabledMenus".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.enabled_menus)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::GsonValue::Bool(*__x0))).collect())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<RadialMenu> {
        let __o = __v.as_object()?;
        let mut __this = RadialMenu::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("tooltipsWithoutShift") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.tooltips_without_shift = __x; }
        }
        if let Some(__v) = __o.get("enabledMenus") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_bool(__y0)?))).collect()) { __this.enabled_menus = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "tooltipsWithoutShift" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.tooltips_without_shift = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `RadialMenu.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<RadialMenu> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$RadialMenu",
            new: RadialMenu::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: RadialMenu::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// InventorySearchConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl InventorySearchConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::str(((&self.enabled)).name())),
            ("ctrlK".to_string(), dandelion::compat::GsonValue::Bool(*(&self.ctrl_k))),
            ("clickableText".to_string(), dandelion::compat::GsonValue::Bool(*(&self.clickable_text))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<InventorySearchConfig> {
        let __o = __v.as_object()?;
        let mut __this = InventorySearchConfig::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = (__v).as_str().and_then(|__s| EnableState::values().into_iter().find(|__c| __c.name() == __s)) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("ctrlK") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.ctrl_k = __x; }
        }
        if let Some(__v) = __o.get("clickableText") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.clickable_text = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match EnableState::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.enabled = __c; true } None => false } } else { false },
                "ctrlK" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.ctrl_k = *__b; true } else { false },
                "clickableText" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.clickable_text = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `InventorySearchConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<InventorySearchConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$InventorySearchConfig",
            new: InventorySearchConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: InventorySearchConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// EnableState's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl EnableState {
    pub fn __class_of() -> dandelion::compat::ClassOf<EnableState> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$InventorySearchConfig$EnableState",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| EnableState::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || EnableState::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// TitleContainer's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TitleContainer {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("titleContainerScale".to_string(), dandelion::compat::GsonValue::float(*(&self.title_container_scale))),
            ("x".to_string(), dandelion::compat::GsonValue::int(*(&self.x) as i64)),
            ("y".to_string(), dandelion::compat::GsonValue::int(*(&self.y) as i64)),
            ("direction".to_string(), dandelion::compat::GsonValue::str(((&self.direction)).name())),
            ("alignment".to_string(), dandelion::compat::GsonValue::str(((&self.alignment)).name())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<TitleContainer> {
        let __o = __v.as_object()?;
        let mut __this = TitleContainer::new();
        if let Some(__v) = __o.get("titleContainerScale") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.title_container_scale = __x; }
        }
        if let Some(__v) = __o.get("x") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.x = __x; }
        }
        if let Some(__v) = __o.get("y") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.y = __x; }
        }
        if let Some(__v) = __o.get("direction") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Direction::values().into_iter().find(|__c| __c.name() == __s)) { __this.direction = __x; }
        }
        if let Some(__v) = __o.get("alignment") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Alignment::values().into_iter().find(|__c| __c.name() == __s)) { __this.alignment = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "direction" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Direction::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.direction = __c; true } None => false } } else { false },
                "alignment" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Alignment::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.alignment = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `TitleContainer.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<TitleContainer> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$TitleContainer",
            new: TitleContainer::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: TitleContainer::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Direction's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Direction {
    pub fn __class_of() -> dandelion::compat::ClassOf<Direction> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$Direction",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| Direction::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || Direction::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// Alignment's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Alignment {
    pub fn __class_of() -> dandelion::compat::ClassOf<Alignment> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$Alignment",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| Alignment::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || Alignment::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// TabHudConf's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TabHudConf {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("tabHudEnabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.tab_hud_enabled))),
            ("tabHudScale".to_string(), dandelion::compat::GsonValue::int(*(&self.tab_hud_scale) as i64)),
            ("showVanillaTabByDefault".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_vanilla_tab_by_default))),
            ("style".to_string(), dandelion::compat::GsonValue::str(((&self.style)).name())),
            ("displayIcons".to_string(), dandelion::compat::GsonValue::Bool(*(&self.display_icons))),
            ("compactWidgets".to_string(), dandelion::compat::GsonValue::Bool(*(&self.compact_widgets))),
            ("enableHudBackground".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_hud_background))),
            ("defaultPositioning".to_string(), dandelion::compat::GsonValue::str(((&self.default_positioning)).name())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<TabHudConf> {
        let __o = __v.as_object()?;
        let mut __this = TabHudConf::new();
        if let Some(__v) = __o.get("tabHudEnabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.tab_hud_enabled = __x; }
        }
        if let Some(__v) = __o.get("tabHudScale") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.tab_hud_scale = __x; }
        }
        if let Some(__v) = __o.get("showVanillaTabByDefault") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_vanilla_tab_by_default = __x; }
        }
        if let Some(__v) = __o.get("style") {
            if let Some(__x) = (__v).as_str().and_then(|__s| TabHudStyle::values().into_iter().find(|__c| __c.name() == __s)) { __this.style = __x; }
        }
        if let Some(__v) = __o.get("displayIcons") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.display_icons = __x; }
        }
        if let Some(__v) = __o.get("compactWidgets") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.compact_widgets = __x; }
        }
        if let Some(__v) = __o.get("enableHudBackground") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_hud_background = __x; }
        }
        if let Some(__v) = __o.get("defaultPositioning") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Positioner::values().into_iter().find(|__c| __c.name() == __s)) { __this.default_positioning = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "tabHudEnabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.tab_hud_enabled = *__b; true } else { false },
                "showVanillaTabByDefault" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_vanilla_tab_by_default = *__b; true } else { false },
                "style" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match TabHudStyle::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.style = __c; true } None => false } } else { false },
                "displayIcons" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.display_icons = *__b; true } else { false },
                "compactWidgets" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.compact_widgets = *__b; true } else { false },
                "enableHudBackground" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_hud_background = *__b; true } else { false },
                "defaultPositioning" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Positioner::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.default_positioning = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `TabHudConf.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<TabHudConf> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$TabHudConf",
            new: TabHudConf::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: TabHudConf::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// TabHudStyle's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TabHudStyle {
    pub fn __class_of() -> dandelion::compat::ClassOf<TabHudStyle> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$TabHudStyle",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| TabHudStyle::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || TabHudStyle::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// NameSorting's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl NameSorting {
    pub fn __class_of() -> dandelion::compat::ClassOf<NameSorting> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$NameSorting",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| NameSorting::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || NameSorting::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// FancyAuctionHouse's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl FancyAuctionHouse {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("highlightCheapBIN".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_cheap_bin))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<FancyAuctionHouse> {
        let __o = __v.as_object()?;
        let mut __this = FancyAuctionHouse::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("highlightCheapBIN") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_cheap_bin = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "highlightCheapBIN" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_cheap_bin = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `FancyAuctionHouse.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<FancyAuctionHouse> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$FancyAuctionHouse",
            new: FancyAuctionHouse::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: FancyAuctionHouse::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Bars's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Bars {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableBars".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_bars))),
            ("enableBarsRift".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_bars_rift))),
            ("riftHealthHP".to_string(), dandelion::compat::GsonValue::Bool(*(&self.rift_health_hp))),
            ("enableVanillaStyleManaBar".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_vanilla_style_mana_bar))),
            ("enableVanillaStyleManaBarBlinking".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_vanilla_style_mana_bar_blinking))),
            ("useHungerBarSprites".to_string(), dandelion::compat::GsonValue::Bool(*(&self.use_hunger_bar_sprites))),
            ("intelligenceDisplay".to_string(), dandelion::compat::GsonValue::str(((&self.intelligence_display)).name())),
            ("showEstimatedTilde".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_estimated_tilde))),
            ("hasSeenVitalityAtLeastOnce".to_string(), dandelion::compat::GsonValue::Bool(*(&self.has_seen_vitality_at_least_once))),
            ("barPositions".to_string(), ((&self.bar_positions)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Bars> {
        let __o = __v.as_object()?;
        let mut __this = Bars::new();
        if let Some(__v) = __o.get("enableBars") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_bars = __x; }
        }
        if let Some(__v) = __o.get("enableBarsRift") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_bars_rift = __x; }
        }
        if let Some(__v) = __o.get("riftHealthHP") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.rift_health_hp = __x; }
        }
        if let Some(__v) = __o.get("enableVanillaStyleManaBar") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_vanilla_style_mana_bar = __x; }
        }
        if let Some(__v) = __o.get("enableVanillaStyleManaBarBlinking") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_vanilla_style_mana_bar_blinking = __x; }
        }
        if let Some(__v) = __o.get("useHungerBarSprites") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.use_hunger_bar_sprites = __x; }
        }
        if let Some(__v) = __o.get("intelligenceDisplay") {
            if let Some(__x) = (__v).as_str().and_then(|__s| IntelligenceDisplay::values().into_iter().find(|__c| __c.name() == __s)) { __this.intelligence_display = __x; }
        }
        if let Some(__v) = __o.get("showEstimatedTilde") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_estimated_tilde = __x; }
        }
        if let Some(__v) = __o.get("hasSeenVitalityAtLeastOnce") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.has_seen_vitality_at_least_once = __x; }
        }
        if let Some(__v) = __o.get("barPositions") {
            if let Some(__x) = LegacyBarPositions::__gson_read(__v) { __this.bar_positions = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableBars" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_bars = *__b; true } else { false },
                "enableBarsRift" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_bars_rift = *__b; true } else { false },
                "riftHealthHP" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.rift_health_hp = *__b; true } else { false },
                "enableVanillaStyleManaBar" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_vanilla_style_mana_bar = *__b; true } else { false },
                "enableVanillaStyleManaBarBlinking" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_vanilla_style_mana_bar_blinking = *__b; true } else { false },
                "useHungerBarSprites" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.use_hunger_bar_sprites = *__b; true } else { false },
                "intelligenceDisplay" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match IntelligenceDisplay::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.intelligence_display = __c; true } None => false } } else { false },
                "showEstimatedTilde" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_estimated_tilde = *__b; true } else { false },
                "hasSeenVitalityAtLeastOnce" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.has_seen_vitality_at_least_once = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "barPositions" => self.bar_positions.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `Bars.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Bars> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$Bars",
            new: Bars::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Bars::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// IntelligenceDisplay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl IntelligenceDisplay {
    pub fn __class_of() -> dandelion::compat::ClassOf<IntelligenceDisplay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$IntelligenceDisplay",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| IntelligenceDisplay::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || IntelligenceDisplay::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// LegacyBarPositions's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl LegacyBarPositions {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("healthBarPosition".to_string(), dandelion::compat::GsonValue::str(((&self.health_bar_position)).name())),
            ("manaBarPosition".to_string(), dandelion::compat::GsonValue::str(((&self.mana_bar_position)).name())),
            ("defenceBarPosition".to_string(), dandelion::compat::GsonValue::str(((&self.defence_bar_position)).name())),
            ("experienceBarPosition".to_string(), dandelion::compat::GsonValue::str(((&self.experience_bar_position)).name())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<LegacyBarPositions> {
        let __o = __v.as_object()?;
        let mut __this = LegacyBarPositions::new();
        if let Some(__v) = __o.get("healthBarPosition") {
            if let Some(__x) = (__v).as_str().and_then(|__s| LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s)) { __this.health_bar_position = __x; }
        }
        if let Some(__v) = __o.get("manaBarPosition") {
            if let Some(__x) = (__v).as_str().and_then(|__s| LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s)) { __this.mana_bar_position = __x; }
        }
        if let Some(__v) = __o.get("defenceBarPosition") {
            if let Some(__x) = (__v).as_str().and_then(|__s| LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s)) { __this.defence_bar_position = __x; }
        }
        if let Some(__v) = __o.get("experienceBarPosition") {
            if let Some(__x) = (__v).as_str().and_then(|__s| LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s)) { __this.experience_bar_position = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "healthBarPosition" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.health_bar_position = __c; true } None => false } } else { false },
                "manaBarPosition" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.mana_bar_position = __c; true } None => false } } else { false },
                "defenceBarPosition" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.defence_bar_position = __c; true } None => false } } else { false },
                "experienceBarPosition" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.experience_bar_position = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `LegacyBarPositions.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<LegacyBarPositions> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$LegacyBarPositions",
            new: LegacyBarPositions::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: LegacyBarPositions::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// LegacyBarPosition's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl LegacyBarPosition {
    pub fn __class_of() -> dandelion::compat::ClassOf<LegacyBarPosition> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$LegacyBarPosition",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| LegacyBarPosition::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || LegacyBarPosition::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// Waypoints's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Waypoints {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_waypoints))),
            ("waypointType".to_string(), dandelion::compat::GsonValue::str(((&self.waypoint_type)).name())),
            ("renderLine".to_string(), dandelion::compat::GsonValue::Bool(*(&self.render_line))),
            ("lineColor".to_string(), dandelion::compat::GsonValue::int(((&self.line_color)).get_rgb() as i64)),
            ("lineWidth".to_string(), dandelion::compat::GsonValue::float(*(&self.line_width))),
            ("allowSkippingWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.allow_skipping_waypoints))),
            ("allowGoingBackwards".to_string(), dandelion::compat::GsonValue::Bool(*(&self.allow_going_backwards))),
            ("waypointActivationRadius".to_string(), dandelion::compat::GsonValue::float(*(&self.waypoint_activation_radius))),
            ("enableChatWaypoints".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_chat_waypoints))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Waypoints> {
        let __o = __v.as_object()?;
        let mut __this = Waypoints::new();
        if let Some(__v) = __o.get("enableWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_waypoints = __x; }
        }
        if let Some(__v) = __o.get("waypointType") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Type::values().into_iter().find(|__c| __c.name() == __s)) { __this.waypoint_type = __x; }
        }
        if let Some(__v) = __o.get("renderLine") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.render_line = __x; }
        }
        if let Some(__v) = __o.get("lineColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.line_color = __x; }
        }
        if let Some(__v) = __o.get("lineWidth") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.line_width = __x; }
        }
        if let Some(__v) = __o.get("allowSkippingWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.allow_skipping_waypoints = __x; }
        }
        if let Some(__v) = __o.get("allowGoingBackwards") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.allow_going_backwards = __x; }
        }
        if let Some(__v) = __o.get("waypointActivationRadius") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.waypoint_activation_radius = __x; }
        }
        if let Some(__v) = __o.get("enableChatWaypoints") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_chat_waypoints = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_waypoints = *__b; true } else { false },
                "waypointType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Type::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.waypoint_type = __c; true } None => false } } else { false },
                "renderLine" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.render_line = *__b; true } else { false },
                "allowSkippingWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.allow_skipping_waypoints = *__b; true } else { false },
                "allowGoingBackwards" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.allow_going_backwards = *__b; true } else { false },
                "enableChatWaypoints" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_chat_waypoints = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Waypoints.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Waypoints> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$Waypoints",
            new: Waypoints::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Waypoints::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// TeleportOverlay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl TeleportOverlay {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableTeleportOverlays".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_teleport_overlays))),
            ("showWhenInAir".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_when_in_air))),
            ("teleportOverlayColor".to_string(), dandelion::compat::GsonValue::int(((&self.teleport_overlay_color)).get_rgb() as i64)),
            ("highlightType".to_string(), dandelion::compat::GsonValue::str(((&self.highlight_type)).name())),
            ("enableWeirdTransmission".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_weird_transmission))),
            ("enableInstantTransmission".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_instant_transmission))),
            ("enableEtherTransmission".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_ether_transmission))),
            ("enableSinrecallTransmission".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_sinrecall_transmission))),
            ("enableWitherImpact".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_wither_impact))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<TeleportOverlay> {
        let __o = __v.as_object()?;
        let mut __this = TeleportOverlay::new();
        if let Some(__v) = __o.get("enableTeleportOverlays") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_teleport_overlays = __x; }
        }
        if let Some(__v) = __o.get("showWhenInAir") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_when_in_air = __x; }
        }
        if let Some(__v) = __o.get("teleportOverlayColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.teleport_overlay_color = __x; }
        }
        if let Some(__v) = __o.get("highlightType") {
            if let Some(__x) = (__v).as_str().and_then(|__s| HighlightType::values().into_iter().find(|__c| __c.name() == __s)) { __this.highlight_type = __x; }
        }
        if let Some(__v) = __o.get("enableWeirdTransmission") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_weird_transmission = __x; }
        }
        if let Some(__v) = __o.get("enableInstantTransmission") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_instant_transmission = __x; }
        }
        if let Some(__v) = __o.get("enableEtherTransmission") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_ether_transmission = __x; }
        }
        if let Some(__v) = __o.get("enableSinrecallTransmission") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_sinrecall_transmission = __x; }
        }
        if let Some(__v) = __o.get("enableWitherImpact") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_wither_impact = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableTeleportOverlays" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_teleport_overlays = *__b; true } else { false },
                "showWhenInAir" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_when_in_air = *__b; true } else { false },
                "highlightType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match HighlightType::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.highlight_type = __c; true } None => false } } else { false },
                "enableWeirdTransmission" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_weird_transmission = *__b; true } else { false },
                "enableInstantTransmission" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_instant_transmission = *__b; true } else { false },
                "enableEtherTransmission" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_ether_transmission = *__b; true } else { false },
                "enableSinrecallTransmission" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_sinrecall_transmission = *__b; true } else { false },
                "enableWitherImpact" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_wither_impact = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `TeleportOverlay.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<TeleportOverlay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$TeleportOverlay",
            new: TeleportOverlay::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: TeleportOverlay::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// HighlightType's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl HighlightType {
    pub fn __class_of() -> dandelion::compat::ClassOf<HighlightType> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$TeleportOverlay$HighlightType",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| HighlightType::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || HighlightType::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// SmoothAOTE's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SmoothAOTE {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("predictive".to_string(), dandelion::compat::GsonValue::Bool(*(&self.predictive))),
            ("thirdPerson".to_string(), dandelion::compat::GsonValue::Bool(*(&self.third_person))),
            ("enableWeirdTransmission".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_weird_transmission))),
            ("enableInstantTransmission".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_instant_transmission))),
            ("enableEtherTransmission".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_ether_transmission))),
            ("enableSinrecallTransmission".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_sinrecall_transmission))),
            ("enableWitherImpact".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_wither_impact))),
            ("maximumAddedLag".to_string(), dandelion::compat::GsonValue::int(*(&self.maximum_added_lag) as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SmoothAOTE> {
        let __o = __v.as_object()?;
        let mut __this = SmoothAOTE::new();
        if let Some(__v) = __o.get("predictive") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.predictive = __x; }
        }
        if let Some(__v) = __o.get("thirdPerson") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.third_person = __x; }
        }
        if let Some(__v) = __o.get("enableWeirdTransmission") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_weird_transmission = __x; }
        }
        if let Some(__v) = __o.get("enableInstantTransmission") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_instant_transmission = __x; }
        }
        if let Some(__v) = __o.get("enableEtherTransmission") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_ether_transmission = __x; }
        }
        if let Some(__v) = __o.get("enableSinrecallTransmission") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_sinrecall_transmission = __x; }
        }
        if let Some(__v) = __o.get("enableWitherImpact") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_wither_impact = __x; }
        }
        if let Some(__v) = __o.get("maximumAddedLag") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.maximum_added_lag = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "predictive" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.predictive = *__b; true } else { false },
                "thirdPerson" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.third_person = *__b; true } else { false },
                "enableWeirdTransmission" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_weird_transmission = *__b; true } else { false },
                "enableInstantTransmission" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_instant_transmission = *__b; true } else { false },
                "enableEtherTransmission" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_ether_transmission = *__b; true } else { false },
                "enableSinrecallTransmission" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_sinrecall_transmission = *__b; true } else { false },
                "enableWitherImpact" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_wither_impact = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SmoothAOTE.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SmoothAOTE> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$SmoothAOTE",
            new: SmoothAOTE::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SmoothAOTE::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SearchOverlay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SearchOverlay {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableBazaar".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_bazaar))),
            ("enableAuctionHouse".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_auction_house))),
            ("enableIronmanAuctionHouse".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_ironman_auction_house))),
            ("enableMuseum".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_museum))),
            ("keepPreviousSearches".to_string(), dandelion::compat::GsonValue::Bool(*(&self.keep_previous_searches))),
            ("maxSuggestions".to_string(), dandelion::compat::GsonValue::int(*(&self.max_suggestions) as i64)),
            ("historyLength".to_string(), dandelion::compat::GsonValue::int(*(&self.history_length) as i64)),
            ("enableCommands".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_commands))),
            ("commandAutocomplete".to_string(), dandelion::compat::GsonValue::Bool(*(&self.command_autocomplete))),
            ("bazaarHistory".to_string(), dandelion::compat::GsonValue::Arr(((&self.bazaar_history)).iter().map(|__x0| dandelion::compat::GsonValue::str(&__x0)).collect())),
            ("auctionHistory".to_string(), dandelion::compat::GsonValue::Arr(((&self.auction_history)).iter().map(|__x0| dandelion::compat::GsonValue::str(&__x0)).collect())),
            ("museumHistory".to_string(), dandelion::compat::GsonValue::Arr(((&self.museum_history)).iter().map(|__x0| dandelion::compat::GsonValue::str(&__x0)).collect())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SearchOverlay> {
        let __o = __v.as_object()?;
        let mut __this = SearchOverlay::new();
        if let Some(__v) = __o.get("enableBazaar") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_bazaar = __x; }
        }
        if let Some(__v) = __o.get("enableAuctionHouse") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_auction_house = __x; }
        }
        if let Some(__v) = __o.get("enableIronmanAuctionHouse") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_ironman_auction_house = __x; }
        }
        if let Some(__v) = __o.get("enableMuseum") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_museum = __x; }
        }
        if let Some(__v) = __o.get("keepPreviousSearches") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.keep_previous_searches = __x; }
        }
        if let Some(__v) = __o.get("maxSuggestions") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.max_suggestions = __x; }
        }
        if let Some(__v) = __o.get("historyLength") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.history_length = __x; }
        }
        if let Some(__v) = __o.get("enableCommands") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_commands = __x; }
        }
        if let Some(__v) = __o.get("commandAutocomplete") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.command_autocomplete = __x; }
        }
        if let Some(__v) = __o.get("bazaarHistory") {
            if let Some(__x) = (__v).as_array().map(|__a| __a.iter().filter_map(|__y0| dandelion::compat::gson_string(__y0)).collect()) { __this.bazaar_history = __x; }
        }
        if let Some(__v) = __o.get("auctionHistory") {
            if let Some(__x) = (__v).as_array().map(|__a| __a.iter().filter_map(|__y0| dandelion::compat::gson_string(__y0)).collect()) { __this.auction_history = __x; }
        }
        if let Some(__v) = __o.get("museumHistory") {
            if let Some(__x) = (__v).as_array().map(|__a| __a.iter().filter_map(|__y0| dandelion::compat::gson_string(__y0)).collect()) { __this.museum_history = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableBazaar" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_bazaar = *__b; true } else { false },
                "enableAuctionHouse" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_auction_house = *__b; true } else { false },
                "enableIronmanAuctionHouse" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_ironman_auction_house = *__b; true } else { false },
                "enableMuseum" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_museum = *__b; true } else { false },
                "keepPreviousSearches" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.keep_previous_searches = *__b; true } else { false },
                "enableCommands" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_commands = *__b; true } else { false },
                "commandAutocomplete" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.command_autocomplete = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SearchOverlay.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SearchOverlay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$SearchOverlay",
            new: SearchOverlay::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SearchOverlay::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// BazaarQuickQuantities's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl BazaarQuickQuantities {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("closeSignOnUse".to_string(), dandelion::compat::GsonValue::Bool(*(&self.close_sign_on_use))),
            ("slot1Quantity".to_string(), dandelion::compat::GsonValue::int(*(&self.slot1_quantity) as i64)),
            ("slot2Quantity".to_string(), dandelion::compat::GsonValue::int(*(&self.slot2_quantity) as i64)),
            ("slot3Quantity".to_string(), dandelion::compat::GsonValue::int(*(&self.slot3_quantity) as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<BazaarQuickQuantities> {
        let __o = __v.as_object()?;
        let mut __this = BazaarQuickQuantities::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("closeSignOnUse") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.close_sign_on_use = __x; }
        }
        if let Some(__v) = __o.get("slot1Quantity") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.slot1_quantity = __x; }
        }
        if let Some(__v) = __o.get("slot2Quantity") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.slot2_quantity = __x; }
        }
        if let Some(__v) = __o.get("slot3Quantity") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.slot3_quantity = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "closeSignOnUse" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.close_sign_on_use = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `BazaarQuickQuantities.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<BazaarQuickQuantities> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$BazaarQuickQuantities",
            new: BazaarQuickQuantities::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: BazaarQuickQuantities::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// InputCalculator's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl InputCalculator {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("requiresEquals".to_string(), dandelion::compat::GsonValue::Bool(*(&self.requires_equals))),
            ("closeSignsWithEnter".to_string(), dandelion::compat::GsonValue::Bool(*(&self.close_signs_with_enter))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<InputCalculator> {
        let __o = __v.as_object()?;
        let mut __this = InputCalculator::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("requiresEquals") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.requires_equals = __x; }
        }
        if let Some(__v) = __o.get("closeSignsWithEnter") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.close_signs_with_enter = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "requiresEquals" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.requires_equals = *__b; true } else { false },
                "closeSignsWithEnter" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.close_signs_with_enter = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `InputCalculator.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<InputCalculator> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$InputCalculator",
            new: InputCalculator::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: InputCalculator::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// FlameOverlay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl FlameOverlay {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("flameHeight".to_string(), dandelion::compat::GsonValue::int(*(&self.flame_height) as i64)),
            ("flameOpacity".to_string(), dandelion::compat::GsonValue::int(*(&self.flame_opacity) as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<FlameOverlay> {
        let __o = __v.as_object()?;
        let mut __this = FlameOverlay::new();
        if let Some(__v) = __o.get("flameHeight") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.flame_height = __x; }
        }
        if let Some(__v) = __o.get("flameOpacity") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.flame_opacity = __x; }
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
                _ => false,
            },
            [] => false,
        }
    }

    /// `FlameOverlay.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<FlameOverlay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$FlameOverlay",
            new: FlameOverlay::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: FlameOverlay::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// CompactDamage's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CompactDamage {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("maxPrecision".to_string(), dandelion::compat::GsonValue::int(*(&self.max_precision) as i64)),
            ("normalDamageColor".to_string(), dandelion::compat::GsonValue::int(((&self.normal_damage_color)).get_rgb() as i64)),
            ("critDamageGradientStart".to_string(), dandelion::compat::GsonValue::int(((&self.crit_damage_gradient_start)).get_rgb() as i64)),
            ("critDamageGradientEnd".to_string(), dandelion::compat::GsonValue::int(((&self.crit_damage_gradient_end)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CompactDamage> {
        let __o = __v.as_object()?;
        let mut __this = CompactDamage::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("maxPrecision") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.max_precision = __x; }
        }
        if let Some(__v) = __o.get("normalDamageColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.normal_damage_color = __x; }
        }
        if let Some(__v) = __o.get("critDamageGradientStart") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.crit_damage_gradient_start = __x; }
        }
        if let Some(__v) = __o.get("critDamageGradientEnd") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.crit_damage_gradient_end = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `CompactDamage.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CompactDamage> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$CompactDamage",
            new: CompactDamage::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CompactDamage::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// HealthBars's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl HealthBars {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enabled".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enabled))),
            ("scale".to_string(), dandelion::compat::GsonValue::float(*(&self.scale))),
            ("removeHealthFromName".to_string(), dandelion::compat::GsonValue::Bool(*(&self.remove_health_from_name))),
            ("removeMaxHealthFromName".to_string(), dandelion::compat::GsonValue::Bool(*(&self.remove_max_health_from_name))),
            ("applyToHealthOnlyMobs".to_string(), dandelion::compat::GsonValue::Bool(*(&self.apply_to_health_only_mobs))),
            ("hideFullHealth".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_full_health))),
            ("fullBarColor".to_string(), dandelion::compat::GsonValue::int(((&self.full_bar_color)).get_rgb() as i64)),
            ("halfBarColor".to_string(), dandelion::compat::GsonValue::int(((&self.half_bar_color)).get_rgb() as i64)),
            ("emptyBarColor".to_string(), dandelion::compat::GsonValue::int(((&self.empty_bar_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<HealthBars> {
        let __o = __v.as_object()?;
        let mut __this = HealthBars::new();
        if let Some(__v) = __o.get("enabled") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enabled = __x; }
        }
        if let Some(__v) = __o.get("scale") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.scale = __x; }
        }
        if let Some(__v) = __o.get("removeHealthFromName") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.remove_health_from_name = __x; }
        }
        if let Some(__v) = __o.get("removeMaxHealthFromName") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.remove_max_health_from_name = __x; }
        }
        if let Some(__v) = __o.get("applyToHealthOnlyMobs") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.apply_to_health_only_mobs = __x; }
        }
        if let Some(__v) = __o.get("hideFullHealth") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_full_health = __x; }
        }
        if let Some(__v) = __o.get("fullBarColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.full_bar_color = __x; }
        }
        if let Some(__v) = __o.get("halfBarColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.half_bar_color = __x; }
        }
        if let Some(__v) = __o.get("emptyBarColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.empty_bar_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enabled" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enabled = *__b; true } else { false },
                "removeHealthFromName" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.remove_health_from_name = *__b; true } else { false },
                "removeMaxHealthFromName" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.remove_max_health_from_name = *__b; true } else { false },
                "applyToHealthOnlyMobs" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.apply_to_health_only_mobs = *__b; true } else { false },
                "hideFullHealth" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_full_health = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `HealthBars.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<HealthBars> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$HealthBars",
            new: HealthBars::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: HealthBars::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// GyrokineticWandOverlay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl GyrokineticWandOverlay {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("gyroOverlayMode".to_string(), dandelion::compat::GsonValue::str(((&self.gyro_overlay_mode)).name())),
            ("gyroOverlayColor".to_string(), dandelion::compat::GsonValue::int(((&self.gyro_overlay_color)).get_rgb() as i64)),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<GyrokineticWandOverlay> {
        let __o = __v.as_object()?;
        let mut __this = GyrokineticWandOverlay::new();
        if let Some(__v) = __o.get("gyroOverlayMode") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Mode::values().into_iter().find(|__c| __c.name() == __s)) { __this.gyro_overlay_mode = __x; }
        }
        if let Some(__v) = __o.get("gyroOverlayColor") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| Color::from_rgba(__n as i32, true)) { __this.gyro_overlay_color = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "gyroOverlayMode" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Mode::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.gyro_overlay_mode = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `GyrokineticWandOverlay.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<GyrokineticWandOverlay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.UIAndVisualsConfig$GyrokineticWandOverlay",
            new: GyrokineticWandOverlay::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: GyrokineticWandOverlay::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
