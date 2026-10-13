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

// Generated Gson forms for src/de/hysky/skyblocker/config/configs/general_config.rs (fabric2seer round 24): APPEND to that file.
// Needs `serde_json` in Cargo.toml and the `compat` of this round (dandelion::compat::GsonValue, ClassOf, ...).

/// GeneralConfig's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl GeneralConfig {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableTips".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_tips))),
            ("acceptReparty".to_string(), dandelion::compat::GsonValue::Bool(*(&self.accept_reparty))),
            ("speedPresets".to_string(), ((&self.speed_presets)).__gson_write()),
            ("shortcuts".to_string(), ((&self.shortcuts)).__gson_write()),
            ("quiverWarning".to_string(), ((&self.quiver_warning)).__gson_write()),
            ("itemList".to_string(), ((&self.item_list)).__gson_write()),
            ("itemTooltip".to_string(), compat::read((&self.item_tooltip), |__t| __t.__gson_write())),
            ("itemInfoDisplay".to_string(), ((&self.item_info_display)).__gson_write()),
            ("itemProtection".to_string(), ((&self.item_protection)).__gson_write()),
            ("wikiLookup".to_string(), ((&self.wiki_lookup)).__gson_write()),
            ("specialEffects".to_string(), ((&self.special_effects)).__gson_write()),
            ("lockedSlots".to_string(), dandelion::compat::GsonValue::Arr(((&self.locked_slots)).iter().map(|__x0| dandelion::compat::GsonValue::int(*__x0 as i64)).collect())),
            ("protectedItems".to_string(), dandelion::compat::GsonValue::sorted_arr(((&self.protected_items)).iter().map(|__x0| dandelion::compat::GsonValue::str(&__x0)).collect())),
            ("customItemNames".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_item_names)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::gson_by_codec(&mcrust::net::minecraft::network::chat::component_serialization::component_codec(), __x0))).collect())),
            ("customDyeColors".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_dye_colors)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::GsonValue::int(*__x0 as i64))).collect())),
            ("customArmorTrims".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_armor_trims)).iter().map(|(__k0, __x0)| (__k0.clone(), (__x0).__gson_write())).collect())),
            ("customAnimatedDyes".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_animated_dyes)).iter().map(|(__k0, __x0)| (__k0.clone(), (__x0).__gson_write())).collect())),
            ("customHelmetTextures".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_helmet_textures)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::GsonValue::str(&__x0))).collect())),
            ("customGlint".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_glint)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::GsonValue::Bool(*__x0))).collect())),
            ("customItemModel".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_item_model)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::GsonValue::Str((__x0).to_string()))).collect())),
            ("customArmorModel".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_armor_model)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::GsonValue::Str((__x0).to_string()))).collect())),
            ("customAnimatedHelmetTextures".to_string(), dandelion::compat::GsonValue::sorted_obj(((&self.custom_animated_helmet_textures)).iter().map(|(__k0, __x0)| (__k0.clone(), dandelion::compat::GsonValue::str(&__x0))).collect())),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<GeneralConfig> {
        let __o = __v.as_object()?;
        let mut __this = GeneralConfig::new();
        if let Some(__v) = __o.get("enableTips") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_tips = __x; }
        }
        if let Some(__v) = __o.get("acceptReparty") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.accept_reparty = __x; }
        }
        if let Some(__v) = __o.get("speedPresets") {
            if let Some(__x) = SpeedPresets::__gson_read(__v) { __this.speed_presets = __x; }
        }
        if let Some(__v) = __o.get("shortcuts") {
            if let Some(__x) = Shortcuts::__gson_read(__v) { __this.shortcuts = __x; }
        }
        if let Some(__v) = __o.get("quiverWarning") {
            if let Some(__x) = QuiverWarning::__gson_read(__v) { __this.quiver_warning = __x; }
        }
        if let Some(__v) = __o.get("itemList") {
            if let Some(__x) = ItemList::__gson_read(__v) { __this.item_list = __x; }
        }
        if let Some(__v) = __o.get("itemTooltip") {
            if let Some(__x) = ItemTooltip::__gson_read(__v) { __this.item_tooltip = __x; }
        }
        if let Some(__v) = __o.get("itemInfoDisplay") {
            if let Some(__x) = ItemInfoDisplay::__gson_read(__v) { __this.item_info_display = __x; }
        }
        if let Some(__v) = __o.get("itemProtection") {
            if let Some(__x) = ItemProtection::__gson_read(__v) { __this.item_protection = __x; }
        }
        if let Some(__v) = __o.get("wikiLookup") {
            if let Some(__x) = WikiLookup::__gson_read(__v) { __this.wiki_lookup = __x; }
        }
        if let Some(__v) = __o.get("specialEffects") {
            if let Some(__x) = SpecialEffects::__gson_read(__v) { __this.special_effects = __x; }
        }
        if let Some(__v) = __o.get("lockedSlots") {
            if let Some(__x) = (__v).as_array().map(|__a| __a.iter().filter_map(|__y0| dandelion::compat::gson_i64(__y0).map(|__n| __n as i32)).collect()) { __this.locked_slots = __x; }
        }
        if let Some(__v) = __o.get("protectedItems") {
            if let Some(__x) = (__v).as_array().map(|__a| __a.iter().filter_map(|__y0| dandelion::compat::gson_string(__y0)).collect()) { __this.protected_items = __x; }
        }
        if let Some(__v) = __o.get("customItemNames") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_parse_codec(&mcrust::net::minecraft::network::chat::component_serialization::component_codec(), __y0)?))).collect()) { __this.custom_item_names = __x; }
        }
        if let Some(__v) = __o.get("customDyeColors") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_i64(__y0).map(|__n| __n as i32)?))).collect()) { __this.custom_dye_colors = __x; }
        }
        if let Some(__v) = __o.get("customArmorTrims") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, ArmorTrimId::__gson_read(__y0)?))).collect()) { __this.custom_armor_trims = __x; }
        }
        if let Some(__v) = __o.get("customAnimatedDyes") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, AnimatedDye::__gson_read(__y0)?))).collect()) { __this.custom_animated_dyes = __x; }
        }
        if let Some(__v) = __o.get("customHelmetTextures") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_string(__y0)?))).collect()) { __this.custom_helmet_textures = __x; }
        }
        if let Some(__v) = __o.get("customGlint") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_bool(__y0)?))).collect()) { __this.custom_glint = __x; }
        }
        if let Some(__v) = __o.get("customItemModel") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_string(__y0).and_then(|__s| Identifier::try_parse(&__s))?))).collect()) { __this.custom_item_model = __x; }
        }
        if let Some(__v) = __o.get("customArmorModel") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_string(__y0).and_then(|__s| Identifier::try_parse(&__s))?))).collect()) { __this.custom_armor_model = __x; }
        }
        if let Some(__v) = __o.get("customAnimatedHelmetTextures") {
            if let Some(__x) = (__v).as_object().map(|__o| __o.iter().filter_map(|(__k0, __y0)| Some((Some(__k0.clone())?, dandelion::compat::gson_string(__y0)?))).collect()) { __this.custom_animated_helmet_textures = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableTips" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_tips = *__b; true } else { false },
                "acceptReparty" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.accept_reparty = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                "speedPresets" => self.speed_presets.__gson_patch(__rest, __value),
                "shortcuts" => self.shortcuts.__gson_patch(__rest, __value),
                "quiverWarning" => self.quiver_warning.__gson_patch(__rest, __value),
                "itemList" => self.item_list.__gson_patch(__rest, __value),
                "itemTooltip" => compat::write(&self.item_tooltip, |__t| __t.__gson_patch(__rest, __value)),
                "itemInfoDisplay" => self.item_info_display.__gson_patch(__rest, __value),
                "itemProtection" => self.item_protection.__gson_patch(__rest, __value),
                "wikiLookup" => self.wiki_lookup.__gson_patch(__rest, __value),
                "specialEffects" => self.special_effects.__gson_patch(__rest, __value),
                _ => false,
            },
            [] => false,
        }
    }

    /// `GeneralConfig.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<GeneralConfig> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig",
            new: GeneralConfig::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: GeneralConfig::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SpeedPresets's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SpeedPresets {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableSpeedPresets".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_speed_presets))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SpeedPresets> {
        let __o = __v.as_object()?;
        let mut __this = SpeedPresets::new();
        if let Some(__v) = __o.get("enableSpeedPresets") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_speed_presets = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableSpeedPresets" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_speed_presets = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SpeedPresets.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SpeedPresets> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$SpeedPresets",
            new: SpeedPresets::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SpeedPresets::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Shortcuts's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Shortcuts {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableShortcuts".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_shortcuts))),
            ("enableCommandShortcuts".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_command_shortcuts))),
            ("enableCommandArgShortcuts".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_command_arg_shortcuts))),
            ("enableKeyBindingShortcuts".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_key_binding_shortcuts))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<Shortcuts> {
        let __o = __v.as_object()?;
        let mut __this = Shortcuts::new();
        if let Some(__v) = __o.get("enableShortcuts") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_shortcuts = __x; }
        }
        if let Some(__v) = __o.get("enableCommandShortcuts") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_command_shortcuts = __x; }
        }
        if let Some(__v) = __o.get("enableCommandArgShortcuts") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_command_arg_shortcuts = __x; }
        }
        if let Some(__v) = __o.get("enableKeyBindingShortcuts") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_key_binding_shortcuts = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableShortcuts" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_shortcuts = *__b; true } else { false },
                "enableCommandShortcuts" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_command_shortcuts = *__b; true } else { false },
                "enableCommandArgShortcuts" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_command_arg_shortcuts = *__b; true } else { false },
                "enableKeyBindingShortcuts" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_key_binding_shortcuts = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `Shortcuts.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<Shortcuts> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$Shortcuts",
            new: Shortcuts::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: Shortcuts::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// QuiverWarning's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl QuiverWarning {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableQuiverWarning".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_quiver_warning))),
            ("enableQuiverWarningInDungeons".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_quiver_warning_in_dungeons))),
            ("enableQuiverWarningAfterDungeon".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_quiver_warning_after_dungeon))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<QuiverWarning> {
        let __o = __v.as_object()?;
        let mut __this = QuiverWarning::new();
        if let Some(__v) = __o.get("enableQuiverWarning") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_quiver_warning = __x; }
        }
        if let Some(__v) = __o.get("enableQuiverWarningInDungeons") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_quiver_warning_in_dungeons = __x; }
        }
        if let Some(__v) = __o.get("enableQuiverWarningAfterDungeon") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_quiver_warning_after_dungeon = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableQuiverWarning" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_quiver_warning = *__b; true } else { false },
                "enableQuiverWarningInDungeons" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_quiver_warning_in_dungeons = *__b; true } else { false },
                "enableQuiverWarningAfterDungeon" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_quiver_warning_after_dungeon = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `QuiverWarning.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<QuiverWarning> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$QuiverWarning",
            new: QuiverWarning::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: QuiverWarning::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ItemList's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ItemList {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableRecipeBook".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_recipe_book))),
            ("enableItemList".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_item_list))),
            ("enableCollapsibleEntries".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_collapsible_entries))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ItemList> {
        let __o = __v.as_object()?;
        let mut __this = ItemList::new();
        if let Some(__v) = __o.get("enableRecipeBook") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_recipe_book = __x; }
        }
        if let Some(__v) = __o.get("enableItemList") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_item_list = __x; }
        }
        if let Some(__v) = __o.get("enableCollapsibleEntries") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_collapsible_entries = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableRecipeBook" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_recipe_book = *__b; true } else { false },
                "enableItemList" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_item_list = *__b; true } else { false },
                "enableCollapsibleEntries" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_collapsible_entries = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ItemList.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ItemList> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$ItemList",
            new: ItemList::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ItemList::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ItemTooltip's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ItemTooltip {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableNPCPrice".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_npc_price))),
            ("enableGeorgePrice".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_george_price))),
            ("enableMotesPrice".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_motes_price))),
            ("enableAvgBIN".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_avg_bin))),
            ("avg".to_string(), dandelion::compat::GsonValue::str(((&self.avg)).name())),
            ("enableLowestBIN".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_lowest_bin))),
            ("enableBazaarPrice".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_bazaar_price))),
            ("enableCraftingCost".to_string(), dandelion::compat::GsonValue::str(((&self.enable_crafting_cost)).name())),
            ("enableObtainedDate".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_obtained_date))),
            ("enableMuseumInfo".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_museum_info))),
            ("enableExoticTooltip".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_exotic_tooltip))),
            ("enableAccessoriesHelper".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_accessories_helper))),
            ("dungeonQuality".to_string(), dandelion::compat::GsonValue::Bool(*(&self.dungeon_quality))),
            ("showEssenceCost".to_string(), dandelion::compat::GsonValue::Bool(*(&self.show_essence_cost))),
            ("enableEstimatedItemValue".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_estimated_item_value))),
            ("enableStackingEnchantProgress".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_stacking_enchant_progress))),
            ("enableEvolvingItemProgress".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_evolving_item_progress))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<compat::Shared<ItemTooltip>> {
        let __o = __v.as_object()?;
        let __h = ItemTooltip::new();
        compat::write(&__h, |__this| {
            if let Some(__v) = __o.get("enableNPCPrice") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_npc_price = __x; }
            }
            if let Some(__v) = __o.get("enableGeorgePrice") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_george_price = __x; }
            }
            if let Some(__v) = __o.get("enableMotesPrice") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_motes_price = __x; }
            }
            if let Some(__v) = __o.get("enableAvgBIN") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_avg_bin = __x; }
            }
            if let Some(__v) = __o.get("avg") {
                if let Some(__x) = (__v).as_str().and_then(|__s| Average::values().into_iter().find(|__c| __c.name() == __s)) { __this.avg = __x; }
            }
            if let Some(__v) = __o.get("enableLowestBIN") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_lowest_bin = __x; }
            }
            if let Some(__v) = __o.get("enableBazaarPrice") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_bazaar_price = __x; }
            }
            if let Some(__v) = __o.get("enableCraftingCost") {
                if let Some(__x) = (__v).as_str().and_then(|__s| Craft::values().into_iter().find(|__c| __c.name() == __s)) { __this.enable_crafting_cost = __x; }
            }
            if let Some(__v) = __o.get("enableObtainedDate") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_obtained_date = __x; }
            }
            if let Some(__v) = __o.get("enableMuseumInfo") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_museum_info = __x; }
            }
            if let Some(__v) = __o.get("enableExoticTooltip") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_exotic_tooltip = __x; }
            }
            if let Some(__v) = __o.get("enableAccessoriesHelper") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_accessories_helper = __x; }
            }
            if let Some(__v) = __o.get("dungeonQuality") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.dungeon_quality = __x; }
            }
            if let Some(__v) = __o.get("showEssenceCost") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.show_essence_cost = __x; }
            }
            if let Some(__v) = __o.get("enableEstimatedItemValue") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_estimated_item_value = __x; }
            }
            if let Some(__v) = __o.get("enableStackingEnchantProgress") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_stacking_enchant_progress = __x; }
            }
            if let Some(__v) = __o.get("enableEvolvingItemProgress") {
                if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_evolving_item_progress = __x; }
            }
        });
        Some(__h)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableNPCPrice" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_npc_price = *__b; true } else { false },
                "enableGeorgePrice" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_george_price = *__b; true } else { false },
                "enableMotesPrice" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_motes_price = *__b; true } else { false },
                "enableAvgBIN" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_avg_bin = *__b; true } else { false },
                "avg" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Average::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.avg = __c; true } None => false } } else { false },
                "enableLowestBIN" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_lowest_bin = *__b; true } else { false },
                "enableBazaarPrice" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_bazaar_price = *__b; true } else { false },
                "enableCraftingCost" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match Craft::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.enable_crafting_cost = __c; true } None => false } } else { false },
                "enableObtainedDate" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_obtained_date = *__b; true } else { false },
                "enableMuseumInfo" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_museum_info = *__b; true } else { false },
                "enableExoticTooltip" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_exotic_tooltip = *__b; true } else { false },
                "enableAccessoriesHelper" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_accessories_helper = *__b; true } else { false },
                "dungeonQuality" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.dungeon_quality = *__b; true } else { false },
                "showEssenceCost" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.show_essence_cost = *__b; true } else { false },
                "enableEstimatedItemValue" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_estimated_item_value = *__b; true } else { false },
                "enableStackingEnchantProgress" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_stacking_enchant_progress = *__b; true } else { false },
                "enableEvolvingItemProgress" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_evolving_item_progress = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ItemTooltip.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<compat::Shared<ItemTooltip>> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$ItemTooltip",
            new: ItemTooltip::new,
            to_gson: |__h| compat::read(__h, |__t| __t.__gson_write()),
            read_gson: ItemTooltip::__gson_read,
            patch: |__h, __p, __v| compat::write(__h, |__t| __t.__gson_patch(__p, __v)),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// Average's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Average {
    pub fn __class_of() -> dandelion::compat::ClassOf<Average> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$Average",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| Average::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || Average::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// Craft's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl Craft {
    pub fn __class_of() -> dandelion::compat::ClassOf<Craft> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$Craft",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| Craft::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || Craft::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// ItemInfoDisplay's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ItemInfoDisplay {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("itemBackgroundStyle".to_string(), dandelion::compat::GsonValue::str(((&self.item_background_style)).name())),
            ("itemBackgroundOpacity".to_string(), dandelion::compat::GsonValue::float(*(&self.item_background_opacity))),
            ("itemRarityBackgrounds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.item_rarity_backgrounds))),
            ("itemRarityBackgroundsLegacyColors".to_string(), dandelion::compat::GsonValue::Bool(*(&self.item_rarity_backgrounds_legacy_colors))),
            ("jacobMedalBackgrounds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.jacob_medal_backgrounds))),
            ("legacyAttributeBackgrounds".to_string(), dandelion::compat::GsonValue::Bool(*(&self.legacy_attribute_backgrounds))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ItemInfoDisplay> {
        let __o = __v.as_object()?;
        let mut __this = ItemInfoDisplay::new();
        if let Some(__v) = __o.get("itemBackgroundStyle") {
            if let Some(__x) = (__v).as_str().and_then(|__s| ItemBackgroundStyle::values().into_iter().find(|__c| __c.name() == __s)) { __this.item_background_style = __x; }
        }
        if let Some(__v) = __o.get("itemBackgroundOpacity") {
            if let Some(__x) = dandelion::compat::gson_f64(__v).map(|__n| __n as f32) { __this.item_background_opacity = __x; }
        }
        if let Some(__v) = __o.get("itemRarityBackgrounds") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.item_rarity_backgrounds = __x; }
        }
        if let Some(__v) = __o.get("itemRarityBackgroundsLegacyColors") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.item_rarity_backgrounds_legacy_colors = __x; }
        }
        if let Some(__v) = __o.get("jacobMedalBackgrounds") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.jacob_medal_backgrounds = __x; }
        }
        if let Some(__v) = __o.get("legacyAttributeBackgrounds") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.legacy_attribute_backgrounds = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "itemBackgroundStyle" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match ItemBackgroundStyle::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.item_background_style = __c; true } None => false } } else { false },
                "itemRarityBackgrounds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.item_rarity_backgrounds = *__b; true } else { false },
                "itemRarityBackgroundsLegacyColors" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.item_rarity_backgrounds_legacy_colors = *__b; true } else { false },
                "jacobMedalBackgrounds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.jacob_medal_backgrounds = *__b; true } else { false },
                "legacyAttributeBackgrounds" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.legacy_attribute_backgrounds = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ItemInfoDisplay.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ItemInfoDisplay> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$ItemInfoDisplay",
            new: ItemInfoDisplay::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ItemInfoDisplay::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// ItemBackgroundStyle's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ItemBackgroundStyle {
    pub fn __class_of() -> dandelion::compat::ClassOf<ItemBackgroundStyle> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$ItemBackgroundStyle",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| ItemBackgroundStyle::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || ItemBackgroundStyle::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// ItemProtection's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl ItemProtection {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("slotLockStyle".to_string(), dandelion::compat::GsonValue::str(((&self.slot_lock_style)).name())),
            ("displayChatNotification".to_string(), dandelion::compat::GsonValue::Bool(*(&self.display_chat_notification))),
            ("protectValuableConsumables".to_string(), dandelion::compat::GsonValue::Bool(*(&self.protect_valuable_consumables))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<ItemProtection> {
        let __o = __v.as_object()?;
        let mut __this = ItemProtection::new();
        if let Some(__v) = __o.get("slotLockStyle") {
            if let Some(__x) = (__v).as_str().and_then(|__s| SlotLockStyle::values().into_iter().find(|__c| __c.name() == __s)) { __this.slot_lock_style = __x; }
        }
        if let Some(__v) = __o.get("displayChatNotification") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.display_chat_notification = __x; }
        }
        if let Some(__v) = __o.get("protectValuableConsumables") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.protect_valuable_consumables = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "slotLockStyle" => if let dandelion::compat::PatchValue::Enum(__s) = __value { match SlotLockStyle::values().into_iter().find(|__c| __c.name() == __s.as_str()) { Some(__c) => { self.slot_lock_style = __c; true } None => false } } else { false },
                "displayChatNotification" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.display_chat_notification = *__b; true } else { false },
                "protectValuableConsumables" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.protect_valuable_consumables = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `ItemProtection.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<ItemProtection> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$ItemProtection",
            new: ItemProtection::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: ItemProtection::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SlotLockStyle's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SlotLockStyle {
    pub fn __class_of() -> dandelion::compat::ClassOf<SlotLockStyle> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$SlotLockStyle",
            new: || panic!("InstantiationException: an enum has no constructor to call"),
            to_gson: |__e| dandelion::compat::GsonValue::str(__e.name()),
            read_gson: |__v| __v.as_str().and_then(|__s| SlotLockStyle::values().into_iter().find(|__c| __c.name() == __s)),
            patch: |_, _, _| false,
            is_enum: true,
            enum_constants: || SlotLockStyle::values().into_iter().map(|__c| __c.name().to_string()).collect(),
        }
    }
}

/// WikiLookup's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl WikiLookup {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("enableWikiLookup".to_string(), dandelion::compat::GsonValue::Bool(*(&self.enable_wiki_lookup))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<WikiLookup> {
        let __o = __v.as_object()?;
        let mut __this = WikiLookup::new();
        if let Some(__v) = __o.get("enableWikiLookup") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.enable_wiki_lookup = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "enableWikiLookup" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.enable_wiki_lookup = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `WikiLookup.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<WikiLookup> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$WikiLookup",
            new: WikiLookup::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: WikiLookup::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}

/// SpecialEffects's Gson form (`Gson.toJson` / `fromJson` by reflection in Java; generated here)
impl SpecialEffects {
    /// `gson.toJson(this)`: the fields in Gson's order (declaration order), nulls written
    pub fn __gson_write(&self) -> dandelion::compat::GsonValue {
        dandelion::compat::GsonValue::Obj(vec![
            ("rareDungeonDropEffects".to_string(), dandelion::compat::GsonValue::Bool(*(&self.rare_dungeon_drop_effects))),
            ("rareDropEffects".to_string(), dandelion::compat::GsonValue::Bool(*(&self.rare_drop_effects))),
            ("rareDyeDropEffects".to_string(), dandelion::compat::GsonValue::Bool(*(&self.rare_dye_drop_effects))),
            ("trophyDropEffects".to_string(), dandelion::compat::GsonValue::Bool(*(&self.trophy_drop_effects))),
            ("displayItemName".to_string(), dandelion::compat::GsonValue::Bool(*(&self.display_item_name))),
        ])
    }

    /// `gson.fromJson(json, X.class)`: a new object from the no-argument constructor, then every key
    /// present set; unknown keys are ignored
    pub fn __gson_read(__v: &serde_json::Value) -> Option<SpecialEffects> {
        let __o = __v.as_object()?;
        let mut __this = SpecialEffects::new();
        if let Some(__v) = __o.get("rareDungeonDropEffects") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.rare_dungeon_drop_effects = __x; }
        }
        if let Some(__v) = __o.get("rareDropEffects") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.rare_drop_effects = __x; }
        }
        if let Some(__v) = __o.get("rareDyeDropEffects") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.rare_dye_drop_effects = __x; }
        }
        if let Some(__v) = __o.get("trophyDropEffects") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.trophy_drop_effects = __x; }
        }
        if let Some(__v) = __o.get("displayItemName") {
            if let Some(__x) = dandelion::compat::gson_bool(__v) { __this.display_item_name = __x; }
        }
        Some(__this)
    }

    /// Dandelion's ConfigPatcher: the field at a dotted path (`getDeclaredField` per segment) set to a
    /// boolean or an enum constant; false where Java's reflective walk fails
    pub fn __gson_patch(&mut self, __path: &[&str], __value: &dandelion::compat::PatchValue) -> bool {
        match __path {
            [__name] => match *__name {
                "rareDungeonDropEffects" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.rare_dungeon_drop_effects = *__b; true } else { false },
                "rareDropEffects" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.rare_drop_effects = *__b; true } else { false },
                "rareDyeDropEffects" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.rare_dye_drop_effects = *__b; true } else { false },
                "trophyDropEffects" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.trophy_drop_effects = *__b; true } else { false },
                "displayItemName" => if let dandelion::compat::PatchValue::Bool(__b) = __value { self.display_item_name = *__b; true } else { false },
                _ => false,
            },
            [__first, __rest @ ..] => match *__first {
                _ => false,
            },
            [] => false,
        }
    }

    /// `SpecialEffects.class` as reflection sees it
    pub fn __class_of() -> dandelion::compat::ClassOf<SpecialEffects> {
        dandelion::compat::ClassOf {
            name: "de.hysky.skyblocker.config.configs.GeneralConfig$SpecialEffects",
            new: SpecialEffects::new,
            to_gson: |__x| __x.__gson_write(),
            read_gson: SpecialEffects::__gson_read,
            patch: |__x, __p, __v| __x.__gson_patch(__p, __v),
            is_enum: false,
            enum_constants: Vec::new,
        }
    }
}
