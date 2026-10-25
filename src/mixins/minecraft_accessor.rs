//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.MinecraftAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.Minecraft)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getProfileKeyPairManagerField -> net.minecraft.client.Minecraft.profileKeyPairManager
/// (`None` only before the game installs it; Java's field is final and set by the constructor.)
pub fn get_profile_key_pair_manager_field(
    obj: &Minecraft,
) -> std::sync::MutexGuard<'static, Option<mcrust::net::minecraft::client::multiplayer::profile_key_pair_manager::ProfileKeyPairManager>> {
    obj.get_profile_key_pair_manager()
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
