//! Ported from de.hysky.skyblocker.compatibility.rei.SkyblockItemComparator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: public class SkyblockItemComparator implements EntryComparator<ItemStack>
#[derive(Clone)]
pub struct SkyblockItemComparator {}

impl SkyblockItemComparator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public long hash(ComparisonContext context, ItemStack itemStack)
    pub fn hash(&self, context: &crate::unresolved::ComparisonContext, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> i64 {
        if item_stack.get_neu_name().is_empty() {
            // TODO(fabric2seer): static call EntryComparator.noop(..)
            //   java: return EntryComparator.noop().hash(context, itemStack);
        }
        return ((compat::string_hash_code(&item_stack.get_neu_name())) as i64) as i64;
    }

}
