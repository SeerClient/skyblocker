//! Ported from de.hysky.skyblocker.utils.render.primitive.BlockHologramFeatureRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::matrix_helper;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRenderer;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererBase;
use crate::de::hysky::skyblocker::utils::render::state::block_hologram_render_state::BlockHologramRenderState;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::com::mojang::blaze3d::textures::address_mode::AddressMode;
use mcrust::com::mojang::blaze3d::textures::filter_mode::FilterMode;
use mcrust::com::mojang::blaze3d::textures::gpu_sampler::GpuSampler;
use mcrust::com::mojang::blaze3d::textures::gpu_texture_view::GpuTextureView;
use mcrust::com::mojang::blaze3d::vertex::pose_stack::PoseStack;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::net::minecraft::client::renderer::block::dispatch::block_state_model::BlockStateModel;
use mcrust::net::minecraft::client::renderer::feature::feature_frame_context::FeatureFrameContext;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type;
use mcrust::net::minecraft::client::renderer::feature::feature_renderer_type::FeatureRendererType;
use mcrust::net::minecraft::client::renderer::feature::submit::submit_node::SubmitNode;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::org::joml::matrix4f::Matrix4f;

/// java: public static final FeatureRendererType<Submit> TYPE = FeatureRendererType.create("Skyblocker Block Hologram");
pub static TYPE: compat::MainThread<FeatureRendererType> = compat::MainThread::new(|| feature_renderer_type::create("Skyblocker Block Hologram"));

/// java: public class BlockHologramFeatureRenderer extends PrimitiveFeatureRenderer<BlockHologramFeatureRenderer.Submit>
#[derive(Clone)]
pub struct BlockHologramFeatureRenderer {
    /// java: extends PrimitiveFeatureRenderer
    pub base: PrimitiveFeatureRendererBase<Submit>,
}

impl std::ops::Deref for BlockHologramFeatureRenderer {
    type Target = PrimitiveFeatureRendererBase<Submit>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BlockHologramFeatureRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BlockHologramFeatureRenderer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: PrimitiveFeatureRendererBase::<Submit>::new(),
        };
        this
    }

}

impl PrimitiveFeatureRenderer<crate::de::hysky::skyblocker::utils::render::primitive::block_hologram_feature_renderer::Submit> for BlockHologramFeatureRenderer {
    fn primitive_feature_renderer_base(&self) -> &PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::block_hologram_feature_renderer::Submit> {
        &self.base
    }
    fn primitive_feature_renderer_base_mut(&mut self) -> &mut PrimitiveFeatureRendererBase<crate::de::hysky::skyblocker::utils::render::primitive::block_hologram_feature_renderer::Submit> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void buildGroup(FeatureFrameContext context, List<Submit> submits)
    fn build_group(&self, context: &FeatureFrameContext, submits: &Vec<Submit>) {
        let blocks_atlas_texture = context.texture_manager().get_texture_or_request(&mcrust::net::minecraft::client::renderer::texture::texture_atlas::location_blocks()).unwrap().get_texture_view();
        // TODO(fabric2seer): `with_sampler_cache` takes 1 argument(s) in the port (`f: impl FnOnce(&SamplerCache) -> R`), Java passes 0
        // rust: let mut sampler: Box<dyn GpuSampler> = render_system::with_sampler_cache().get_sampler(AddressMode::ClampToEdge, AddressMode::ClampToEdge, FilterMode::Linear, FilterMode::Nearest, true);
        let mut sampler: Box<dyn GpuSampler> = todo!("fabric2seer: the initialiser of `sampler` (TODO above)");
        // TODO(fabric2seer): Fabric API `Renderer.get` has no SeerClient equivalent
        //   java: AltModelBlockRenderer altModelBlockRenderer = Renderer.get().altModelBlockRenderer(context.options().ambientOcclusion, false, context.blockColors());
        let mut alt_model_block_renderer: crate::unresolved::AltModelBlockRenderer = todo!("fabric2seer: the initialiser of `altModelBlockRenderer` (TODO above)");
        for submit in submits.iter() {
            for state in submit.states() {
                let mut position_matrix: Matrix4f = Matrix4f::new().translate(((((state.pos().get_x() as f64) - submit.camera().pos.x())) as f32), ((((state.pos().get_y() as f64) - submit.camera().pos.y())) as f32), ((((state.pos().get_z() as f64) - submit.camera().pos.z())) as f32));
                let mut pose: PoseStack = matrix_helper::to_stack(position_matrix.clone());
                // TODO(fabric2seer): `single_texture_with_lightmap` takes 3 argument(s) in the port (`texture: Rc<dyn GpuTextureView>, sampler: Rc<dyn GpuSampler>, lightmap: Option<Rc<dyn GpuTextureView>>`), Java passes 2
                // rust: let mut builder: Box<dyn VertexConsumer> = self.get_vertex_builder_render_pipeline_texture_setup(&mcrust::net::minecraft::client::renderer::render_pipelines::translucent_block(), &TextureSetup::single_texture_with_lightmap(blocks_atlas_texture.clone(), sampler.clone()));
                let mut builder: Box<dyn VertexConsumer> = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
                // TODO(fabric2seer): Fabric API `Renderer.get` has no SeerClient equivalent
                //   java: QuadEmitter quadEmitter = Renderer.get().quadEmitter(quad -> {
                //   java: 					int colour = ARGB.color(state.alpha(), CommonColors.WHITE);
                //   java:
                //   java: 					quad.color(colour, colour, colour, colour);
                //   java: 					quad.buffer(OverlayTexture.NO_OVERLAY, pose.last(), builder);
                //   java: 				});
                let mut quad_emitter: crate::unresolved::QuadEmitter = todo!("fabric2seer: the initialiser of `quadEmitter` (TODO above)");
                let mut model: Arc<dyn BlockStateModel> = context.block_state_model_set().get(&state.state());
                // TODO(fabric2seer): `get_seed` takes 2 argument(s) in the port (`state: &BlockState, pos: &BlockPos`), Java passes 1
                // rust: let mut block_seed: i64 = state.state().base.get_seed(&state.pos());
                let mut block_seed: i64 = todo!("fabric2seer: the initialiser of `blockSeed` (TODO above)");
                // TODO(fabric2seer): `.tesselateBlock(..)` on a AltModelBlockRenderer
                //   java: altModelBlockRenderer.tesselateBlock(quadEmitter, 0, 0, 0, Minecraft.getInstance().level, state.pos(), state.state(), model, blockSeed);
            }
        }
    }

}

/// java: public record Submit(List<BlockHologramRenderState> states, CameraRenderState camera) implements SubmitNode
#[derive(Clone, Debug)]
pub struct Submit {
    /// java: List<BlockHologramRenderState> states
    pub states: Vec<BlockHologramRenderState>,
    /// java: CameraRenderState camera
    pub camera: CameraRenderState,
}

impl Submit {
    /// java: the canonical record constructor
    pub fn new(mut states: Vec<BlockHologramRenderState>, mut camera: CameraRenderState) -> Self {
        Self { states, camera }
    }

    /// java: the record accessor `states()`
    pub fn states(&self) -> Vec<BlockHologramRenderState> {
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
