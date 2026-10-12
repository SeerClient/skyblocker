//! Ported from de.hysky.skyblocker.utils.render.state.FilledCircleRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public record FilledCircleRenderState(Vec3 centre, float radius, int segments, int colour)
#[derive(Clone, Debug, PartialEq)]
pub struct FilledCircleRenderState {
    /// java: Vec3 centre
    pub centre: Vec3,
    /// java: float radius
    pub radius: f32,
    /// java: int segments
    pub segments: i32,
    /// java: int colour
    pub colour: i32,
}

impl FilledCircleRenderState {
    /// java: the canonical record constructor
    pub fn new(mut centre: Vec3, mut radius: f32, mut segments: i32, mut colour: i32) -> Self {
        Self { centre, radius, segments, colour }
    }

    /// java: the record accessor `centre()`
    pub fn centre(&self) -> Vec3 {
        self.centre.clone()
    }

    /// java: the record accessor `radius()`
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// java: the record accessor `segments()`
    pub fn segments(&self) -> i32 {
        self.segments
    }

    /// java: the record accessor `colour()`
    pub fn colour(&self) -> i32 {
        self.colour
    }

}
