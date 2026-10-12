//! Ported from de.hysky.skyblocker.utils.render.state.TextRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::font::PreparedText;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public record TextRenderState(Font.PreparedText glyphs, Vec3 pos, float scale, float yOffset)
#[derive(Clone, Debug)]
pub struct TextRenderState {
    /// java: Font.PreparedText glyphs
    pub glyphs: PreparedText,
    /// java: Vec3 pos
    pub pos: Vec3,
    /// java: float scale
    pub scale: f32,
    /// java: float yOffset
    pub y_offset: f32,
}

impl TextRenderState {
    /// java: the canonical record constructor
    pub fn new(mut glyphs: PreparedText, mut pos: Vec3, mut scale: f32, mut y_offset: f32) -> Self {
        Self { glyphs, pos, scale, y_offset }
    }

    /// java: the record accessor `glyphs()`
    pub fn glyphs(&self) -> PreparedText {
        self.glyphs.clone()
    }

    /// java: the record accessor `pos()`
    pub fn pos(&self) -> Vec3 {
        self.pos.clone()
    }

    /// java: the record accessor `scale()`
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// java: the record accessor `yOffset()`
    pub fn y_offset(&self) -> f32 {
        self.y_offset
    }

}
