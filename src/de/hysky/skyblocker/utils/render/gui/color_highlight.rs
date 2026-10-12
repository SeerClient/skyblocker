//! Ported from de.hysky.skyblocker.utils.render.gui.ColorHighlight by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static final int RED_HIGHLIGHT = 64 << 24 | 255 << 16;
pub const RED_HIGHLIGHT: i32 = (64i32.wrapping_shl((24) as u32) | 255i32.wrapping_shl((16) as u32));
/// java: private static final int YELLOW_HIGHLIGHT = 128 << 24 | 255 << 16 | 255 << 8;
pub const YELLOW_HIGHLIGHT: i32 = ((128i32.wrapping_shl((24) as u32) | 255i32.wrapping_shl((16) as u32)) | 255i32.wrapping_shl((8) as u32));
/// java: private static final int GREEN_HIGHLIGHT = 128 << 24 | 64 << 16 | 196 << 8 | 64;
pub const GREEN_HIGHLIGHT: i32 = (((128i32.wrapping_shl((24) as u32) | 64i32.wrapping_shl((16) as u32)) | 196i32.wrapping_shl((8) as u32)) | 64);
/// java: private static final int GRAY_HIGHLIGHT = 128 << 24 | 64 << 16 | 64 << 8 | 64;
pub const GRAY_HIGHLIGHT: i32 = (((128i32.wrapping_shl((24) as u32) | 64i32.wrapping_shl((16) as u32)) | 64i32.wrapping_shl((8) as u32)) | 64);

/// java: public record ColorHighlight(int slot, int color)
#[derive(Clone, Debug, PartialEq)]
pub struct ColorHighlight {
    /// java: int slot
    pub slot: i32,
    /// java: int color
    pub color: i32,
}

impl ColorHighlight {
    /// java: the canonical record constructor
    pub fn new(mut slot: i32, mut color: i32) -> Self {
        Self { slot, color }
    }

    /// java: the record accessor `slot()`
    pub fn slot(&self) -> i32 {
        self.slot
    }

    /// java: the record accessor `color()`
    pub fn color(&self) -> i32 {
        self.color
    }

    /// java: public static ColorHighlight red(int slot)
    pub fn red(mut slot: i32) -> ColorHighlight {
        return ColorHighlight::new(slot, RED_HIGHLIGHT);
    }

    /// java: public static ColorHighlight yellow(int slot)
    pub fn yellow(mut slot: i32) -> ColorHighlight {
        return ColorHighlight::new(slot, YELLOW_HIGHLIGHT);
    }

    /// java: public static ColorHighlight green(int slot)
    pub fn green(mut slot: i32) -> ColorHighlight {
        return ColorHighlight::new(slot, GREEN_HIGHLIGHT);
    }

    /// java: public static ColorHighlight gray(int slot)
    pub fn gray(mut slot: i32) -> ColorHighlight {
        return ColorHighlight::new(slot, GRAY_HIGHLIGHT);
    }

}
