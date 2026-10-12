//! Ported from de.hysky.skyblocker.utils.render.OutlinedBoxDataUniform by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::abstract_uniform_texel_buffer::AbstractUniformTexelBuffer;
use crate::de::hysky::skyblocker::utils::render::abstract_uniform_texel_buffer::AbstractUniformTexelBufferBase;
use crate::de::hysky::skyblocker::utils::render::state::outlined_box_render_state::OutlinedBoxRenderState;
use mcrust::com::mojang::blaze3d::gpu_format::GpuFormat;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::net::minecraft::util::argb;

/// java: private static final int TEXELS_PER_INSTANCE = 2;
pub const TEXELS_PER_INSTANCE: i32 = 2;
/// java: private static final int BYTES_PER_BOX = GpuFormat.RGBA32_FLOAT.blockSize() * TEXELS_PER_INSTANCE;
pub static BYTES_PER_BOX: LazyLock<i32> = LazyLock::new(|| (GpuFormat::Rgba32Float.block_size()).wrapping_mul(TEXELS_PER_INSTANCE));

/// java: public class OutlinedBoxDataUniform extends AbstractUniformTexelBuffer<OutlinedBoxRenderState>
#[derive(Clone)]
pub struct OutlinedBoxDataUniform {
    /// java: extends AbstractUniformTexelBuffer
    pub base: AbstractUniformTexelBufferBase<OutlinedBoxRenderState>,
}

impl std::ops::Deref for OutlinedBoxDataUniform {
    type Target = AbstractUniformTexelBufferBase<OutlinedBoxRenderState>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OutlinedBoxDataUniform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OutlinedBoxDataUniform {
    /// java: public OutlinedBoxDataUniform()
    pub fn new() -> Self {
        let mut this = Self {
            base: AbstractUniformTexelBufferBase::<OutlinedBoxRenderState>::new(*BYTES_PER_BOX),
        };
        this
    }

}

impl AbstractUniformTexelBuffer<crate::de::hysky::skyblocker::utils::render::state::outlined_box_render_state::OutlinedBoxRenderState> for OutlinedBoxDataUniform {
    fn abstract_uniform_texel_buffer_base(&self) -> &AbstractUniformTexelBufferBase<crate::de::hysky::skyblocker::utils::render::state::outlined_box_render_state::OutlinedBoxRenderState> {
        &self.base
    }
    fn abstract_uniform_texel_buffer_base_mut(&mut self) -> &mut AbstractUniformTexelBufferBase<crate::de::hysky::skyblocker::utils::render::state::outlined_box_render_state::OutlinedBoxRenderState> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void writeToBuffer(int index, OutlinedBoxRenderState state, CameraRenderState camera, MemorySegment buffer)
    fn write_to_buffer_4(&self, mut index: i32, state: &OutlinedBoxRenderState, camera: &CameraRenderState, buffer: &mut crate::unresolved::MemorySegment) {
        let mut offset: i64 = ((index.wrapping_mul(*BYTES_PER_BOX)) as i64) as i64;
        let mut colour: i32 = AbstractUniformTexelBufferBase::<OutlinedBoxRenderState>::to_native_rgba(argb::color_from_float(state.alpha(), state.colour_components()[(0) as usize], state.colour_components()[(1) as usize], state.colour_components()[(2) as usize]));
        // TODO(fabric2seer): `.set(..)` on a MemorySegment
        //   java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 0L, (float) (state.minX() - camera.pos.x));
        // TODO(fabric2seer): `.set(..)` on a MemorySegment
        //   java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 4L, (float) (state.minY() - camera.pos.y));
        // TODO(fabric2seer): `.set(..)` on a MemorySegment
        //   java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 8L, (float) (state.minZ() - camera.pos.z));
        // TODO(fabric2seer): `.set(..)` on a MemorySegment
        //   java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 12L, (float) (state.maxX() - camera.pos.x));
        // TODO(fabric2seer): `.set(..)` on a MemorySegment
        //   java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 16L, (float) (state.maxY() - camera.pos.y));
        // TODO(fabric2seer): `.set(..)` on a MemorySegment
        //   java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 20L, (float) (state.maxZ() - camera.pos.z));
        // TODO(fabric2seer): `.set(..)` on a MemorySegment
        //   java: buffer.set(ValueLayout.JAVA_INT,   offset + 24L, colour);
        // TODO(fabric2seer): `.set(..)` on a MemorySegment
        //   java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 28L, state.lineWidth());
    }

}
