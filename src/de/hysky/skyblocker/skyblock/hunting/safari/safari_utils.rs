//! Ported from de.hysky.skyblocker.skyblock.hunting.safari.SafariUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::sky_block_biomes;
use crate::de::hysky::skyblocker::utils::utils::Utils;

/// java: public class SafariUtils
#[derive(Clone)]
pub struct SafariUtils {}

impl SafariUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isInCavernBiome()
    pub fn is_in_cavern_biome() -> bool {
        return (Utils::is_in_safari() && Utils::is_in_biome(sky_block_biomes::CAVERN.get_mut()));
    }

    /// java: public static boolean isInForestBiome()
    pub fn is_in_forest_biome() -> bool {
        return (Utils::is_in_safari() && Utils::is_in_biome(sky_block_biomes::FOREST.get_mut()));
    }

    /// java: public static boolean isInHauntedBiome()
    pub fn is_in_haunted_biome() -> bool {
        return (Utils::is_in_safari() && Utils::is_in_biome(sky_block_biomes::HAUNTED.get_mut()));
    }

    /// java: public static boolean isInIcyBiome()
    pub fn is_in_icy_biome() -> bool {
        return Utils::is_in_safari() && (Utils::is_in_biome(sky_block_biomes::ICY.get_mut()) || Utils::is_in_biome(sky_block_biomes::ICY_CAVES.get_mut()));
    }

}
