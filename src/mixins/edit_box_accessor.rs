//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.EditBoxAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.EditBox)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getHighlightPos -> net.minecraft.client.gui.components.EditBox.highlightPos
pub fn get_highlight_pos(obj: &mcrust::net::minecraft::client::gui::components::edit_box::EditBox) -> i32 {
    obj.get_highlight_pos()
}

/// @Accessor getMaxLength -> net.minecraft.client.gui.components.EditBox.maxLength
pub fn get_max_length(obj: &mcrust::net::minecraft::client::gui::components::edit_box::EditBox) -> i32 {
    obj.get_max_length()
}

/// @Invoker invokeOnValueChange -> net.minecraft.client.gui.components.EditBox.onValueChange
// @Invoker: native mods call the method directly: `obj.on_value_change(..)` (or `mcrust::net::minecraft::client::gui::components::edit_box::EditBox::on_value_change(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
