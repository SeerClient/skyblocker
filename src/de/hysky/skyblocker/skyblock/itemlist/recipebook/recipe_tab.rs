//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.RecipeTab by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipebook::recipe_area_display::RecipeAreaDisplay;

/// java: public interface RecipeTab extends RecipeAreaDisplay
pub trait RecipeTab: RecipeAreaDisplay {
    /// java: ItemStack icon(); (dispatch: the body for this class)
    fn icon(&self) -> fn(&compat::Shared<dyn RecipeTab>) -> mcrust::net::minecraft::world::item::item_stack::ItemStack;

}
