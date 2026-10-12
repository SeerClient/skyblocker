//! Ported from de.hysky.skyblocker.compatibility.rei.info.SkyblockInfoDisplay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class SkyblockInfoDisplay implements Display
#[derive(Clone)]
pub struct SkyblockInfoDisplay {
    /// java: private final ItemStack displayItem;
    pub display_item: mcrust::net::minecraft::world::item::item_stack::ItemStack,
}

impl SkyblockInfoDisplay {
    /// java: public SkyblockInfoDisplay(ItemStack item)
    pub fn new(mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> Self {
        let __init_display_item = item.clone();
        let mut this = Self {
            display_item: __init_display_item,
        };
        this
    }

    /// java: @Override public List<EntryIngredient> getInputEntries()
    pub fn get_input_entries(&self) -> Vec<crate::unresolved::EntryIngredient> {
        // TODO(fabric2seer): static call EntryIngredient.of(..)
        //   java: return List.of(EntryIngredient.of(EntryStacks.of(displayItem)));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public List<EntryIngredient> getOutputEntries()
    pub fn get_output_entries(&self) -> Vec<crate::unresolved::EntryIngredient> {
        // TODO(fabric2seer): static call EntryIngredient.of(..)
        //   java: return List.of(EntryIngredient.of(EntryStacks.of(displayItem)));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public CategoryIdentifier<?> getCategoryIdentifier()
    pub fn get_category_identifier(&self) -> crate::unresolved::CategoryIdentifier {
        // TODO(fabric2seer): static call CategoryIdentifier.of(..)
        //   java: return CategoryIdentifier.of(SkyblockerMod.id("skyblock_info"));
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
