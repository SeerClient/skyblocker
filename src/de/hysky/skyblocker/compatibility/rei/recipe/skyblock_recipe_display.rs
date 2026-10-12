//! Ported from de.hysky.skyblocker.compatibility.rei.recipe.SkyblockRecipeDisplay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::itemlist::recipes::skyblock_recipe::SkyblockRecipe;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class SkyblockRecipeDisplay implements Display
#[derive(Clone)]
pub struct SkyblockRecipeDisplay {
    /// java: private final Identifier category;
    pub category: Identifier,
    /// java: private final SkyblockRecipe recipe;
    pub recipe: compat::Shared<dyn SkyblockRecipe>,
}

impl SkyblockRecipeDisplay {
    /// java: public SkyblockRecipeDisplay(SkyblockRecipe recipe)
    pub fn new(mut recipe: compat::Shared<dyn SkyblockRecipe>) -> Self {
        let __init_category = ({ let __f = compat::dispatch(&recipe, |__t| __t.get_category_identifier()); __f(&recipe) });
        let __init_recipe = recipe.clone();
        let mut this = Self {
            category: __init_category,
            recipe: __init_recipe,
        };
        this
    }

    /// java: @Override public List<EntryIngredient> getInputEntries()
    pub fn get_input_entries(&self) -> Vec<crate::unresolved::EntryIngredient> {
        // TODO(fabric2seer): `of` on a Object
        //   java: return recipe.getInputs().stream().map(FlexibleItemStack::getStackOrThrow).map(EntryStacks::of).map(EntryIngredient::of).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public List<EntryIngredient> getOutputEntries()
    pub fn get_output_entries(&self) -> Vec<crate::unresolved::EntryIngredient> {
        // TODO(fabric2seer): `of` on a Object
        //   java: return recipe.getOutputs().stream().map(FlexibleItemStack::getStackOrThrow).map(EntryStacks::of).map(EntryIngredient::of).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public SkyblockRecipe getRecipe()
    pub fn get_recipe(&self) -> compat::Shared<dyn SkyblockRecipe> {
        return self.recipe.clone();
    }

    /// java: @Override public CategoryIdentifier<?> getCategoryIdentifier()
    pub fn get_category_identifier(&self) -> crate::unresolved::CategoryIdentifier {
        // TODO(fabric2seer): static call CategoryIdentifier.of(..)
        //   java: return CategoryIdentifier.of(category);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Optional<Identifier> getDisplayLocation()
    pub fn get_display_location(&self) -> Option<Identifier> {
        return None;
    }

    /// java: @Override public @Nullable DisplaySerializer<? extends Display> getSerializer()
    pub fn get_serializer(&self) -> Option<crate::unresolved::DisplaySerializer> {
        return None;
    }

}
