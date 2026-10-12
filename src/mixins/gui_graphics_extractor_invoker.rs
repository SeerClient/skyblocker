//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.GuiGraphicsExtractorInvoker by fabric2seer.
//! @Mixin(net.minecraft.client.gui.GuiGraphicsExtractor)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeInnerFill -> net.minecraft.client.gui.GuiGraphicsExtractor.innerFill
// @Invoker: native mods call the method directly: `obj.inner_fill(..)` (or `mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor::inner_fill(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
