//! java: public record SkyblockItemData(String id, Optional<String> category, Optional<List<List<GearUpgrade>>> upgradeCosts,
//! Optional<List<GemstoneSlot>> gemstoneSlots, Optional<PrestigeItem> prestige) (net.azureaaron.networth.data)
//!
//! Skyblock item information from the Hypixel API in record form. Use the `MAP_CODEC` to obtain a map of this that
//! can be passed to the `ItemCalculator`. The inner records (`GearUpgrade`, `PrestigeItem`, `GemstoneSlot` and its
//! `GemstoneSlotCost`) are in this file, as Java declares them inside this one. Each record's private `CODEC` is a
//! module `static` named after its record.

use std::collections::HashMap;
use std::sync::LazyLock;

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

use crate::net::azureaaron::networth::utils::codec_utils::CodecUtils;

/// java: private static final Codec<SkyblockItemData> CODEC
static CODEC: LazyLock<CodecRef<SkyblockItemData>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply5(
        record_codec_builder::of(
            |o: &SkyblockItemData| o.id.clone(),
            codec::field_of(codec::string_codec(), "id"),
        ),
        record_codec_builder::of(
            |o: &SkyblockItemData| o.category.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "category"),
        ),
        record_codec_builder::of(
            |o: &SkyblockItemData| o.upgrade_costs.clone(),
            codec::optional_field_of_opt(
                codec::list(codec::list(GEAR_UPGRADE_CODEC.clone())),
                "upgrade_costs",
            ),
        ),
        record_codec_builder::of(
            |o: &SkyblockItemData| o.gemstone_slots.clone(),
            codec::optional_field_of_opt(codec::list(GEMSTONE_SLOT_CODEC.clone()), "gemstone_slots"),
        ),
        record_codec_builder::of(
            |o: &SkyblockItemData| o.prestige.clone(),
            codec::optional_field_of_opt(PRESTIGE_ITEM_CODEC.clone(), "prestige"),
        ),
        SkyblockItemData::new,
    ))
});

/// java: private static final Codec<List<SkyblockItemData>> LIST_CODEC = CODEC.listOf();
static LIST_CODEC: LazyLock<CodecRef<Vec<SkyblockItemData>>> =
    LazyLock::new(|| codec::list(CODEC.clone()));

/// java: public static final Codec<Object2ObjectMap<String, SkyblockItemData>> MAP_CODEC
/// = LIST_CODEC.xmap(SkyblockItemData::list2Map, map -> new ObjectArrayList<>(map.values()));
pub static MAP_CODEC: LazyLock<CodecRef<HashMap<String, SkyblockItemData>>> = LazyLock::new(|| {
    codec::xmap(
        LIST_CODEC.clone(),
        SkyblockItemData::list2_map,
        |map: &HashMap<String, SkyblockItemData>| map.values().cloned().collect(),
    )
});

/// java: public record SkyblockItemData(..)
#[derive(Clone, Debug, PartialEq)]
pub struct SkyblockItemData {
    pub id: String,
    pub category: Option<String>,
    pub upgrade_costs: Option<Vec<Vec<GearUpgrade>>>,
    pub gemstone_slots: Option<Vec<GemstoneSlot>>,
    pub prestige: Option<PrestigeItem>,
}

impl SkyblockItemData {
    /// java: the canonical record constructor
    pub fn new(
        id: String,
        category: Option<String>,
        upgrade_costs: Option<Vec<Vec<GearUpgrade>>>,
        gemstone_slots: Option<Vec<GemstoneSlot>>,
        prestige: Option<PrestigeItem>,
    ) -> Self {
        Self {
            id,
            category,
            upgrade_costs,
            gemstone_slots,
            prestige,
        }
    }

    /// java: private static Object2ObjectMap<String, SkyblockItemData> list2Map(List<SkyblockItemData> items)
    /// - `Collectors.toMap(id, identity, (a, b) -> a, Object2ObjectOpenHashMap::new)`: the FIRST of two equal ids wins
    fn list2_map(items: Vec<SkyblockItemData>) -> HashMap<String, SkyblockItemData> {
        let mut map: HashMap<String, SkyblockItemData> = HashMap::new();

        for item in items {
            map.entry(item.id.clone()).or_insert(item);
        }

        map
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> &str {
        &self.id
    }

    /// java: the record accessor `category()`
    pub fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    /// java: the record accessor `upgradeCosts()`
    pub fn upgrade_costs(&self) -> Option<&Vec<Vec<GearUpgrade>>> {
        self.upgrade_costs.as_ref()
    }

    /// java: the record accessor `gemstoneSlots()`
    pub fn gemstone_slots(&self) -> Option<&Vec<GemstoneSlot>> {
        self.gemstone_slots.as_ref()
    }

    /// java: the record accessor `prestige()`
    pub fn prestige(&self) -> Option<&PrestigeItem> {
        self.prestige.as_ref()
    }
}

/// java: GearUpgrade's private static final Codec<GearUpgrade> CODEC
static GEAR_UPGRADE_CODEC: LazyLock<CodecRef<GearUpgrade>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply3(
        record_codec_builder::of(
            |o: &GearUpgrade| o.item_id.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "item_id"),
        ),
        record_codec_builder::of(
            |o: &GearUpgrade| o.essence_type.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "essence_type"),
        ),
        record_codec_builder::of(
            |o: &GearUpgrade| o.amount,
            CodecUtils::optional_int(codec::optional_field_of_opt(codec::int_codec(), "amount")),
        ),
        GearUpgrade::new,
    ))
});

/// java: public record GearUpgrade(Optional<String> itemId, Optional<String> essenceType, OptionalInt amount)
#[derive(Clone, Debug, PartialEq)]
pub struct GearUpgrade {
    pub item_id: Option<String>,
    pub essence_type: Option<String>,
    pub amount: Option<i32>,
}

impl GearUpgrade {
    /// java: the canonical record constructor
    pub fn new(item_id: Option<String>, essence_type: Option<String>, amount: Option<i32>) -> Self {
        Self {
            item_id,
            essence_type,
            amount,
        }
    }

    /// java: the record accessor `itemId()`
    pub fn item_id(&self) -> Option<&String> {
        self.item_id.as_ref()
    }

    /// java: the record accessor `essenceType()`
    pub fn essence_type(&self) -> Option<&String> {
        self.essence_type.as_ref()
    }

    /// java: the record accessor `amount()`
    pub fn amount(&self) -> Option<i32> {
        self.amount
    }
}

/// java: PrestigeItem's private static final Codec<PrestigeItem> CODEC
static PRESTIGE_ITEM_CODEC: LazyLock<CodecRef<PrestigeItem>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply2(
        record_codec_builder::of(
            |o: &PrestigeItem| o.item_id.clone(),
            codec::field_of(codec::string_codec(), "item_id"),
        ),
        record_codec_builder::of(
            |o: &PrestigeItem| o.costs.clone(),
            codec::field_of(codec::list(GEAR_UPGRADE_CODEC.clone()), "costs"),
        ),
        PrestigeItem::new,
    ))
});

/// java: public record PrestigeItem(String itemId, List<GearUpgrade> costs)
#[derive(Clone, Debug, PartialEq)]
pub struct PrestigeItem {
    pub item_id: String,
    pub costs: Vec<GearUpgrade>,
}

impl PrestigeItem {
    /// java: the canonical record constructor
    pub fn new(item_id: String, costs: Vec<GearUpgrade>) -> Self {
        Self { item_id, costs }
    }

    /// java: the record accessor `itemId()`
    pub fn item_id(&self) -> &str {
        &self.item_id
    }

    /// java: the record accessor `costs()`
    pub fn costs(&self) -> &Vec<GearUpgrade> {
        &self.costs
    }
}

/// java: GemstoneSlot's private static final Codec<GemstoneSlot> CODEC
static GEMSTONE_SLOT_CODEC: LazyLock<CodecRef<GemstoneSlot>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply2(
        record_codec_builder::of(
            |o: &GemstoneSlot| o.r#type.clone(),
            codec::field_of(codec::string_codec(), "slot_type"),
        ),
        record_codec_builder::of(
            |o: &GemstoneSlot| o.costs.clone(),
            codec::optional_field_of_opt(codec::list(GEMSTONE_SLOT_COST_CODEC.clone()), "costs"),
        ),
        GemstoneSlot::new,
    ))
});

/// java: public record GemstoneSlot(String type, Optional<List<GemstoneSlotCost>> costs)
#[derive(Clone, Debug, PartialEq)]
pub struct GemstoneSlot {
    pub r#type: String,
    pub costs: Option<Vec<GemstoneSlotCost>>,
}

impl GemstoneSlot {
    /// java: the canonical record constructor
    pub fn new(r#type: String, costs: Option<Vec<GemstoneSlotCost>>) -> Self {
        Self { r#type, costs }
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    /// java: the record accessor `costs()`
    pub fn costs(&self) -> Option<&Vec<GemstoneSlotCost>> {
        self.costs.as_ref()
    }
}

/// java: GemstoneSlotCost's private static final Codec<GemstoneSlotCost> CODEC
static GEMSTONE_SLOT_COST_CODEC: LazyLock<CodecRef<GemstoneSlotCost>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply4(
        record_codec_builder::of(
            |o: &GemstoneSlotCost| o.r#type.clone(),
            codec::field_of(codec::string_codec(), "type"),
        ),
        record_codec_builder::of(
            |o: &GemstoneSlotCost| o.item_id.clone(),
            codec::optional_field_of_opt(codec::string_codec(), "item_id"),
        ),
        record_codec_builder::of(
            |o: &GemstoneSlotCost| o.amount,
            CodecUtils::optional_int(codec::optional_field_of_opt(codec::int_codec(), "amount")),
        ),
        record_codec_builder::of(
            |o: &GemstoneSlotCost| o.coins,
            CodecUtils::optional_int(codec::optional_field_of_opt(codec::int_codec(), "coins")),
        ),
        GemstoneSlotCost::new,
    ))
});

/// java: public record GemstoneSlotCost(String type, Optional<String> itemId, OptionalInt amount, OptionalInt coins)
/// (inside GemstoneSlot)
#[derive(Clone, Debug, PartialEq)]
pub struct GemstoneSlotCost {
    pub r#type: String,
    pub item_id: Option<String>,
    pub amount: Option<i32>,
    pub coins: Option<i32>,
}

impl GemstoneSlotCost {
    /// java: the canonical record constructor
    pub fn new(r#type: String, item_id: Option<String>, amount: Option<i32>, coins: Option<i32>) -> Self {
        Self {
            r#type,
            item_id,
            amount,
            coins,
        }
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    /// java: the record accessor `itemId()`
    pub fn item_id(&self) -> Option<&String> {
        self.item_id.as_ref()
    }

    /// java: the record accessor `amount()`
    pub fn amount(&self) -> Option<i32> {
        self.amount
    }

    /// java: the record accessor `coins()`
    pub fn coins(&self) -> Option<i32> {
        self.coins
    }
}
