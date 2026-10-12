//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.EnderManAccessor by fabric2seer.
//! @Mixin(net.minecraft.world.entity.monster.EnderMan)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getDATA_CARRY_STATE -> net.minecraft.world.entity.monster.EnderMan.DATA_CARRY_STATE
// TODO(fabric2seer): `EntityDataAccessor<Optional<BlockState>>` - return `&obj.data_carry_state` with its Rust type
// pub fn get_data_carry_state(obj: &mcrust::net::minecraft::world::entity::monster::ender_man::EnderMan) -> &T { &obj.data_carry_state }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
