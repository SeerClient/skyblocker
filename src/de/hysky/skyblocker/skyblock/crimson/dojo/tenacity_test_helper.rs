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
///
/// The fireballs by entity id (Java's key is the live stand, compared by identity).
pub static FIRE_BALLS_WITH_START_POS: compat::MainThread<std::collections::HashMap<i32, Vec3>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Object2ObjectOpenHashMap<ArmorStand, Vec3> particleOffsets = new Object2ObjectOpenHashMap<>();
///
/// Keyed by entity id, as `FIRE_BALLS_WITH_START_POS`.
pub static PARTICLE_OFFSETS: compat::MainThread<std::collections::HashMap<i32, Vec3>> = compat::MainThread::new(|| std::collections::HashMap::new());

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
        let fireballs: Vec<i32> = FIRE_BALLS_WITH_START_POS.get().keys().cloned().collect();
        for fireball in fireballs {
            // the stand by id; one the level dropped has no position to read (Java keeps it until it despawns)
            let Some(position) = mcrust::mcrust::modding::game::with_entity(fireball, |e| e.base().position()) else {
                continue;
            };
            let mut line_start: Vec3 = FIRE_BALLS_WITH_START_POS.get_mut().get(&fireball).cloned().unwrap().add_vec(&PARTICLE_OFFSETS.get_mut().get(&fireball).cloned().unwrap_or_else(|| mcrust::net::minecraft::world::phys::vec3::ZERO));
            let mut fireball_pos: Vec3 = position.add_vec(&PARTICLE_OFFSETS.get_mut().get(&fireball).cloned().unwrap_or_else(|| mcrust::net::minecraft::world::phys::vec3::ZERO));
            let mut distance: Vec3 = fireball_pos.subtract_vec(&line_start);
            if distance.length() > 0.02 {
                distance = distance.scale((100) as f64);
                let mut line_end: Vec3 = line_start.add_vec(&distance);
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![line_start.clone(), line_end.clone()], &vec![1.0f32, 0.0f32, 0.0f32], ((1) as f32) as f32, ((3) as f32) as f32, false) });
                let mut hit_result: Option<mcrust::net::minecraft::world::phys::hit_result::AnyHitResult> = TenacityTestHelper::raycast(&line_start, &line_end, fireball);
                if let Some(mcrust::net::minecraft::world::phys::hit_result::AnyHitResult::Block(block_hit_result)) = &hit_result {
                    if block_hit_result.get_type() == mcrust::net::minecraft::world::phys::hit_result::Type::Block {
                        ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &block_hit_result.get_block_pos(), &vec![1.0f32, 0.0f32, 0.0f32], 0.5f32, false) });
                    }
                }
            }
        }
    }

    /// java: protected static HitResult raycast(Vec3 start, Vec3 end, ArmorStand fireball)
    ///
    /// `fireball` by entity id.
    pub fn raycast(start: &Vec3, end: &Vec3, fireball: i32) -> Option<mcrust::net::minecraft::world::phys::hit_result::AnyHitResult> {
        use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;
        let Some(level) = mcrust::mcrust::modding::game::level() else {
            return None;
        };
        // CLIENT.level.clip(new ClipContext(start, end, ClipContext.Block.OUTLINE, ClipContext.Fluid.ANY, fireball)):
        // the entity constructor is `CollisionContext.of(fireball)`
        return mcrust::mcrust::modding::game::with_entity(fireball, |e| {
            let context = mcrust::net::minecraft::world::phys::shapes::collision_context::of(&*e, &*level);
            mcrust::net::minecraft::world::phys::hit_result::AnyHitResult::Block(level.clip(&ClipContext::new(start.clone(), end.clone(), mcrust::net::minecraft::world::level::clip_context::Block::Outline, mcrust::net::minecraft::world::level::clip_context::Fluid::Any, &*context)))
        });
    }

    /// java: protected static void onEntitySpawn(Entity entity)
    pub fn on_entity_spawn(entity: &dyn Entity) {
        if entity.as_any().is_some_and(|a| a.is::<ArmorStand>()) {
            FIRE_BALLS_WITH_START_POS.get_mut().insert(entity.base().get_id(), entity.base().position());
        }
    }

    /// java: protected static void onEntityDespawn(Entity entity)
    pub fn on_entity_despawn(entity: &dyn Entity) {
        if entity.as_any().is_some_and(|a| a.is::<ArmorStand>()) {
            FIRE_BALLS_WITH_START_POS.get_mut().remove(&entity.base().get_id());
        }
    }

    /// java: protected static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if !(compat::particle_type_is(packet.particle().unwrap(), mcrust::net::minecraft::core::particles::particle_types::flame())) {
            return;
        }
        let mut particle_pos: Vec3 = Vec3::new(packet.x(), packet.y(), packet.z());
        // the nearest fireball: (id, position)
        let mut neareast_fireball: Option<(i32, Vec3)> = None;
        let mut clostest_distance: f64 = ((50) as f64) as f64;
        let fireballs: Vec<i32> = FIRE_BALLS_WITH_START_POS.get().keys().cloned().collect();
        for fireball in fireballs {
            let Some(position) = mcrust::mcrust::modding::game::with_entity(fireball, |e| e.base().position()) else {
                continue;
            };
            let mut distance: f64 = position.distance_to(&particle_pos);
            if distance < clostest_distance {
                neareast_fireball = Some((fireball, position));
                clostest_distance = distance;
            }
        }
        let Some((neareast_fireball, neareast_position)) = neareast_fireball else {
            return;
        };
        let mut delta: Vec3 = particle_pos.subtract_vec(&neareast_position);
        PARTICLE_OFFSETS.get_mut().insert(neareast_fireball, delta.clone());
    }

}
