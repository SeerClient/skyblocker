//! Ported from de.hysky.skyblocker.skyblock.garden.GardenConstants by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: Map<String, String> CROP_BY_PEST = Map.ofEntries( entry("Fly", "Wheat"), entry("Mosquito", "Sugar Cane"), entry("Cricket", "Carrot"), entry("Locust", "Potato"), entry("Earthworm", "Melon Slice"), entr
pub static CROP_BY_PEST: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("Fly".to_string(), "Wheat".to_string()), ("Mosquito".to_string(), "Sugar Cane".to_string()), ("Cricket".to_string(), "Carrot".to_string()), ("Locust".to_string(), "Potato".to_string()), ("Earthworm".to_string(), "Melon Slice".to_string()), ("Rat".to_string(), "Pumpkin".to_string()), ("Moth".to_string(), "Cocoa Beans".to_string()), ("Beetle".to_string(), "Nether Wart".to_string()), ("Mite".to_string(), "Cactus".to_string()), ("Slug".to_string(), "Mushroom".to_string()), ("Firefly".to_string(), "Moonflower".to_string()), ("Dragonfly".to_string(), "Sunflower".to_string()), ("Praying Mantis".to_string(), "Wild Rose".to_string())]));
/// java: Multimap<String, String> PEST_HEAD_BY_CROP = Util.make(ArrayListMultimap.create(), map -> {
///   map.put("Wheat", HeadTextures.FLY_PEST);
///   map.put("Sugar Cane", HeadTextures.MOSQUITO_PEST);
///   map.put("Carrot", HeadTextures.CRICKET_PEST);
///   map.put("Potato", HeadTextures.LOCUST_PEST);
///   map.put("Melon Slice", HeadTextures.EARTHWORM_PEST);
///   map.put("Melon Slice", HeadTextures.EARTHWORM_PEST_TAIL);
///   map.put("Pumpkin", HeadTextures.RAT_PEST);
///   map.put("Cocoa Beans", HeadTextures.MOTH_PEST);
///   map.put("Nether Wart", HeadTextures.BEETLE_PEST);
///   map.put("Cactus", HeadTextures.MITE_PEST);
///   map.put("Mushroom", HeadTextures.SLUG_PEST);
///   map.put("Moonflower", HeadTextures.FIREFLY_PEST);
///   map.put("Moonflower", HeadTextures.FIREFLY_PEST_FLASH);
///   map.put("Sunflower", HeadTextures.DRAGONFLY_PEST);
///   map.put("Wild Rose", HeadTextures.PRAYING_MANTIS_PEST);
///   map.put("Wheat", HeadTextures.FLY_PEST);
///   map.put("Sugar Cane", HeadTextures.MOSQUITO_PEST);
/// java: });
/// (`ArrayListMultimap`: each key's values in insertion order; `get(key)` of an absent key - also `null` - is an empty
/// collection, which is `get(..).map_or(&[], ..)` here)
pub static PEST_HEAD_BY_CROP: LazyLock<std::collections::HashMap<String, Vec<String>>> = LazyLock::new(|| {
    use crate::de::hysky::skyblocker::skyblock::item::head_textures;
    let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for (crop, head) in [
        ("Wheat", head_textures::FLY_PEST),
        ("Sugar Cane", head_textures::MOSQUITO_PEST),
        ("Carrot", head_textures::CRICKET_PEST),
        ("Potato", head_textures::LOCUST_PEST),
        ("Melon Slice", head_textures::EARTHWORM_PEST),
        ("Melon Slice", head_textures::EARTHWORM_PEST_TAIL),
        ("Pumpkin", head_textures::RAT_PEST),
        ("Cocoa Beans", head_textures::MOTH_PEST),
        ("Nether Wart", head_textures::BEETLE_PEST),
        ("Cactus", head_textures::MITE_PEST),
        ("Mushroom", head_textures::SLUG_PEST),
        ("Moonflower", head_textures::FIREFLY_PEST),
        ("Moonflower", head_textures::FIREFLY_PEST_FLASH),
        ("Sunflower", head_textures::DRAGONFLY_PEST),
        ("Wild Rose", head_textures::PRAYING_MANTIS_PEST),
        ("Wheat", head_textures::FLY_PEST),
        ("Sugar Cane", head_textures::MOSQUITO_PEST),
    ] {
        map.entry(crop.to_string()).or_default().push(head.to_string());
    }
    map
});
/// java: Map<String, String> CROP_BY_VINYL = Map.ofEntries( entry("VINYL_PRETTY_FLY", "Wheat"), entry("VINYL_BUZZIN_BEATS", "Sugar Cane"), entry("VINYL_CRICKET_CHOIR", "Carrot"), entry("VINYL_CICADA_SYMPHONY",
pub static CROP_BY_VINYL: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("VINYL_PRETTY_FLY".to_string(), "Wheat".to_string()), ("VINYL_BUZZIN_BEATS".to_string(), "Sugar Cane".to_string()), ("VINYL_CRICKET_CHOIR".to_string(), "Carrot".to_string()), ("VINYL_CICADA_SYMPHONY".to_string(), "Potato".to_string()), ("VINYL_EARTHWORM_ENSEMBLE".to_string(), "Melon Slice".to_string()), ("VINYL_RODENT_REVOLUTION".to_string(), "Pumpkin".to_string()), ("VINYL_WINGS_OF_HARMONY".to_string(), "Cocoa Beans".to_string()), ("VINYL_BEETLE".to_string(), "Nether Wart".to_string()), ("VINYL_DYNAMITES".to_string(), "Cactus".to_string()), ("VINYL_SLOW_AND_GROOVY".to_string(), "Mushroom".to_string()), ("VINYL_FIREFLY".to_string(), "Moonflower".to_string()), ("VINYL_IMAGINE_DRAGONFLIES".to_string(), "Sunflower".to_string()), ("VINYL_PRAY_FOR_ME".to_string(), "Wild Rose".to_string())]));

/// java: public interface GardenConstants
pub trait GardenConstants: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
}
