//! java: public interface PrestigeConstants (net.azureaaron.networth.utils)

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// java: Object2ObjectMap<String, Set<String>> PRESTIGES
pub static PRESTIGES: LazyLock<HashMap<&'static str, HashSet<&'static str>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        //Crimson Armour
        map.insert("HOT_CRIMSON_HELMET", HashSet::from(["CRIMSON_HELMET"]));
        map.insert(
            "HOT_CRIMSON_CHESTPLATE",
            HashSet::from(["CRIMSON_CHESTPLATE"]),
        );
        map.insert("HOT_CRIMSON_LEGGINGS", HashSet::from(["CRIMSON_LEGGINGS"]));
        map.insert("HOT_CRIMSON_BOOTS", HashSet::from(["CRIMSON_BOOTS"]));
        map.insert(
            "BURNING_CRIMSON_HELMET",
            HashSet::from(["HOT_CRIMSON_HELMET", "CRIMSON_HELMET"]),
        );
        map.insert(
            "BURNING_CRIMSON_CHESTPLATE",
            HashSet::from(["HOT_CRIMSON_CHESTPLATE", "CRIMSON_CHESTPLATE"]),
        );
        map.insert(
            "BURNING_CRIMSON_LEGGINGS",
            HashSet::from(["HOT_CRIMSON_LEGGINGS", "CRIMSON_LEGGINGS"]),
        );
        map.insert(
            "BURNING_CRIMSON_BOOTS",
            HashSet::from(["HOT_CRIMSON_BOOTS", "CRIMSON_BOOTS"]),
        );
        map.insert(
            "FIERY_CRIMSON_HELMET",
            HashSet::from([
                "BURNING_CRIMSON_HELMET",
                "HOT_CRIMSON_HELMET",
                "CRIMSON_HELMET",
            ]),
        );
        map.insert(
            "FIERY_CRIMSON_CHESTPLATE",
            HashSet::from([
                "BURNING_CRIMSON_CHESTPLATE",
                "HOT_CRIMSON_CHESTPLATE",
                "CRIMSON_CHESTPLATE",
            ]),
        );
        map.insert(
            "FIERY_CRIMSON_LEGGINGS",
            HashSet::from([
                "BURNING_CRIMSON_LEGGINGS",
                "HOT_CRIMSON_LEGGINGS",
                "CRIMSON_LEGGINGS",
            ]),
        );
        map.insert(
            "FIERY_CRIMSON_BOOTS",
            HashSet::from([
                "BURNING_CRIMSON_BOOTS",
                "HOT_CRIMSON_BOOTS",
                "CRIMSON_BOOTS",
            ]),
        );
        map.insert(
            "INFERNAL_CRIMSON_HELMET",
            HashSet::from([
                "FIERY_CRIMSON_HELMET",
                "BURNING_CRIMSON_HELMET",
                "HOT_CRIMSON_HELMET",
                "CRIMSON_HELMET",
            ]),
        );
        map.insert(
            "INFERNAL_CRIMSON_CHESTPLATE",
            HashSet::from([
                "FIERY_CRIMSON_CHESTPLATE",
                "BURNING_CRIMSON_CHESTPLATE",
                "HOT_CRIMSON_CHESTPLATE",
                "CRIMSON_CHESTPLATE",
            ]),
        );
        map.insert(
            "INFERNAL_CRIMSON_LEGGINGS",
            HashSet::from([
                "FIERY_CRIMSON_LEGGINGS",
                "BURNING_CRIMSON_LEGGINGS",
                "HOT_CRIMSON_LEGGINGS",
                "CRIMSON_LEGGINGS",
            ]),
        );
        map.insert(
            "INFERNAL_CRIMSON_BOOTS",
            HashSet::from([
                "FIERY_CRIMSON_BOOTS",
                "BURNING_CRIMSON_BOOTS",
                "HOT_CRIMSON_BOOTS",
                "CRIMSON_BOOTS",
            ]),
        );
        //Aurora Armour
        map.insert("HOT_AURORA_HELMET", HashSet::from(["AURORA_HELMET"]));
        map.insert(
            "HOT_AURORA_CHESTPLATE",
            HashSet::from(["AURORA_CHESTPLATE"]),
        );
        map.insert("HOT_AURORA_LEGGINGS", HashSet::from(["AURORA_LEGGINGS"]));
        map.insert("HOT_AURORA_BOOTS", HashSet::from(["AURORA_BOOTS"]));
        map.insert(
            "BURNING_AURORA_HELMET",
            HashSet::from(["HOT_AURORA_HELMET", "AURORA_HELMET"]),
        );
        map.insert(
            "BURNING_AURORA_CHESTPLATE",
            HashSet::from(["HOT_AURORA_CHESTPLATE", "AURORA_CHESTPLATE"]),
        );
        map.insert(
            "BURNING_AURORA_LEGGINGS",
            HashSet::from(["HOT_AURORA_LEGGINGS", "AURORA_LEGGINGS"]),
        );
        map.insert(
            "BURNING_AURORA_BOOTS",
            HashSet::from(["HOT_AURORA_BOOTS", "AURORA_BOOTS"]),
        );
        map.insert(
            "FIERY_AURORA_HELMET",
            HashSet::from([
                "BURNING_AURORA_HELMET",
                "HOT_AURORA_HELMET",
                "AURORA_HELMET",
            ]),
        );
        map.insert(
            "FIERY_AURORA_CHESTPLATE",
            HashSet::from([
                "BURNING_AURORA_CHESTPLATE",
                "HOT_AURORA_CHESTPLATE",
                "AURORA_CHESTPLATE",
            ]),
        );
        map.insert(
            "FIERY_AURORA_LEGGINGS",
            HashSet::from([
                "BURNING_AURORA_LEGGINGS",
                "HOT_AURORA_LEGGINGS",
                "AURORA_LEGGINGS",
            ]),
        );
        map.insert(
            "FIERY_AURORA_BOOTS",
            HashSet::from(["BURNING_AURORA_BOOTS", "HOT_AURORA_BOOTS", "AURORA_BOOTS"]),
        );
        map.insert(
            "INFERNAL_AURORA_HELMET",
            HashSet::from([
                "FIERY_AURORA_HELMET",
                "BURNING_AURORA_HELMET",
                "HOT_AURORA_HELMET",
                "AURORA_HELMET",
            ]),
        );
        map.insert(
            "INFERNAL_AURORA_CHESTPLATE",
            HashSet::from([
                "FIERY_AURORA_CHESTPLATE",
                "BURNING_AURORA_CHESTPLATE",
                "HOT_AURORA_CHESTPLATE",
                "AURORA_CHESTPLATE",
            ]),
        );
        map.insert(
            "INFERNAL_AURORA_LEGGINGS",
            HashSet::from([
                "FIERY_AURORA_LEGGINGS",
                "BURNING_AURORA_LEGGINGS",
                "HOT_AURORA_LEGGINGS",
                "AURORA_LEGGINGS",
            ]),
        );
        map.insert(
            "INFERNAL_AURORA_BOOTS",
            HashSet::from([
                "FIERY_AURORA_BOOTS",
                "BURNING_AURORA_BOOTS",
                "HOT_AURORA_BOOTS",
                "AURORA_BOOTS",
            ]),
        );
        //Terror Armour
        map.insert("HOT_TERROR_HELMET", HashSet::from(["TERROR_HELMET"]));
        map.insert(
            "HOT_TERROR_CHESTPLATE",
            HashSet::from(["TERROR_CHESTPLATE"]),
        );
        map.insert("HOT_TERROR_LEGGINGS", HashSet::from(["TERROR_LEGGINGS"]));
        map.insert("HOT_TERROR_BOOTS", HashSet::from(["TERROR_BOOTS"]));
        map.insert(
            "BURNING_TERROR_HELMET",
            HashSet::from(["HOT_TERROR_HELMET", "TERROR_HELMET"]),
        );
        map.insert(
            "BURNING_TERROR_CHESTPLATE",
            HashSet::from(["HOT_TERROR_CHESTPLATE", "TERROR_CHESTPLATE"]),
        );
        map.insert(
            "BURNING_TERROR_LEGGINGS",
            HashSet::from(["HOT_TERROR_LEGGINGS", "TERROR_LEGGINGS"]),
        );
        map.insert(
            "BURNING_TERROR_BOOTS",
            HashSet::from(["HOT_TERROR_BOOTS", "TERROR_BOOTS"]),
        );
        map.insert(
            "FIERY_TERROR_HELMET",
            HashSet::from([
                "BURNING_TERROR_HELMET",
                "HOT_TERROR_HELMET",
                "TERROR_HELMET",
            ]),
        );
        map.insert(
            "FIERY_TERROR_CHESTPLATE",
            HashSet::from([
                "BURNING_TERROR_CHESTPLATE",
                "HOT_TERROR_CHESTPLATE",
                "TERROR_CHESTPLATE",
            ]),
        );
        map.insert(
            "FIERY_TERROR_LEGGINGS",
            HashSet::from([
                "BURNING_TERROR_LEGGINGS",
                "HOT_TERROR_LEGGINGS",
                "TERROR_LEGGINGS",
            ]),
        );
        map.insert(
            "FIERY_TERROR_BOOTS",
            HashSet::from(["BURNING_TERROR_BOOTS", "HOT_TERROR_BOOTS", "TERROR_BOOTS"]),
        );
        map.insert(
            "INFERNAL_TERROR_HELMET",
            HashSet::from([
                "FIERY_TERROR_HELMET",
                "BURNING_TERROR_HELMET",
                "HOT_TERROR_HELMET",
                "TERROR_HELMET",
            ]),
        );
        map.insert(
            "INFERNAL_TERROR_CHESTPLATE",
            HashSet::from([
                "FIERY_TERROR_CHESTPLATE",
                "BURNING_TERROR_CHESTPLATE",
                "HOT_TERROR_CHESTPLATE",
                "TERROR_CHESTPLATE",
            ]),
        );
        map.insert(
            "INFERNAL_TERROR_LEGGINGS",
            HashSet::from([
                "FIERY_TERROR_LEGGINGS",
                "BURNING_TERROR_LEGGINGS",
                "HOT_TERROR_LEGGINGS",
                "TERROR_LEGGINGS",
            ]),
        );
        map.insert(
            "INFERNAL_TERROR_BOOTS",
            HashSet::from([
                "FIERY_TERROR_BOOTS",
                "BURNING_TERROR_BOOTS",
                "HOT_TERROR_BOOTS",
                "TERROR_BOOTS",
            ]),
        );
        //Hollow Armour
        map.insert("HOT_HOLLOW_HELMET", HashSet::from(["HOLLOW_HELMET"]));
        map.insert(
            "HOT_HOLLOW_CHESTPLATE",
            HashSet::from(["HOLLOW_CHESTPLATE"]),
        );
        map.insert("HOT_HOLLOW_LEGGINGS", HashSet::from(["HOLLOW_LEGGINGS"]));
        map.insert("HOT_HOLLOW_BOOTS", HashSet::from(["HOLLOW_BOOTS"]));
        map.insert(
            "BURNING_HOLLOW_HELMET",
            HashSet::from(["HOT_HOLLOW_HELMET", "HOLLOW_HELMET"]),
        );
        map.insert(
            "BURNING_HOLLOW_CHESTPLATE",
            HashSet::from(["HOT_HOLLOW_CHESTPLATE", "HOLLOW_CHESTPLATE"]),
        );
        map.insert(
            "BURNING_HOLLOW_LEGGINGS",
            HashSet::from(["HOT_HOLLOW_LEGGINGS", "HOLLOW_LEGGINGS"]),
        );
        map.insert(
            "BURNING_HOLLOW_BOOTS",
            HashSet::from(["HOT_HOLLOW_BOOTS", "HOLLOW_BOOTS"]),
        );
        map.insert(
            "FIERY_HOLLOW_HELMET",
            HashSet::from([
                "BURNING_HOLLOW_HELMET",
                "HOT_HOLLOW_HELMET",
                "HOLLOW_HELMET",
            ]),
        );
        map.insert(
            "FIERY_HOLLOW_CHESTPLATE",
            HashSet::from([
                "BURNING_HOLLOW_CHESTPLATE",
                "HOT_HOLLOW_CHESTPLATE",
                "HOLLOW_CHESTPLATE",
            ]),
        );
        map.insert(
            "FIERY_HOLLOW_LEGGINGS",
            HashSet::from([
                "BURNING_HOLLOW_LEGGINGS",
                "HOT_HOLLOW_LEGGINGS",
                "HOLLOW_LEGGINGS",
            ]),
        );
        map.insert(
            "FIERY_HOLLOW_BOOTS",
            HashSet::from(["BURNING_HOLLOW_BOOTS", "HOT_HOLLOW_BOOTS", "HOLLOW_BOOTS"]),
        );
        map.insert(
            "INFERNAL_HOLLOW_HELMET",
            HashSet::from([
                "FIERY_HOLLOW_HELMET",
                "BURNING_HOLLOW_HELMET",
                "HOT_HOLLOW_HELMET",
                "HOLLOW_HELMET",
            ]),
        );
        map.insert(
            "INFERNAL_HOLLOW_CHESTPLATE",
            HashSet::from([
                "FIERY_HOLLOW_CHESTPLATE",
                "BURNING_HOLLOW_CHESTPLATE",
                "HOT_HOLLOW_CHESTPLATE",
                "HOLLOW_CHESTPLATE",
            ]),
        );
        map.insert(
            "INFERNAL_HOLLOW_LEGGINGS",
            HashSet::from([
                "FIERY_HOLLOW_LEGGINGS",
                "BURNING_HOLLOW_LEGGINGS",
                "HOT_HOLLOW_LEGGINGS",
                "HOLLOW_LEGGINGS",
            ]),
        );
        map.insert(
            "INFERNAL_HOLLOW_BOOTS",
            HashSet::from([
                "FIERY_HOLLOW_BOOTS",
                "BURNING_HOLLOW_BOOTS",
                "HOT_HOLLOW_BOOTS",
                "HOLLOW_BOOTS",
            ]),
        );
        //Fervor Armour
        map.insert("HOT_FERVOR_HELMET", HashSet::from(["FERVOR_HELMET"]));
        map.insert(
            "HOT_FERVOR_CHESTPLATE",
            HashSet::from(["FERVOR_CHESTPLATE"]),
        );
        map.insert("HOT_FERVOR_LEGGINGS", HashSet::from(["FERVOR_LEGGINGS"]));
        map.insert("HOT_FERVOR_BOOTS", HashSet::from(["FERVOR_BOOTS"]));
        map.insert(
            "BURNING_FERVOR_HELMET",
            HashSet::from(["HOT_FERVOR_HELMET", "FERVOR_HELMET"]),
        );
        map.insert(
            "BURNING_FERVOR_CHESTPLATE",
            HashSet::from(["HOT_FERVOR_CHESTPLATE", "FERVOR_CHESTPLATE"]),
        );
        map.insert(
            "BURNING_FERVOR_LEGGINGS",
            HashSet::from(["HOT_FERVOR_LEGGINGS", "FERVOR_LEGGINGS"]),
        );
        map.insert(
            "BURNING_FERVOR_BOOTS",
            HashSet::from(["HOT_FERVOR_BOOTS", "FERVOR_BOOTS"]),
        );
        map.insert(
            "FIERY_FERVOR_HELMET",
            HashSet::from([
                "BURNING_FERVOR_HELMET",
                "HOT_FERVOR_HELMET",
                "FERVOR_HELMET",
            ]),
        );
        map.insert(
            "FIERY_FERVOR_CHESTPLATE",
            HashSet::from([
                "BURNING_FERVOR_CHESTPLATE",
                "HOT_FERVOR_CHESTPLATE",
                "FERVOR_CHESTPLATE",
            ]),
        );
        map.insert(
            "FIERY_FERVOR_LEGGINGS",
            HashSet::from([
                "BURNING_FERVOR_LEGGINGS",
                "HOT_FERVOR_LEGGINGS",
                "FERVOR_LEGGINGS",
            ]),
        );
        map.insert(
            "FIERY_FERVOR_BOOTS",
            HashSet::from(["BURNING_FERVOR_BOOTS", "HOT_FERVOR_BOOTS", "FERVOR_BOOTS"]),
        );
        map.insert(
            "INFERNAL_FERVOR_HELMET",
            HashSet::from([
                "FIERY_FERVOR_HELMET",
                "BURNING_FERVOR_HELMET",
                "HOT_FERVOR_HELMET",
                "FERVOR_HELMET",
            ]),
        );
        map.insert(
            "INFERNAL_FERVOR_CHESTPLATE",
            HashSet::from([
                "FIERY_FERVOR_CHESTPLATE",
                "BURNING_FERVOR_CHESTPLATE",
                "HOT_FERVOR_CHESTPLATE",
                "FERVOR_CHESTPLATE",
            ]),
        );
        map.insert(
            "INFERNAL_FERVOR_LEGGINGS",
            HashSet::from([
                "FIERY_FERVOR_LEGGINGS",
                "BURNING_FERVOR_LEGGINGS",
                "HOT_FERVOR_LEGGINGS",
                "FERVOR_LEGGINGS",
            ]),
        );
        map.insert(
            "INFERNAL_FERVOR_BOOTS",
            HashSet::from([
                "FIERY_FERVOR_BOOTS",
                "BURNING_FERVOR_BOOTS",
                "HOT_FERVOR_BOOTS",
                "FERVOR_BOOTS",
            ]),
        );
        map
    });
