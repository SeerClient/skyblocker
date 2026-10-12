//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.list.entries.slot.WidgetsListSlotEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry::WidgetsListEntry;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::entries::widgets_list_entry::WidgetsListEntryBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::list::widgets_list_screen::WidgetsListScreen;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: public abstract class WidgetsListSlotEntry extends WidgetsListEntry
pub struct WidgetsListSlotEntryBase {
    /// java: extends WidgetsListEntry
    pub base: WidgetsListEntryBase,
    /// java: protected final int slotId;
    pub slot_id: i32,
    /// java: protected final WidgetsListScreen parent;
    pub parent: compat::Shared<WidgetsListScreen>,
    /// java: protected final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
}

impl std::ops::Deref for WidgetsListSlotEntryBase {
    type Target = WidgetsListEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WidgetsListSlotEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WidgetsListSlotEntryBase {
    /// java: public WidgetsListSlotEntry(WidgetsListScreen parent, int slotId, ItemStack icon) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut parent: compat::Shared<WidgetsListScreen>, mut slot_id: i32, mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn WidgetsListSlotEntry>, __R)) -> (compat::Shared<dyn WidgetsListSlotEntry>, __R) {
        let __init_parent = parent.clone();
        let __init_slot_id = slot_id;
        let __init_icon = icon.clone();
        let (_, (__this, __r)) = WidgetsListEntryBase::new_in(move |__base| {
            let this = Self {
                base: __base,
                slot_id: __init_slot_id,
                parent: __init_parent,
                icon: __init_icon,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn WidgetsListEntry>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of WidgetsListSlotEntry (implemented by WidgetsListSlotEntryBase and every subclass)
pub trait WidgetsListSlotEntry: WidgetsListEntry {
    fn widgets_list_slot_entry_base(&self) -> &WidgetsListSlotEntryBase;
    fn widgets_list_slot_entry_base_mut(&mut self) -> &mut WidgetsListSlotEntryBase;

    /// java: protected void extractIconAndText(GuiGraphicsExtractor graphics, int y, int x, int entryHeight) (dispatch: the body for this class)
    fn extract_icon_and_text(&self) -> fn(&compat::Shared<dyn WidgetsListSlotEntry>, &GuiGraphicsExtractor, i32, i32, i32) {
        <dyn WidgetsListSlotEntry>::extract_icon_and_text_body
    }

}

impl dyn WidgetsListSlotEntry {
    /// java: protected void extractIconAndText(GuiGraphicsExtractor graphics, int y, int x, int entryHeight)
    pub fn extract_icon_and_text_body(__this: &compat::Shared<dyn WidgetsListSlotEntry>, graphics: &GuiGraphicsExtractor, mut y: i32, mut x: i32, mut entry_height: i32) {
        <dyn WidgetsListEntry>::extract_icon_and_text_body(&compat::view(__this, |a| a as compat::Shared<dyn WidgetsListEntry>), graphics, &compat::read(__this, |__t| __t.widgets_list_slot_entry_base().icon.clone()), y, x, entry_height);
    }

}
