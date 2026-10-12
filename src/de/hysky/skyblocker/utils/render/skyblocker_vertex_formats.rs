//! Ported from de.hysky.skyblocker.utils.render.SkyblockerVertexFormats by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::vertex::vertex_format::VertexFormat;

/// java: public static final VertexFormat POSITION_NORMAL = VertexFormat.builder(0) .addAttribute(DefaultVertexFormat.POSITION_SEMANTIC_NAME, DefaultVertexFormat.POSITION_FORMAT) .addAttribute(DefaultVertexFor
pub static POSITION_NORMAL: compat::MainThread<VertexFormat> = compat::MainThread::new(|| (VertexFormat::builder(0).add_attribute(&mcrust::com::mojang::blaze3d::vertex::default_vertex_format::POSITION_SEMANTIC_NAME, mcrust::com::mojang::blaze3d::vertex::default_vertex_format::POSITION_FORMAT)).add_attribute(&mcrust::com::mojang::blaze3d::vertex::default_vertex_format::NORMAL_SEMANTIC_NAME, mcrust::com::mojang::blaze3d::vertex::default_vertex_format::NORMAL_FORMAT).build());

/// java: public class SkyblockerVertexFormats
#[derive(Clone)]
pub struct SkyblockerVertexFormats {}

impl SkyblockerVertexFormats {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}
