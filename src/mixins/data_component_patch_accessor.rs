//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.DataComponentPatchAccessor by fabric2seer.
//! @Mixin(net.minecraft.core.component.DataComponentPatch)

#[allow(unused_imports)]
use crate::prelude::*;

use mcrust::net::minecraft::core::component::data_component_patch::{DataComponentPatch, PatchMap};

/// @Accessor getMap -> net.minecraft.core.component.DataComponentPatch.map
/// (`Reference2ObjectMap<DataComponentType<?>, Optional<?>>` is the port's `PatchMap`: `(type, Some(value) | None)` pairs.)
pub fn get_map(obj: &DataComponentPatch) -> &PatchMap {
    obj.entry_set()
}

/// @Invoker invokeInit -> net.minecraft.core.component.DataComponentPatch.<init>(Reference2ObjectMap)
pub fn invoke_init(map: PatchMap) -> DataComponentPatch {
    DataComponentPatch::new(map)
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
