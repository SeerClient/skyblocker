//! java: public interface ItemConstants (net.azureaaron.networth.utils)

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use super::prestige_constants;

/// java: Set<String> COLOURED_ANNIVERSARY_ITEMS
pub static COLOURED_ANNIVERSARY_ITEMS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "PARTY_HAT_CRAB",
        "PARTY_HAT_CRAB_ANIMATED",
        "BALLOON_HAT_2024",
    ])
});

/// java: int PICKONIMBUS_DURABILITY = 2000;
pub const PICKONIMBUS_DURABILITY: i32 = 2000;

/// java: Map<String, Set<String>> BLOCKED_ENCHANTMENTS
pub static BLOCKED_ENCHANTMENTS: LazyLock<HashMap<&'static str, HashSet<&'static str>>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                "BONE_BOOMERANG",
                HashSet::from(["OVERLOAD", "POWER", "ULTIMATE_SOUL_EATER"]),
            ),
            (
                "DEATH_BOW",
                HashSet::from(["OVERLOAD", "POWER", "ULTIMATE_SOUL_EATER"]),
            ),
            ("GARDENING_AXE", HashSet::from(["REPLENISH"])),
            ("GARDENING_HOE", HashSet::from(["REPLENISH"])),
            ("ADVANCED_GARDENING_AXE", HashSet::from(["REPLENISH"])),
            ("ADVANCED_GARDENING_HOE", HashSet::from(["REPLENISH"])),
        ])
    });

/// java: Object2IntMap<String> IGNORED_ENCHANTMENTS
pub static IGNORED_ENCHANTMENTS: LazyLock<HashMap<&'static str, i32>> = LazyLock::new(|| {
    super::utils::Utils::make(HashMap::new(), |map| {
        map.insert("SCAVENGER", 5);
    })
});

/// java: Set<String> STACKING_ENCHANTMENTS
pub static STACKING_ENCHANTMENTS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "EXPERTISE",
        "COMPACT",
        "CULTIVATING",
        "CHAMPION",
        "HECATOMB",
        "TOXOPHILITE",
    ])
});

/// java: Set<String> IGNORE_SILEX
pub static IGNORE_SILEX: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["PROMISING_SPADE", "STONK_PICKAXE"]));

/// java: Map<String, String> ATTRIBUTE_BASE_COSTS
pub static ATTRIBUTE_BASE_COSTS: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([
            ("GLOWSTONE_GAUNTLET", "GLOWSTONE_GAUNTLET"),
            ("VANQUISHED_GLOWSTONE_GAUNTLET", "GLOWSTONE_GAUNTLET"),
            ("BLAZE_BELT", "BLAZE_BELT"),
            ("VANQUISHED_BLAZE_BELT", "BLAZE_BELT"),
            ("MAGMA_NECKLACE", "MAGMA_NECKLACE"),
            ("VANQUISHED_MAGMA_NECKLACE", "MAGMA_NECKLACE"),
            ("MAGMA_ROD", "MAGMA_ROD"),
            ("INFERNO_ROD", "MAGMA_ROD"),
            ("HELLFIRE_ROD", "MAGMA_ROD"),
        ])
    });

/// java: Set<String> VALID_RUNES
pub static VALID_RUNES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "MUSIC_RUNE_1",
        "MUSIC_RUNE_2",
        "MUSIC_RUNE_3",
        "MEOW_MUSIC_RUNE_3",
        "ENCHANT_RUNE_1",
        "ENCHANT_RUNE_2",
        "ENCHANT_RUNE_3",
        "GRAND_SEARING_RUNE_3",
        "SPELLBOUND_RUNE_3",
        "GRAND_FREEZING_RUNE_3",
        "PRIMAL_FEAR_RUNE_3",
        "GOLDEN_CARPET_RUNE_3",
        "ICE_SKATES_RUNE_3",
        "BARK_TUNES_RUNE_3",
        "SMITTEN_RUNE_3",
        "RAINY_DAY_RUNE_3",
        "SUPER_PUMPKIN_RUNE_3",
    ])
});

/// java: Set<String> ENRICHMENTS
pub static ENRICHMENTS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "TALISMAN_ENRICHMENT_ATTACK_SPEED",
        "TALISMAN_ENRICHMENT_CRITICAL_CHANCE",
        "TALISMAN_ENRICHMENT_CRITICAL_DAMAGE",
        "TALISMAN_ENRICHMENT_DEFENSE",
        "TALISMAN_ENRICHMENT_FEROCITY",
        "TALISMAN_ENRICHMENT_HEALTH",
        "TALISMAN_ENRICHMENT_INTELLIGENCE",
        "TALISMAN_ENRICHMENT_MAGIC_FIND",
        "TALISMAN_ENRICHMENT_SEA_CREATURE_CHANCE",
        "TALISMAN_ENRICHMENT_STRENGTH",
        "TALISMAN_ENRICHMENT_WALK_SPEED",
    ])
});

/// java: Object2ObjectMap<String, String> REFORGES
pub static REFORGES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    super::utils::Utils::make(HashMap::new(), |map| {
        //Page 1
        map.insert("stiff", "HARDENED_WOOD");
        map.insert("salty", "SALT_CUBE");
        map.insert("aote_stone", "AOTE_STONE");
        map.insert("blazing", "BLAZEN_SPHERE");
        map.insert("waxed", "BLAZE_WAX");
        map.insert("rooted", "BURROWING_SPORES");
        map.insert("candied", "CANDY_CORN");
        map.insert("perfect", "DIAMOND_ATOM");
        map.insert("fleet", "DIAMONITE");
        map.insert("fabled", "DRAGON_CLAW");
        map.insert("spiked", "DRAGON_SCALE");
        map.insert("royal", "DWARVEN_TREASURE");
        map.insert("hyper", "ENDSTONE_GEODE");
        map.insert("coldfusion", "ENTROPY_SUPPRESSOR");
        map.insert("blooming", "FLOWERING_BOUQUET");
        map.insert("fanged", "FULL_JAW_FANGED_KIT");
        map.insert("jaded", "JADERALD");
        map.insert("jerry", "JERRY_STONE");
        map.insert("magnetic", "LAPIS_CRYSTAL");
        map.insert("earthy", "LARGE_WALNUT");
        map.insert("fortified", "METEOR_SHARD");
        map.insert("gilded", "MIDAS_JEWEL");
        map.insert("cubic", "MOLTEN_CUBE");
        map.insert("necrotic", "NECROMANCER_BROOCH");
        map.insert("fruitful", "ONYX"); //Black Diamond
        map.insert("precise", "OPTICAL_LENS");
        map.insert("mossy", "OVERGROWN_GRASS");
        map.insert("pitchin", "PITCHIN_KOI");
        //Page 2
        map.insert("undead", "PREMIUM_FLESH");
        map.insert("blood_soaked", "PRESUMED_GALLON_OF_RED_PAINT");
        map.insert("mithraic", "PURE_MITHRIL");
        map.insert("reinforced", "RARE_DIAMOND");
        map.insert("ridiculous", "RED_NOSE");
        map.insert("loving", "RED_SCARF");
        map.insert("auspicious", "ROCK_GEMSTONE"); //Dwarven Geode
        map.insert("treacherous", "RUSTY_ANCHOR");
        map.insert("headstrong", "SALMON_OPAL"); //Kaleidoscopic Mineral
        map.insert("strengthened", "SEARING_STONE");
        map.insert("glistening", "SHINY_PRISM");
        map.insert("bustling", "SKYMART_BROCHURE");
        map.insert("spiritual", "SPIRIT_DECOY");
        map.insert("suspicious", "SUSPICIOUS_VIAL");
        map.insert("snowy", "TERRY_SNOWGLOBE");
        map.insert("dimensional", "TITANIUM_TESSERACT");
        map.insert("ambered", "AMBER_MATERIAL");
        map.insert("beady", "BEADY_EYES");
        map.insert("blessed", "BLESSED_FRUIT");
        map.insert("bulky", "BULKY_STONE");
        map.insert("buzzing", "CLIPPED_WINGS");
        map.insert("submerged", "DEEP_SEA_ORB");
        map.insert("renowned", "DRAGON_HORN");
        map.insert("festive", "FROZEN_BAUBLE");
        map.insert("giant", "GIANT_TOOTH");
        map.insert("lustrous", "GLEAMING_CRYSTAL");
        map.insert("bountiful", "GOLDEN_BALL");
        map.insert("chomp", "KUUDRA_MANDIBLE");
        //Page 3
        map.insert("lucky", "LUCKY_DICE");
        map.insert("stellar", "PETRIFIED_STARFALL");
        map.insert("scraped", "POCKET_ICEBERG");
        map.insert("ancient", "PRECURSOR_GEAR");
        map.insert("refined", "REFINED_AMBER"); //Andesite Whetstone
        map.insert("empowered", "SADAN_BROOCH");
        map.insert("withered", "WITHER_BLOOD");
        map.insert("glacial", "FRIGID_HUSK");
        map.insert("heated", "HOT_STUFF"); //Scorched Topaz
        //Non Bazaar
        map.insert("dirty", "DIRT_BOTTLE");
        map.insert("moil", "MOIL_LOG");
        map.insert("toil", "TOIL_LOG");
        map.insert("greater_spook", "BOO_STONE");
    })
});

/// java: Set<String> GEMSTONE_GROUP_SLOT_TYPES
pub static GEMSTONE_GROUP_SLOT_TYPES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "COMBAT",
        "OFFENSIVE",
        "DEFENSIVE",
        "MINING",
        "UNIVERSAL",
        "CHISEL",
    ])
});
/// java: Set<String> DIVAN_GEAR
pub static DIVAN_GEAR: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "DIVAN_HELMET",
        "DIVAN_CHESTPLATE",
        "DIVAN_LEGGINGS",
        "DIVAN_BOOTS",
        "DIVAN_PENDANT",
    ])
});

/// java: Set<String> KUUDRA_ARMOUR = getKuudraArmourIds().collect(Collectors.toUnmodifiableSet());
pub static KUUDRA_ARMOUR: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| get_kuudra_armour_ids().collect());
/// java: Set<String> KUUDRA_HELMETS = getKuudraArmourIds().filter(id -> id.endsWith("_HELMET"))..
pub static KUUDRA_HELMETS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    get_kuudra_armour_ids()
        .filter(|id| id.ends_with("_HELMET"))
        .collect()
});
/// java: Set<String> KUUDRA_OTHERS = getKuudraArmourIds().filter(id -> !id.endsWith("_HELMET"))..
pub static KUUDRA_OTHERS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    get_kuudra_armour_ids()
        .filter(|id| !id.ends_with("_HELMET"))
        .collect()
});

/// java: List<String> MASTER_STARS
pub static MASTER_STARS: [&str; 5] = [
    "FIRST_MASTER_STAR",
    "SECOND_MASTER_STAR",
    "THIRD_MASTER_STAR",
    "FOURTH_MASTER_STAR",
    "FIFTH_MASTER_STAR",
];

/// java: private static Stream<String> getKuudraArmourIds()
fn get_kuudra_armour_ids() -> impl Iterator<Item = &'static str> {
    let mut collected_ids: HashSet<&'static str> = HashSet::new();

    for (key, value) in prestige_constants::PRESTIGES.iter() {
        collected_ids.insert(*key);

        //We also have to iterate the set since there isn't any map entries for basic kuudra armour pieces
        for id in value.iter() {
            collected_ids.insert(*id);
        }
    }

    collected_ids.into_iter()
}
