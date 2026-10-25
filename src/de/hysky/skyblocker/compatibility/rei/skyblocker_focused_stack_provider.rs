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
            // java: return CompoundEventResult.pass();
            // (REI, a mod SeerClient never loads, is the only caller; its `CompoundEventResult` is the
            // `crate::unresolved` name, whose value stands for `pass()`)
            return crate::unresolved::CompoundEventResult;
        };
        //   java: return CompoundEventResult.interruptTrue(EntryStacks.of(focusedItem));
        // (the placeholder value stands for `interruptTrue(..)` too - see `pass()` above)
        let _ = &mut focused_item;
        return crate::unresolved::CompoundEventResult;
    }

}
