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
/// (`Vec3` keys by `Vec3.equals`, which the port's `Vec3` has as `PartialEq` but not as `Hash`: kept as a list of entries)
pub static INVISIBUG_LOCATIONS: compat::MainThread<Vec<(Vec3, i64)>> = compat::MainThread::new(|| Vec::new());
/// java: private static final long PARTICLE_EXPIRY = 500;
pub const PARTICLE_EXPIRY: i64 = ((500) as i64) as i64;
/// java: private static final float[] COLOR = new float[3];
pub static COLOR: compat::MainThread<Vec<f32>> = compat::MainThread::new(|| vec![0.0; (3) as usize]);
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
        // java: level.getEntitiesOfClass(ArmorStand.class, AABB.ofSize(pos, 10, 10, 10)) - `EntitySelector.NO_SPECTATORS`
        // over the armor stands (the port's class tests stop at LivingEntity; the type is ArmorStand's)
        let armor_stands = level.get_entities(None, &AABB::of_size(pos, (10) as f64, (10) as f64, (10) as f64), &|entity| {
            compat::is_entity_type(entity, "armor_stand") && mcrust::net::minecraft::world::entity::entity_selector::no_spectators(entity)
        });
        // java: .min(Comparator.comparingDouble(armorStand -> armorStand.distanceToSqr(pos)))
        let mut closest: Option<(f64, bool)> = None;
        for armor_stand in armor_stands.iter() {
            let Some((distance, is_pet)) = compat::with_entity_handle(armor_stand, |armor_stand| {
                use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
                let is_pet = armor_stand.as_living_entity().is_some_and(|living| living.has_item_in_slot(mcrust::net::minecraft::world::entity::equipment_slot::EquipmentSlot::Mainhand))
                    || armor_stand.base().get_custom_name().is_some();
                (armor_stand.base().distance_to_sqr_vec(pos), is_pet)
            }) else {
                continue;
            };
            // `Stream.min` keeps the first of equal elements
            if closest.map_or(true, |(best, _)| distance < best) {
                closest = Some((distance, is_pet));
            }
        }
        // java: .map(armorStand -> armorStand.hasItemInSlot(EquipmentSlot.MAINHAND) || armorStand.getCustomName() != null).orElse(false)
        return closest.map_or(false, |(_, is_pet)| is_pet);
    }

    /// java: private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if InvisibugHighlighter::is_active() && packet.get_particle().is_some_and(|particle| compat::particle_type_is(particle, mcrust::net::minecraft::core::particles::particle_types::crit())) && mcrust::mcrust::modding::game::level().is_some() {
            let mut pos: Vec3 = Vec3::new(packet.get_x(), packet.get_y(), packet.get_z());
            let __level = mcrust::mcrust::modding::game::level().unwrap();
            if (__level.get_entities_of_class_no_spectators(mcrust::net::minecraft::world::level::entity::entity_type_test::EntityClass::LivingEntity, None, &AABB::of_size(&pos, (1) as f64, (1) as f64, (1) as f64))).is_empty() && !(InvisibugHighlighter::is_near_pet(&__level, &pos)) {
                InvisibugHighlighter::handle_invisibug_particle(pos.clone());
            }
        }
    }

    /// java: private static void handleInvisibugParticle(Vec3 pos)
    pub fn handle_invisibug_particle(mut pos: Vec3) {
        INVISIBUG_LOCATIONS.get_mut().retain(|(__k, _)| !(__k.distance_to_sqr_vec(&pos) <= (1 as f64)));
        // java: invisibugLocations.put(pos, System.currentTimeMillis()) (replaces an equal key's value)
        let __now = compat::millis();
        match INVISIBUG_LOCATIONS.get_mut().iter_mut().find(|(__k, _)| *__k == pos) {
            Some(__e) => __e.1 = __now,
            None => INVISIBUG_LOCATIONS.get_mut().push((pos.clone(), __now)),
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(InvisibugHighlighter::is_active()) || INVISIBUG_LOCATIONS.get_mut().is_empty() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut now: i64 = compat::millis();
        // java: invisibugLocations.object2LongEntrySet().removeIf(e -> now - e.getLongValue() > PARTICLE_EXPIRY);
        INVISIBUG_LOCATIONS.get_mut().retain(|(_, __time)| !(now.wrapping_sub(*__time) > PARTICLE_EXPIRY));
        // java: SkyblockerConfigManager.get().hunting.moongladeMobs.invisibugGlowColor.getRGBColorComponents(COLOR);
        let __components = SkyblockerConfigManager::get().hunting.moonglade_mobs.invisibug_glow_color.get_rgb_color_components(None);
        COLOR.get_mut().copy_from_slice(&__components);
        for pos in INVISIBUG_LOCATIONS.get().iter().map(|(__k, _)| __k.clone()).collect::<Vec<_>>() {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_5()); __f(collector, &pos.subtract_vec(&HIGHLIGHT_SIZE.get_mut().scale(0.5)), HIGHLIGHT_SIZE.get_mut(), COLOR.get(), ALPHA, false) });
        }
    }

}
