//! Ported from de.hysky.skyblocker.skyblock.foraging.galatea.InvisibugHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::particle_events;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::core::particles::particle_options::ParticleOptions as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: private static final Object2LongOpenHashMap<Vec3> invisibugLocations = new Object2LongOpenHashMap<>();
pub static INVISIBUG_LOCATIONS: compat::MainThread<std::collections::HashMap<Vec3, i64>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final long PARTICLE_EXPIRY = 500;
pub const PARTICLE_EXPIRY: i64 = ((500) as i64) as i64;
/// java: private static final float[] COLOR = new float[3];
pub static COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0; (3) as usize]);
/// java: private static final float ALPHA = 0.20f;
pub const ALPHA: f32 = 0.2f32;
/// java: private static final Vec3 HIGHLIGHT_SIZE = new Vec3(0.5f, 0.5f, 0.5f);
pub static HIGHLIGHT_SIZE: compat::MainThread<Vec3> = compat::MainThread::new(|| Vec3::new((0.5f32) as f64, (0.5f32) as f64, (0.5f32) as f64));

/// java: public class InvisibugHighlighter
#[derive(Clone)]
pub struct InvisibugHighlighter {}

impl InvisibugHighlighter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { InvisibugHighlighter::extract_rendering(&__a0); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            INVISIBUG_LOCATIONS.get_mut().clear();
        }));
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { InvisibugHighlighter::on_particle(&__a0); }));
    }

    /// java: private static boolean isActive()
    pub fn is_active() -> bool {
        return (SkyblockerConfigManager::get().hunting.moonglade_mobs.highlight_invisibug && Utils::is_in_galatea());
    }

    /// java: private static boolean isNearPet(ClientLevel level, Vec3 pos)
    pub fn is_near_pet(level: &ClientLevel, pos: &Vec3) -> bool {
        // TODO(fabric2seer): `.distanceToSqr(..)` on a T
        //   java: return level.getEntitiesOfClass(ArmorStand.class, AABB.ofSize(pos, 10, 10, 10)).stream()
        //   java: 				.min(Comparator.comparingDouble(armorStand -> armorStand.distanceToSqr(pos)))
        //   java: 				.map(armorStand -> armorStand.hasItemInSlot(EquipmentSlot.MAINHAND) || armorStand.getCustomName() != null)
        //   java: 				.orElse(false);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if InvisibugHighlighter::is_active() && mcrust::net::minecraft::core::particles::particle_types::crit() == packet.get_particle().unwrap().get_type() && mcrust::mcrust::modding::game::level().is_some() {
            let mut pos: Vec3 = Vec3::new(packet.get_x(), packet.get_y(), packet.get_z());
            // TODO(fabric2seer): the port's `get_entities_of_class_no_spectators` ports this Java method with other parameters (base_class, except, bb for Java's base_class, bb)
            // TODO(fabric2seer): `get_entities_of_class_no_spectators` takes 3 argument(s) in the port (`base_class: crate::net::minecraft::world::level::entity::entity_type_test::EntityClass, except: Option<i32>, bb: &AABB`), Java passes 2
            if (mcrust::mcrust::modding::game::level().unwrap().get_entities_of_class_no_spectators(std::any::TypeId::of::<Box<dyn LivingEntity>>(), Some(AABB::of_size(&pos, (1) as f64, (1) as f64, (1) as f64)))).is_empty() && !(InvisibugHighlighter::is_near_pet(&mcrust::mcrust::modding::game::level().unwrap(), &pos)) {
                InvisibugHighlighter::handle_invisibug_particle(pos.clone());
            }
        }
    }

    /// java: private static void handleInvisibugParticle(Vec3 pos)
    pub fn handle_invisibug_particle(mut pos: Vec3) {
        INVISIBUG_LOCATIONS.get_mut().retain(|__k, _| !(|e| (e.distance_to_sqr_vec(&pos) <= (1 as f64)))(__k));
        INVISIBUG_LOCATIONS.get_mut().insert(pos.clone(), compat::millis());
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(InvisibugHighlighter::is_active()) || INVISIBUG_LOCATIONS.get_mut().is_empty() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut now: i64 = compat::millis();
        // TODO(fabric2seer): Map.object2LongEntrySet(..)
        //   java: invisibugLocations.object2LongEntrySet().removeIf(e -> now - e.getLongValue() > PARTICLE_EXPIRY);
        // TODO(fabric2seer): `.getRGBColorComponents(..)` on a Color
        //   java: SkyblockerConfigManager.get().hunting.moongladeMobs.invisibugGlowColor.getRGBColorComponents(COLOR);
        for pos in INVISIBUG_LOCATIONS.get_mut().keys().cloned() {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_5()); __f(collector, &pos.subtract_vec(&HIGHLIGHT_SIZE.get_mut().scale(0.5)), HIGHLIGHT_SIZE.get_mut(), &COLOR, ALPHA, false) });
        }
    }

}
