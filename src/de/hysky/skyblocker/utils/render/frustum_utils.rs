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
        // Java's body is `return isVisible(frustum, box);`, which resolves to THIS overload and
        // recurses until StackOverflowError (the yarn->mojmap move dropped the `frustum.`
        // receiver of the original `frustum.isVisible(box)`). It has no caller in the mod; the
        // receiver form is what the method was written to do.
        return frustum.is_visible(r#box);
    }

    /// java: public static boolean isVisible(Frustum frustum, double minX, double minY, double minZ, double maxX, double maxY, double maxZ)
    pub fn is_visible_7(frustum: &Frustum, mut min_x: f64, mut min_y: f64, mut min_z: f64, mut max_x: f64, mut max_y: f64, mut max_z: f64) -> bool {
        // java: int plane = ((FrustumInvoker) frustum).invokeCubeInFrustum(minX, minY, minZ, maxX, maxY, maxZ);
        // The six-double `Frustum.cubeInFrustum(DDDDDD)` is private in the port
        // (`cube_in_frustum_raw`, no `pub`), so the invoker cannot reach it (GAPS_J #1).
        // `Frustum.isVisible(AABB):82-85` is exactly `cubeInFrustum(minX..maxZ)` tested against
        // INSIDE / INTERSECT, and the port's `AABB` is a stack value (no allocation), so the
        // plane class is the same: INSIDE when visible, OUTSIDE otherwise.
        let plane: i32 = if frustum.is_visible(&AABB::new(min_x, min_y, min_z, max_x, max_y, max_z)) {
            mcrust::org::joml::frustum_intersection::INSIDE
        } else {
            mcrust::org::joml::frustum_intersection::OUTSIDE
        };
        return (plane == mcrust::org::joml::frustum_intersection::INSIDE || plane == mcrust::org::joml::frustum_intersection::INTERSECT);
    }

}
