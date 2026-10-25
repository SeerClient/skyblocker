//! Ported from de.hysky.skyblocker.utils.render.SkyblockerRenderPipelines by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::compatibility::iris_compatibility::IrisCompatibility;
use crate::de::hysky::skyblocker::utils::render::{skyblocker_bind_group_layouts, skyblocker_vertex_formats};
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::blaze3d::pipeline::blend_function;
use mcrust::com::mojang::blaze3d::pipeline::color_target_state::ColorTargetState;
use mcrust::com::mojang::blaze3d::pipeline::depth_stencil_state::{self, DepthStencilState};
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::{RenderPipeline, Snippet};
use mcrust::com::mojang::blaze3d::platform::compare_op::CompareOp;
use mcrust::com::mojang::blaze3d::primitive_topology::PrimitiveTopology;
use mcrust::com::mojang::blaze3d::vertex::default_vertex_format as dvf;
use mcrust::net::minecraft::client::renderer::bind_group_layouts as bgl;

// `RenderPipelines.register(..)` and the snippets Java reads (`DEBUG_FILLED_SNIPPET`, `LINES_SNIPPET`,
// `GUI_SNIPPET`, `GUI_TEXTURED_SNIPPET`) are private in the port (`render_pipelines.rs:60`, `:547-739`). The
// snippets are rebuilt below from the same public builder calls, statement for statement
// (`RenderPipelines.java:23-169`); `register` keeps Java's list of the registered static pipelines. The port's
// device compiles a pipeline lazily on its first draw (`GlDevice::get_or_compile_pipeline`), so a pipeline that
// is not in the game's own registry draws exactly as a registered one does.

thread_local! {
    /// java: RenderPipelines.PIPELINES_BY_LOCATION (the mod's share of it)
    static REGISTERED: std::cell::RefCell<Vec<std::sync::Arc<RenderPipeline>>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// java: RenderPipelines.register(RenderPipeline)
fn register(pipeline: RenderPipeline) -> std::sync::Arc<RenderPipeline> {
    let pipeline = std::sync::Arc::new(pipeline);
    REGISTERED.with(|r| r.borrow_mut().push(pipeline.clone()));
    pipeline
}

/// java: RenderPipelines.GLOBALS_SNIPPET
fn globals_snippet() -> Snippet {
    let mut b = RenderPipeline::builder(&[]);
    b.with_bind_group_layout(bgl::globals());
    b.build_snippet()
}

/// java: RenderPipelines.MATRICES_FOG_SNIPPET
fn matrices_fog_snippet() -> Snippet {
    let mut b = RenderPipeline::builder(&[globals_snippet()]);
    // 26.3: `MATRICES_PROJECTION` split into `PROJECTION` + `DYNAMIC_TRANSFORMS` (RenderPipelines.java:26-30, :247-249, :306-317)
    b.with_bind_group_layout(bgl::projection());
    b.with_bind_group_layout(bgl::dynamic_transforms());
    b.with_bind_group_layout(bgl::fog());
    b.build_snippet()
}

/// java: RenderPipelines.LINES_SNIPPET
fn lines_snippet() -> Snippet {
    let mut b = RenderPipeline::builder(&[matrices_fog_snippet()]);
    b.with_vertex_shader("core/rendertype_lines");
    b.with_fragment_shader("core/rendertype_lines");
    // 26.3 `LINES_SNIPPET` (RenderPipelines.java:230-237) sets no colour target state
    b.with_cull(false);
    b.with_vertex_binding(0, dvf::position_color_normal_line_width().clone());
    b.with_primitive_topology(PrimitiveTopology::Lines);
    b.with_depth_stencil_state(Some(depth_stencil_state::DEFAULT));
    b.build_snippet()
}

/// java: RenderPipelines.DEBUG_FILLED_SNIPPET
fn debug_filled_snippet() -> Snippet {
    let mut b = RenderPipeline::builder(&[globals_snippet()]);
    // 26.3: `MATRICES_PROJECTION` split into `PROJECTION` + `DYNAMIC_TRANSFORMS` (RenderPipelines.java:26-30, :247-249, :306-317)
    b.with_bind_group_layout(bgl::projection());
    b.with_bind_group_layout(bgl::dynamic_transforms());
    b.with_vertex_shader("core/position_color");
    b.with_fragment_shader("core/position_color");
    b.with_color_target_state(ColorTargetState::of_blend(blend_function::TRANSLUCENT));
    b.with_vertex_binding(0, dvf::position_color().clone());
    b.with_primitive_topology(PrimitiveTopology::Quads);
    b.with_depth_stencil_state(Some(DepthStencilState::new(CompareOp::GreaterThanOrEqual, false)));
    b.with_cull(false);
    b.build_snippet()
}

/// java: RenderPipelines.GUI_SNIPPET
fn gui_snippet() -> Snippet {
    let mut b = RenderPipeline::builder(&[globals_snippet()]);
    // 26.3: `MATRICES_PROJECTION` split into `PROJECTION` + `DYNAMIC_TRANSFORMS` (RenderPipelines.java:26-30, :247-249, :306-317)
    b.with_bind_group_layout(bgl::projection());
    b.with_bind_group_layout(bgl::dynamic_transforms());
    b.with_vertex_shader("core/gui");
    b.with_fragment_shader("core/gui");
    b.with_color_target_state(ColorTargetState::of_blend(blend_function::TRANSLUCENT));
    b.with_vertex_binding(0, dvf::position_color().clone());
    b.with_primitive_topology(PrimitiveTopology::Quads);
    b.build_snippet()
}

/// java: RenderPipelines.GUI_TEXTURED_SNIPPET
fn gui_textured_snippet() -> Snippet {
    let mut b = RenderPipeline::builder(&[globals_snippet()]);
    // 26.3: `MATRICES_PROJECTION` split into `PROJECTION` + `DYNAMIC_TRANSFORMS` (RenderPipelines.java:26-30, :247-249, :306-317)
    b.with_bind_group_layout(bgl::projection());
    b.with_bind_group_layout(bgl::dynamic_transforms());
    b.with_vertex_shader("core/position_tex_color");
    b.with_fragment_shader("core/position_tex_color");
    b.with_bind_group_layout(bgl::sampler0());
    b.with_color_target_state(ColorTargetState::of_blend(blend_function::TRANSLUCENT));
    b.with_vertex_binding(0, dvf::position_tex_color().clone());
    b.with_primitive_topology(PrimitiveTopology::Quads);
    b.build_snippet()
}

//   java: public static final RenderPipeline FILLED_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/debug_filled_box_instanced"))
//   java: 			.withVertexShader(SkyblockerMod.id("core/filled_box"))
//   java: 			.withBindGroupLayout(SkyblockerBindGroupLayouts.BOX_DATA)
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.QUADS)
//   java: 			.withCull(false)
//   java: 			.build());
pub static FILLED_INSTANCED: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[debug_filled_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/debug_filled_box_instanced"));
    b.with_vertex_shader_id(skyblocker_mod::id("core/filled_box"));
    b.with_bind_group_layout(skyblocker_bind_group_layouts::BOX_DATA.get().clone());
    b.with_vertex_binding(0, dvf::position().clone());
    b.with_primitive_topology(PrimitiveTopology::Quads);
    b.with_cull(false);
    register(b.build())
});
//   java: public static final RenderPipeline FILLED_THROUGH_WALLS_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/debug_filled_box_through_walls_instanced"))
//   java: 			.withVertexShader(SkyblockerMod.id("core/filled_box"))
//   java: 			.withBindGroupLayout(SkyblockerBindGroupLayouts.BOX_DATA)
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.QUADS)
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.build());
pub static FILLED_THROUGH_WALLS_INSTANCED: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[debug_filled_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/debug_filled_box_through_walls_instanced"));
    b.with_vertex_shader_id(skyblocker_mod::id("core/filled_box"));
    b.with_bind_group_layout(skyblocker_bind_group_layouts::BOX_DATA.get().clone());
    b.with_vertex_binding(0, dvf::position().clone());
    b.with_primitive_topology(PrimitiveTopology::Quads);
    b.with_depth_stencil_state(None);
    register(b.build())
});
//   java: public static final RenderPipeline FILLED_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/debug_filled_box_through_walls"))
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.build());
pub static FILLED_THROUGH_WALLS: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[debug_filled_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/debug_filled_box_through_walls"));
    b.with_depth_stencil_state(None);
    register(b.build())
});
//   java: public static final RenderPipeline OUTLINED_BOX_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/outlined_box_instanced"))
//   java: 			.withVertexShader(SkyblockerMod.id("core/outlined_box"))
//   java: 			.withBindGroupLayout(SkyblockerBindGroupLayouts.OUTLINED_BOX_DATA)
//   java: 			.withVertexBinding(0, SkyblockerVertexFormats.POSITION_NORMAL)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.LINES)
//   java: 			.build());
pub static OUTLINED_BOX_INSTANCED: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[lines_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/outlined_box_instanced"));
    b.with_vertex_shader_id(skyblocker_mod::id("core/outlined_box"));
    b.with_bind_group_layout(skyblocker_bind_group_layouts::OUTLINED_BOX_DATA.get().clone());
    b.with_vertex_binding(0, skyblocker_vertex_formats::POSITION_NORMAL.get().clone());
    b.with_primitive_topology(PrimitiveTopology::Lines);
    register(b.build())
});
//   java: public static final RenderPipeline OUTLINED_BOX_THROUGH_WALLS_INSTANCED = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/outlined_box_through_walls_instanced"))
//   java: 			.withVertexShader(SkyblockerMod.id("core/outlined_box"))
//   java: 			.withBindGroupLayout(SkyblockerBindGroupLayouts.OUTLINED_BOX_DATA)
//   java: 			.withVertexBinding(0, SkyblockerVertexFormats.POSITION_NORMAL)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.LINES)
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.build());
pub static OUTLINED_BOX_THROUGH_WALLS_INSTANCED: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[lines_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/outlined_box_through_walls_instanced"));
    b.with_vertex_shader_id(skyblocker_mod::id("core/outlined_box"));
    b.with_bind_group_layout(skyblocker_bind_group_layouts::OUTLINED_BOX_DATA.get().clone());
    b.with_vertex_binding(0, skyblocker_vertex_formats::POSITION_NORMAL.get().clone());
    b.with_primitive_topology(PrimitiveTopology::Lines);
    b.with_depth_stencil_state(None);
    register(b.build())
});
//   java: public static final RenderPipeline LINES_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.LINES_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/lines_through_walls"))
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.build());
pub static LINES_THROUGH_WALLS: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[lines_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/lines_through_walls"));
    b.with_depth_stencil_state(None);
    register(b.build())
});
//   java: public static final RenderPipeline QUADS_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/debug_quads_through_walls"))
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.withCull(false)
//   java: 			.build());
pub static QUADS_THROUGH_WALLS: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[debug_filled_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/debug_quads_through_walls"));
    b.with_depth_stencil_state(None);
    b.with_cull(false);
    register(b.build())
});
//   java: public static final RenderPipeline TEXTURE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_TEXTURED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/texture"))
//   java: 			.withDepthStencilState(new DepthStencilState(CompareOp.GREATER_THAN_OR_EQUAL, false))
//   java: 			.withCull(false)
//   java: 			.build());
pub static TEXTURE: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[gui_textured_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/texture"));
    b.with_depth_stencil_state(Some(DepthStencilState::new(CompareOp::GreaterThanOrEqual, false)));
    b.with_cull(false);
    register(b.build())
});
//   java: public static final RenderPipeline TEXTURE_THROUGH_WALLS = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_TEXTURED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/texture_through_walls"))
//   java: 			.withDepthStencilState(Optional.empty())
//   java: 			.withCull(false)
//   java: 			.build());
pub static TEXTURE_THROUGH_WALLS: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[gui_textured_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/texture_through_walls"));
    b.with_depth_stencil_state(None);
    b.with_cull(false);
    register(b.build())
});
//   java: public static final RenderPipeline CYLINDER = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/cylinder"))
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION_COLOR)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.TRIANGLE_STRIP)
//   java: 			.withCull(false)
//   java: 			.build());
pub static CYLINDER: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[debug_filled_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/cylinder"));
    b.with_vertex_binding(0, dvf::position_color().clone());
    b.with_primitive_topology(PrimitiveTopology::TriangleStrip);
    b.with_cull(false);
    register(b.build())
});
//   java: public static final RenderPipeline CIRCLE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/circle"))
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION_COLOR)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.TRIANGLE_FAN)
//   java: 			.withCull(false)
//   java: 			.build());
pub static CIRCLE: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[debug_filled_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/circle"));
    b.with_vertex_binding(0, dvf::position_color().clone());
    b.with_primitive_topology(PrimitiveTopology::TriangleFan);
    b.with_cull(false);
    register(b.build())
});
//   java: public static final RenderPipeline CIRCLE_LINES = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.DEBUG_FILLED_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/circle_lines"))
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION_COLOR)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.QUADS)
//   java: 			.withCull(false)
//   java: 			.build());
pub static CIRCLE_LINES: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[debug_filled_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/circle_lines"));
    b.with_vertex_binding(0, dvf::position_color().clone());
    b.with_primitive_topology(PrimitiveTopology::Quads);
    b.with_cull(false);
    register(b.build())
});
//   java: public static final RenderPipeline BLURRED_RECTANGLE = RenderPipelines.register(RenderPipeline.builder(RenderPipelines.GUI_SNIPPET)
//   java: 			.withLocation(SkyblockerMod.id("pipeline/blurred_rectangle"))
//   java: 			.withVertexShader("core/position_color")
//   java: 			.withFragmentShader(SkyblockerMod.id("core/box_blur"))
//   java: 			.withBindGroupLayout(BindGroupLayouts.SAMPLER0)
//   java: 			.withVertexBinding(0, DefaultVertexFormat.POSITION_COLOR)
//   java: 			.withPrimitiveTopology(PrimitiveTopology.QUADS)
//   java: 			.build());
pub static BLURRED_RECTANGLE: compat::MainThread<std::sync::Arc<RenderPipeline>> = compat::MainThread::new(|| {
    let mut b = RenderPipeline::builder(&[gui_snippet()]);
    b.with_location_id(skyblocker_mod::id("pipeline/blurred_rectangle"));
    b.with_vertex_shader("core/position_color");
    b.with_fragment_shader_id(skyblocker_mod::id("core/box_blur"));
    b.with_bind_group_layout(bgl::sampler0());
    b.with_vertex_binding(0, dvf::position_color().clone());
    b.with_primitive_topology(PrimitiveTopology::Quads);
    register(b.build())
});

/// java: public class SkyblockerRenderPipelines
#[derive(Clone)]
pub struct SkyblockerRenderPipelines {}

impl SkyblockerRenderPipelines {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        IrisCompatibility::assign_pipelines();
    }

}
