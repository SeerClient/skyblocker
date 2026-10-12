//! Ported from de.hysky.skyblocker.config.configs.GeneralConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::AnimatedDye;
use crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_trims::ArmorTrimId;
use crate::de::hysky::skyblocker::skyblock::item::slottext::slot_text_mode::SlotTextMode;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class GeneralConfig
#[derive(Clone)]
pub struct GeneralConfig {
    /// java: public boolean enableTips = true;
    pub enable_tips: bool,
    /// java: public boolean acceptReparty = true;
    pub accept_reparty: bool,
    /// java: public SpeedPresets speedPresets = new SpeedPresets();
    pub speed_presets: SpeedPresets,
    /// java: public Shortcuts shortcuts = new Shortcuts();
    pub shortcuts: Shortcuts,
    /// java: public QuiverWarning quiverWarning = new QuiverWarning();
    pub quiver_warning: QuiverWarning,
    /// java: public ItemList itemList = new ItemList();
    pub item_list: ItemList,
    /// java: public ItemTooltip itemTooltip = new ItemTooltip();
    pub item_tooltip: compat::Shared<ItemTooltip>,
    /// java: public ItemInfoDisplay itemInfoDisplay = new ItemInfoDisplay();
    pub item_info_display: ItemInfoDisplay,
    /// java: public ItemProtection itemProtection = new ItemProtection();
    pub item_protection: ItemProtection,
    /// java: public WikiLookup wikiLookup = new WikiLookup();
    pub wiki_lookup: WikiLookup,
    /// java: public SpecialEffects specialEffects = new SpecialEffects();
    pub special_effects: SpecialEffects,
    /// java: @Deprecated public transient Hitbox hitbox = new Hitbox();
    pub hitbox: Hitbox,
    /// java: public List<Integer> lockedSlots = new ArrayList<>();
    pub locked_slots: Vec<i32>,
    /// java: public ObjectOpenHashSet<String> protectedItems = new ObjectOpenHashSet<>();
    pub protected_items: std::collections::HashSet<String>,
    /// java: public Object2ObjectOpenHashMap<String, Component> customItemNames = new Object2ObjectOpenHashMap<>();
    pub custom_item_names: std::collections::HashMap<String, MutableComponent>,
    /// java: public Object2IntOpenHashMap<String> customDyeColors = new Object2IntOpenHashMap<>();
    pub custom_dye_colors: std::collections::HashMap<String, i32>,
    /// java: public Object2ObjectOpenHashMap<String, CustomArmorTrims.ArmorTrimId> customArmorTrims = new Object2ObjectOpenHashMap<>();
    pub custom_armor_trims: std::collections::HashMap<String, ArmorTrimId>,
    /// java: public Object2ObjectOpenHashMap<String, CustomArmorAnimatedDyes.AnimatedDye> customAnimatedDyes = new Object2ObjectOpenHashMap<>();
    pub custom_animated_dyes: std::collections::HashMap<String, AnimatedDye>,
    /// java: public Object2ObjectOpenHashMap<String, String> customHelmetTextures = new Object2ObjectOpenHashMap<>();
    pub custom_helmet_textures: std::collections::HashMap<String, String>,
    /// java: public Object2BooleanOpenHashMap<String> customGlint = new Object2BooleanOpenHashMap<>();
    pub custom_glint: std::collections::HashMap<String, bool>,
    /// java: public Object2ObjectOpenHashMap<String, Identifier> customItemModel = new Object2ObjectOpenHashMap<>();
    pub custom_item_model: std::collections::HashMap<String, Identifier>,
    /// java: public Object2ObjectOpenHashMap<String, Identifier> customArmorModel = new Object2ObjectOpenHashMap<>();
    pub custom_armor_model: std::collections::HashMap<String, Identifier>,
    /// java: public Object2ObjectOpenHashMap<String, String> customAnimatedHelmetTextures = new Object2ObjectOpenHashMap<>();
    pub custom_animated_helmet_textures: std::collections::HashMap<String, String>,
}

impl GeneralConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_tips: true,
            accept_reparty: true,
            speed_presets: SpeedPresets::new(),
            shortcuts: Shortcuts::new(),
            quiver_warning: QuiverWarning::new(),
            item_list: ItemList::new(),
            item_tooltip: ItemTooltip::new(),
            item_info_display: ItemInfoDisplay::new(),
            item_protection: ItemProtection::new(),
            wiki_lookup: WikiLookup::new(),
            special_effects: SpecialEffects::new(),
            hitbox: Hitbox::new(),
            locked_slots: Vec::new(),
            protected_items: std::collections::HashSet::new(),
            custom_item_names: std::collections::HashMap::new(),
            custom_dye_colors: std::collections::HashMap::new(),
            custom_armor_trims: std::collections::HashMap::new(),
            custom_animated_dyes: std::collections::HashMap::new(),
            custom_helmet_textures: std::collections::HashMap::new(),
            custom_glint: std::collections::HashMap::new(),
            custom_item_model: std::collections::HashMap::new(),
            custom_armor_model: std::collections::HashMap::new(),
            custom_animated_helmet_textures: std::collections::HashMap::new(),
        };
        this
    }

}

/// java: public static class SpeedPresets
#[derive(Clone)]
pub struct SpeedPresets {
    /// java: public boolean enableSpeedPresets = true;
    pub enable_speed_presets: bool,
}

impl SpeedPresets {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_speed_presets: true,
        };
        this
    }

}

/// java: public static class Shortcuts
#[derive(Clone)]
pub struct Shortcuts {
    /// java: public boolean enableShortcuts = true;
    pub enable_shortcuts: bool,
    /// java: public boolean enableCommandShortcuts = true;
    pub enable_command_shortcuts: bool,
    /// java: public boolean enableCommandArgShortcuts = true;
    pub enable_command_arg_shortcuts: bool,
    /// java: public boolean enableKeyBindingShortcuts = true;
    pub enable_key_binding_shortcuts: bool,
}

impl Shortcuts {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_shortcuts: true,
            enable_command_shortcuts: true,
            enable_command_arg_shortcuts: true,
            enable_key_binding_shortcuts: true,
        };
        this
    }

}

/// java: public static class QuiverWarning
#[derive(Clone)]
pub struct QuiverWarning {
    /// java: public boolean enableQuiverWarning = true;
    pub enable_quiver_warning: bool,
    /// java: public boolean enableQuiverWarningInDungeons = true;
    pub enable_quiver_warning_in_dungeons: bool,
    /// java: public boolean enableQuiverWarningAfterDungeon = true;
    pub enable_quiver_warning_after_dungeon: bool,
}

impl QuiverWarning {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_quiver_warning: true,
            enable_quiver_warning_in_dungeons: true,
            enable_quiver_warning_after_dungeon: true,
        };
        this
    }

}

/// java: public static class ItemList
#[derive(Clone)]
pub struct ItemList {
    /// java: public boolean enableRecipeBook = true;
    pub enable_recipe_book: bool,
    /// java: public boolean enableItemList = true;
    pub enable_item_list: bool,
    /// java: public boolean enableCollapsibleEntries = true;
    pub enable_collapsible_entries: bool,
}

impl ItemList {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_recipe_book: true,
            enable_item_list: true,
            enable_collapsible_entries: true,
        };
        this
    }

}

/// java: public static class ItemTooltip
#[derive(Clone)]
pub struct ItemTooltip {
    /// java: public boolean enableNPCPrice = true;
    pub enable_npc_price: bool,
    /// java: public boolean enableGeorgePrice = true;
    pub enable_george_price: bool,
    /// java: public boolean enableMotesPrice = true;
    pub enable_motes_price: bool,
    /// java: public boolean enableAvgBIN = true;
    pub enable_avg_bin: bool,
    /// java: public Average avg = Average.THREE_DAY;
    pub avg: Average,
    /// java: public boolean enableLowestBIN = true;
    pub enable_lowest_bin: bool,
    /// java: public boolean enableBazaarPrice = true;
    pub enable_bazaar_price: bool,
    /// java: public Craft enableCraftingCost = Craft.OFF;
    pub enable_crafting_cost: Craft,
    /// java: public boolean enableObtainedDate = true;
    pub enable_obtained_date: bool,
    /// java: public boolean enableMuseumInfo = true;
    pub enable_museum_info: bool,
    /// java: public boolean enableExoticTooltip = true;
    pub enable_exotic_tooltip: bool,
    /// java: public boolean enableAccessoriesHelper = true;
    pub enable_accessories_helper: bool,
    /// java: public boolean dungeonQuality = true;
    pub dungeon_quality: bool,
    /// java: public boolean showEssenceCost = true;
    pub show_essence_cost: bool,
    /// java: public boolean enableEstimatedItemValue = true;
    pub enable_estimated_item_value: bool,
    /// java: public boolean enableStackingEnchantProgress = true;
    pub enable_stacking_enchant_progress: bool,
    /// java: public boolean enableEvolvingItemProgress = true;
    pub enable_evolving_item_progress: bool,
}

impl ItemTooltip {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            enable_npc_price: true,
            enable_george_price: true,
            enable_motes_price: true,
            enable_avg_bin: true,
            avg: Average::ThreeDay,
            enable_lowest_bin: true,
            enable_bazaar_price: true,
            enable_crafting_cost: Craft::Off,
            enable_obtained_date: true,
            enable_museum_info: true,
            enable_exotic_tooltip: true,
            enable_accessories_helper: true,
            dungeon_quality: true,
            show_essence_cost: true,
            enable_estimated_item_value: true,
            enable_stacking_enchant_progress: true,
            enable_evolving_item_progress: true,
        });
        __this
    }

}

/// java: public enum Average
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Average {
    OneDay,
    ThreeDay,
    Both,
}

impl Average {
    pub const VALUES: [Average; 3] = [Average::OneDay, Average::ThreeDay, Average::Both];

    /// java: values()
    pub fn values() -> Vec<Average> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Average::OneDay => "ONE_DAY",
            Average::ThreeDay => "THREE_DAY",
            Average::Both => "BOTH",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Average {
        match name {
            "ONE_DAY" => Average::OneDay,
            "THREE_DAY" => Average::ThreeDay,
            "BOTH" => Average::Both,
            _ => panic!("IllegalArgumentException: No enum constant Average.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.general.itemTooltip.avg.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Average {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Average::to_string(self))
    }
}

/// java: public enum Craft
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Craft {
    SellOrder,
    BuyOrder,
    Off,
}

impl Craft {
    pub const VALUES: [Craft; 3] = [Craft::SellOrder, Craft::BuyOrder, Craft::Off];

    /// java: values()
    pub fn values() -> Vec<Craft> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Craft::SellOrder => "SELL_ORDER",
            Craft::BuyOrder => "BUY_ORDER",
            Craft::Off => "OFF",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Craft {
        match name {
            "SELL_ORDER" => Craft::SellOrder,
            "BUY_ORDER" => Craft::BuyOrder,
            "OFF" => Craft::Off,
            _ => panic!("IllegalArgumentException: No enum constant Craft.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.general.itemTooltip.craft.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for Craft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Craft::to_string(self))
    }
}

/// java: public static class ItemInfoDisplay
#[derive(Clone)]
pub struct ItemInfoDisplay {
    /// java: @Deprecated public transient boolean slotText = true;
    pub slot_text: bool,
    /// java: @Deprecated public transient SlotTextMode slotTextMode = SlotTextMode.ENABLED;
    pub slot_text_mode: SlotTextMode,
    /// java: @Deprecated public transient boolean slotTextToggled = true;
    pub slot_text_toggled: bool,
    /// java: @Deprecated public transient boolean attributeShardInfo = true;
    pub attribute_shard_info: bool,
    /// java: public ItemBackgroundStyle itemBackgroundStyle = ItemBackgroundStyle.SQUARE;
    pub item_background_style: ItemBackgroundStyle,
    /// java: public float itemBackgroundOpacity = 0.5f;
    pub item_background_opacity: f32,
    /// java: public boolean itemRarityBackgrounds = true;
    pub item_rarity_backgrounds: bool,
    /// java: public boolean itemRarityBackgroundsLegacyColors = false;
    pub item_rarity_backgrounds_legacy_colors: bool,
    /// java: public boolean jacobMedalBackgrounds = true;
    pub jacob_medal_backgrounds: bool,
    /// java: public boolean legacyAttributeBackgrounds = true;
    pub legacy_attribute_backgrounds: bool,
}

impl ItemInfoDisplay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            slot_text: true,
            slot_text_mode: SlotTextMode::Enabled,
            slot_text_toggled: true,
            attribute_shard_info: true,
            item_background_style: ItemBackgroundStyle::Square,
            item_background_opacity: 0.5f32,
            item_rarity_backgrounds: true,
            item_rarity_backgrounds_legacy_colors: false,
            jacob_medal_backgrounds: true,
            legacy_attribute_backgrounds: true,
        };
        this
    }

}

/// java: public enum ItemBackgroundStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ItemBackgroundStyle {
    Circular,
    Square,
}

impl ItemBackgroundStyle {
    pub const VALUES: [ItemBackgroundStyle; 2] = [ItemBackgroundStyle::Circular, ItemBackgroundStyle::Square];

    /// java: values()
    pub fn values() -> Vec<ItemBackgroundStyle> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ItemBackgroundStyle::Circular => "CIRCULAR",
            ItemBackgroundStyle::Square => "SQUARE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ItemBackgroundStyle {
        match name {
            "CIRCULAR" => ItemBackgroundStyle::Circular,
            "SQUARE" => ItemBackgroundStyle::Square,
            _ => panic!("IllegalArgumentException: No enum constant ItemBackgroundStyle.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static ItemBackgroundStyleFields {
        static FIELDS: LazyLock<Vec<ItemBackgroundStyleFields>> = LazyLock::new(|| vec![
            ItemBackgroundStyleFields::new(crate::entry::skyblocker_mod::id("item_background_circular")),
            ItemBackgroundStyleFields::new(crate::entry::skyblocker_mod::id("item_background_square")),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.general.itemInfoDisplay.itemBackgroundStyle.style.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for ItemBackgroundStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ItemBackgroundStyle::to_string(self))
    }
}

/// the constructor data of each ItemBackgroundStyle constant
pub struct ItemBackgroundStyleFields {
    pub tex: Identifier,
}

impl ItemBackgroundStyleFields {
    fn new(mut tex: Identifier) -> Self {
        Self { tex }
    }

}

/// java: public static class ItemProtection
#[derive(Clone)]
pub struct ItemProtection {
    /// java: public SlotLockStyle slotLockStyle = SlotLockStyle.CLASSIC;
    pub slot_lock_style: SlotLockStyle,
    /// java: public boolean displayChatNotification = true;
    pub display_chat_notification: bool,
    /// java: public boolean protectValuableConsumables = true;
    pub protect_valuable_consumables: bool,
}

impl ItemProtection {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            slot_lock_style: SlotLockStyle::Classic,
            display_chat_notification: true,
            protect_valuable_consumables: true,
        };
        this
    }

}

/// java: public enum SlotLockStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SlotLockStyle {
    Classic,
    Fancy,
}

impl SlotLockStyle {
    pub const VALUES: [SlotLockStyle; 2] = [SlotLockStyle::Classic, SlotLockStyle::Fancy];

    /// java: values()
    pub fn values() -> Vec<SlotLockStyle> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SlotLockStyle::Classic => "CLASSIC",
            SlotLockStyle::Fancy => "FANCY",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SlotLockStyle {
        match name {
            "CLASSIC" => SlotLockStyle::Classic,
            "FANCY" => SlotLockStyle::Fancy,
            _ => panic!("IllegalArgumentException: No enum constant SlotLockStyle.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static SlotLockStyleFields {
        static FIELDS: LazyLock<Vec<SlotLockStyleFields>> = LazyLock::new(|| vec![
            SlotLockStyleFields::new(crate::entry::skyblocker_mod::id("slot_lock")),
            SlotLockStyleFields::new(crate::entry::skyblocker_mod::id("fancy_slot_lock")),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return mcrust::net::minecraft::client::resources::language::i18n::get(&format!("skyblocker.config.general.itemProtection.slotLockStyle.style.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for SlotLockStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SlotLockStyle::to_string(self))
    }
}

/// the constructor data of each SlotLockStyle constant
pub struct SlotLockStyleFields {
    pub tex: Identifier,
}

impl SlotLockStyleFields {
    fn new(mut tex: Identifier) -> Self {
        Self { tex }
    }

}

/// java: public static class WikiLookup
#[derive(Clone)]
pub struct WikiLookup {
    /// java: public boolean enableWikiLookup = true;
    pub enable_wiki_lookup: bool,
    /// java: @Deprecated public transient boolean officialWiki = true;
    pub official_wiki: bool,
}

impl WikiLookup {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enable_wiki_lookup: true,
            official_wiki: true,
        };
        this
    }

}

/// java: public static class SpecialEffects
#[derive(Clone)]
pub struct SpecialEffects {
    /// java: public boolean rareDungeonDropEffects = true;
    pub rare_dungeon_drop_effects: bool,
    /// java: public boolean rareDropEffects = true;
    pub rare_drop_effects: bool,
    /// java: public boolean rareDyeDropEffects = true;
    pub rare_dye_drop_effects: bool,
    /// java: public boolean trophyDropEffects = true;
    pub trophy_drop_effects: bool,
    /// java: public boolean displayItemName = true;
    pub display_item_name: bool,
}

impl SpecialEffects {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            rare_dungeon_drop_effects: true,
            rare_drop_effects: true,
            rare_dye_drop_effects: true,
            trophy_drop_effects: true,
            display_item_name: true,
        };
        this
    }

}

/// java: @Deprecated public static class Hitbox
#[derive(Clone)]
pub struct Hitbox {
    /// java: @Deprecated public transient boolean oldCactusHitbox = false;
    pub old_cactus_hitbox: bool,
    /// java: @Deprecated public transient boolean oldFarmlandHitbox = false;
    pub old_farmland_hitbox: bool,
    /// java: @Deprecated public transient boolean oldLeverHitbox = false;
    pub old_lever_hitbox: bool,
    /// java: @Deprecated public transient boolean oldMushroomHitbox = false;
    pub old_mushroom_hitbox: bool,
}

impl Hitbox {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            old_cactus_hitbox: false,
            old_farmland_hitbox: false,
            old_lever_hitbox: false,
            old_mushroom_hitbox: false,
        };
        this
    }

}
