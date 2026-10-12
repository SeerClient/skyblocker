//! Ported from de.hysky.skyblocker.utils.render.primitive.CylinderFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::cylinder_render_state::CylinderRenderState;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::org::joml::matrix4f::Matrix4f;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Cylinder");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Cylinder"));

/// java: public class CylinderFeatureRenderer extends PrimitiveFeatureRenderer<CylinderFeatureRenderer.Submit>
#[derive(Clone)]
pub struct CylinderFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for CylinderFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CylinderFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CylinderFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::cylinder_feature_renderer::Submit> for CylinderFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::cylinder_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::cylinder_feature_renderer::Submit> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void buildGroup(FeatureFrameContext context, List<Submit> submits)
    fn build_group(&self, context: &FeatureFrameContext, submits: &Vec<Submit>) {
        for submit in submits.iter() {
            let mut position_matrix: Matrix4f = Matrix4f::new().translate((((-submit.camera().pos.x)) as f32), (((-submit.camera().pos.y)) as f32), (((-submit.camera().pos.z)) as f32));
            for state in submit.states() {
                let mut builder: Box<dyn VertexConsumer> = self.get_vertex_builder(skyblocker_render_pipelines::CYLINDER.get_mut());
                let mut half_height: f32 = (state.height() / 2.0f32);
                for i in 0..=state.segments() {
                    // TODO(fabric2seer): static field Math.TAU
                    //   java: double angle = Math.TAU * i / state.segments();
                    let mut angle: f64 = todo!("fabric2seer: the initialiser of `angle` (TODO above)");
                    let mut dx: f32 = (((angle.cos()) as f32) * state.radius());
                    let mut dz: f32 = (((angle.sin()) as f32) * state.radius());
                    { builder.add_vertex_matrix(&position_matrix, (((state.centre().x()) as f32) + dx), (((state.centre().y()) as f32) + half_height), (((state.centre().z()) as f32) + dz)); builder.set_color(state.colour()); &mut builder };
                    { builder.add_vertex_matrix(&position_matrix, (((state.centre().x()) as f32) + dx), (((state.centre().y()) as f32) - half_height), (((state.centre().z()) as f32) + dz)); builder.set_color(state.colour()); &mut builder };
                }
            }
        }
    }

}

/// java: public record Submit(List<CylinderRenderState> states, CameraRenderState camera) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<CylinderRenderState> states
    pub states: Vec<CylinderRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<CylinderRenderState>, mut camera: CameraRenderState) -> Self {
        Self { states, camera }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<CylinderRenderState> {
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
