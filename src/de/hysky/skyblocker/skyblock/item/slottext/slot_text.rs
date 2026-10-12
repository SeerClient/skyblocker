//! Ported from de.hysky.skyblocker.skyblock.item.slottext.SlotText by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::slottext::text_position::TextPosition;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final int CREAM = 0xFFDDC1;
pub const CREAM: i32 = 0xFFDDC1;
/// java: public static final int GOLD = 0xE5B80B;
pub const GOLD: i32 = 0xE5B80B;
/// java: public static final int LIGHT_BLUE = 0xCFF8F8;
pub const LIGHT_BLUE: i32 = 0xCFF8F8;
/// java: public static final int LIGHT_ORANGE = 0xFAB387;
pub const LIGHT_ORANGE: i32 = 0xFAB387;
/// java: public static final int LIGHT_YELLOW = 0xF9E2AF;
pub const LIGHT_YELLOW: i32 = 0xF9E2AF;
/// java: public static final int LIGHT_PURPLE = 0xCBA6F7;
pub const LIGHT_PURPLE: i32 = 0xCBA6F7;
/// java: public static final int LIGHT_RED = 0xFF7276;
pub const LIGHT_RED: i32 = 0xFF7276;
/// java: public static final int LIGHT_GREEN = 0x90EE90;
pub const LIGHT_GREEN: i32 = 0x90EE90;
/// java: public static final int MID_BLUE = 0x74C7EC;
pub const MID_BLUE: i32 = 0x74C7EC;
/// java: public static final int WHITE = 0xFFFFFF;
pub const WHITE: i32 = 0xFFFFFF;

/// java: public record SlotText(Component text, TextPosition position)
#[derive(Clone, Debug)]
pub struct SlotText {
    /// java: Component text
    pub text: MutableComponent,
    /// java: TextPosition position
    pub position: TextPosition,
}

impl SlotText {
    /// java: the canonical record constructor
    pub fn new(mut text: MutableComponent, mut position: TextPosition) -> Self {
        Self { text, position }
    }

    /// java: the record accessor `text()`
    pub fn text(&self) -> MutableComponent {
        self.text.clone()
    }

    /// java: the record accessor `position()`
    pub fn position(&self) -> TextPosition {
        self.position
    }

    /// java: public static SlotText bottomLeft(Component text)
    pub fn bottom_left(text: &MutableComponent) -> SlotText {
        return SlotText::new(text.clone(), TextPosition::BottomLeft);
    }

    /// java: public static SlotText bottomRight(Component text)
    pub fn bottom_right(text: &MutableComponent) -> SlotText {
        return SlotText::new(text.clone(), TextPosition::BottomRight);
    }

    /// java: public static SlotText topLeft(Component text)
    pub fn top_left(text: &MutableComponent) -> SlotText {
        return SlotText::new(text.clone(), TextPosition::TopLeft);
    }

    /// java: public static SlotText topRight(Component text)
    pub fn top_right(text: &MutableComponent) -> SlotText {
        return SlotText::new(text.clone(), TextPosition::TopRight);
    }

    /// java: public static List<SlotText> topLeftList(Component text)
    pub fn top_left_list(text: &MutableComponent) -> Vec<SlotText> {
        // TODO(fabric2seer): static call ObjectLists.singleton(..)
        //   java: return ObjectLists.singleton(topLeft(text));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static List<SlotText> topRightList(Component text)
    pub fn top_right_list(text: &MutableComponent) -> Vec<SlotText> {
        // TODO(fabric2seer): static call ObjectLists.singleton(..)
        //   java: return ObjectLists.singleton(topRight(text));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static List<SlotText> bottomLeftList(Component text)
    pub fn bottom_left_list(text: &MutableComponent) -> Vec<SlotText> {
        // TODO(fabric2seer): static call ObjectLists.singleton(..)
        //   java: return ObjectLists.singleton(bottomLeft(text));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static List<SlotText> bottomRightList(Component text)
    pub fn bottom_right_list(text: &MutableComponent) -> Vec<SlotText> {
        // TODO(fabric2seer): static call ObjectLists.singleton(..)
        //   java: return ObjectLists.singleton(bottomRight(text));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
