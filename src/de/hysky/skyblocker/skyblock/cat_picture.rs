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
        // TODO(fabric2seer): field `CLASSIC` of a value of unknown type
        //   java: Minecraft.getInstance().getSoundManager().play(SimpleSoundInstance.forUI(SoundEvents.CAT_SOUNDS.get(CatSoundVariants.SoundSet.CLASSIC).adultSounds().ambientSound(), 1));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if compat::get_state(&MOVE_AROUND) || SkyblockerConfigManager::get().misc.cat && Utils::get_location() == Location::Hub {
            // TODO(fabric2seer): no overload of `extractRenderState` takes 1 argument(s)
            //   java: collector.submitVanilla(EmptyRenderState.INSTANCE, CatPicture::extractRenderState);
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
        // TODO(fabric2seer): `.nextDouble(..)` on a Random
        //   java: Vec3 randomVector = new Vec3(RANDOM.nextDouble(-1, 1), RANDOM.nextDouble(-0.2, 0.2), RANDOM.nextDouble(-1, 1));
        let mut random_vector: Vec3 = todo!("fabric2seer: the initialiser of `randomVector` (TODO above)");
        if look_angle.dot(&random_vector) > -0.1 {
            random_vector = random_vector.scale((-1) as f64);
        }
        // TODO(fabric2seer): field `COLLIDER` of a value of unknown type
        //   java: BlockHitResult result = level.clip(new ClipContext(eyePosition, eyePosition.add(randomVector.scale(15)), ClipContext.Block.COLLIDER, ClipContext.Fluid.NONE, player));
        let mut result: BlockHitResult = todo!("fabric2seer: the initialiser of `result` (TODO above)");
        // TODO(fabric2seer): static `HitResult.Type` has no Rust counterpart in the port's HitResult file
        // TODO(fabric2seer): the condition: field `MISS` of a value of unknown type
        //   java: result.getType() == HitResult.Type.MISS || result.getDirection().getAxis() == Direction.Axis.Y
        if todo!("fabric2seer: the condition (TODO above)") {
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
        // TODO(fabric2seer): `.updateForItemFrame(..)` on a BlockModelResolver
        //   java: ((EntityRenderDispatcherAccessor) Minecraft.getInstance().getEntityRenderDispatcher()).getBlockModelResolver().updateForItemFrame(itemFrameState.frameModel, false, true);
        let mut matrices: PoseStack = PoseStack::new();
        matrices.push_pose();
        matrices.translate_d((((-level_state.camera_render_state.pos.x) + RENDER_POSITION.get_mut().as_ref().unwrap().x) + 0.5), (((-level_state.camera_render_state.pos.y) + RENDER_POSITION.get_mut().as_ref().unwrap().y) + 0.5), (((-level_state.camera_render_state.pos.z) + RENDER_POSITION.get_mut().as_ref().unwrap().z) + 0.5));
        matrices.mul_pose_quat(&mcrust::com::mojang::math::axis::YP.rotation_degrees(compat::get_state(&ROTATION)));
        matrices.translate_d(-0.5, -0.5, -0.5);
        item_frame_state.frame_model.borrow().submit_with_z_offset(&mut matrices, submit_node_collector, mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT, mcrust::net::minecraft::client::renderer::texture::overlay_texture::NO_OVERLAY, mcrust::net::minecraft::client::renderer::entity::state::entity_render_state::NO_OUTLINE);
        matrices.translate_d((1) as f64, (1) as f64, (0) as f64);
        matrices.mul_pose_quat(&mcrust::com::mojang::math::axis::ZP.rotation_degrees(180.0f32));
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.client.renderer.SubmitNodeCollector.CustomGeometryRenderer`)
        // rust: submit_node_collector.submit_custom_geometry(&matrices, render_types::text(TEXTURE.get_mut()), |matrices_entry, buffer| {
        // rust: let mut z: f32 = ((1.0f32 - ((1 as f32) / 16.0f32)) - ((1 as f32) / 2048.0f32));
        // rust: { buffer.add_vertex_pose(&matrices_entry, 0.0f32, (1) as f32, z); buffer.set_color(mcrust::net::minecraft::util::common_colors::WHITE); buffer.set_uv(0.0f32, 1.0f32); buffer.set_light(mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT); &mut buffer };
        // rust: { buffer.add_vertex_pose(&matrices_entry, (1) as f32, (1) as f32, z); buffer.set_color(mcrust::net::minecraft::util::common_colors::WHITE); buffer.set_uv(1.0f32, 1.0f32); buffer.set_light(mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT); &mut buffer };
        // rust: { buffer.add_vertex_pose(&matrices_entry, (1) as f32, 0.0f32, z); buffer.set_color(mcrust::net::minecraft::util::common_colors::WHITE); buffer.set_uv(1.0f32, 0.0f32); buffer.set_light(mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT); &mut buffer };
        // rust: { buffer.add_vertex_pose(&matrices_entry, 0.0f32, 0.0f32, z); buffer.set_color(mcrust::net::minecraft::util::common_colors::WHITE); buffer.set_uv(0.0f32, 0.0f32); buffer.set_light(mcrust::net::minecraft::util::light_coords_util::FULL_BRIGHT); &mut buffer };
        // rust: });
        matrices.pop_pose();
    }

}
