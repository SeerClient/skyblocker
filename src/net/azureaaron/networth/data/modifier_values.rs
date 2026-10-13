//! java: public record ModifierValues(Object2DoubleMap<String> regular, Object2DoubleMap<String> enchantments)
//! (net.azureaaron.networth.data)

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::net::azureaaron::networth::utils::utils::Utils;

/// java: it.unimi.dsi.fastutil.objects.Object2DoubleMap<String> - the two members the library reads
/// (`getDouble`, `getOrDefault`) over a map with a settable default return value. fastutil is not ported; this is
/// the shape of the one type the record holds.
#[derive(Clone, Debug, PartialEq)]
pub struct Object2DoubleMap {
    map: HashMap<String, f64>,
    default_return_value: f64,
}

impl Object2DoubleMap {
    /// java: new Object2DoubleOpenHashMap<>()
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            default_return_value: 0.0,
        }
    }

    /// java: put(String, double)
    pub fn put(&mut self, key: &str, value: f64) {
        self.map.insert(key.to_string(), value);
    }

    /// java: defaultReturnValue(double)
    pub fn default_return_value(&mut self, value: f64) {
        self.default_return_value = value;
    }

    /// java: getDouble(Object) - the default return value for a missing key
    pub fn get_double(&self, key: &str) -> f64 {
        self.map
            .get(key)
            .copied()
            .unwrap_or(self.default_return_value)
    }

    /// java: getOrDefault(Object, double)
    pub fn get_or_default(&self, key: &str, default_value: f64) -> f64 {
        self.map.get(key).copied().unwrap_or(default_value)
    }

    /// java: containsKey(Object)
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}

impl Default for Object2DoubleMap {
    fn default() -> Self {
        Self::new()
    }
}

/// java: public static final Object2DoubleMap<String> REGULAR_DEFAULTS
pub static REGULAR_DEFAULTS: LazyLock<Object2DoubleMap> = LazyLock::new(|| {
    Utils::make(Object2DoubleMap::new(), |map| {
        map.put("enrichment", 0.5);
        map.put("farming4Dummies", 0.5);
        map.put("powerScroll", 0.5);
        map.put("woodSingularity", 0.5);
        map.put("artOfWar", 0.6);
        map.put("fumingPotatoBook", 0.6);
        map.put("gemstoneSlots", 0.6);
        map.put("runes", 0.6);
        map.put("skins", 0.67); //From some quick math skins have an average of 33% loss in value when applied
        map.put("transmissionTuner", 0.7);
        map.put("pocketSackInASack", 0.7);
        map.put("essence", 0.75);
        map.put("goldenBounty", 0.75);
        map.put("silex", 0.75);
        map.put("artOfPeace", 0.8);
        map.put("divanPowderCoating", 0.8);
        map.put("jalapenoBook", 0.8);
        map.put("manaDisintegrator", 0.8);
        map.put("recombobulator", 0.8);
        map.put("thunderInABottle", 0.8);
        map.put("enchantments", 0.85);
        map.put("shensAuctionPrice", 0.85);
        map.put("dye", 0.9);
        map.put("gemstoneChambers", 0.9);
        map.put("attributes", 1.0);
        map.put("drillPart", 1.0);
        map.put("etherwarp", 1.0);
        map.put("masterStars", 1.0);
        map.put("gemstones", 1.0);
        map.put("hotPotatoBook", 1.0);
        map.put("polarvoidBook", 1.0);
        map.put("prestigeItem", 1.0);
        map.put("reforge", 1.0);
        map.put("winningBid", 1.0);
        map.put("witherScroll", 1.0);
        map.put("petCandy", 0.65);
        map.put("soulboundPetSkins", 0.8);
        map.put("petItem", 1.0);

        //Set default return value
        map.default_return_value(1.0);
    })
});

/// java: public static final Object2DoubleMap<String> ENCHANTMENT_DEFAULTS
pub static ENCHANTMENT_DEFAULTS: LazyLock<Object2DoubleMap> = LazyLock::new(|| {
    Utils::make(Object2DoubleMap::new(), |map| {
        map.put("COUNTER_STRIKE", 0.2);
        map.put("BIG_BRAIN", 0.35);
        map.put("ULTIMATE_INFERNO", 0.35);
        map.put("OVERLOAD", 0.35);
        map.put("ULTIMATE_SOUL_EATER", 0.35);
        map.put("ULTIMATE_FATAL_TEMPO", 0.65);

        //Set default return value
        map.default_return_value(1.0);
    })
});

/// java: public static final ModifierValues DEFAULT = new ModifierValues(REGULAR_DEFAULTS, ENCHANTMENT_DEFAULTS);
pub static DEFAULT: LazyLock<ModifierValues> =
    LazyLock::new(|| ModifierValues::new(REGULAR_DEFAULTS.clone(), ENCHANTMENT_DEFAULTS.clone()));

/// java: public record ModifierValues(Object2DoubleMap<String> regular, Object2DoubleMap<String> enchantments)
///
/// @implSpec The default return value of both maps must be `1`.
#[derive(Clone, Debug, PartialEq)]
pub struct ModifierValues {
    pub regular: Object2DoubleMap,
    pub enchantments: Object2DoubleMap,
}

impl ModifierValues {
    /// java: the canonical record constructor
    pub fn new(regular: Object2DoubleMap, enchantments: Object2DoubleMap) -> Self {
        Self {
            regular,
            enchantments,
        }
    }

    /// java: the record accessor `regular()`
    pub fn regular(&self) -> &Object2DoubleMap {
        &self.regular
    }

    /// java: the record accessor `enchantments()`
    pub fn enchantments(&self) -> &Object2DoubleMap {
        &self.enchantments
    }
}
