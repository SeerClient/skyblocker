//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.TenacityTestHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::level::clip_context::ClipContext;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::core::particles::particle_options::ParticleOptions as _;

/// java: private static final Object2ObjectOpenHashMap<ArmorStand, Vec3> fireBallsWithStartPos = new Object2ObjectOpenHashMap<>();
pub static FIRE_BALLS_WITH_START_POS: compat::MainThread<std::collections::HashMap<ArmorStand, Vec3>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Object2ObjectOpenHashMap<ArmorStand, Vec3> particleOffsets = new Object2ObjectOpenHashMap<>();
pub static PARTICLE_OFFSETS: compat::MainThread<std::collections::HashMap<ArmorStand, Vec3>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class TenacityTestHelper
#[derive(Clone)]
pub struct TenacityTestHelper {}

impl TenacityTestHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void reset()
    pub fn reset() {
        FIRE_BALLS_WITH_START_POS.get_mut().clear();
        PARTICLE_OFFSETS.get_mut().clear();
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        for fireball in FIRE_BALLS_WITH_START_POS.get_mut().keys().cloned() {
            let mut line_start: Vec3 = FIRE_BALLS_WITH_START_POS.get_mut().get(&fireball).cloned().unwrap().add_vec(&PARTICLE_OFFSETS.get_mut().get(&fireball).cloned().unwrap_or_else(|| mcrust::net::minecraft::world::phys::vec3::ZERO));
            let mut fireball_pos: Vec3 = fireball.base.base.position().add_vec(&PARTICLE_OFFSETS.get_mut().get(&fireball).cloned().unwrap_or_else(|| mcrust::net::minecraft::world::phys::vec3::ZERO));
            let mut distance: Vec3 = fireball_pos.subtract_vec(&line_start);
            if distance.length() > 0.02 {
                distance = distance.scale((100) as f64);
                let mut line_end: Vec3 = line_start.add_vec(&distance);
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![line_start.clone(), line_end.clone()], &vec![1.0f32, 0.0f32, 0.0f32], ((1) as f32) as f32, ((3) as f32) as f32, false) });
                let mut hit_result: Option<HitResult> = TenacityTestHelper::raycast(&line_start, &line_end, &fireball);
                // TODO(fabric2seer): field `BLOCK` of a value of unknown type
                //   java: if (hitResult != null && hitResult.getType() == HitResult.Type.BLOCK && hitResult instanceof BlockHitResult blockHitResult) {
                //   java: 					collector.submitFilledBox(blockHitResult.getBlockPos(), new float[]{1f, 0f, 0f}, 0.5f, false);
                //   java: 				}
            }
        }
    }

    /// java: protected static HitResult raycast(Vec3 start, Vec3 end, ArmorStand fireball)
    pub fn raycast(start: &Vec3, end: &Vec3, fireball: &ArmorStand) -> Option<HitResult> {
        if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::level().is_none() {
            return None;
        }
        // TODO(fabric2seer): field `OUTLINE` of a value of unknown type
        //   java: return CLIENT.level.clip(new ClipContext(start, end, ClipContext.Block.OUTLINE, ClipContext.Fluid.ANY, fireball));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected static void onEntitySpawn(Entity entity)
    pub fn on_entity_spawn(entity: &dyn Entity) {
        if let Some(armor_stand) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() {
            // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
            // rust: FIRE_BALLS_WITH_START_POS.get_mut().insert(armor_stand.clone(), armor_stand.base.base.position());
        }
    }

    /// java: protected static void onEntityDespawn(Entity entity)
    pub fn on_entity_despawn(entity: &dyn Entity) {
        if let Some(armor_stand) = entity.as_any().unwrap_or(&() as &dyn std::any::Any).downcast_ref::<ArmorStand>() {
            FIRE_BALLS_WITH_START_POS.get_mut().remove(armor_stand);
        }
    }

    /// java: protected static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if !(mcrust::net::minecraft::core::particles::particle_types::flame() == packet.get_particle().unwrap().get_type()) {
            return;
        }
        let mut particle_pos: Vec3 = Vec3::new(packet.get_x(), packet.get_y(), packet.get_z());
        let mut neareast_fireball: Option<ArmorStand> = None;
        let mut clostest_distance: f64 = ((50) as f64) as f64;
        for fireball in FIRE_BALLS_WITH_START_POS.get_mut().keys().cloned() {
            let mut distance: f64 = fireball.base.base.position().distance_to(&particle_pos);
            if distance < clostest_distance {
                // TODO(fabric2seer): the mod keeps the game's `ArmorStand` (Java holds a reference to the live object); the port's `ArmorStand` is not a value to copy and the port lends the mod no handle to it
                // rust: neareast_fireball = Some(fireball.clone());
                clostest_distance = distance;
            }
        }
        if let Some(neareast_fireball) = &neareast_fireball {
        } else {
            return;
        }
        let mut delta: Vec3 = particle_pos.subtract_vec(&neareast_fireball.as_ref().unwrap().base.base.position());
        PARTICLE_OFFSETS.get_mut().insert(neareast_fireball.clone().unwrap(), delta.clone());
    }

}
