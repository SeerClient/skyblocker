//! Ported from de.hysky.skyblocker.utils.render.RenderHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
            let mut profiler: Arc<dyn ProfilerFiller> = profiler::get();
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
        let mut profiler: Arc<dyn ProfilerFiller> = profiler::get();
        profiler.push("skyblockerPrimitiveCollection");
        // TODO(fabric2seer): `.levelState(..)` on a LevelExtractionContext
        //   java: collector = new PrimitiveCollectorImpl(context.levelState(), context.levelState().cameraRenderState.cullFrustum);
        (level_render_extraction_callback::event_invoker())((COLLECTOR.get_mut().clone() as compat::Shared<dyn PrimitiveCollector>));
        PrimitiveCollectorImpl::end_collection(COLLECTOR.get_mut().as_ref().unwrap());
        profiler.pop();
    }

    /// java: private static void dispatchSubmits(LevelRenderContext context)
    pub fn dispatch_submits(context: &crate::unresolved::LevelRenderContext) {
        let mut profiler: Arc<dyn ProfilerFiller> = profiler::get();
        profiler.push("skyblockerDispatchSubmits");
        // TODO(fabric2seer): `.levelState(..)` on a LevelRenderContext
        //   java: collector.dispatchSubmits(context.levelState(), context.submitNodeCollector());
        profiler.pop();
    }

    /// java: public static void runOnRenderThread(Runnable runnable)
    pub fn run_on_render_thread(runnable: Arc<dyn Fn()>) {
        if render_system::is_on_render_thread() {
            (runnable)();
        } else {
            mcrust::mcrust::modding::game::execute(move || (runnable.clone())());
        }
    }

    /// java: public static void assertOnRenderThread(String message)
    pub fn assert_on_render_thread(message: &str) {
        if !(render_system::is_on_render_thread()) {
            panic!("IllegalStateException: {}", message.to_string());
        }
    }

    /// java: public static DeltaTracker getTickCounter()
    pub fn get_tick_counter() -> crate::unresolved::DeltaTracker {
        return compat::minecraft().get_delta_tracker().clone();
    }

    /// java: public static Camera getCamera()
    pub fn get_camera() -> Camera {
        // TODO(fabric2seer): the mod keeps the game's `Camera` (Java holds a reference to the live object); the port's `Camera` is not a value to copy and the port lends the mod no handle to it
        // rust: return compat::minecraft().game_renderer.main_camera().clone();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public static @Nullable AABB getBlockBoundingBox(ClientLevel world, BlockPos pos)
    pub fn get_block_bounding_box(world: &ClientLevel, pos: &BlockPos) -> Option<AABB> {
        return RenderHelper::get_block_bounding_box_3(world, &world.get_block_state_typed(pos), pos);
    }

    /// java: public static @Nullable AABB getBlockBoundingBox(ClientLevel world, BlockState state, BlockPos pos)
    pub fn get_block_bounding_box_3(world: &ClientLevel, state: &Arc<BlockState>, pos: &BlockPos) -> Option<AABB> {
        let mut shape: VoxelShape = (state.get_shape_no_context(world.as_block_getter(), pos)).single_encompassing();
        return (if shape.is_empty() { None } else { Some(shape.bounds().unwrap().move_block_pos(pos)) });
    }

}
