//! Ported from de.hysky.skyblocker.utils.render.RenderHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector_impl::PrimitiveCollectorImpl;
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
        // TODO(fabric2seer): LevelExtractionEvents.END_EXTRACTION.register: no SeerClient equivalent yet
        //   java: LevelExtractionEvents.END_EXTRACTION.register(RenderHelper::startExtraction);
        world_render_events::BEFORE_ENTITIES.register(Arc::new(|ctx: &mut world_render_events::WorldRenderContext<'_>| {
            let mut profiler = profiler::get();
            profiler.push("skyblockerDispatchSubmits");
            // TODO(fabric2seer): WorldRenderContext.levelState() - the port's context only carries the GameRenderer (`gameRenderer`)
            //   java: collector.dispatchSubmits(context.levelState(), context.submitNodeCollector());
            profiler.pop();
        }));
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(FilledBoxInstancedFeatureRenderer.TYPE, FilledBoxInstancedFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(FilledBoxFeatureRenderer.TYPE, FilledBoxFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(OutlinedBoxInstancedFeatureRenderer.TYPE, OutlinedBoxInstancedFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(OutlinedBoxFeatureRenderer.TYPE, OutlinedBoxFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(LinesFeatureRenderer.TYPE, LinesFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(CursorLineFeatureRenderer.TYPE, CursorLineFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(QuadFeatureRenderer.TYPE, QuadFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(TexturedQuadFeatureRenderer.TYPE, TexturedQuadFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(BlockHologramFeatureRenderer.TYPE, BlockHologramFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(TextFeatureRenderer.TYPE, TextFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(CylinderFeatureRenderer.TYPE, CylinderFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(SphereFeatureRenderer.TYPE, SphereFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(FilledCircleFeatureRenderer.TYPE, FilledCircleFeatureRenderer::new);
        // TODO(fabric2seer): Fabric API `FeatureRendererRegistry.register` has no SeerClient equivalent
        //   java: FeatureRendererRegistry.register(OutlinedCircleFeatureRenderer.TYPE, OutlinedCircleFeatureRenderer::new);
    }

    /// java: private static void startExtraction(LevelExtractionContext context)
    pub fn start_extraction(context: &crate::unresolved::LevelExtractionContext) {
        let mut profiler = profiler::get();
        profiler.push("skyblockerPrimitiveCollection");
        // TODO(fabric2seer): `.levelState(..)` on a LevelExtractionContext
        //   java: collector = new PrimitiveCollectorImpl(context.levelState(), context.levelState().cameraRenderState.cullFrustum);
        (level_render_extraction_callback::event_invoker())((COLLECTOR.get_mut().clone().expect("RenderHelper.collector") as compat::Shared<dyn PrimitiveCollector>));
        PrimitiveCollectorImpl::end_collection(COLLECTOR.get_mut().as_ref().unwrap());
        profiler.pop();
    }

    /// java: private static void dispatchSubmits(LevelRenderContext context)
    pub fn dispatch_submits(context: &crate::unresolved::LevelRenderContext) {
        let mut profiler = profiler::get();
        profiler.push("skyblockerDispatchSubmits");
        // TODO(fabric2seer): `.levelState(..)` on a LevelRenderContext
        //   java: collector.dispatchSubmits(context.levelState(), context.submitNodeCollector());
        profiler.pop();
    }

    /// java: public static void runOnRenderThread(Runnable runnable)
    pub fn run_on_render_thread(runnable: Arc<dyn Fn() + Send + Sync>) {
        if render_system::is_on_render_thread() {
            (runnable)();
        } else {
            // java: Minecraft.getInstance().execute(runnable); - the runnable crosses to the render thread, so it is `Send`
            mcrust::mcrust::modding::game::execute(move || (runnable)());
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
