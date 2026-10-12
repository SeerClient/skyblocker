//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.BatMixin by fabric2seer.
//! @Mixin(net.minecraft.world.entity.ambient.Bat)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Override
fn on_client_removal(this: &mut mcrust::net::minecraft::world::entity::ambient::bat::Bat) {
    // TODO(fabric2seer): `super.m(..)` outside an instance method
    //   java: super.onClientRemoval();
    if this.base.base.base.get_health() <= (0 as f32) {
        crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager::on_bat_removed(&this);
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
