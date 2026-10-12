//! Ported from de.hysky.skyblocker.utils.render.MatrixHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::vertex::pose_stack::PoseStack;
use mcrust::org::joml::matrix4f::Matrix4f;

/// java: public interface MatrixHelper
pub trait MatrixHelper: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// java: static Matrix4f copyOf(Matrix4f matrix)
pub fn copy_of(matrix: &Matrix4f) -> Matrix4f {
    // java: return new Matrix4f(matrix); - JOML's copy constructor copies the sixteen
    // elements and `properties`; the port's `Matrix4f` is `Copy`.
    return *matrix;
}

/// java: static PoseStack toStack(Matrix4f positionMatrix)
pub fn to_stack(mut position_matrix: Matrix4f) -> PoseStack {
    let mut matrices: PoseStack = PoseStack::new();
    // java: matrices.last().pose().set(positionMatrix); - `Matrix4f.set(Matrix4fc)` copies every
    // element and `properties`, which is the port's `Copy` assignment.
    *matrices.last_mut().pose_mut() = position_matrix;
    return matrices;
}
