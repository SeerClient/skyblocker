//! Ported from de.hysky.skyblocker.utils.container.StackDisplayModifier by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface StackDisplayModifier
pub trait StackDisplayModifier: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: ItemStack modifyDisplayStack(int slotIndex, ItemStack stack); (dispatch: the body for this class)
    fn modify_display_stack(&self) -> fn(&compat::Shared<dyn StackDisplayModifier>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> mcrust::net::minecraft::world::item::item_stack::ItemStack;

}
