//! Ported from de.hysky.skyblocker.utils.container.TooltipAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: public interface TooltipAdder extends ContainerMatcher
pub trait TooltipAdder: ContainerMatcher {
    /// java: void addToTooltip(@Nullable Slot focusedSlot, ItemStack stack, List<Component> lines); (dispatch: the body for this class)
    fn add_to_tooltip(&self) -> fn(&compat::Shared<dyn TooltipAdder>, Option<Slot>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &mut Vec<MutableComponent>);

    /// java: int getPriority(); (dispatch: the body for this class)
    fn get_priority(&self) -> fn(&compat::Shared<dyn TooltipAdder>) -> i32;

}
