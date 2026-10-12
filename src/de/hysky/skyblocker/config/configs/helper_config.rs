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
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.helpers.fishing.fishingHookDisplay." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for FishingHookDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", FishingHookDisplay::to_string(*self))
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
