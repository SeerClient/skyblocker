//! Ported from de.hysky.skyblocker.compatibility.rei.SkyblockerFocusedStackProvider by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::hovered_item::hovered_item_stack_utils::HoveredItemStackUtils;

/// java: public class SkyblockerFocusedStackProvider implements FocusedStackProvider
#[derive(Clone)]
pub struct SkyblockerFocusedStackProvider {}

impl SkyblockerFocusedStackProvider {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public CompoundEventResult<EntryStack<?>> provide(Screen screen, Point point)
    pub fn provide(&self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, point: &crate::unresolved::Point) -> crate::unresolved::CompoundEventResult {
        let Some(mut focused_item) = HoveredItemStackUtils::get_hovered_item_stack(screen) else {
            // TODO(fabric2seer): static call CompoundEventResult.pass(..)
            //   java: return CompoundEventResult.pass();
        };
        // TODO(fabric2seer): static call CompoundEventResult.interruptTrue(..)
        //   java: return CompoundEventResult.interruptTrue(EntryStacks.of(focusedItem));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
