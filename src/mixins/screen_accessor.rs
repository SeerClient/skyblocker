//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.ScreenAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.Screen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor setTitle -> net.minecraft.client.gui.screens.Screen.title
// TODO(fabric2seer): `Component` - assign `obj.title` with its Rust type

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
