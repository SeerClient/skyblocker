//! Ported from de.hysky.skyblocker.utils.render.state.CursorLineRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public record CursorLineRenderState(Vec3 point, float[] colourComponents, float alpha, float lineWidth)
#[derive(Clone, Debug, PartialEq)]
pub struct CursorLineRenderState {
    /// java: Vec3 point
    pub point: Vec3,
    /// java: float[] colourComponents
    pub colour_components: Vec<f32>,
    /// java: float alpha
    pub alpha: f32,
    /// java: float lineWidth
    pub line_width: f32,
}

impl CursorLineRenderState {
    /// java: the canonical record constructor
    pub fn new(mut point: Vec3, mut colour_components: Vec<f32>, mut alpha: f32, mut line_width: f32) -> Self {
        Self { point, colour_components, alpha, line_width }
    }

    /// java: the record accessor `point()`
    pub fn point(&self) -> Vec3 {
        self.point.clone()
    }

    /// java: the record accessor `colourComponents()`
    pub fn colour_components(&self) -> Vec<f32> {
        self.colour_components.clone()
    }

    /// java: the record accessor `alpha()`
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// java: the record accessor `lineWidth()`
    pub fn line_width(&self) -> f32 {
        self.line_width
    }

}
