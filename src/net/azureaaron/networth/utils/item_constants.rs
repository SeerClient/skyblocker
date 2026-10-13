//! java: public interface ItemConstants (net.azureaaron.networth.utils)
//!
//! Only `REFORGES` is read outside the library (ValueBreakdownPopup's reforge formatter); the other constants are
//! read by `ItemCalculator`, which is not ported yet (GAPS_Q #1).

use std::collections::HashMap;
use std::sync::LazyLock;

/// java: Object2ObjectMap<String, String> REFORGES
pub static REFORGES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
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
    map
});
