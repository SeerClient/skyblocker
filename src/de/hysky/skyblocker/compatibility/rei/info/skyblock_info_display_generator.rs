//! Ported from de.hysky.skyblocker.compatibility.rei.info.SkyblockInfoDisplayGenerator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::rei::info::skyblock_info_display::SkyblockInfoDisplay;

/// java: public class SkyblockInfoDisplayGenerator implements DynamicDisplayGenerator<SkyblockInfoDisplay>
#[derive(Clone)]
pub struct SkyblockInfoDisplayGenerator {}

impl SkyblockInfoDisplayGenerator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public Optional<List<SkyblockInfoDisplay>> getRecipeFor(EntryStack<?> entry)
    pub fn get_recipe_for(&self, entry: &crate::unresolved::EntryStack) -> Option<Vec<SkyblockInfoDisplay>> {
        // java: if (!(entry.getValue() instanceof ItemStack entryStack)) return Optional.empty();
        let Some(mut entry_stack) = crate::de::hysky::skyblocker::compatibility::rei::rei_entry_stack_item(entry) else {
            return None;
        };
        // java: if (entryStack.getSkyblockId().isEmpty()) return Optional.empty();
        if entry_stack.get_skyblock_id().is_empty() {
            return None;
        }
        // java: ItemStack stack = entryStack.copy();
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = entry_stack.copy();
        stack.set_count(1);
        return Some(vec![SkyblockInfoDisplay::new(stack.clone())]);
    }

    /// java: @Override public Optional<List<SkyblockInfoDisplay>> getUsageFor(EntryStack<?> entry)
    pub fn get_usage_for(&self, entry: &crate::unresolved::EntryStack) -> Option<Vec<SkyblockInfoDisplay>> {
        return self.get_recipe_for(entry);
    }

}
