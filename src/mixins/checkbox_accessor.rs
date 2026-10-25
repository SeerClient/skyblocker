//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.CheckboxAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.Checkbox)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor setSelected -> net.minecraft.client.gui.components.Checkbox.selected
pub fn set_selected(obj: &mut mcrust::net::minecraft::client::gui::components::checkbox::Checkbox, value: bool) {
    obj.selected = value;
}

/// @Accessor getTextWidget -> net.minecraft.client.gui.components.Checkbox.textWidget
pub fn get_text_widget(
    obj: &mcrust::net::minecraft::client::gui::components::checkbox::Checkbox,
) -> &mcrust::net::minecraft::client::gui::components::multi_line_text_widget::MultiLineTextWidget {
    &obj.text_widget
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
