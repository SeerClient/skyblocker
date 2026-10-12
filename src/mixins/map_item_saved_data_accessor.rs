//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.MapItemSavedDataAccessor by fabric2seer.
//! @Mixin(net.minecraft.world.level.saveddata.maps.MapItemSavedData)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getDecorations -> net.minecraft.world.level.saveddata.maps.MapItemSavedData.decorations
// TODO(fabric2seer): `Map<String, MapDecoration>` - return `&obj.decorations` with its Rust type
// pub fn get_decorations(obj: &mcrust::net::minecraft::world::level::saveddata::maps::map_item_saved_data::MapItemSavedData) -> &T { &obj.decorations }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
