//! Ported from de.hysky.skyblocker.utils.render.state.QuadRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public record QuadRenderState(Vec3[] points, float[] colourComponents, float alpha)
#[derive(Clone, Debug, PartialEq)]
pub struct QuadRenderState {
    /// java: Vec3[] points
    pub points: Vec<Vec3>,
    /// java: float[] colourComponents
    pub colour_components: Vec<f32>,
    /// java: float alpha
    pub alpha: f32,
}

impl QuadRenderState {
    /// java: the canonical record constructor
    pub fn new(mut points: Vec<Vec3>, mut colour_components: Vec<f32>, mut alpha: f32) -> Self {
        Self { points, colour_components, alpha }
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

}
