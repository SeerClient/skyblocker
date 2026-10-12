//! Ported from de.hysky.skyblocker.utils.render.FrustumUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::renderer::culling::frustum::Frustum;
use mcrust::net::minecraft::world::phys::aabb::AABB;

/// java: public class FrustumUtils
#[derive(Clone)]
pub struct FrustumUtils {}

impl FrustumUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isVisible(Frustum frustum, AABB box)
    pub fn is_visible(frustum: &Frustum, r#box: &AABB) -> bool {
        return FrustumUtils::is_visible(frustum, r#box);
    }

    /// java: public static boolean isVisible(Frustum frustum, double minX, double minY, double minZ, double maxX, double maxY, double maxZ)
    pub fn is_visible_7(frustum: &Frustum, mut min_x: f64, mut min_y: f64, mut min_z: f64, mut max_x: f64, mut max_y: f64, mut max_z: f64) -> bool {
        // TODO(fabric2seer): `FrustumInvoker.cubeInFrustum` has no Rust method in the port (looked for `cube_in_frustum` in FrustumInvoker and JavaSrc)
        // rust: let mut plane: i32 = frustum.cube_in_frustum(min_x, min_y, min_z, max_x, max_y, max_z);
        let mut plane: i32 = todo!("fabric2seer: the initialiser of `plane` (TODO above)");
        return (plane == mcrust::org::joml::frustum_intersection::INSIDE || plane == mcrust::org::joml::frustum_intersection::INTERSECT);
    }

}
