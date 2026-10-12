//! Ported from de.hysky.skyblocker.skyblock.item.slottext.TextPosition by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public enum TextPosition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl TextPosition {
    pub const VALUES: [TextPosition; 4] = [TextPosition::TopLeft, TextPosition::TopRight, TextPosition::BottomLeft, TextPosition::BottomRight];

    /// java: values()
    pub fn values() -> Vec<TextPosition> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            TextPosition::TopLeft => "TOP_LEFT",
            TextPosition::TopRight => "TOP_RIGHT",
            TextPosition::BottomLeft => "BOTTOM_LEFT",
            TextPosition::BottomRight => "BOTTOM_RIGHT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> TextPosition {
        match name {
            "TOP_LEFT" => TextPosition::TopLeft,
            "TOP_RIGHT" => TextPosition::TopRight,
            "BOTTOM_LEFT" => TextPosition::BottomLeft,
            "BOTTOM_RIGHT" => TextPosition::BottomRight,
            _ => panic!("IllegalArgumentException: No enum constant TextPosition.{name}"),
        }
    }

}

impl std::fmt::Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
