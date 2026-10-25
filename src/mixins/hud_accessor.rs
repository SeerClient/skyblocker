//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.HudAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.Hud)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getOverlayMessageString -> net.minecraft.client.gui.Hud.overlayMessageString
pub fn get_overlay_message_string(obj: &mcrust::net::minecraft::client::gui::hud::Hud) -> Option<mcrust::net::minecraft::network::chat::formatted_text::FormattedText> {
    obj.overlay_message.clone()
}

/// @Accessor getFOOD_EMPTY_HUNGER_SPRITE -> net.minecraft.client.gui.Hud.FOOD_EMPTY_HUNGER_SPRITE
/// (`Identifier.withDefaultNamespace("hud/food_empty_hunger")`, the constant `Hud` draws with)
pub fn get_food_empty_hunger_sprite() -> mcrust::net::minecraft::resources::identifier::Identifier {
    mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("hud/food_empty_hunger")
}

/// @Accessor getFOOD_HALF_HUNGER_SPRITE -> net.minecraft.client.gui.Hud.FOOD_HALF_HUNGER_SPRITE
/// (`Identifier.withDefaultNamespace("hud/food_half_hunger")`, the constant `Hud` draws with)
pub fn get_food_half_hunger_sprite() -> mcrust::net::minecraft::resources::identifier::Identifier {
    mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("hud/food_half_hunger")
}

/// @Accessor getFOOD_FULL_HUNGER_SPRITE -> net.minecraft.client.gui.Hud.FOOD_FULL_HUNGER_SPRITE
/// (`Identifier.withDefaultNamespace("hud/food_full_hunger")`, the constant `Hud` draws with)
pub fn get_food_full_hunger_sprite() -> mcrust::net::minecraft::resources::identifier::Identifier {
    mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("hud/food_full_hunger")
}

/// @Accessor getFOOD_EMPTY_SPRITE -> net.minecraft.client.gui.Hud.FOOD_EMPTY_SPRITE
/// (`Identifier.withDefaultNamespace("hud/food_empty")`, the constant `Hud` draws with)
pub fn get_food_empty_sprite() -> mcrust::net::minecraft::resources::identifier::Identifier {
    mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("hud/food_empty")
}

/// @Accessor getFOOD_HALF_SPRITE -> net.minecraft.client.gui.Hud.FOOD_HALF_SPRITE
/// (`Identifier.withDefaultNamespace("hud/food_half")`, the constant `Hud` draws with)
pub fn get_food_half_sprite() -> mcrust::net::minecraft::resources::identifier::Identifier {
    mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("hud/food_half")
}

/// @Accessor getFOOD_FULL_SPRITE -> net.minecraft.client.gui.Hud.FOOD_FULL_SPRITE
/// (`Identifier.withDefaultNamespace("hud/food_full")`, the constant `Hud` draws with)
pub fn get_food_full_sprite() -> mcrust::net::minecraft::resources::identifier::Identifier {
    mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("hud/food_full")
}

/// @Invoker extractSidebar -> net.minecraft.client.gui.Hud.displayScoreboardSidebar
// @Invoker: native mods call the method directly: `obj.display_scoreboard_sidebar(..)` (or `mcrust::net::minecraft::client::gui::hud::Hud::display_scoreboard_sidebar(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
