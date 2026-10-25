//! Ported from de.hysky.skyblocker.utils.render.primitive.BlockHologramFeatureRenderer by fabric2seer.

use std::sync::Arc;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer;
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
use mcrust::mcrust::modding::game;
use mcrust::net::minecraft::client::resources::model::geometry::baked_quad::BakedQuad;
use mcrust::com::mojang::blaze3d::vertex::quad_instance::QuadInstance;
use mcrust::net::minecraft::client::renderer::block::block_and_tint_getter::BlockAndTintGetter;
use mcrust::net::minecraft::client::renderer::block::block_quad_output::BlockQuadOutput;
use mcrust::net::minecraft::client::renderer::block::model_block_renderer::ModelBlockRenderer;
use mcrust::net::minecraft::client::renderer::texture::overlay_texture;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::common_colors;

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
    fn build_group(&mut self, context: &mut FeatureFrameContext<'_>, submits: &[std::sync::Arc<dyn SubmitNode>]) {
        // java: GpuTextureView blocksAtlasTexture = context.textureManager().getTexture(TextureAtlas.LOCATION_BLOCKS).getTextureView();
        let blocks_atlas_texture = context.main.texture_manager.get_texture(context.shared.resource_manager, &mcrust::net::minecraft::client::renderer::texture::texture_atlas::location_blocks()).get_texture_view().clone();
        // java: GpuSampler sampler = RenderSystem.getSamplerCache().getSampler(CLAMP_TO_EDGE, CLAMP_TO_EDGE, LINEAR, NEAREST, true);
        let sampler = render_system::with_sampler_cache(|cache| cache.get_sampler(AddressMode::ClampToEdge, AddressMode::ClampToEdge, FilterMode::Linear, FilterMode::Nearest, true));
        // java: AltModelBlockRenderer altModelBlockRenderer = Renderer.get().altModelBlockRenderer(context.options().ambientOcclusion, false, context.blockColors());
        // Fabric's default renderer answers a vanilla model with the vanilla `ModelBlockRenderer` (same arguments), each
        // emitted quad handed to the `QuadEmitter` below.
        let mut alt_model_block_renderer = ModelBlockRenderer::new(context.shared.options.ambient_occlusion, false, context.shared.block_colors.clone());
        // java: Minecraft.getInstance().level
        let Some(level) = game::level() else {
            return;
        };
        let level = ClientLevelBlockAndTintGetter(&level);
        for submit in primitive_feature_renderer::submits::<Submit>(submits) {
            for state in submit.states.iter() {
                let position_matrix: Matrix4f = Matrix4f::new().translate((((state.pos().get_x() as f64) - submit.camera.pos.x()) as f32), (((state.pos().get_y() as f64) - submit.camera.pos.y()) as f32), (((state.pos().get_z() as f64) - submit.camera.pos.z()) as f32));
                let mut pose: PoseStack = matrix_helper::to_stack(position_matrix.clone());
                let block_state = state.state();
                // java: BlockStateModel model = context.blockStateModelSet().get(state.state());
                let model: Arc<dyn BlockStateModel> = context.block_state_model_set().get(&block_state);
                // java: long blockSeed = state.state().getSeed(state.pos());
                let block_seed: i64 = block_state.base.get_seed(&block_state, &state.pos());
                // java: VertexConsumer builder = this.getVertexBuilder(RenderPipelines.TRANSLUCENT_BLOCK, TextureSetup.singleTextureWithLightmap(blocksAtlasTexture, sampler));
                let texture_setup = TextureSetup::single_texture_with_lightmap(blocks_atlas_texture.clone(), sampler.clone(), Some(context.main.lightmap.clone()));
                let builder = self.get_vertex_builder_render_pipeline_texture_setup(context, &mcrust::net::minecraft::client::renderer::render_pipelines::translucent_block(), &texture_setup);
                // java: QuadEmitter quadEmitter = Renderer.get().quadEmitter(quad -> { int colour = ARGB.color(state.alpha(), CommonColors.WHITE);
                //           quad.color(colour, colour, colour, colour); quad.buffer(OverlayTexture.NO_OVERLAY, pose.last(), builder); });
                let mut quad_emitter = HologramQuadEmitter {
                    colour: argb::color_alpha_f32_rgb(state.alpha(), common_colors::WHITE),
                    pose: &mut pose,
                    builder,
                };
                // java: altModelBlockRenderer.tesselateBlock(quadEmitter, 0, 0, 0, Minecraft.getInstance().level, state.pos(), state.state(), model, blockSeed);
                alt_model_block_renderer.tesselate_block(&mut quad_emitter, 0.0f32, 0.0f32, 0.0f32, &level, &state.pos(), &block_state, &model, block_seed);
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

/// java: `implements SubmitNode` - the record's `featureType()`; `as_any` is the downcast `FeatureRenderer` reads it back through
impl SubmitNode for Submit {
    fn feature_type(&self) -> FeatureRendererType {
        *TYPE.get()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// java: the `QuadEmitter` lambda of `buildGroup` - each quad the block renderer emits, recoloured to the hologram's
/// alpha over white and buffered with no overlay under the block's pose (`quad.buffer(NO_OVERLAY, pose.last(), builder)`).
/// The quad's block-local offset (`x`, `y`, `z`: a model's random offset) moves the pose for that quad, as
/// `MutableQuadView.buffer` writes the quad's emitted positions.
struct HologramQuadEmitter<'p, 'b> {
    colour: i32,
    pose: &'p mut PoseStack,
    builder: &'b mut mcrust::com::mojang::blaze3d::vertex::buffer_builder::BufferBuilder,
}

impl BlockQuadOutput for HologramQuadEmitter<'_, '_> {
    fn put(&mut self, x: f32, y: f32, z: f32, quad: &BakedQuad, instance: &mut QuadInstance) {
        // java: quad.color(colour, colour, colour, colour);
        instance.set_color(self.colour);
        // java: OverlayTexture.NO_OVERLAY
        instance.set_overlay_coords(overlay_texture::NO_OVERLAY);
        self.pose.push_pose();
        self.pose.translate(x, y, z);
        VertexConsumer::put_baked_quad(self.builder, self.pose.last(), quad, instance);
        self.pose.pop_pose();
    }
}

/// `Minecraft.getInstance().level` handed where Java hands it, as a `BlockAndTintGetter` (`ClientLevel implements
/// BlockAndTintGetter`): the level's own `cardinalLighting` / `getBlockTint` / `getBrightness`.
struct ClientLevelBlockAndTintGetter<'l>(&'l mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel);

impl mcrust::net::minecraft::world::level::level_height_accessor::LevelHeightAccessor for ClientLevelBlockAndTintGetter<'_> {
    fn get_height(&self) -> i32 {
        mcrust::net::minecraft::world::level::level_height_accessor::LevelHeightAccessor::get_height(self.0)
    }

    fn get_min_y(&self) -> i32 {
        mcrust::net::minecraft::world::level::level_height_accessor::LevelHeightAccessor::get_min_y(self.0)
    }
}

impl mcrust::net::minecraft::world::level::block_getter::BlockGetter for ClientLevelBlockAndTintGetter<'_> {
    fn as_block_getter(&self) -> &dyn mcrust::net::minecraft::world::level::block_getter::BlockGetter {
        self
    }

    fn get_block_state(&self, pos: &mcrust::net::minecraft::core::block_pos::BlockPos) -> Arc<mcrust::net::minecraft::world::level::block::state::block_state::BlockState> {
        mcrust::net::minecraft::world::level::block_getter::BlockGetter::get_block_state(self.0, pos)
    }

    fn get_fluid_state(&self, pos: &mcrust::net::minecraft::core::block_pos::BlockPos) -> Arc<mcrust::net::minecraft::world::level::material::fluid_state::FluidState> {
        mcrust::net::minecraft::world::level::block_getter::BlockGetter::get_fluid_state(self.0, pos)
    }
}

impl BlockAndTintGetter for ClientLevelBlockAndTintGetter<'_> {
    fn cardinal_lighting(&self) -> mcrust::net::minecraft::world::level::cardinal_lighting::CardinalLighting {
        self.0.cardinal_lighting()
    }

    fn get_block_tint(&self, pos: &mcrust::net::minecraft::core::block_pos::BlockPos, color: mcrust::net::minecraft::client::renderer::block::block_and_tint_getter::ColorResolver) -> i32 {
        self.0.get_block_tint(pos, color)
    }

    fn get_brightness(&self, layer: mcrust::net::minecraft::world::level::light_layer::LightLayer, pos: &mcrust::net::minecraft::core::block_pos::BlockPos) -> i32 {
        self.0.get_brightness(layer, pos)
    }
}
