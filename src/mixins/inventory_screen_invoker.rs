//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.InventoryScreenInvoker by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.InventoryScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeExtractRenderState -> net.minecraft.client.gui.screens.inventory.InventoryScreen.extractRenderState
// @Invoker: native mods call the method directly: `obj.extract_render_state(..)` (or `mcrust::net::minecraft::client::gui::screens::inventory::inventory_screen::InventoryScreen::extract_render_state(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
