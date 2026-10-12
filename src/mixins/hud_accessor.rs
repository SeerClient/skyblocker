//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.HudAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.Hud)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getOverlayMessageString -> net.minecraft.client.gui.Hud.overlayMessageString
// TODO(fabric2seer): `Component` - return `&obj.overlay_message_string` with its Rust type
// pub fn get_overlay_message_string(obj: &mcrust::net::minecraft::client::gui::hud::Hud) -> &T { &obj.overlay_message_string }

/// @Accessor getFOOD_EMPTY_HUNGER_SPRITE -> net.minecraft.client.gui.Hud.FOOD_EMPTY_HUNGER_SPRITE
// TODO(fabric2seer): `Identifier` - return `&obj.food_empty_hunger_sprite` with its Rust type
// pub fn get_food_empty_hunger_sprite(obj: &mcrust::net::minecraft::client::gui::hud::Hud) -> &T { &obj.food_empty_hunger_sprite }

/// @Accessor getFOOD_HALF_HUNGER_SPRITE -> net.minecraft.client.gui.Hud.FOOD_HALF_HUNGER_SPRITE
// TODO(fabric2seer): `Identifier` - return `&obj.food_half_hunger_sprite` with its Rust type
// pub fn get_food_half_hunger_sprite(obj: &mcrust::net::minecraft::client::gui::hud::Hud) -> &T { &obj.food_half_hunger_sprite }

/// @Accessor getFOOD_FULL_HUNGER_SPRITE -> net.minecraft.client.gui.Hud.FOOD_FULL_HUNGER_SPRITE
// TODO(fabric2seer): `Identifier` - return `&obj.food_full_hunger_sprite` with its Rust type
// pub fn get_food_full_hunger_sprite(obj: &mcrust::net::minecraft::client::gui::hud::Hud) -> &T { &obj.food_full_hunger_sprite }

/// @Accessor getFOOD_EMPTY_SPRITE -> net.minecraft.client.gui.Hud.FOOD_EMPTY_SPRITE
// TODO(fabric2seer): `Identifier` - return `&obj.food_empty_sprite` with its Rust type
// pub fn get_food_empty_sprite(obj: &mcrust::net::minecraft::client::gui::hud::Hud) -> &T { &obj.food_empty_sprite }

/// @Accessor getFOOD_HALF_SPRITE -> net.minecraft.client.gui.Hud.FOOD_HALF_SPRITE
// TODO(fabric2seer): `Identifier` - return `&obj.food_half_sprite` with its Rust type
// pub fn get_food_half_sprite(obj: &mcrust::net::minecraft::client::gui::hud::Hud) -> &T { &obj.food_half_sprite }

/// @Accessor getFOOD_FULL_SPRITE -> net.minecraft.client.gui.Hud.FOOD_FULL_SPRITE
// TODO(fabric2seer): `Identifier` - return `&obj.food_full_sprite` with its Rust type
// pub fn get_food_full_sprite(obj: &mcrust::net::minecraft::client::gui::hud::Hud) -> &T { &obj.food_full_sprite }

/// @Invoker extractSidebar -> net.minecraft.client.gui.Hud.displayScoreboardSidebar
// @Invoker: native mods call the method directly: `obj.display_scoreboard_sidebar(..)` (or `mcrust::net::minecraft::client::gui::hud::Hud::display_scoreboard_sidebar(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
