//! Ported from de.hysky.skyblocker.utils.render.primitive.OutlinedCircleFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::outlined_circle_render_state::OutlinedCircleRenderState;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::org::joml::matrix4f::Matrix4f;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Outlined Circle");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Outlined Circle"));

/// java: public class OutlinedCircleFeatureRenderer extends PrimitiveFeatureRenderer<OutlinedCircleFeatureRenderer.Submit>
#[derive(Clone)]
pub struct OutlinedCircleFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for OutlinedCircleFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OutlinedCircleFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OutlinedCircleFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::outlined_circle_feature_renderer::Submit> for OutlinedCircleFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::outlined_circle_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::outlined_circle_feature_renderer::Submit> {
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
                let builder = self.get_vertex_builder(context, skyblocker_render_pipelines::CIRCLE_LINES.get_mut());
                let mut inner_radius: f32 = (state.radius() - (state.thickness() / 2.0f32));
                let mut outer_radius: f32 = (state.radius() + (state.thickness() / 2.0f32));
                for i in 0..state.segments() {
                    //   java: double angle1 = Math.TAU * i / state.segments();
                    let mut angle1: f64 = std::f64::consts::TAU * (i as f64) / (state.segments() as f64);
                    //   java: double angle2 = Math.TAU * (i + 1) / state.segments();
                    let mut angle2: f64 = std::f64::consts::TAU * ((i + 1) as f64) / (state.segments() as f64);
                    let mut x1_inner: f32 = (((angle1.cos()) as f32) * inner_radius);
                    let mut z1_inner: f32 = (((angle1.sin()) as f32) * inner_radius);
                    let mut x1_outer: f32 = (((angle1.cos()) as f32) * outer_radius);
                    let mut z1_outer: f32 = (((angle1.sin()) as f32) * outer_radius);
                    let mut x2_inner: f32 = (((angle2.cos()) as f32) * inner_radius);
                    let mut z2_inner: f32 = (((angle2.sin()) as f32) * inner_radius);
                    let mut x2_outer: f32 = (((angle2.cos()) as f32) * outer_radius);
                    let mut z2_outer: f32 = (((angle2.sin()) as f32) * outer_radius);
                    let mut cx: f32 = ((state.centre().x()) as f32);
                    let mut cy: f32 = ((state.centre().y()) as f32);
                    let mut cz: f32 = ((state.centre().z()) as f32);
                    { builder.add_vertex_matrix(&position_matrix, (cx + x1_inner), cy, (cz + z1_inner)); builder.set_color(state.colour()); };
                    { builder.add_vertex_matrix(&position_matrix, (cx + x1_outer), cy, (cz + z1_outer)); builder.set_color(state.colour()); };
                    { builder.add_vertex_matrix(&position_matrix, (cx + x2_outer), cy, (cz + z2_outer)); builder.set_color(state.colour()); };
                    { builder.add_vertex_matrix(&position_matrix, (cx + x2_inner), cy, (cz + z2_inner)); builder.set_color(state.colour()); };
                }
            }
        }
    }

}

/// java: public record Submit(List<OutlinedCircleRenderState> states, CameraRenderState camera) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<OutlinedCircleRenderState> states
    pub states: Vec<OutlinedCircleRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<OutlinedCircleRenderState>, mut camera: CameraRenderState) -> Self {
        Self { states, camera }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<OutlinedCircleRenderState> {
        self.states.clone()
    }

    /// java: the record accessor `camera()`
    pub fn camera(&self) -> CameraRenderState {
        self.camera.clone()
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
