//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.RecipeBookComponentAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.recipebook.RecipeBookComponent)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Invoker invokeGetYOrigin -> net.minecraft.client.gui.screens.recipebook.RecipeBookComponent.getYOrigin
// @Invoker: native mods call the method directly: `obj.get_y_origin(..)` (or `mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::RecipeBookComponent::get_y_origin(..)`)

/// @Invoker invokeGetXOrigin -> net.minecraft.client.gui.screens.recipebook.RecipeBookComponent.getXOrigin
// @Invoker: native mods call the method directly: `obj.get_x_origin(..)` (or `mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_component::RecipeBookComponent::get_x_origin(..)`)

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
