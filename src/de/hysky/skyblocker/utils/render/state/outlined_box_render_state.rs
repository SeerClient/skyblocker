//! Ported from de.hysky.skyblocker.utils.render.state.OutlinedBoxRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public record OutlinedBoxRenderState(double minX, double minY, double minZ, double maxX, double maxY, double maxZ, float[] colourComponents, float alpha, float lineWidth)
#[derive(Clone, Debug, PartialEq)]
pub struct OutlinedBoxRenderState {
    /// java: double minX
    pub min_x: f64,
    /// java: double minY
    pub min_y: f64,
    /// java: double minZ
    pub min_z: f64,
    /// java: double maxX
    pub max_x: f64,
    /// java: double maxY
    pub max_y: f64,
    /// java: double maxZ
    pub max_z: f64,
    /// java: float[] colourComponents
    pub colour_components: Vec<f32>,
    /// java: float alpha
    pub alpha: f32,
    /// java: float lineWidth
    pub line_width: f32,
}

impl OutlinedBoxRenderState {
    /// java: the canonical record constructor
    pub fn new(mut min_x: f64, mut min_y: f64, mut min_z: f64, mut max_x: f64, mut max_y: f64, mut max_z: f64, mut colour_components: Vec<f32>, mut alpha: f32, mut line_width: f32) -> Self {
        Self { min_x, min_y, min_z, max_x, max_y, max_z, colour_components, alpha, line_width }
    }

    /// java: the record accessor `minX()`
    pub fn min_x(&self) -> f64 {
        self.min_x
    }

    /// java: the record accessor `minY()`
    pub fn min_y(&self) -> f64 {
        self.min_y
    }

    /// java: the record accessor `minZ()`
    pub fn min_z(&self) -> f64 {
        self.min_z
    }

    /// java: the record accessor `maxX()`
    pub fn max_x(&self) -> f64 {
        self.max_x
    }

    /// java: the record accessor `maxY()`
    pub fn max_y(&self) -> f64 {
        self.max_y
    }

    /// java: the record accessor `maxZ()`
    pub fn max_z(&self) -> f64 {
        self.max_z
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
