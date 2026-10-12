//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.CustomDataAccessor by fabric2seer.
//! @Mixin(net.minecraft.world.item.component.CustomData)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getTag -> net.minecraft.world.item.component.CustomData.tag
// TODO(fabric2seer): `CompoundTag` - return `&obj.tag` with its Rust type
// pub fn get_tag(obj: &mcrust::net::minecraft::world::item::component::custom_data::CustomData) -> &T { &obj.tag }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
