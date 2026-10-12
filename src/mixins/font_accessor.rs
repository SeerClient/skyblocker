//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.FontAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.Font)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeGetGlyph -> net.minecraft.client.gui.Font.getGlyph
// @Invoker: native mods call the method directly: `obj.get_glyph(..)` (or `mcrust::net::minecraft::client::gui::font::Font::get_glyph(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
