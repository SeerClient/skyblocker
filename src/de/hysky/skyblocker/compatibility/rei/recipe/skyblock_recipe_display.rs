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
        // java: return recipe.getInputs().stream().map(FlexibleItemStack::getStackOrThrow).map(EntryStacks::of).map(EntryIngredient::of).toList();
        return { let __f = compat::dispatch(&self.recipe, |__t| __t.get_inputs()); __f(&self.recipe) }.iter().map(|s| FlexibleItemStack::get_stack_or_throw(s)).map(|s| crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stacks_of(&s)).map(crate::de::hysky::skyblocker::compatibility::rei::rei_entry_ingredient_of).collect();
    }

    /// java: @Override public List<EntryIngredient> getOutputEntries()
    pub fn get_output_entries(&self) -> Vec<crate::unresolved::EntryIngredient> {
        // java: return recipe.getOutputs().stream().map(FlexibleItemStack::getStackOrThrow).map(EntryStacks::of).map(EntryIngredient::of).toList();
        return { let __f = compat::dispatch(&self.recipe, |__t| __t.get_outputs()); __f(&self.recipe) }.iter().map(|s| FlexibleItemStack::get_stack_or_throw(s)).map(|s| crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stacks_of(&s)).map(crate::de::hysky::skyblocker::compatibility::rei::rei_entry_ingredient_of).collect();
    }

    /// java: public SkyblockRecipe getRecipe()
    pub fn get_recipe(&self) -> compat::Shared<dyn SkyblockRecipe> {
        return self.recipe.clone();
    }

    /// java: @Override public CategoryIdentifier<?> getCategoryIdentifier()
    pub fn get_category_identifier(&self) -> crate::unresolved::CategoryIdentifier {
        // java: return CategoryIdentifier.of(category);
        return crate::de::hysky::skyblocker::compatibility::rei::rei_category_identifier_of(&self.category);
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
