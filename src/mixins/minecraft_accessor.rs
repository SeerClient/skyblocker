//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.MinecraftAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.Minecraft)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getProfileKeyPairManagerField -> net.minecraft.client.Minecraft.profileKeyPairManager
// TODO(fabric2seer): `ProfileKeyPairManager` - return `&obj.profile_key_pair_manager` with its Rust type
// pub fn get_profile_key_pair_manager_field(obj: &mcrust::net::minecraft::client::minecraft::Minecraft) -> &T { &obj.profile_key_pair_manager }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
