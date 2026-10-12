//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.PopupScreenAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.PopupScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getUnderlyingScreen -> net.minecraft.client.gui.components.PopupScreen.backgroundScreen
// TODO(fabric2seer): `Screen` - return `&obj.background_screen` with its Rust type
// pub fn get_underlying_screen(obj: &mcrust::net::minecraft::client::gui::components::popup_screen::PopupScreen) -> &T { &obj.background_screen }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
