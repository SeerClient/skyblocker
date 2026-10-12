//! Ported from de.hysky.skyblocker.utils.render.state.TexturedQuadRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public record TexturedQuadRenderState(Vec3 pos, float width, float height, float textureWidth, float textureHeight, Vec3 renderOffset, Identifier texture, float[] shaderColour, float alpha)
#[derive(Clone, Debug, PartialEq)]
pub struct TexturedQuadRenderState {
    /// java: Vec3 pos
    pub pos: Vec3,
    /// java: float width
    pub width: f32,
    /// java: float height
    pub height: f32,
    /// java: float textureWidth
    pub texture_width: f32,
    /// java: float textureHeight
    pub texture_height: f32,
    /// java: Vec3 renderOffset
    pub render_offset: Vec3,
    /// java: Identifier texture
    pub texture: Identifier,
    /// java: float[] shaderColour
    pub shader_colour: Vec<f32>,
    /// java: float alpha
    pub alpha: f32,
}

impl TexturedQuadRenderState {
    /// java: the canonical record constructor
    pub fn new(mut pos: Vec3, mut width: f32, mut height: f32, mut texture_width: f32, mut texture_height: f32, mut render_offset: Vec3, mut texture: Identifier, mut shader_colour: Vec<f32>, mut alpha: f32) -> Self {
        Self { pos, width, height, texture_width, texture_height, render_offset, texture, shader_colour, alpha }
    }

    /// java: the record accessor `pos()`
    pub fn pos(&self) -> Vec3 {
        self.pos.clone()
    }

    /// java: the record accessor `width()`
    pub fn width(&self) -> f32 {
        self.width
    }

    /// java: the record accessor `height()`
    pub fn height(&self) -> f32 {
        self.height
    }

    /// java: the record accessor `textureWidth()`
    pub fn texture_width(&self) -> f32 {
        self.texture_width
    }

    /// java: the record accessor `textureHeight()`
    pub fn texture_height(&self) -> f32 {
        self.texture_height
    }

    /// java: the record accessor `renderOffset()`
    pub fn render_offset(&self) -> Vec3 {
        self.render_offset.clone()
    }

    /// java: the record accessor `texture()`
    pub fn texture(&self) -> Identifier {
        self.texture.clone()
    }

    /// java: the record accessor `shaderColour()`
    pub fn shader_colour(&self) -> Vec<f32> {
        self.shader_colour.clone()
    }

    /// java: the record accessor `alpha()`
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

}
