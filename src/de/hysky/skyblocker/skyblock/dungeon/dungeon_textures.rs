//! Ported from de.hysky.skyblocker.skyblock.dungeon.DungeonTextures by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class DungeonTextures
#[derive(Clone)]
pub struct DungeonTextures {}

impl DungeonTextures {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): Fabric API `ResourceLoader.registerBuiltinPack` has no SeerClient equivalent
        //   java: ResourceLoader.registerBuiltinPack(
        //   java: 				SkyblockerMod.id("recolored_dungeon_items"),
        //   java: 				SkyblockerMod.SKYBLOCKER_MOD,
        //   java: 				PackActivationType.NORMAL
        //   java: 		);
    }

}
