//! Ported from de.hysky.skyblocker.utils.render.primitive.FilledBoxInstancedFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::box_data_uniform::BoxDataUniform;
use crate::de::hysky::skyblocker::utils::render::instancing_parameters::InstancingParameters;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::skyblocker_render_pipelines;
use crate::de::hysky::skyblocker::utils::render::state::filled_box_render_state::FilledBoxRenderState;
use mcrust::com::mojang::blaze3d::buffers::gpu_buffer_slice::GpuBufferSlice;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::com::mojang::blaze3d::systems::gpu_device::GpuDevice;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use crate::de::hysky::skyblocker::utils::render::abstract_uniform_texel_buffer::AbstractUniformTexelBuffer as _;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Filled Box Instanced");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Filled Box Instanced"));
/// java: private static final AABB UNIT_BOX = new AABB(BlockPos.ZERO);
pub static UNIT_BOX: compat::MainThread<AABB> = compat::MainThread::new(|| AABB::of_block_pos(&mcrust::net::minecraft::core::block_pos::ZERO));

/// java: public class FilledBoxInstancedFeatureRenderer extends PrimitiveFeatureRenderer<FilledBoxInstancedFeatureRenderer.Submit>
#[derive(Clone)]
pub struct FilledBoxInstancedFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
    /// java: private final BoxDataUniform boxData = new BoxDataUniform();
    pub box_data: BoxDataUniform,
}

impl std::ops::Deref for FilledBoxInstancedFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FilledBoxInstancedFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FilledBoxInstancedFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
            box_data: BoxDataUniform::new(),
        };
        this
    }

    /// java: private static void buildUnitBox(float minX, float minY, float minZ, float maxX, float maxY, float maxZ, VertexConsumer builder)
    pub fn build_unit_box(mut min_x: f32, mut min_y: f32, mut min_z: f32, mut max_x: f32, mut max_y: f32, mut max_z: f32, builder: &mut dyn VertexConsumer) {
        { builder.add_vertex(min_x, min_y, min_z); &mut *builder };
        { builder.add_vertex(max_x, min_y, min_z); &mut *builder };
        { builder.add_vertex(max_x, max_y, min_z); &mut *builder };
        { builder.add_vertex(min_x, max_y, min_z); &mut *builder };
        { builder.add_vertex(max_x, min_y, max_z); &mut *builder };
        { builder.add_vertex(min_x, min_y, max_z); &mut *builder };
        { builder.add_vertex(min_x, max_y, max_z); &mut *builder };
        { builder.add_vertex(max_x, max_y, max_z); &mut *builder };
        { builder.add_vertex(min_x, min_y, max_z); &mut *builder };
        { builder.add_vertex(min_x, min_y, min_z); &mut *builder };
        { builder.add_vertex(min_x, max_y, min_z); &mut *builder };
        { builder.add_vertex(min_x, max_y, max_z); &mut *builder };
        { builder.add_vertex(max_x, min_y, min_z); &mut *builder };
        { builder.add_vertex(max_x, min_y, max_z); &mut *builder };
        { builder.add_vertex(max_x, max_y, max_z); &mut *builder };
        { builder.add_vertex(max_x, max_y, min_z); &mut *builder };
        { builder.add_vertex(min_x, max_y, min_z); &mut *builder };
        { builder.add_vertex(max_x, max_y, min_z); &mut *builder };
        { builder.add_vertex(max_x, max_y, max_z); &mut *builder };
        { builder.add_vertex(min_x, max_y, max_z); &mut *builder };
        { builder.add_vertex(min_x, min_y, max_z); &mut *builder };
        { builder.add_vertex(max_x, min_y, max_z); &mut *builder };
        { builder.add_vertex(max_x, min_y, min_z); &mut *builder };
        { builder.add_vertex(min_x, min_y, min_z); &mut *builder };
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::filled_box_instanced_feature_renderer::Submit> for FilledBoxInstancedFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::filled_box_instanced_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::filled_box_instanced_feature_renderer::Submit> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void buildGroup(FeatureFrameContext context, List<Submit> submits)
    fn build_group(&self, context: &FeatureFrameContext, submits: &Vec<Submit>) {
        {
            // TODO(fabric2seer): static call Arena.ofConfined(..)
            //   java: Arena arena = Arena.ofConfined(..);
            let mut arena: crate::unresolved::Arena = todo!("fabric2seer: the initialiser of `arena` (TODO above)");
            for submit in submits.iter() {
                let mut boxes: i32 = (submit.states().len() as i32);
                // TODO(fabric2seer): `.allocate(..)` on a Arena
                //   java: MemorySegment uniformSegment = arena.allocate(this.boxData.calculateRequiredSize(boxes));
                let mut uniform_segment: crate::unresolved::MemorySegment = todo!("fabric2seer: the initialiser of `uniformSegment` (TODO above)");
                self.box_data.write_to_buffer(&submit.states(), &submit.camera(), &uniform_segment);
                let mut device: GpuDevice = render_system::get_device();
                // TODO(fabric2seer): `.asByteBuffer(..)` on a MemorySegment
                //   java: GpuBufferSlice tbo = device.createCommandEncoder()
                //   java: 						.transientMemory()
                //   java: 						.uploadGpu(uniformSegment.asByteBuffer(), device.getDeviceInfo().limits().minUniformOffsetAlignment(), GpuBuffer.USAGE_UNIFORM_TEXEL_BUFFER);
                let mut tbo: GpuBufferSlice = todo!("fabric2seer: the initialiser of `tbo` (TODO above)");
                let mut pipeline: RenderPipeline = (if submit.through_walls() { skyblocker_render_pipelines::FILLED_THROUGH_WALLS_INSTANCED.get_mut().clone() } else { skyblocker_render_pipelines::FILLED_INSTANCED.get_mut().clone() });
                let mut builder: Box<dyn VertexConsumer> = self.get_vertex_builder_render_pipeline_instancing_parameters(&pipeline, &InstancingParameters::new(boxes, Some("BoxData".to_string()), Some(tbo.clone())));
                FilledBoxInstancedFeatureRenderer::build_unit_box(((UNIT_BOX.get_mut().min_x) as f32), ((UNIT_BOX.get_mut().min_y) as f32), ((UNIT_BOX.get_mut().min_z) as f32), ((UNIT_BOX.get_mut().max_x) as f32), ((UNIT_BOX.get_mut().max_y) as f32), ((UNIT_BOX.get_mut().max_z) as f32), &mut builder);
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
