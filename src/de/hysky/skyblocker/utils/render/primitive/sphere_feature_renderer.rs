//! Ported from de.hysky.skyblocker.utils.render.primitive.SphereFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::sphere_render_state::SphereRenderState;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::org::joml::matrix4f::Matrix4f;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Sphere");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Sphere"));

/// java: public class SphereFeatureRenderer extends PrimitiveFeatureRenderer<SphereFeatureRenderer.Submit>
#[derive(Clone)]
pub struct SphereFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for SphereFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SphereFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SphereFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::sphere_feature_renderer::Submit> for SphereFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::sphere_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::sphere_feature_renderer::Submit> {
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
                let builder = self.get_vertex_builder(context, skyblocker_render_pipelines::CYLINDER.get_mut());
                for lat in 0..state.rings() {
                    let mut lat0: f64 = ((std::f64::consts::PI * ((lat) as f64)) / (state.rings() as f64));
                    let mut lat1: f64 = ((std::f64::consts::PI * ((lat.wrapping_add(1)) as f64)) / (state.rings() as f64));
                    let mut y0: f32 = (((lat0.cos()) as f32) * state.radius());
                    let mut y1: f32 = (((lat1.cos()) as f32) * state.radius());
                    let mut r0: f32 = (((lat0.sin()) as f32) * state.radius());
                    let mut r1: f32 = (((lat1.sin()) as f32) * state.radius());
                    for lon in 0..=state.segments() {
                        //   java: double angle = Math.TAU * (double) lon / state.segments();
                        let mut angle: f64 = std::f64::consts::TAU * (lon as f64) / (state.segments() as f64);
                        let mut x0: f32 = ((angle.cos()) as f32);
                        let mut z0: f32 = ((angle.sin()) as f32);
                        // First Triangle (java: Math.fma(float, float, float) - one rounding, `f32::mul_add`)
                        builder.add_vertex_matrix(&position_matrix, x0.mul_add(r0, state.centre().x() as f32), (state.centre().y() as f32) + y0, z0.mul_add(r0, state.centre().z() as f32));
                        builder.set_color(state.colour());
                        builder.add_vertex_matrix(&position_matrix, x0.mul_add(r1, state.centre().x() as f32), (state.centre().y() as f32) + y1, z0.mul_add(r1, state.centre().z() as f32));
                        builder.set_color(state.colour());
                    }
                }
            }
        }
    }

}

/// java: public record Submit(List<SphereRenderState> states, CameraRenderState camera) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<SphereRenderState> states
    pub states: Vec<SphereRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<SphereRenderState>, mut camera: CameraRenderState) -> Self {
        Self { states, camera }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<SphereRenderState> {
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
