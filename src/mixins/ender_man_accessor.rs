//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.EnderManAccessor by fabric2seer.
//! @Mixin(net.minecraft.world.entity.monster.EnderMan) - 26.3: renamed `Enderman` (`world/entity/monster/Enderman.java`)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getDATA_CARRY_STATE -> net.minecraft.world.entity.monster.EnderMan.DATA_CARRY_STATE (a static field)
pub fn get_data_carry_state() -> &'static mcrust::net::minecraft::network::syncher::entity_data_accessor::EntityDataAccessor<Option<mcrust::net::minecraft::world::level::block::block::ErasedBlockState>> {
    &mcrust::net::minecraft::world::entity::monster::enderman::accessors().carry_state
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
