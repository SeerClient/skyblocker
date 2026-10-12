//! Ported from de.hysky.skyblocker.utils.render.primitive.CursorLineFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::cursor_line_render_state::CursorLineRenderState;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::matrix4f::Matrix4f;
use mcrust::org::joml::vector3f::Vector3f;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Cursor Line");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Cursor Line"));

/// java: public class CursorLineFeatureRenderer extends PrimitiveFeatureRenderer<CursorLineFeatureRenderer.Submit>
#[derive(Clone)]
pub struct CursorLineFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for CursorLineFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CursorLineFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CursorLineFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::cursor_line_feature_renderer::Submit> for CursorLineFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::cursor_line_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::cursor_line_feature_renderer::Submit> {
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
                let mut builder: Box<dyn VertexConsumer> = self.get_vertex_builder(skyblocker_render_pipelines::LINES_THROUGH_WALLS.get_mut());
                let mut point: Vec3 = state.point();
                // TODO(fabric2seer): `transform` takes 3 argument(s) in the port (`x: f32, y: f32, z: f32`), Java passes 1
                // rust: let mut camera_point: Vec3 = submit.camera().pos.add_vec(&Vec3::from_vector3f(&submit.camera().orientation.transform(Vector3f::new((0) as f32, (0) as f32, (-1) as f32))));
                let mut camera_point: Vec3 = todo!("fabric2seer: the initialiser of `cameraPoint` (TODO above)");
                // TODO(fabric2seer): `sub` takes 1 argument(s) in the port (`v: &Vector3f`), Java passes 3
                // rust: let mut normal: Vector3f = (point.to_vector3f().sub(&((camera_point.x) as f32), ((camera_point.y) as f32), ((camera_point.z) as f32))).normalize();
                let mut normal: Vector3f = todo!("fabric2seer: the initialiser of `normal` (TODO above)");
                { builder.add_vertex_matrix(&position_matrix, ((camera_point.x) as f32), ((camera_point.y) as f32), ((camera_point.z) as f32)); builder.set_color_f32(state.colour_components()[(0) as usize], state.colour_components()[(1) as usize], state.colour_components()[(2) as usize], state.alpha()); builder.set_normal(normal.x(), normal.y(), normal.z()); builder.set_line_width(state.line_width()); &mut builder };
                { builder.add_vertex_matrix(&position_matrix, ((point.x()) as f32), ((point.y()) as f32), ((point.z()) as f32)); builder.set_color_f32(state.colour_components()[(0) as usize], state.colour_components()[(1) as usize], state.colour_components()[(2) as usize], state.alpha()); builder.set_normal(normal.x(), normal.y(), normal.z()); builder.set_line_width(state.line_width()); &mut builder };
            }
        }
    }

}

/// java: public record Submit(List<CursorLineRenderState> states, CameraRenderState camera) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<CursorLineRenderState> states
    pub states: Vec<CursorLineRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<CursorLineRenderState>, mut camera: CameraRenderState) -> Self {
        Self { states, camera }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<CursorLineRenderState> {
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
