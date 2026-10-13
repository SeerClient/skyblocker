//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.ControlTestHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::crimson::dojo::dojo_manager;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::monster::skeleton::wither_skeleton::WitherSkeleton;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: private static WitherSkeleton correctWitherSkeleton;
pub static CORRECT_WITHER_SKELETON: compat::MainThread<Option<WitherSkeleton>> = compat::MainThread::new(|| None);
/// java: private static Vec3 lastPos;
pub static LAST_POS: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);
/// java: private static long lastUpdate;
pub static LAST_UPDATE: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static Vec3 pingOffset;
pub static PING_OFFSET: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);
/// java: private static Vec3 lastPingOffset;
pub static LAST_PING_OFFSET: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);

/// java: public class ControlTestHelper
#[derive(Clone)]
pub struct ControlTestHelper {}

impl ControlTestHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void reset()
    pub fn reset() {
        *CORRECT_WITHER_SKELETON.get_mut() = None;
        *LAST_POS.get_mut() = None;
        compat::set_state(&LAST_UPDATE, ((-1) as i64) as i64);
        *PING_OFFSET.get_mut() = None;
        *LAST_PING_OFFSET.get_mut() = None;
    }

    /// java: protected static void onEntitySpawn(Entity entity)
    pub fn on_entity_spawn(entity: &dyn Entity) {
        if let Some(wither_skeleton) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<WitherSkeleton>() && CORRECT_WITHER_SKELETON.get_mut().is_none() {
            // TODO(fabric2seer): the mod keeps the game's `WitherSkeleton` (Java holds a reference to the live object); the port's `WitherSkeleton` is not a value to copy and the port lends the mod no handle to it
            // rust: *CORRECT_WITHER_SKELETON.get_mut() = Some(wither_skeleton.clone());
        }
    }

    /// java: protected static void update()
    pub fn update() {
        if CORRECT_WITHER_SKELETON.get_mut().is_some() {
            if LAST_POS.get_mut().is_some() {
                *LAST_PING_OFFSET.get_mut() = PING_OFFSET.get_mut().clone();
                let mut ping: f64 = ((compat::get_state(&dojo_manager::PING) as f64) / 1000.0);
                let mut movement_vector: Vec3 = CORRECT_WITHER_SKELETON.get_mut().as_ref().unwrap().base.base.base.base.base.position().subtract_vec(LAST_POS.get_mut().as_ref().unwrap()).multiply((1) as f64, 0.1, (1) as f64);
                *PING_OFFSET.get_mut() = Some(movement_vector.scale((((1 as f64) + ((3 as f64) / 20.0)) + ping)));
            }
            *LAST_POS.get_mut() = Some(CORRECT_WITHER_SKELETON.get_mut().as_ref().unwrap().base.base.base.base.base.position());
            compat::set_state(&LAST_UPDATE, compat::millis());
        }
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && CORRECT_WITHER_SKELETON.get_mut().is_some() && PING_OFFSET.get_mut().is_some() && LAST_PING_OFFSET.get_mut().is_some() {
            // TODO(fabric2seer): `DeltaTracker.getGameTimeDeltaPartialTick` has no Rust method in the port (looked for `get_game_time_delta_partial_tick` in DeltaTracker and its supertypes)
            // rust: let mut tick_delta: f32 = RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false);
            let mut tick_delta: f32 = RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false);
            let mut update_percent: f64 = ((((compat::millis()).wrapping_sub(compat::get_state(&LAST_UPDATE))) as f64) / (150 as f64));
            let mut aim_pos: Vec3 = CORRECT_WITHER_SKELETON.get_mut().as_ref().unwrap().base.base.base.base.base.get_eye_position_partial(tick_delta).add_vec(&PING_OFFSET.get_mut().as_ref().unwrap().scale(update_percent)).add_vec(&LAST_PING_OFFSET.get_mut().as_ref().unwrap().scale(((1 as f64) - update_percent)));
            let mut target_box: AABB = AABB::of_corners(&aim_pos.add(-0.5, -0.5, -0.5), &aim_pos.add(0.5, 0.5, 0.5));
            let mut player_looking_at_box: bool = target_box.clip(&mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_eye_position_partial(tick_delta)).unwrap(), &mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_eye_position_partial(tick_delta).add_vec(&player.base.base.base.base.get_view_vector(tick_delta).scale((30) as f64))).unwrap()).is_some();
            // TODO(fabric2seer): `.getColorComponents(..)` on a value of unknown type
            //   java: float[] boxColor = playerLookingAtBox ? Color.GREEN.getColorComponents(new float[]{0, 0, 0}) : Color.LIGHT_GRAY.getColorComponents(new float[]{0, 0, 0});
            let mut box_color: Vec<f32> = if player_looking_at_box { vec![0.0f32, 1.0, 0.0] /* Color.GREEN */ } else { vec![192.0f32 / 255.0, 192.0 / 255.0, 192.0 / 255.0] /* Color.LIGHT_GRAY */ };
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &target_box, &box_color, ((3) as f32) as f32, true) });
        }
    }

}
