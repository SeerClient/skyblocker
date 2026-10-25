//! Ported from de.hysky.skyblocker.skyblock.CatPicture by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::fun_utils::FunUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::state::empty_render_state;
use crate::de::hysky::skyblocker::utils::render::state::empty_render_state::EmptyRenderState;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::blaze3d::vertex::pose_stack::PoseStack;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::client::renderer::entity::state::item_frame_render_state::ItemFrameRenderState;
use mcrust::net::minecraft::client::renderer::rendertype::render_types;
use mcrust::net::minecraft::client::renderer::state::level::level_render_state::LevelRenderState;
use mcrust::net::minecraft::client::renderer::submit_node_collector::SubmitNodeCollector;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::level::clip_context::ClipContext;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer as _;
use mcrust::com::mojang::math::axis::Axis as _;
use mcrust::net::minecraft::client::renderer::ordered_submit_node_collector::OrderedSubmitNodeCollector as _;

/// java: private static @Nullable Vec3 renderPosition = new Vec3(-3, 79, 3);
pub static RENDER_POSITION: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| Some(Vec3::new((-3) as f64, (79) as f64, (3) as f64)));
/// java: private static final Random RANDOM = new Random();
pub static RANDOM: compat::MainThread<compat::Random> = compat::MainThread::new(|| compat::Random::new());
/// java: private static final Identifier TEXTURE = SkyblockerMod.id("textures/cat.png");
pub static TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("textures/cat.png"));
/// java: private static float rotation = 180;
pub static ROTATION: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(((180) as f32) as f32));
/// java: private static boolean recipeBookEasterEgg = false;
pub static RECIPE_BOOK_EASTER_EGG: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean moveAround = FunUtils.shouldEnableFun();
pub static MOVE_AROUND: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(FunUtils::should_enable_fun()));

/// java: public class CatPicture
#[derive(Clone)]
pub struct CatPicture {}

impl CatPicture {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { CatPicture::extract_rendering(&__a0); }));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { CatPicture::update(); }), 61) });
        skyblock_events::LOCATION_CHANGE.register(Arc::new(move |_| { CatPicture::reset_position(); }));
        skyblock_events::LEAVE.register(Arc::new(move || { CatPicture::reset_position(); }));
    }

    /// java: public static void recipeBook()
    pub fn recipe_book() {
        if compat::get_state(&RECIPE_BOOK_EASTER_EGG) {
            return;
        }
        compat::set_state(&RECIPE_BOOK_EASTER_EGG, true);
        compat::set_state(&MOVE_AROUND, true);
        *RENDER_POSITION.get_mut() = None;
        // java: Minecraft.getInstance().getSoundManager().play(SimpleSoundInstance.forUI(SoundEvents.CAT_SOUNDS.get(CatSoundVariants.SoundSet.CLASSIC).adultSounds().ambientSound(), 1));
        // (`registerCatSoundVariants` registers CLASSIC's adult ambient as `entity.<cat>.ambient`; `forUI(sound, 1)` is pitch 1, volume 0.25)
        let classic = mcrust::net::minecraft::world::entity::animal::feline::cat_sound_variants::SoundSet::Classic.get_sound_event_identifier();
        let ambient = mcrust::net::minecraft::sounds::sound_event::SoundEvent::create_variable_range_event(mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace(&format!("entity.{classic}.ambient")));
        mcrust::mcrust::modding::game::play_ui_sound(&ambient, 1.0);
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if compat::get_state(&MOVE_AROUND) || SkyblockerConfigManager::get().misc.cat && Utils::get_location() == Location::Hub {
            // java: collector.submitVanilla(EmptyRenderState.INSTANCE, CatPicture::extractRenderState);
            let renderer: compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::primitive::vanilla_renderer::VanillaRenderer<EmptyRenderState>> = compat::shared(CatPictureRenderer) as compat::Shared<dyn crate::de::hysky::skyblocker::utils::render::primitive::vanilla_renderer::VanillaRenderer<EmptyRenderState>>;
            compat::write_as::<crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector_impl::PrimitiveCollectorImpl, dyn PrimitiveCollector, _>(collector, |collector| collector.submit_vanilla(&*empty_render_state::INSTANCE, &renderer));
        }
    }

    /// java: private static void resetPosition()
    pub fn reset_position() {
        if compat::get_state(&MOVE_AROUND) {
            *RENDER_POSITION.get_mut() = None;
        }
    }

    /// java: private static void update()
    pub fn update() {
        if !(Utils::is_on_skyblock()) {
            return;
        }
        if !(compat::get_state(&MOVE_AROUND)) {
            return;
        }
        let mut level: Option<Arc<ClientLevel>> = mcrust::mcrust::modding::game::level();
        if level.is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut look_angle: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_look_angle()).unwrap();
        let mut eye_position: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_eye_position()).unwrap();
        if RENDER_POSITION.get_mut().is_some() && RENDER_POSITION.get_mut().as_ref().unwrap().subtract_vec(&eye_position).normalize().dot(&look_angle) > 0.0 {
            return;
        }
        //   java: Vec3 randomVector = new Vec3(RANDOM.nextDouble(-1, 1), RANDOM.nextDouble(-0.2, 0.2), RANDOM.nextDouble(-1, 1));
        let random_x = next_double_between(RANDOM.get_mut(), -1.0, 1.0);
        let random_y = next_double_between(RANDOM.get_mut(), -0.2, 0.2);
        let random_z = next_double_between(RANDOM.get_mut(), -1.0, 1.0);
        let mut random_vector: Vec3 = Vec3::new(random_x, random_y, random_z);
        if look_angle.dot(&random_vector) > -0.1 {
            random_vector = random_vector.scale((-1) as f64);
        }
        //   java: BlockHitResult result = level.clip(new ClipContext(eyePosition, eyePosition.add(randomVector.scale(15)), ClipContext.Block.COLLIDER, ClipContext.Fluid.NONE, player));
        // (`new ClipContext(.., entity)` is `this(.., CollisionContext.of(entity))`)
        let level_ref: &ClientLevel = level.as_deref().unwrap();
        let to: Vec3 = eye_position.add_vec(&random_vector.scale((15) as f64));
        let Some(mut result) = mcrust::mcrust::modding::game::with_player(|player| {
            let collision_context = mcrust::net::minecraft::world::phys::shapes::collision_context::of(&*player, level_ref);
            mcrust::net::minecraft::world::level::block_getter::BlockGetter::clip(level_ref, &ClipContext::new(eye_position.clone(), to.clone(), mcrust::net::minecraft::world::level::clip_context::Block::Collider, mcrust::net::minecraft::world::level::clip_context::Fluid::None, &*collision_context))
        }) else {
            return;
        };
        //   java: if (result.getType() == HitResult.Type.MISS || result.getDirection().getAxis() == Direction.Axis.Y) return;
        if result.get_type() == mcrust::net::minecraft::world::phys::hit_result::Type::Miss || result.get_direction().get_axis() == mcrust::net::minecraft::core::direction::Axis::Y {
            return;
        }
        if !(level.as_ref().unwrap().get_block_state_typed(&result.get_block_pos()).base.is_solid_render()) {
            return;
        }
        let mut relative: BlockPos = result.get_block_pos().relative(result.get_direction());
        if !(level.as_ref().unwrap().get_block_state_typed(&relative).is_air()) {
            return;
        }
        *RENDER_POSITION.get_mut() = Some(Vec3::from_vec3i(relative.as_vec3i()));
        compat::set_state(&ROTATION, (180.0f32 - result.get_direction().to_y_rot()));
    }

    /// java: private static void extractRenderState(EmptyRenderState state, LevelRenderState levelState, SubmitNodeCollector submitNodeCollector)
    pub fn extract_render_state(state: &EmptyRenderState, level_state: &LevelRenderState, submit_node_collector: &mut dyn SubmitNodeCollector) {
        if RENDER_POSITION.get_mut().is_none() {
            return;
        }
        let mut item_frame_state: ItemFrameRenderState = ItemFrameRenderState::new();
        // java: ((EntityRenderDispatcherAccessor) Minecraft.getInstance().getEntityRenderDispatcher()).getBlockModelResolver().updateForItemFrame(itemFrameState.frameModel, false, true);
        // (the dispatcher's resolver is `new BlockModelResolver(modelManager)` over the game's one baked `BlockModelSet`)
        if let Some(resolver) = mcrust::net::minecraft::client::renderer::game_renderer::block_model_set().map(mcrust::net::minecraft::client::renderer::block::block_model_resolver::BlockModelResolver::new) {
            resolver.update_for_item_frame(&mut item_frame_state.frame_model.borrow_mut(), false, true);
        }
        let mut matrices: PoseStack = PoseStack::new();
        matrices.push_pose();
        matrices.translate_d((((-level_state.camera_render_state.pos.x) + RENDER_POSITION.get_mut().as_ref().unwrap().x) + 0.5), (((-level_state.camera_render_state.pos.y) + RENDER_POSITION.get_mut().as_ref().unwrap().y) + 0.5), (((-level_state.camera_render_state.pos.z) + RENDER_POSITION.get_mut().as_ref().unwrap().z) + 0.5));
        matrices.rotate(&mcrust::com::mojang::math::axis::YP.rotation_degrees(compat::get_state(&ROTATION)));
        matrices.translate_d(-0.5, -0.5, -0.5);
        item_frame_state.frame_model.borrow().submit_with_z_offset(&mut matrices, submit_node_collector, mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT, mcrust::net::minecraft::client::renderer::texture::overlay_texture::NO_OVERLAY, mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::NO_OUTLINE);
        matrices.translate_d((1) as f64, (1) as f64, (0) as f64);
        matrices.rotate(&mcrust::com::mojang::math::axis::ZP.rotation_degrees(180.0f32));
        //   java: submitNodeCollector.submitCustomGeometry(matrices, RenderTypes.text(TEXTURE), (matricesEntry, buffer) -> { .. });
        submit_node_collector.submit_custom_geometry(&matrices, render_types::text(TEXTURE.get()), std::sync::Arc::new(CatGeometry));
        matrices.pop_pose();
    }

}

/// The `CustomGeometryRenderer` lambda of `extractRenderState` (it captures nothing).
struct CatGeometry;

impl mcrust::net::minecraft::client::renderer::submit_node_collector::CustomGeometryRenderer for CatGeometry {
    fn render(&self, matrices_entry: &mcrust::com::mojang::blaze3d::vertex::pose_stack::Pose, buffer: &mut dyn mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer) {
        //   java: float z = 1 - 1 / 16f - 1 / 2048f;
        let mut z: f32 = ((1.0f32 - ((1 as f32) / 16.0f32)) - ((1 as f32) / 2048.0f32));
        //   java: buffer.addVertex(matricesEntry, 0, 1, z).setColor(CommonColors.WHITE).setUv(0, 1).setLight(LightCoordsUtil.FULL_BRIGHT); (x4)
        for (x, y, u, v) in [(0.0f32, 1.0f32, 0.0f32, 1.0f32), (1.0, 1.0, 1.0, 1.0), (1.0, 0.0, 1.0, 0.0), (0.0, 0.0, 0.0, 0.0)] {
            buffer.add_vertex_pose(matrices_entry, x, y, z);
            buffer.set_color(mcrust::net::minecraft::util::common_colors::WHITE);
            buffer.set_uv(u, v);
            buffer.set_light(mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT);
        }
    }
}

/// `RandomGenerator.nextDouble(origin, bound)` (`java.util.Random`): `origin + (bound - origin) * nextDouble()`,
/// pulled below `bound` when rounding reaches it (`RandomSupport.boundedNextDouble`).
fn next_double_between(random: &mut compat::Random, origin: f64, bound: f64) -> f64 {
    let mut r = random.next_double();
    r = r * (bound - origin) + origin;
    if r >= bound {
        // `Math.nextDown(bound)`
        r = if bound > 0.0 { f64::from_bits(bound.to_bits() - 1) } else if bound < 0.0 { f64::from_bits(bound.to_bits() + 1) } else { -f64::from_bits(1) };
    }
    r
}

/// `CatPicture::extractRenderState` as the `VanillaRenderer` `submitVanilla` is handed (a method reference in Java).
struct CatPictureRenderer;

impl crate::de::hysky::skyblocker::utils::render::primitive::vanilla_renderer::VanillaRenderer<EmptyRenderState> for CatPictureRenderer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn submit_vanilla(&self, state: &EmptyRenderState, world_state: &LevelRenderState, submit_node_collector: &mut dyn SubmitNodeCollector) {
        CatPicture::extract_render_state(state, world_state, submit_node_collector);
    }
}
