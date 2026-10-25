//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.ScreenAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.Screen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor setTitle -> net.minecraft.client.gui.screens.Screen.title (`@Mutable`: the field is final in Java)
pub fn set_title(obj: &mut Screen, value: impl Into<String>) {
    obj.title = value.into();
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
