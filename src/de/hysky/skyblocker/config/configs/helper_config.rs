//! Ported from de.hysky.skyblocker.config.configs.HelperConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;

/// java: public class HelperConfig
#[derive(Clone)]
pub struct HelperConfig {
    /// java: public boolean enableNewYearCakesHelper = true;
    pub enable_new_year_cakes_helper: bool,
    /// java: public boolean enableBitsTooltip = true;
    pub enable_bits_tooltip: bool,
    /// java: public boolean enableWardrobeHelper = true;
    pub enable_wardrobe_helper: bool,
    /// java: public boolean enableDateCalculator = true;
    pub enable_date_calculator: bool,
    /// java: public boolean enableCopyUnderbidPrice = false;
    pub enable_copy_underbid_price: bool,
    /// java: public boolean enableAnvilHelper = true;
    pub enable_anvil_helper: bool,
    /// java: public boolean enableAccessoriesHelperWidget = true;
    pub enable_accessories_helper_widget: bool,
    /// java: public BuildersWand buildersWand = new BuildersWand();
    pub builders_wand: BuildersWand,
    /// java: public MythologicalRitual mythologicalRitual = new MythologicalRitual();
    pub mythological_ritual: MythologicalRitual,
    /// java: public Jerry jerry = new Jerry();
    pub jerry: Jerry,
    /// java: public Experiments experiments = new Experiments();
    pub experiments: Experiments,
    /// java: public Fishing fishing = new Fishing();
    pub fishing: Fishing,
    /// java: public FairySouls fairySouls = new FairySouls();
    pub fairy_souls: FairySouls,
    /// java: public ChocolateFactory chocolateFactory = new ChocolateFactory();
    pub chocolate_factory: ChocolateFactory,
    /// java: public Carnival carnival = new Carnival();
    pub carnival: Carnival,
    /// java: public Bazaar bazaar = new Bazaar();
    pub bazaar: Bazaar,
    /// java: public ItemPrice itemPrice = new ItemPrice();
    pub item_price: ItemPrice,
    /// java: public GreatSpookEvent greatSpookEvent = new GreatSpookEvent();
    pub great_spook_event: GreatSpookEvent,
    /// java: public CenturyRaffle centuryRaffle = new CenturyRaffle();
    pub century_raffle: CenturyRaffle,
}

impl HelperConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_new_year_cakes_helper: true,
            enable_bits_tooltip: true,
            enable_wardrobe_helper: true,
            enable_date_calculator: true,
            enable_copy_underbid_price: false,
            enable_anvil_helper: true,
            enable_accessories_helper_widget: true,
            builders_wand: BuildersWand::new(),
            mythological_ritual: MythologicalRitual::new(),
            jerry: Jerry::new(),
            experiments: Experiments::new(),
            fishing: Fishing::new(),
            fairy_souls: FairySouls::new(),
            chocolate_factory: ChocolateFactory::new(),
            carnival: Carnival::new(),
            bazaar: Bazaar::new(),
            item_price: ItemPrice::new(),
            great_spook_event: GreatSpookEvent::new(),
            century_raffle: CenturyRaffle::new(),
        };
        this
    }

}

/// java: public static class BuildersWand
#[derive(Clone)]
pub struct BuildersWand {
    /// java: public boolean enableBuildersWandPreview = true;
    pub enable_builders_wand_preview: bool,
    /// java: public float previewOpacity = 0.5f;
    pub preview_opacity: f32,
}

impl BuildersWand {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_builders_wand_preview: true,
            preview_opacity: 0.5f32,
        };
        this
    }

}

/// java: public static class MythologicalRitual
#[derive(Clone)]
pub struct MythologicalRitual {
    /// java: public boolean enableMythologicalRitualHelper = true;
    pub enable_mythological_ritual_helper: bool,
}

impl MythologicalRitual {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_mythological_ritual_helper: true,
        };
        this
    }

}

/// java: public static class Jerry
#[derive(Clone)]
pub struct Jerry {
    /// java: public boolean enableJerryTimer = false;
    pub enable_jerry_timer: bool,
}

impl Jerry {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_jerry_timer: false,
        };
        this
    }

}

/// java: public static class Experiments
#[derive(Clone)]
pub struct Experiments {
    /// java: public boolean enableChronomatronSolver = true;
    pub enable_chronomatron_solver: bool,
    /// java: public boolean enableSuperpairsSolver = true;
    pub enable_superpairs_solver: bool,
    /// java: public boolean enableUltrasequencerSolver = true;
    pub enable_ultrasequencer_solver: bool,
    /// java: public boolean blockIncorrectClicks = false;
    pub block_incorrect_clicks: bool,
}

impl Experiments {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_chronomatron_solver: true,
            enable_superpairs_solver: true,
            enable_ultrasequencer_solver: true,
            block_incorrect_clicks: false,
        };
        this
    }

}

/// java: public static class Fishing
#[derive(Clone)]
pub struct Fishing {
    /// java: public boolean enableFishingHelper = true;
    pub enable_fishing_helper: bool,
    /// java: @Deprecated public transient boolean enableFishingHookDisplay = true;
    pub enable_fishing_hook_display: bool,
    /// java: @Deprecated public transient boolean enableFishingTimer = false;
    pub enable_fishing_timer: bool,
    /// java: @Deprecated public transient boolean changeTimerColor = true;
    pub change_timer_color: bool,
    /// java: @Deprecated public transient float fishingTimerScale = 1f;
    pub fishing_timer_scale: f32,
    /// java: public boolean hideOtherPlayersRods = false;
    pub hide_other_players_rods: bool,
    /// java: @Deprecated public transient boolean enableFishingHud = true;
    pub enable_fishing_hud: bool,
    /// java: @Deprecated public transient List<Location> fishingHudEnabledLocations = new ArrayList<>(List.of(Location.values()));
    pub fishing_hud_enabled_locations: Vec<Location>,
    /// java: @Deprecated public transient boolean enableSeaCreatureCounter = true;
    pub enable_sea_creature_counter: bool,
    /// java: public FishingHookDisplay fishingHookDisplay = FishingHookDisplay.HUD;
    pub fishing_hook_display: FishingHookDisplay,
    /// java: @Deprecated public transient boolean onlyShowHudInBarn = true;
    pub only_show_hud_in_barn: bool,
    /// java: public int timerLength = 340;
    pub timer_length: i32,
    /// java: public boolean seaCreatureTimerNotification = true;
    pub sea_creature_timer_notification: bool,
    /// java: @Deprecated public transient int seaCreatureCap;
    pub sea_creature_cap: i32,
    /// java: public boolean seaCreatureCapNotification = true;
    pub sea_creature_cap_notification: bool,
    /// java: public SkyblockItemRarity minimumNotificationRarity = SkyblockItemRarity.EPIC;
    pub minimum_notification_rarity: SkyblockItemRarity,
}

impl Fishing {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_fishing_helper: true,
            enable_fishing_hook_display: true,
            enable_fishing_timer: false,
            change_timer_color: true,
            fishing_timer_scale: 1.0f32,
            hide_other_players_rods: false,
            enable_fishing_hud: true,
            fishing_hud_enabled_locations: Location::values().to_vec().into_iter().collect::<Vec<_>>(),
            enable_sea_creature_counter: true,
            fishing_hook_display: FishingHookDisplay::Hud,
            only_show_hud_in_barn: true,
            timer_length: 340,
            sea_creature_timer_notification: true,
            sea_creature_cap: 0,
            sea_creature_cap_notification: true,
            minimum_notification_rarity: SkyblockItemRarity::Epic,
        };
        this
    }

}

/// java: public enum FishingHookDisplay
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FishingHookDisplay {
    Off,
    Crosshair,
    Hud,
}

impl FishingHookDisplay {
    pub const VALUES: [FishingHookDisplay; 3] = [FishingHookDisplay::Off, FishingHookDisplay::Crosshair, FishingHookDisplay::Hud];

    /// java: values()
    pub fn values() -> Vec<FishingHookDisplay> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            FishingHookDisplay::Off => "OFF",
            FishingHookDisplay::Crosshair => "CROSSHAIR",
            FishingHookDisplay::Hud => "HUD",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> FishingHookDisplay {
        match name {
            "OFF" => FishingHookDisplay::Off,
            "CROSSHAIR" => FishingHookDisplay::Crosshair,
            "HUD" => FishingHookDisplay::Hud,
            _ => panic!("IllegalArgumentException: No enum constant FishingHookDisplay.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.helpers.fishing.fishingHookDisplay.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for FishingHookDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", FishingHookDisplay::to_string(self))
    }
}

/// java: public static class Loadouts
#[derive(Clone)]
pub struct Loadouts {}

impl Loadouts {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: public static class FairySouls
#[derive(Clone)]
pub struct FairySouls {
    /// java: public boolean enableFairySoulsHelper = false;
    pub enable_fairy_souls_helper: bool,
    /// java: public boolean highlightFoundSouls = true;
    pub highlight_found_souls: bool,
    /// java: public boolean highlightOnBingo = true;
    pub highlight_on_bingo: bool,
    /// java: public boolean highlightOnlyNearbySouls = false;
    pub highlight_only_nearby_souls: bool,
}

impl FairySouls {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_fairy_souls_helper: false,
            highlight_found_souls: true,
            highlight_on_bingo: true,
            highlight_only_nearby_souls: false,
        };
        this
    }

}

/// java: public static class ChocolateFactory
#[derive(Clone)]
pub struct ChocolateFactory {
    /// java: public boolean enableChocolateFactoryHelper = true;
    pub enable_chocolate_factory_helper: bool,
    /// java: public boolean enableTimeTowerReminder = true;
    pub enable_time_tower_reminder: bool,
    /// java: public boolean straySound = true;
    pub stray_sound: bool,
    /// java: public boolean enableEggFinder = true;
    pub enable_egg_finder: bool,
    /// java: public boolean sendEggFoundMessages = true;
    pub send_egg_found_messages: bool,
    /// java: public Waypoint.Type waypointType = Waypoint.Type.WAYPOINT;
    pub waypoint_type: Type,
    /// java: public boolean showThroughWalls = false;
    pub show_through_walls: bool,
}

impl ChocolateFactory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_chocolate_factory_helper: true,
            enable_time_tower_reminder: true,
            stray_sound: true,
            enable_egg_finder: true,
            send_egg_found_messages: true,
            waypoint_type: Type::Waypoint,
            show_through_walls: false,
        };
        this
    }

}

/// java: public static class Carnival
#[derive(Clone)]
pub struct Carnival {
    /// java: public boolean catchAFishHelper = true;
    pub catch_a_fish_helper: bool,
    /// java: public boolean zombieShootoutHelper = true;
    pub zombie_shootout_helper: bool,
}

impl Carnival {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            catch_a_fish_helper: true,
            zombie_shootout_helper: true,
        };
        this
    }

}

/// java: public static class Bazaar
#[derive(Clone)]
pub struct Bazaar {
    /// java: public boolean enableBazaarHelper = true;
    pub enable_bazaar_helper: bool,
    /// java: public boolean enableReorderHelper = true;
    pub enable_reorder_helper: bool,
    /// java: public boolean enableOrderTracker = true;
    pub enable_order_tracker: bool,
    /// java: public boolean enableBazaarMax = true;
    pub enable_bazaar_max: bool,
}

impl Bazaar {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_bazaar_helper: true,
            enable_reorder_helper: true,
            enable_order_tracker: true,
            enable_bazaar_max: true,
        };
        this
    }

}

/// java: public static class ItemPrice
#[derive(Clone)]
pub struct ItemPrice {
    /// java: public boolean enableItemPriceLookup = true;
    pub enable_item_price_lookup: bool,
    /// java: @Deprecated public transient boolean enableItemPriceRefresh;
    pub enable_item_price_refresh: bool,
}

impl ItemPrice {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_item_price_lookup: true,
            enable_item_price_refresh: false,
        };
        this
    }

}

/// java: public static class GreatSpookEvent
#[derive(Clone)]
pub struct GreatSpookEvent {
    /// java: public boolean enableMathTeacherHelper = true;
    pub enable_math_teacher_helper: bool,
}

impl GreatSpookEvent {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_math_teacher_helper: true,
        };
        this
    }

}

/// java: public static class CenturyRaffle
#[derive(Clone)]
pub struct CenturyRaffle {
    /// java: public boolean enableRaffleTaskHighlight = true;
    pub enable_raffle_task_highlight: bool,
    /// java: public boolean enableRaffleRewardHighlight = true;
    pub enable_raffle_reward_highlight: bool,
}

impl CenturyRaffle {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_raffle_task_highlight: true,
            enable_raffle_reward_highlight: true,
        };
        this
    }

}

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/helper_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// HelperConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl HelperConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableNewYearCakesHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_new_year_cakes_helper))),
            ("enableBitsTooltip".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_bits_tooltip))),
            ("enableWardrobeHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_wardrobe_helper))),
            ("enableDateCalculator".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_date_calculator))),
            ("enableCopyUnderbidPrice".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_copy_underbid_price))),
            ("enableAnvilHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_anvil_helper))),
            ("enableAccessoriesHelperWidget".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_accessories_helper_widget))),
            ("buildersWand".to_string(), ((&self.builders_wand)).__gson_write()),
            ("mythologicalRitual".to_string(), ((&self.mythological_ritual)).__gson_write()),
            ("jerry".to_string(), ((&self.jerry)).__gson_write()),
            ("experiments".to_string(), ((&self.experiments)).__gson_write()),
            ("fishing".to_string(), ((&self.fishing)).__gson_write()),
            ("fairySouls".to_string(), ((&self.fairy_souls)).__gson_write()),
            ("chocolateFactory".to_string(), ((&self.chocolate_factory)).__gson_write()),
            ("carnival".to_string(), ((&self.carnival)).__gson_write()),
            ("bazaar".to_string(), ((&self.bazaar)).__gson_write()),
            ("itemPrice".to_string(), ((&self.item_price)).__gson_write()),
            ("greatSpookEvent".to_string(), ((&self.great_spook_event)).__gson_write()),
            ("centuryRaffle".to_string(), ((&self.century_raffle)).__gson_write()),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<HelperConfig> {
        let __o = __v.as_object()?;
        let mut __this = HelperConfig::new();
        if let Some(__v) = __o.get("enableNewYearCakesHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_new_year_cakes_helper = __x; }
        }
        if let Some(__v) = __o.get("enableBitsTooltip") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_bits_tooltip = __x; }
        }
        if let Some(__v) = __o.get("enableWardrobeHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_wardrobe_helper = __x; }
        }
        if let Some(__v) = __o.get("enableDateCalculator") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_date_calculator = __x; }
        }
        if let Some(__v) = __o.get("enableCopyUnderbidPrice") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_copy_underbid_price = __x; }
        }
        if let Some(__v) = __o.get("enableAnvilHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_anvil_helper = __x; }
        }
        if let Some(__v) = __o.get("enableAccessoriesHelperWidget") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_accessories_helper_widget = __x; }
        }
        if let Some(__v) = __o.get("buildersWand") {
            if let Some(__x) = BuildersWand::__gson_read(__v) { __this.builders_wand = __x; }
        }
        if let Some(__v) = __o.get("mythologicalRitual") {
            if let Some(__x) = MythologicalRitual::__gson_read(__v) { __this.mythological_ritual = __x; }
        }
        if let Some(__v) = __o.get("jerry") {
            if let Some(__x) = Jerry::__gson_read(__v) { __this.jerry = __x; }
        }
        if let Some(__v) = __o.get("experiments") {
            if let Some(__x) = Experiments::__gson_read(__v) { __this.experiments = __x; }
        }
        if let Some(__v) = __o.get("fishing") {
            if let Some(__x) = Fishing::__gson_read(__v) { __this.fishing = __x; }
        }
        if let Some(__v) = __o.get("fairySouls") {
            if let Some(__x) = FairySouls::__gson_read(__v) { __this.fairy_souls = __x; }
        }
        if let Some(__v) = __o.get("chocolateFactory") {
            if let Some(__x) = ChocolateFactory::__gson_read(__v) { __this.chocolate_factory = __x; }
        }
        if let Some(__v) = __o.get("carnival") {
            if let Some(__x) = Carnival::__gson_read(__v) { __this.carnival = __x; }
        }
        if let Some(__v) = __o.get("bazaar") {
            if let Some(__x) = Bazaar::__gson_read(__v) { __this.bazaar = __x; }
        }
        if let Some(__v) = __o.get("itemPrice") {
            if let Some(__x) = ItemPrice::__gson_read(__v) { __this.item_price = __x; }
        }
        if let Some(__v) = __o.get("greatSpookEvent") {
            if let Some(__x) = GreatSpookEvent::__gson_read(__v) { __this.great_spook_event = __x; }
        }
        if let Some(__v) = __o.get("centuryRaffle") {
            if let Some(__x) = CenturyRaffle::__gson_read(__v) { __this.century_raffle = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableNewYearCakesHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_new_year_cakes_helper = *__b; true } else { false },
                "enableBitsTooltip" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_bits_tooltip = *__b; true } else { false },
                "enableWardrobeHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_wardrobe_helper = *__b; true } else { false },
                "enableDateCalculator" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_date_calculator = *__b; true } else { false },
                "enableCopyUnderbidPrice" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_copy_underbid_price = *__b; true } else { false },
                "enableAnvilHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_anvil_helper = *__b; true } else { false },
                "enableAccessoriesHelperWidget" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_accessories_helper_widget = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "buildersWand" => self.builders_wand.__gson_patch(__rest, __value),
                "mythologicalRitual" => self.mythological_ritual.__gson_patch(__rest, __value),
                "jerry" => self.jerry.__gson_patch(__rest, __value),
                "experiments" => self.experiments.__gson_patch(__rest, __value),
                "fishing" => self.fishing.__gson_patch(__rest, __value),
                "fairySouls" => self.fairy_souls.__gson_patch(__rest, __value),
                "chocolateFactory" => self.chocolate_factory.__gson_patch(__rest, __value),
                "carnival" => self.carnival.__gson_patch(__rest, __value),
                "bazaar" => self.bazaar.__gson_patch(__rest, __value),
                "itemPrice" => self.item_price.__gson_patch(__rest, __value),
                "greatSpookEvent" => self.great_spook_event.__gson_patch(__rest, __value),
                "centuryRaffle" => self.century_raffle.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `HelperConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<HelperConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig",
            new: HelperConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: HelperConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// BuildersWand's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl BuildersWand {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableBuildersWandPreview".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_builders_wand_preview))),
            ("previewOpacity".to_string(), dandelion::compat::GsonValue::float(*(&self.preview_opacity))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<BuildersWand> {
        let __o = __v.as_object()?;
        let mut __this = BuildersWand::new();
        if let Some(__v) = __o.get("enableBuildersWandPreview") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_builders_wand_preview = __x; }
        }
        if let Some(__v) = __o.get("previewOpacity") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.preview_opacity = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableBuildersWandPreview" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_builders_wand_preview = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `BuildersWand.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<BuildersWand> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$BuildersWand",
            new: BuildersWand::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: BuildersWand::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// MythologicalRitual's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl MythologicalRitual {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableMythologicalRitualHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_mythological_ritual_helper))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<MythologicalRitual> {
        let __o = __v.as_object()?;
        let mut __this = MythologicalRitual::new();
        if let Some(__v) = __o.get("enableMythologicalRitualHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_mythological_ritual_helper = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableMythologicalRitualHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_mythological_ritual_helper = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `MythologicalRitual.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<MythologicalRitual> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$MythologicalRitual",
            new: MythologicalRitual::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: MythologicalRitual::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Jerry's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Jerry {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableJerryTimer".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_jerry_timer))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Jerry> {
        let __o = __v.as_object()?;
        let mut __this = Jerry::new();
        if let Some(__v) = __o.get("enableJerryTimer") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_jerry_timer = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableJerryTimer" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_jerry_timer = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Jerry.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Jerry> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$Jerry",
            new: Jerry::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Jerry::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Experiments's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Experiments {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableChronomatronSolver".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_chronomatron_solver))),
            ("enableSuperpairsSolver".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_superpairs_solver))),
            ("enableUltrasequencerSolver".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_ultrasequencer_solver))),
            ("blockIncorrectClicks".to_string(), dandelion::compat::GsonValue::Bool(*(&self.block_incorrect_clicks))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Experiments> {
        let __o = __v.as_object()?;
        let mut __this = Experiments::new();
        if let Some(__v) = __o.get("enableChronomatronSolver") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_chronomatron_solver = __x; }
        }
        if let Some(__v) = __o.get("enableSuperpairsSolver") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_superpairs_solver = __x; }
        }
        if let Some(__v) = __o.get("enableUltrasequencerSolver") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_ultrasequencer_solver = __x; }
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
                "enableChronomatronSolver" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_chronomatron_solver = *__b; true } else { false },
                "enableSuperpairsSolver" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_superpairs_solver = *__b; true } else { false },
                "enableUltrasequencerSolver" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_ultrasequencer_solver = *__b; true } else { false },
                "blockIncorrectClicks" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.block_incorrect_clicks = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Experiments.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Experiments> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$Experiments",
            new: Experiments::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Experiments::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Fishing's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Fishing {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableFishingHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_fishing_helper))),
            ("hideOtherPlayersRods".to_string(), dandelion::compat::GsonValue::Bool(*(&self.hide_other_players_rods))),
            ("fishingHookDisplay".to_string(), dandelion::compat::GsonValue::str(((&self.fishing_hook_display)).name())),
            ("timerLength".to_string(), dandelion::compat::GsonValue::int(*(&self.timer_length) as i64)),
            ("seaCreatureTimerNotification".to_string(), dandelion::compat::GsonValue::Bool(*(&self.sea_creature_timer_notification))),
            ("seaCreatureCapNotification".to_string(), dandelion::compat::GsonValue::Bool(*(&self.sea_creature_cap_notification))),
            ("minimumNotificationRarity".to_string(), dandelion::compat::GsonValue::str(((&self.minimum_notification_rarity)).name())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Fishing> {
        let __o = __v.as_object()?;
        let mut __this = Fishing::new();
        if let Some(__v) = __o.get("enableFishingHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_fishing_helper = __x; }
        }
        if let Some(__v) = __o.get("hideOtherPlayersRods") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.hide_other_players_rods = __x; }
        }
        if let Some(__v) = __o.get("fishingHookDisplay") {
            if let Some(__x) = (__v).as_str().and_then(|__s| FishingHookDisplay::values().into_iter().find(|__c| __c.name() == __s)) { __this.fishing_hook_display = __x; }
        }
        if let Some(__v) = __o.get("timerLength") {
            if let Some(__x) = dandelion::compat::gson_i64(__v).map(|__n| __n as i32) { __this.timer_length = __x; }
        }
        if let Some(__v) = __o.get("seaCreatureTimerNotification") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.sea_creature_timer_notification = __x; }
        }
        if let Some(__v) = __o.get("seaCreatureCapNotification") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.sea_creature_cap_notification = __x; }
        }
        if let Some(__v) = __o.get("minimumNotificationRarity") {
            if let Some(__x) = (__v).as_str().and_then(|__s| SkyblockItemRarity::values().into_iter().find(|__c| __c.name() == __s)) { __this.minimum_notification_rarity = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableFishingHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_fishing_helper = *__b; true } else { false },
                "hideOtherPlayersRods" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.hide_other_players_rods = *__b; true } else { false },
                "fishingHookDisplay" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match FishingHookDisplay::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.fishing_hook_display = __c; true } None => false } } else { false },
                "seaCreatureTimerNotification" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.sea_creature_timer_notification = *__b; true } else { false },
                "seaCreatureCapNotification" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.sea_creature_cap_notification = *__b; true } else { false },
                "minimumNotificationRarity" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match SkyblockItemRarity::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.minimum_notification_rarity = __c; true } None => false } } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Fishing.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Fishing> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$Fishing",
            new: Fishing::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Fishing::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// FishingHookDisplay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl FishingHookDisplay {
    pub fn __class_of() -> dandelion::compat::ClassOf<FishingHookDisplay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$Fishing$FishingHookDisplay",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| FishingHookDisplay::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || FishingHookDisplay::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// FairySouls's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl FairySouls {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableFairySoulsHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_fairy_souls_helper))),
            ("highlightFoundSouls".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_found_souls))),
            ("highlightOnBingo".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_on_bingo))),
            ("highlightOnlyNearbySouls".to_string(), dandelion::compat::GsonValue::Bool(*(&self.highlight_only_nearby_souls))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<FairySouls> {
        let __o = __v.as_object()?;
        let mut __this = FairySouls::new();
        if let Some(__v) = __o.get("enableFairySoulsHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_fairy_souls_helper = __x; }
        }
        if let Some(__v) = __o.get("highlightFoundSouls") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_found_souls = __x; }
        }
        if let Some(__v) = __o.get("highlightOnBingo") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_on_bingo = __x; }
        }
        if let Some(__v) = __o.get("highlightOnlyNearbySouls") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.highlight_only_nearby_souls = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableFairySoulsHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_fairy_souls_helper = *__b; true } else { false },
                "highlightFoundSouls" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_found_souls = *__b; true } else { false },
                "highlightOnBingo" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_on_bingo = *__b; true } else { false },
                "highlightOnlyNearbySouls" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.highlight_only_nearby_souls = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `FairySouls.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<FairySouls> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$FairySouls",
            new: FairySouls::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: FairySouls::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ChocolateFactory's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ChocolateFactory {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableChocolateFactoryHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_chocolate_factory_helper))),
            ("enableTimeTowerReminder".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_time_tower_reminder))),
            ("straySound".to_string(), dandelion::compat::GsonValue::Bool(*(&self.stray_sound))),
            ("enableEggFinder".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_egg_finder))),
            ("sendEggFoundMessages".to_string(), dandelion::compat::GsonValue::Bool(*(&self.send_egg_found_messages))),
            ("waypointType".to_string(), dandelion::compat::GsonValue::str(((&self.waypoint_type)).name())),
            ("showThroughWalls".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_through_walls))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ChocolateFactory> {
        let __o = __v.as_object()?;
        let mut __this = ChocolateFactory::new();
        if let Some(__v) = __o.get("enableChocolateFactoryHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_chocolate_factory_helper = __x; }
        }
        if let Some(__v) = __o.get("enableTimeTowerReminder") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_time_tower_reminder = __x; }
        }
        if let Some(__v) = __o.get("straySound") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.stray_sound = __x; }
        }
        if let Some(__v) = __o.get("enableEggFinder") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_egg_finder = __x; }
        }
        if let Some(__v) = __o.get("sendEggFoundMessages") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.send_egg_found_messages = __x; }
        }
        if let Some(__v) = __o.get("waypointType") {
            if let Some(__x) = (__v).as_str().and_then(|__s| Type::values().into_iter().find(|__c| __c.name() == __s)) { __this.waypoint_type = __x; }
        }
        if let Some(__v) = __o.get("showThroughWalls") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_through_walls = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableChocolateFactoryHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_chocolate_factory_helper = *__b; true } else { false },
                "enableTimeTowerReminder" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_time_tower_reminder = *__b; true } else { false },
                "straySound" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.stray_sound = *__b; true } else { false },
                "enableEggFinder" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_egg_finder = *__b; true } else { false },
                "sendEggFoundMessages" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.send_egg_found_messages = *__b; true } else { false },
                "waypointType" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Type::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.waypoint_type = __c; true } None => false } } else { false },
                "showThroughWalls" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_through_walls = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ChocolateFactory.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ChocolateFactory> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$ChocolateFactory",
            new: ChocolateFactory::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ChocolateFactory::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Carnival's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Carnival {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("catchAFishHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.catch_a_fish_helper))),
            ("zombieShootoutHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.zombie_shootout_helper))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Carnival> {
        let __o = __v.as_object()?;
        let mut __this = Carnival::new();
        if let Some(__v) = __o.get("catchAFishHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.catch_a_fish_helper = __x; }
        }
        if let Some(__v) = __o.get("zombieShootoutHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.zombie_shootout_helper = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "catchAFishHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.catch_a_fish_helper = *__b; true } else { false },
                "zombieShootoutHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.zombie_shootout_helper = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Carnival.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Carnival> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$Carnival",
            new: Carnival::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Carnival::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Bazaar's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Bazaar {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableBazaarHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_bazaar_helper))),
            ("enableReorderHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_reorder_helper))),
            ("enableOrderTracker".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_order_tracker))),
            ("enableBazaarMax".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_bazaar_max))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Bazaar> {
        let __o = __v.as_object()?;
        let mut __this = Bazaar::new();
        if let Some(__v) = __o.get("enableBazaarHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_bazaar_helper = __x; }
        }
        if let Some(__v) = __o.get("enableReorderHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_reorder_helper = __x; }
        }
        if let Some(__v) = __o.get("enableOrderTracker") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_order_tracker = __x; }
        }
        if let Some(__v) = __o.get("enableBazaarMax") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_bazaar_max = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableBazaarHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_bazaar_helper = *__b; true } else { false },
                "enableReorderHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_reorder_helper = *__b; true } else { false },
                "enableOrderTracker" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_order_tracker = *__b; true } else { false },
                "enableBazaarMax" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_bazaar_max = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Bazaar.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Bazaar> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$Bazaar",
            new: Bazaar::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Bazaar::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ItemPrice's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ItemPrice {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableItemPriceLookup".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_item_price_lookup))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ItemPrice> {
        let __o = __v.as_object()?;
        let mut __this = ItemPrice::new();
        if let Some(__v) = __o.get("enableItemPriceLookup") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_item_price_lookup = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableItemPriceLookup" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_item_price_lookup = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ItemPrice.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ItemPrice> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$ItemPrice",
            new: ItemPrice::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ItemPrice::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// GreatSpookEvent's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl GreatSpookEvent {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableMathTeacherHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_math_teacher_helper))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<GreatSpookEvent> {
        let __o = __v.as_object()?;
        let mut __this = GreatSpookEvent::new();
        if let Some(__v) = __o.get("enableMathTeacherHelper") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_math_teacher_helper = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableMathTeacherHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_math_teacher_helper = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `GreatSpookEvent.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<GreatSpookEvent> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$GreatSpookEvent",
            new: GreatSpookEvent::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: GreatSpookEvent::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// CenturyRaffle's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl CenturyRaffle {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableRaffleTaskHighlight".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_raffle_task_highlight))),
            ("enableRaffleRewardHighlight".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_raffle_reward_highlight))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<CenturyRaffle> {
        let __o = __v.as_object()?;
        let mut __this = CenturyRaffle::new();
        if let Some(__v) = __o.get("enableRaffleTaskHighlight") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_raffle_task_highlight = __x; }
        }
        if let Some(__v) = __o.get("enableRaffleRewardHighlight") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_raffle_reward_highlight = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableRaffleTaskHighlight" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_raffle_task_highlight = *__b; true } else { false },
                "enableRaffleRewardHighlight" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_raffle_reward_highlight = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `CenturyRaffle.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<CenturyRaffle> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.HelperConfig$CenturyRaffle",
            new: CenturyRaffle::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: CenturyRaffle::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
