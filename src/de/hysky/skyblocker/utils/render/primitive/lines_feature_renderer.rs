//! Ported from de.hysky.skyblocker.utils.render.primitive.LinesFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::lines_render_state::LinesRenderState;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::matrix4f::Matrix4f;
use mcrust::org::joml::vector3f::Vector3f;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Lines");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Lines"));

/// java: public class LinesFeatureRenderer extends PrimitiveFeatureRenderer<LinesFeatureRenderer.Submit>
#[derive(Clone)]
pub struct LinesFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for LinesFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LinesFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LinesFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::lines_feature_renderer::Submit> for LinesFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::lines_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::lines_feature_renderer::Submit> {
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
                let mut points: Vec<Vec3> = state.points();
                let builder = self.get_vertex_builder(context, &(if submit.through_walls() { skyblocker_render_pipelines::LINES_THROUGH_WALLS.get_mut().clone() } else { mcrust::net::minecraft::client::renderer::render_pipelines::lines() }));
                for i in 0..(points.len() as i32) {
                    let next_point = &points[((if i.wrapping_add(1) == (points.len() as i32) { i.wrapping_sub(1) } else { i.wrapping_add(1) })) as usize];
                    // java: Vector3f normalVec = nextPoint.toVector3f().sub((float) points[i].x(), (float) points[i].y(), (float) points[i].z()).normalize();
                    let mut normal_vec: Vector3f = next_point.to_vector3f().sub(&Vector3f::new((points[i as usize]).x() as f32, (points[i as usize]).y() as f32, (points[i as usize]).z() as f32)).normalize();
                    if i.wrapping_add(1) == (points.len() as i32) {
                        // java: normalVec.negate(); - JOML negates in place
                        normal_vec = normal_vec.negate();
                    }
                    { builder.add_vertex_matrix(&position_matrix, (((points[(i) as usize]).x()) as f32), (((points[(i) as usize]).y()) as f32), (((points[(i) as usize]).z()) as f32)); builder.set_color_f32(state.colour_components()[(0) as usize], state.colour_components()[(1) as usize], state.colour_components()[(2) as usize], state.alpha()); builder.set_normal(normal_vec.x(), normal_vec.y(), normal_vec.z()); builder.set_line_width(state.line_width()); };
                }
            }
        }
    }

}

/// java: public record Submit(List<LinesRenderState> states, CameraRenderState camera, boolean throughWalls) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<LinesRenderState> states
    pub states: Vec<LinesRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
    /// java: boolean throughWalls
    pub through_walls: bool,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<LinesRenderState>, mut camera: CameraRenderState, mut through_walls: bool) -> Self {
        Self { states, camera, through_walls }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<LinesRenderState> {
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
