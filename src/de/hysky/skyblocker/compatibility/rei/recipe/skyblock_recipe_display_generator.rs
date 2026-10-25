//! Ported from de.hysky.skyblocker.compatibility.rei.recipe.SkyblockRecipeDisplayGenerator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::rei::recipe::skyblock_recipe_display::SkyblockRecipeDisplay;

/// java: public class SkyblockRecipeDisplayGenerator implements DynamicDisplayGenerator<SkyblockRecipeDisplay>
#[derive(Clone)]
pub struct SkyblockRecipeDisplayGenerator {}

impl SkyblockRecipeDisplayGenerator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public Optional<List<SkyblockRecipeDisplay>> getRecipeFor(EntryStack<?> entry)
    pub fn get_recipe_for(&self, entry: &crate::unresolved::EntryStack) -> Option<Vec<SkyblockRecipeDisplay>> {
        // java: if (!(entry.getValue() instanceof ItemStack entryStack)) return Optional.empty();
        let Some(entry_stack) = crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stack_item(entry) else {
            return None;
        };
        // java: return Optional.of(ItemRepository.getRecipes(entryStack).map(SkyblockRecipeDisplay::new).toList());
        return Some(crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository::get_recipes(&entry_stack).map(SkyblockRecipeDisplay::new).collect());
    }

    /// java: @Override public Optional<List<SkyblockRecipeDisplay>> getUsageFor(EntryStack<?> entry)
    pub fn get_usage_for(&self, entry: &crate::unresolved::EntryStack) -> Option<Vec<SkyblockRecipeDisplay>> {
        // java: if (!(entry.getValue() instanceof ItemStack entryStack)) return Optional.empty();
        let Some(entry_stack) = crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stack_item(entry) else {
            return None;
        };
        // java: return Optional.of(ItemRepository.getUsages(entryStack).map(SkyblockRecipeDisplay::new).toList());
        return Some(crate::de::hysky::skyblocker::skyblock::itemlist::item_repository::ItemRepository::get_usages(&entry_stack).map(SkyblockRecipeDisplay::new).collect());
    }

}
