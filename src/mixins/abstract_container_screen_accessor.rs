//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.AbstractContainerScreenAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.AbstractContainerScreen)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getX -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.leftPos
pub fn get_x(obj: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen) -> i32 {
    obj.left_pos
}

/// @Accessor getY -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.topPos
pub fn get_y(obj: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen) -> i32 {
    obj.top_pos
}

/// @Accessor setX -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.leftPos
pub fn set_x(obj: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, value: i32) {
    obj.left_pos = value;
}

/// @Accessor setY -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.topPos
pub fn set_y(obj: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen, value: i32) {
    obj.top_pos = value;
}

/// @Accessor getImageWidth -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.imageWidth
pub fn get_image_width(obj: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen) -> i32 {
    obj.image_width
}

/// @Accessor getImageHeight -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.imageHeight
pub fn get_image_height(obj: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen) -> i32 {
    obj.image_height
}

/// @Accessor setHandler -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.menu
// TODO(fabric2seer): `AbstractContainerMenu` - assign `obj.menu` with its Rust type

/// @Accessor getFocusedSlot -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.hoveredSlot
// TODO(fabric2seer): `Slot` - return `&obj.hovered_slot` with its Rust type
// pub fn get_focused_slot(obj: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen) -> &T { &obj.hovered_slot }

/// @Accessor getSLOT_HIGHLIGHT_BACK_SPRITE -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.SLOT_HIGHLIGHT_BACK_SPRITE
// TODO(fabric2seer): `Identifier` - return `&obj.slot_highlight_back_sprite` with its Rust type
// pub fn get_slot_highlight_back_sprite(obj: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen) -> &T { &obj.slot_highlight_back_sprite }

/// @Accessor getSLOT_HIGHLIGHT_FRONT_SPRITE -> net.minecraft.client.gui.screens.inventory.AbstractContainerScreen.SLOT_HIGHLIGHT_FRONT_SPRITE
// TODO(fabric2seer): `Identifier` - return `&obj.slot_highlight_front_sprite` with its Rust type
// pub fn get_slot_highlight_front_sprite(obj: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen) -> &T { &obj.slot_highlight_front_sprite }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
