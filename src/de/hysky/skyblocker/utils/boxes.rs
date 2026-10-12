//! Ported from de.hysky.skyblocker.utils.Boxes by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::core::direction::Axis;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public class Boxes
#[derive(Clone)]
pub struct Boxes {}

impl Boxes {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Deprecated(since = "1.22") public static Vec3 getMinVec(AABB box)
    pub fn get_min_vec(r#box: &AABB) -> Vec3 {
        return (r#box).get_min_position();
    }

    /// java: @Deprecated(since = "1.22") public static Vec3 getMaxVec(AABB box)
    pub fn get_max_vec(r#box: &AABB) -> Vec3 {
        return (r#box).get_max_position();
    }

    /// java: public static Vec3 getLengthVec(AABB box)
    pub fn get_length_vec(r#box: &AABB) -> Vec3 {
        return Vec3::new((r#box).get_xsize(), (r#box).get_ysize(), (r#box).get_zsize());
    }

    /// java: public static AABB moveToZero(AABB box)
    pub fn move_to_zero(r#box: &AABB) -> AABB {
        return (r#box).move_vec(&(r#box).get_min_position().reverse());
    }

    /// java: public static double getCornerLength(AABB box)
    pub fn get_corner_length(r#box: &AABB) -> f64 {
        return (r#box).get_min_position().distance_to(&(r#box).get_max_position());
    }

    /// java: public static double getAxisLength(AABB box, Axis axis)
    pub fn get_axis_length(r#box: &AABB, axis: &Axis) -> f64 {
        return ((r#box).max(*axis) - (r#box).min(*axis));
    }

    /// java: public static AABB multiply(AABB box, double amount)
    pub fn multiply(r#box: &AABB, mut amount: f64) -> AABB {
        return Boxes::multiply_4(r#box, amount, amount, amount);
    }

    /// java: public static AABB multiply(AABB box, double x, double y, double z)
    pub fn multiply_4(r#box: &AABB, mut x: f64, mut y: f64, mut z: f64) -> AABB {
        return (r#box).inflate3(((Boxes::get_axis_length(r#box, &Axis::X) * (x - (1 as f64))) / 2.0), ((Boxes::get_axis_length(r#box, &Axis::Y) * (y - (1 as f64))) / 2.0), ((Boxes::get_axis_length(r#box, &Axis::Z) * (z - (1 as f64))) / 2.0));
    }

    /// java: public static AABB lerpEntityBoundingBox(Entity entity, float partialTick)
    pub fn lerp_entity_bounding_box(entity: &dyn Entity, mut partial_tick: f32) -> AABB {
        let mut remaining_tick: f32 = ((1 as f32) - partial_tick);
        let mut backwards_delta_movement: Vec3 = entity.old_position().subtract_vec(&entity.position()).multiply((remaining_tick) as f64, (remaining_tick) as f64, (remaining_tick) as f64);
        return entity.get_bounding_box().move_vec(&backwards_delta_movement);
    }

}
