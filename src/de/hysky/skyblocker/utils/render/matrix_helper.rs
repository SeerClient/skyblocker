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
    // TODO(fabric2seer): `new` takes 0 argument(s) in the port (``), Java passes 1
    // rust: return Matrix4f::new(matrix.clone());
    return todo!("fabric2seer: see the TODO above");
}

/// java: static PoseStack toStack(Matrix4f positionMatrix)
pub fn to_stack(mut position_matrix: Matrix4f) -> PoseStack {
    let mut matrices: PoseStack = PoseStack::new();
    // TODO(fabric2seer): `Matrix4f.angle` has no Rust field in the port (searched org.joml.Matrix4f and its supertypes)
    // TODO(fabric2seer): `Matrix4f.x` has no Rust field in the port (searched org.joml.Matrix4f and its supertypes)
    // TODO(fabric2seer): `Matrix4f.y` has no Rust field in the port (searched org.joml.Matrix4f and its supertypes)
    // TODO(fabric2seer): `Matrix4f.z` has no Rust field in the port (searched org.joml.Matrix4f and its supertypes)
    // TODO(fabric2seer): `Matrix4f.rotation` has no Rust method in the port (looked for `rotation` in Matrix4f and its supertypes)
    // rust: matrices.last().pose().rotation(&position_matrix.angle, &position_matrix.x, &position_matrix.y, &position_matrix.z);
    return matrices;
}
