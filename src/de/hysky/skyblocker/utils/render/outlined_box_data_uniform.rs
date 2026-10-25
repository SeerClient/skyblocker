//! Ported from de.hysky.skyblocker.utils.render.OutlinedBoxDataUniform by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::abstract_uniform_texel_buffer::AbstractUniformTexelBuffer;
use crate::de::hysky::skyblocker::utils::render::abstract_uniform_texel_buffer::MemorySegmentBytes;
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
    fn write_to_buffer_4<S: MemorySegmentBytes + ?Sized>(&self, mut index: i32, state: &OutlinedBoxRenderState, camera: &CameraRenderState, buffer: &mut S) {
        let mut offset: i64 = ((index.wrapping_mul(*BYTES_PER_BOX)) as i64) as i64;
        let mut colour: i32 = AbstractUniformTexelBufferBase::<OutlinedBoxRenderState>::to_native_rgba(argb::color_from_float(state.alpha(), state.colour_components()[(0) as usize], state.colour_components()[(1) as usize], state.colour_components()[(2) as usize]));
        // java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 0L, (float) (state.minX() - camera.pos.x));
        buffer.set_java_float(offset + 0, (state.min_x() - camera.pos.x) as f32);
        // java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 4L, (float) (state.minY() - camera.pos.y));
        buffer.set_java_float(offset + 4, (state.min_y() - camera.pos.y) as f32);
        // java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 8L, (float) (state.minZ() - camera.pos.z));
        buffer.set_java_float(offset + 8, (state.min_z() - camera.pos.z) as f32);
        // java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 12L, (float) (state.maxX() - camera.pos.x));
        buffer.set_java_float(offset + 12, (state.max_x() - camera.pos.x) as f32);
        // java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 16L, (float) (state.maxY() - camera.pos.y));
        buffer.set_java_float(offset + 16, (state.max_y() - camera.pos.y) as f32);
        // java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 20L, (float) (state.maxZ() - camera.pos.z));
        buffer.set_java_float(offset + 20, (state.max_z() - camera.pos.z) as f32);
        // java: buffer.set(ValueLayout.JAVA_INT,   offset + 24L, colour);
        buffer.set_java_int(offset + 24, colour);
        // java: buffer.set(ValueLayout.JAVA_FLOAT, offset + 28L, state.lineWidth());
        buffer.set_java_float(offset + 28, state.line_width());
    }

}
