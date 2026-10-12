//! Ported from de.hysky.skyblocker.skyblock.garden.GardenConstants by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: Map<String, String> CROP_BY_PEST = Map.ofEntries( entry("Fly", "Wheat"), entry("Mosquito", "Sugar Cane"), entry("Cricket", "Carrot"), entry("Locust", "Potato"), entry("Earthworm", "Melon Slice"), entr
pub static CROP_BY_PEST: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("Fly".to_string(), "Wheat".to_string()), ("Mosquito".to_string(), "Sugar Cane".to_string()), ("Cricket".to_string(), "Carrot".to_string()), ("Locust".to_string(), "Potato".to_string()), ("Earthworm".to_string(), "Melon Slice".to_string()), ("Rat".to_string(), "Pumpkin".to_string()), ("Moth".to_string(), "Cocoa Beans".to_string()), ("Beetle".to_string(), "Nether Wart".to_string()), ("Mite".to_string(), "Cactus".to_string()), ("Slug".to_string(), "Mushroom".to_string()), ("Firefly".to_string(), "Moonflower".to_string()), ("Dragonfly".to_string(), "Sunflower".to_string()), ("Praying Mantis".to_string(), "Wild Rose".to_string())]));
// TODO(fabric2seer): static call ArrayListMultimap.create(..)
//   java: Multimap<String, String> PEST_HEAD_BY_CROP = Util.make(ArrayListMultimap.create(), map -> {
//   java: 			map.put("Wheat", HeadTextures.FLY_PEST);
//   java: 			map.put("Sugar Cane", HeadTextures.MOSQUITO_PEST);
//   java: 			map.put("Carrot", HeadTextures.CRICKET_PEST);
//   java: 			map.put("Potato", HeadTextures.LOCUST_PEST);
//   java: 			map.put("Melon Slice", HeadTextures.EARTHWORM_PEST);
//   java: 			map.put("Melon Slice", HeadTextures.EARTHWORM_PEST_TAIL);
//   java: 			map.put("Pumpkin", HeadTextures.RAT_PEST);
//   java: 			map.put("Cocoa Beans", HeadTextures.MOTH_PEST);
//   java: 			map.put("Nether Wart", HeadTextures.BEETLE_PEST);
//   java: 			map.put("Cactus", HeadTextures.MITE_PEST);
//   java: 			map.put("Mushroom", HeadTextures.SLUG_PEST);
//   java: 			map.put("Moonflower", HeadTextures.FIREFLY_PEST);
//   java: 			map.put("Moonflower", HeadTextures.FIREFLY_PEST_FLASH);
//   java: 			map.put("Sunflower", HeadTextures.DRAGONFLY_PEST);
//   java: 			map.put("Wild Rose", HeadTextures.PRAYING_MANTIS_PEST);
//   java: 	});
/// java: Multimap<String, String> PEST_HEAD_BY_CROP = Util.make(ArrayListMultimap.create(), map -> { map.put("Wheat", HeadTextures.FLY_PEST); map.put("Sugar Cane", HeadTextures.MOSQUITO_PEST); map.put("Carrot"
pub static PEST_HEAD_BY_CROP: compat::MainThread<crate::unresolved::Multimap> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `PEST_HEAD_BY_CROP` - see the TODO above"));
/// java: Map<String, String> CROP_BY_VINYL = Map.ofEntries( entry("VINYL_PRETTY_FLY", "Wheat"), entry("VINYL_BUZZIN_BEATS", "Sugar Cane"), entry("VINYL_CRICKET_CHOIR", "Carrot"), entry("VINYL_CICADA_SYMPHONY",
pub static CROP_BY_VINYL: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("VINYL_PRETTY_FLY".to_string(), "Wheat".to_string()), ("VINYL_BUZZIN_BEATS".to_string(), "Sugar Cane".to_string()), ("VINYL_CRICKET_CHOIR".to_string(), "Carrot".to_string()), ("VINYL_CICADA_SYMPHONY".to_string(), "Potato".to_string()), ("VINYL_EARTHWORM_ENSEMBLE".to_string(), "Melon Slice".to_string()), ("VINYL_RODENT_REVOLUTION".to_string(), "Pumpkin".to_string()), ("VINYL_WINGS_OF_HARMONY".to_string(), "Cocoa Beans".to_string()), ("VINYL_BEETLE".to_string(), "Nether Wart".to_string()), ("VINYL_DYNAMITES".to_string(), "Cactus".to_string()), ("VINYL_SLOW_AND_GROOVY".to_string(), "Mushroom".to_string()), ("VINYL_FIREFLY".to_string(), "Moonflower".to_string()), ("VINYL_IMAGINE_DRAGONFLIES".to_string(), "Sunflower".to_string()), ("VINYL_PRAY_FOR_ME".to_string(), "Wild Rose".to_string())]));

/// java: public interface GardenConstants
pub trait GardenConstants: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
}
