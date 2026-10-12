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
        // TODO(fabric2seer): `.getValue(..)` on a EntryStack
        //   java: if (!(entry.getValue() instanceof ItemStack entryStack)) return Optional.empty();
        // TODO(fabric2seer): unknown name `entryStack`
        //   java: return Optional.of(ItemRepository.getRecipes(entryStack).map(SkyblockRecipeDisplay::new).toList());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Optional<List<SkyblockRecipeDisplay>> getUsageFor(EntryStack<?> entry)
    pub fn get_usage_for(&self, entry: &crate::unresolved::EntryStack) -> Option<Vec<SkyblockRecipeDisplay>> {
        // TODO(fabric2seer): `.getValue(..)` on a EntryStack
        //   java: if (!(entry.getValue() instanceof ItemStack entryStack)) return Optional.empty();
        // TODO(fabric2seer): unknown name `entryStack`
        //   java: return Optional.of(ItemRepository.getUsages(entryStack).map(SkyblockRecipeDisplay::new).toList());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
