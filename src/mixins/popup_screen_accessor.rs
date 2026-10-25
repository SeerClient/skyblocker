//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.PopupScreenAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.PopupScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getUnderlyingScreen -> net.minecraft.client.gui.components.PopupScreen.backgroundScreen
/// A vanilla `PopupScreen` is the port's `ScreenExtra::Popup(PopupState)`; its `backgroundScreen` is the state's
/// `background_widgets` (the screen it was opened over).
pub fn get_underlying_screen(obj: &mcrust::net::minecraft::client::gui::gui::PopupState) -> &Screen {
    &obj.background_widgets
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
