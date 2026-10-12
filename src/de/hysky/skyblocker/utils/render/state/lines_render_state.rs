//! Ported from de.hysky.skyblocker.utils.render.state.LinesRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public record LinesRenderState(Vec3[] points, float[] colourComponents, float alpha, float lineWidth)
#[derive(Clone, Debug, PartialEq)]
pub struct LinesRenderState {
    /// java: Vec3[] points
    pub points: Vec<Vec3>,
    /// java: float[] colourComponents
    pub colour_components: Vec<f32>,
    /// java: float alpha
    pub alpha: f32,
    /// java: float lineWidth
    pub line_width: f32,
}

impl LinesRenderState {
    /// java: the canonical record constructor
    pub fn new(mut points: Vec<Vec3>, mut colour_components: Vec<f32>, mut alpha: f32, mut line_width: f32) -> Self {
        Self { points, colour_components, alpha, line_width }
    }

    /// java: the record accessor `points()`
    pub fn points(&self) -> Vec<Vec3> {
        self.points.clone()
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
