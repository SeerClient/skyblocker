//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.MapItemSavedDataAccessor by fabric2seer.
//! @Mixin(net.minecraft.world.level.saveddata.maps.MapItemSavedData)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getDecorations -> net.minecraft.world.level.saveddata.maps.MapItemSavedData.decorations
/// (`Map<String, MapDecoration>`, kept in insertion order as the port's `(key, decoration)` list.)
pub fn get_decorations(
    obj: &mcrust::net::minecraft::world::level::saveddata::maps::map_item_saved_data::MapItemSavedData,
) -> &Vec<(String, mcrust::net::minecraft::world::level::saveddata::maps::map_decoration::MapDecoration)> {
    &obj.decorations
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
