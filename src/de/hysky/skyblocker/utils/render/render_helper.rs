//! Ported from de.hysky.skyblocker.utils.render.RenderHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector_impl::PrimitiveCollectorImpl;
use crate::de::hysky::skyblocker::utils::render::primitive;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_feature_renderer::PrimitiveFeatureRendererAdapter;
use mcrust::net::minecraft::client::renderer::culling::frustum::Frustum;
use mcrust::net::minecraft::client::renderer::state::level::level_render_state::LevelRenderState;
use mcrust::net::minecraft::client::renderer::submit_node_storage::SubmitNodeStorage;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::net::minecraft::client::camera::Camera;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::util::profiling::profiler;
use mcrust::net::minecraft::util::profiling::profiler_filler::ProfilerFiller;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::shapes::voxel_shape::VoxelShape;

/// java: private static PrimitiveCollectorImpl collector;
pub static COLLECTOR: compat::MainThread<Option<compat::Shared<PrimitiveCollectorImpl>>> = compat::MainThread::new(|| None);

/// java: public class RenderHelper
#[derive(Clone)]
pub struct RenderHelper {}

impl RenderHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: LevelExtractionEvents.END_EXTRACTION.register(RenderHelper::startExtraction);
        // java: LevelRenderEvents.COLLECT_SUBMITS.register(RenderHelper::dispatchSubmits);
        // Both read the frame's `LevelRenderState`; the port lends it (and the submit collector) with the `GameRenderer`
        // at `WorldRenderEvents.BEFORE_ENTITIES` - the HEAD of `LevelRenderer.submitFeatures`, which is after the whole
        // level extraction (Fabric's END_EXTRACTION) and before any feature of the frame is submitted (COLLECT_SUBMITS
        // runs inside it). The two run there in Java's order: the collection, then its submits.
        world_render_events::BEFORE_ENTITIES.register(Arc::new(|ctx: &mut world_render_events::WorldRenderContext<'_>| {
            // java: context.levelState().cameraRenderState.cullFrustum
            let Some(frustum) = ctx.game_renderer.main_camera().get_cull_frustum().copied() else {
                return;
            };
            let (level_state, submit_node_collector) = compat::leftovers_level_render_context(ctx.game_renderer);
            RenderHelper::start_extraction(level_state, frustum);
            RenderHelper::dispatch_submits(level_state, submit_node_collector);
        }));
        // java: FeatureRendererRegistry.register(FilledBoxInstancedFeatureRenderer.TYPE, FilledBoxInstancedFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::filled_box_instanced_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::filled_box_instanced_feature_renderer::Submit>::new(primitive::filled_box_instanced_feature_renderer::FilledBoxInstancedFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(FilledBoxFeatureRenderer.TYPE, FilledBoxFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::filled_box_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::filled_box_feature_renderer::Submit>::new(primitive::filled_box_feature_renderer::FilledBoxFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(OutlinedBoxInstancedFeatureRenderer.TYPE, OutlinedBoxInstancedFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::outlined_box_instanced_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::outlined_box_instanced_feature_renderer::Submit>::new(primitive::outlined_box_instanced_feature_renderer::OutlinedBoxInstancedFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(OutlinedBoxFeatureRenderer.TYPE, OutlinedBoxFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::outlined_box_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::outlined_box_feature_renderer::Submit>::new(primitive::outlined_box_feature_renderer::OutlinedBoxFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(LinesFeatureRenderer.TYPE, LinesFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::lines_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::lines_feature_renderer::Submit>::new(primitive::lines_feature_renderer::LinesFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(CursorLineFeatureRenderer.TYPE, CursorLineFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::cursor_line_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::cursor_line_feature_renderer::Submit>::new(primitive::cursor_line_feature_renderer::CursorLineFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(QuadFeatureRenderer.TYPE, QuadFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::quad_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::quad_feature_renderer::Submit>::new(primitive::quad_feature_renderer::QuadFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(TexturedQuadFeatureRenderer.TYPE, TexturedQuadFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::textured_quad_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::textured_quad_feature_renderer::Submit>::new(primitive::textured_quad_feature_renderer::TexturedQuadFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(BlockHologramFeatureRenderer.TYPE, BlockHologramFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::block_hologram_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::block_hologram_feature_renderer::Submit>::new(primitive::block_hologram_feature_renderer::BlockHologramFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(TextFeatureRenderer.TYPE, TextFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::text_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::text_feature_renderer::Submit>::new(primitive::text_feature_renderer::TextFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(CylinderFeatureRenderer.TYPE, CylinderFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::cylinder_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::cylinder_feature_renderer::Submit>::new(primitive::cylinder_feature_renderer::CylinderFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(SphereFeatureRenderer.TYPE, SphereFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::sphere_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::sphere_feature_renderer::Submit>::new(primitive::sphere_feature_renderer::SphereFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(FilledCircleFeatureRenderer.TYPE, FilledCircleFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::filled_circle_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::filled_circle_feature_renderer::Submit>::new(primitive::filled_circle_feature_renderer::FilledCircleFeatureRenderer::new())));
        // java: FeatureRendererRegistry.register(OutlinedCircleFeatureRenderer.TYPE, OutlinedCircleFeatureRenderer::new);
        compat::leftovers_register_feature_renderer(*primitive::outlined_circle_feature_renderer::TYPE.get(), || Box::new(PrimitiveFeatureRendererAdapter::<_, primitive::outlined_circle_feature_renderer::Submit>::new(primitive::outlined_circle_feature_renderer::OutlinedCircleFeatureRenderer::new())));
    }

    /// java: private static void startExtraction(LevelExtractionContext context)
    /// (`context.levelState()` and its `cameraRenderState.cullFrustum`)
    pub fn start_extraction(level_state: &mut LevelRenderState, frustum: Frustum) {
        let mut profiler = profiler::get();
        profiler.push("skyblockerPrimitiveCollection");
        // java: collector = new PrimitiveCollectorImpl(context.levelState(), context.levelState().cameraRenderState.cullFrustum);
        let collector = PrimitiveCollectorImpl::new(frustum);
        *COLLECTOR.get_mut() = Some(collector.clone());
        level_render_extraction_callback::invoke(collector.clone() as compat::Shared<dyn PrimitiveCollector>);
        // java: collector.endCollection(); (and the states it added to `worldState.blockEntityRenderStates`)
        PrimitiveCollectorImpl::end_collection_into(&collector, level_state);
        profiler.pop();
    }

    /// java: private static void dispatchSubmits(LevelRenderContext context)
    /// (`context.levelState()`, `context.submitNodeCollector()`)
    pub fn dispatch_submits(level_state: &LevelRenderState, submit_node_collector: &mut SubmitNodeStorage) {
        let mut profiler = profiler::get();
        profiler.push("skyblockerDispatchSubmits");
        let collector = COLLECTOR.get_mut().clone();
        if let Some(collector) = collector {
            PrimitiveCollectorImpl::dispatch_submits(&collector, level_state, submit_node_collector);
        }
        profiler.pop();
    }

    /// java: public static void runOnRenderThread(Runnable runnable)
    pub fn run_on_render_thread(runnable: Arc<dyn Fn() + Send + Sync>) {
        if render_system::is_on_render_thread() {
            (runnable)();
        } else {
            // java: Minecraft.getInstance().execute(runnable); - the runnable crosses to the render thread, so it is `Send`
            crate::compat::execute(move || (runnable)());
        }
    }

    /// java: public static void assertOnRenderThread(String message)
    pub fn assert_on_render_thread(message: &str) {
        if !(render_system::is_on_render_thread()) {
            panic!("IllegalStateException: {}", message.to_string());
        }
    }

    /// java: public static DeltaTracker getTickCounter()
    pub fn get_tick_counter() -> &'static mcrust::net::minecraft::client::delta_tracker::Timer {
        // java: return CLIENT.getDeltaTracker(); - the game's object
        return compat::minecraft().get_delta_tracker();
    }

    /// java: public static Camera getCamera()
    pub fn get_camera() -> &'static Camera {
        // java: return CLIENT.gameRenderer.mainCamera(); - the live object, by reference
        // (`Minecraft.gameRenderer` is a field of the port's `Minecraft`).
        return compat::minecraft().game_renderer.main_camera();
    }

    /// java: public static @Nullable AABB getBlockBoundingBox(ClientLevel world, BlockPos pos)
    pub fn get_block_bounding_box(world: &ClientLevel, pos: &BlockPos) -> Option<AABB> {
        return RenderHelper::get_block_bounding_box_3(world, &world.get_block_state_typed(pos), pos);
    }

    /// java: public static @Nullable AABB getBlockBoundingBox(ClientLevel world, BlockState state, BlockPos pos)
    pub fn get_block_bounding_box_3(world: &ClientLevel, state: &Arc<BlockState>, pos: &BlockPos) -> Option<AABB> {
        let mut shape = (state.get_shape_no_context(world.as_block_getter(), pos)).single_encompassing();
        return (if shape.is_empty() { None } else { Some(shape.bounds().unwrap().move_block_pos(pos)) });
    }

}
