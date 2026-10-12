//! Ported from de.hysky.skyblocker.skyblock.auction.SlotClickHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @FunctionalInterface public interface SlotClickHandler
pub trait SlotClickHandler: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void click(int slot, int button); (dispatch: the body for this class)
    fn click(&self) -> fn(&compat::Shared<dyn SlotClickHandler>, i32, i32);

    /// java: default void click(int slot) (dispatch: the body for this class)
    fn click_1(&self) -> fn(&compat::Shared<dyn SlotClickHandler>, i32) {
        <dyn SlotClickHandler>::click_1_body
    }

}

impl dyn SlotClickHandler {
    /// java: default void click(int slot)
    pub fn click_1_body(__this: &compat::Shared<dyn SlotClickHandler>, mut slot: i32) {
        ({ let __f = compat::dispatch(__this, |__t| __t.click()); __f(__this, slot, 0) });
    }

}
