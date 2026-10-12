//! Ported from de.hysky.skyblocker.utils.hoveredItem.HoveredItemStackProvider by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface HoveredItemStackProvider
pub trait HoveredItemStackProvider: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @Nullable ItemStack getFocusedItem(); (dispatch: the body for this class)
    fn get_focused_item(&self) -> fn(&compat::Shared<dyn HoveredItemStackProvider>) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>;

}
