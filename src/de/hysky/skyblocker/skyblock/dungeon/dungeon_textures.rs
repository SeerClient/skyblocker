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
        // java: ResourceLoader.registerBuiltinPack(SkyblockerMod.id("recolored_dungeon_items"), SkyblockerMod.SKYBLOCKER_MOD, PackActivationType.NORMAL);
        // (the pack is the mod package's `resourcepacks/recolored_dungeon_items`)
        mcrust::mcrust::modding::resources::register_builtin_pack(crate::entry::skyblocker_mod::NAMESPACE, "recolored_dungeon_items", crate::entry::skyblocker_mod::NAMESPACE, mcrust::mcrust::modding::resources::PackActivationType::Normal);
    }

}
