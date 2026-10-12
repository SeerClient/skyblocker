//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.SlotAccessor by fabric2seer.
//! @Mixin(net.minecraft.world.inventory.Slot)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor setX -> net.minecraft.world.inventory.Slot.x
pub fn set_x(obj: &mut mcrust::net::minecraft::world::inventory::slot::Slot, value: i32) {
    obj.x = value;
}

/// @Accessor setY -> net.minecraft.world.inventory.Slot.y
pub fn set_y(obj: &mut mcrust::net::minecraft::world::inventory::slot::Slot, value: i32) {
    obj.y = value;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
