//! Ported from de.hysky.skyblocker.skyblock.auction.SlotClickHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @FunctionalInterface public interface SlotClickHandler - `void click(int slot, int button);`
/// (a functional interface: a lambda / method reference is this closure type)
pub type SlotClickHandler = Arc<dyn Fn(i32, i32)>;

/// java: default void click(int slot) - `click(slot, 0)`
pub fn click_1(handler: &SlotClickHandler, slot: i32) {
    handler(slot, 0);
}
