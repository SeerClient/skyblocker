//! Ported from de.hysky.skyblocker.utils.render.primitive.FilledBoxFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::filled_box_render_state::FilledBoxRenderState;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::org::joml::matrix4f::Matrix4f;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Filled Box");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Filled Box"));

/// java: public class FilledBoxFeatureRenderer extends PrimitiveFeatureRenderer<FilledBoxFeatureRenderer.Submit>
#[derive(Clone)]
pub struct FilledBoxFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for FilledBoxFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FilledBoxFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FilledBoxFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::filled_box_feature_renderer::Submit> for FilledBoxFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::filled_box_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::filled_box_feature_renderer::Submit> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void buildGroup(FeatureFrameContext context, List<Submit> submits)
    fn build_group(&mut self, context: &mut FeatureFrameContext<'_>, submits: &[std::sync::Arc<dyn SubmitNode>]) {
        for submit in primitive_feature_renderer::submits::<Submit>(submits) {
            let mut position_matrix: Matrix4f = Matrix4f::new().translate((((-submit.camera.pos.x)) as f32), (((-submit.camera.pos.y)) as f32), (((-submit.camera.pos.z)) as f32));
            for state in submit.states.iter() {
                let builder = self.get_vertex_builder(context, &(if submit.through_walls() { skyblocker_render_pipelines::FILLED_THROUGH_WALLS.get_mut().clone() } else { mcrust::net::minecraft::client::renderer::render_pipelines::debug_filled_box() }));
                let mut min_x: f32 = ((state.min_x()) as f32);
                let mut min_y: f32 = ((state.min_y()) as f32);
                let mut min_z: f32 = ((state.min_z()) as f32);
                let mut max_x: f32 = ((state.max_x()) as f32);
                let mut max_y: f32 = ((state.max_y()) as f32);
                let mut max_z: f32 = ((state.max_z()) as f32);
                let mut red: f32 = state.colour_components()[(0) as usize];
                let mut green: f32 = state.colour_components()[(1) as usize];
                let mut blue: f32 = state.colour_components()[(2) as usize];
                let mut alpha: f32 = state.alpha();
                { builder.add_vertex_matrix(&position_matrix, min_x, min_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, min_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, max_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, max_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, min_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, min_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, max_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, max_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, min_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, min_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, max_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, max_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, min_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, min_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, max_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, max_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, max_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, max_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, max_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, max_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, min_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, min_y, min_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, max_x, min_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
                { builder.add_vertex_matrix(&position_matrix, min_x, min_y, max_z); builder.set_color_f32(red, green, blue, alpha); };
            }
        }
    }

}

/// java: public record Submit(List<FilledBoxRenderState> states, CameraRenderState camera, boolean throughWalls) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<FilledBoxRenderState> states
    pub states: Vec<FilledBoxRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
    /// java: boolean throughWalls
    pub through_walls: bool,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<FilledBoxRenderState>, mut camera: CameraRenderState, mut through_walls: bool) -> Self {
        Self { states, camera, through_walls }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<FilledBoxRenderState> {
        self.states.clone()
    }

    /// java: the record accessor `camera()`
    pub fn camera(&self) -> CameraRenderState {
        self.camera.clone()
    }

    /// java: the record accessor `throughWalls()`
    pub fn through_walls(&self) -> bool {
        self.through_walls
    }

    /// java: @Override public FeatureRendererType<? extends SubmitNode> featureType()
    pub fn feature_type(&self) -> FeatureRendererType {
        return TYPE.get_mut().clone();
    }

}

/// java: `implements SubmitNode` - the record's `featureType()`; `as_any` is the downcast `FeatureRenderer` reads it back through
impl SubmitNode for Submit {
    fn feature_type(&self) -> FeatureRendererType {
        *TYPE.get()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
