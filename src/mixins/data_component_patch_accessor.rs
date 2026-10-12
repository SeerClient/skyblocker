//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.DataComponentPatchAccessor by fabric2seer.
//! @Mixin(net.minecraft.core.component.DataComponentPatch)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getMap -> net.minecraft.core.component.DataComponentPatch.map
// TODO(fabric2seer): `Reference2ObjectMap<DataComponentType<?>, Optional<?>>` - return `&obj.map` with its Rust type
// pub fn get_map(obj: &mcrust::net::minecraft::core::component::data_component_patch::DataComponentPatch) -> &T { &obj.map }

/// @Invoker invokeInit -> net.minecraft.core.component.DataComponentPatch.<init>
// @Invoker: native mods call the method directly: `obj.<init>(..)` (or `mcrust::net::minecraft::core::component::data_component_patch::DataComponentPatch::<init>(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
