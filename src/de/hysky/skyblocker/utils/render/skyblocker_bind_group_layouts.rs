//! Ported from de.hysky.skyblocker.utils.render.SkyblockerBindGroupLayouts by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::gpu_format::GpuFormat;
use mcrust::com::mojang::blaze3d::pipeline::bind_group_layout::BindGroupLayout;
use mcrust::com::mojang::blaze3d::shaders::uniform_type::UniformType;

/// java: public static final BindGroupLayout BOX_DATA = BindGroupLayout.builder() .withUniform("BoxData", UniformType.TEXEL_BUFFER, GpuFormat.RGBA32_FLOAT) .build();
pub static BOX_DATA: compat::MainThread<BindGroupLayout> = compat::MainThread::new(|| (BindGroupLayout::builder().with_uniform_format("BoxData", UniformType::TexelBuffer, GpuFormat::Rgba32Float)).build());
/// java: public static final BindGroupLayout OUTLINED_BOX_DATA = BindGroupLayout.builder() .withUniform("OutlinedBoxData", UniformType.TEXEL_BUFFER, GpuFormat.RGBA32_FLOAT) .build();
pub static OUTLINED_BOX_DATA: compat::MainThread<BindGroupLayout> = compat::MainThread::new(|| (BindGroupLayout::builder().with_uniform_format("OutlinedBoxData", UniformType::TexelBuffer, GpuFormat::Rgba32Float)).build());

/// java: public class SkyblockerBindGroupLayouts
#[derive(Clone)]
pub struct SkyblockerBindGroupLayouts {}

impl SkyblockerBindGroupLayouts {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}
