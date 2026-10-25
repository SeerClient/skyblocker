//! Ported from de.hysky.skyblocker.skyblock.dwarven.CrystalsChestHighlighter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::particle_events;
use crate::de::hysky::skyblocker::events::play_sound_events;
use crate::de::hysky::skyblocker::events::world_events;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::network::protocol::game::clientbound_sound_packet::ClientboundSoundPacket;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::clip_block_state_context::ClipBlockStateContext;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::core::particles::particle_options::ParticleOptions as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;

/// java: private static final String CHEST_SPAWN_MESSAGE = "You uncovered a treasure chest!";
pub const CHEST_SPAWN_MESSAGE: &str = "You uncovered a treasure chest!";
/// java: private static final long MAX_PARTICLE_LIFE_TIME = 250;
pub const MAX_PARTICLE_LIFE_TIME: i64 = ((250) as i64) as i64;
/// java: private static final Vec3 LOCK_HIGHLIGHT_SIZE = new Vec3(0.1, 0.1, 0.1);
pub static LOCK_HIGHLIGHT_SIZE: compat::MainThread<Vec3> = compat::MainThread::new(|| Vec3::new(0.1, 0.1, 0.1));
/// java: private static int waitingForChest = 0;
pub static WAITING_FOR_CHEST: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final BlockPosSet activeChests = new BlockPosSet();
pub static ACTIVE_CHESTS: compat::MainThread<compat::Shared<BlockPosSet>> = compat::MainThread::new(|| BlockPosSet::new());
/// java: private static final Object2LongOpenHashMap<Vec3> activeParticles = new Object2LongOpenHashMap<>();
/// `Object2LongOpenHashMap<Vec3>` keyed by `Vec3.equals` (the three doubles); `Vec3` has no `Hash`/`Eq` in the port
/// (`f64` fields), so the map is its entries, looked up by `==`.
pub static ACTIVE_PARTICLES: compat::MainThread<Vec<(Vec3, i64)>> = compat::MainThread::new(|| Vec::new());
/// java: private static int currentLockCount = 0;
pub static CURRENT_LOCK_COUNT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int neededLockCount = 0;
pub static NEEDED_LOCK_COUNT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

/// java: public class CrystalsChestHighlighter
#[derive(Clone)]
pub struct CrystalsChestHighlighter {}

impl CrystalsChestHighlighter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(CrystalsChestHighlighter::extractLocationFromMessage);
        compat::register_allow_game(|text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent, overlay: bool| CrystalsChestHighlighter::extract_location_from_message(text, overlay));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { CrystalsChestHighlighter::extract_rendering(&__a0); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            CrystalsChestHighlighter::reset();
        }));
        world_events::BLOCK_STATE_UPDATE.register(Arc::new(move |__a0, __a1, __a2| { CrystalsChestHighlighter::on_block_update(__a0.clone(), &__a1, &__a2); }));
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { CrystalsChestHighlighter::on_particle(&__a0); }));
        play_sound_events::FROM_SERVER.register(Arc::new(move |__a0| { CrystalsChestHighlighter::on_sound(&__a0); }));
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&WAITING_FOR_CHEST, 0);
        BlockPosSet::clear(ACTIVE_CHESTS.get_mut());
        ACTIVE_PARTICLES.get_mut().clear();
        compat::set_state(&CURRENT_LOCK_COUNT, 0);
    }

    /// java: private static boolean extractLocationFromMessage(Component text, boolean b)
    pub fn extract_location_from_message(text: &MutableComponent, mut b: bool) -> bool {
        if !(Utils::is_in_crystal_hollows()) || !(SkyblockerConfigManager::get().mining.crystal_hollows.chest_highlighter) {
            return true;
        }
        if compat::regex::Pattern::matches(CHEST_SPAWN_MESSAGE, &text.get_string()) {
            compat::set_state(&WAITING_FOR_CHEST, (compat::get_state(&WAITING_FOR_CHEST)).wrapping_add(1));
        }
        return true;
    }

    /// java: private static void onBlockUpdate(BlockPos pos, BlockState oldState, BlockState newState)
    pub fn on_block_update(mut pos: BlockPos, old_state: &Arc<BlockState>, new_state: &Arc<BlockState>) {
        if !(SkyblockerConfigManager::get().mining.crystal_hollows.chest_highlighter) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(Utils::is_in_crystal_hollows()) {
            return;
        }
        if compat::get_state(&WAITING_FOR_CHEST) > 0 && new_state.is_block(&blocks::blocks().get("CHEST")) {
            // `pos.distToCenterSqr(player.position())` - the `Position` overload is the three coordinates.
            let player_position: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap();
            if pos.dist_to_center_sqr(player_position.x, player_position.y, player_position.z) > (100 as f64) {
                return;
            }
            BlockPosSet::add(ACTIVE_CHESTS.get_mut(), &pos);
            compat::set_state(&CURRENT_LOCK_COUNT, 0);
            compat::set_state(&WAITING_FOR_CHEST, (compat::get_state(&WAITING_FOR_CHEST)).wrapping_sub(1));
        } else if new_state.is_air() && BlockPosSet::contains_block_pos(ACTIVE_CHESTS.get_mut(), &pos) {
            compat::set_state(&CURRENT_LOCK_COUNT, 0);
            BlockPosSet::remove_block_pos(ACTIVE_CHESTS.get_mut(), &pos);
        }
    }

    /// java: private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if !(Utils::is_in_crystal_hollows()) || !(SkyblockerConfigManager::get().mining.crystal_hollows.chest_highlighter) {
            return;
        }
        if packet.particle().is_some_and(|particle| compat::particle_type_is(particle, mcrust::net::minecraft::core::particles::particle_types::crit())) {
            // `activeParticles.put(pos, now)`
            let particle_pos = Vec3::new(packet.x(), packet.y(), packet.z());
            let now = compat::millis();
            match ACTIVE_PARTICLES.get_mut().iter_mut().find(|(pos, _)| *pos == particle_pos) {
                Some((_, time)) => *time = now,
                None => ACTIVE_PARTICLES.get_mut().push((particle_pos, now)),
            }
        }
    }

    /// java: private static void onSound(ClientboundSoundPacket packet)
    pub fn on_sound(packet: &ClientboundSoundPacket) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(Utils::is_in_crystal_hollows()) || !(SkyblockerConfigManager::get().mining.crystal_hollows.chest_highlighter) {
            return;
        }
        let sound_value = packet.get_sound().value();
        let Some(sound) = mcrust::com::mojang::serialization::cast::<SoundEvent>(&sound_value) else { return; };
        if sound.location() == mcrust::net::minecraft::sounds::sound_events::experience_orb_pickup().location() && packet.get_pitch() == (1 as f32) && !(BlockPosSet::is_empty(ACTIVE_CHESTS.get_mut())) {
            let mut eye_pos: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_eye_position_partial((0) as f32)).unwrap();
            let mut rotation_vec: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_view_vector((0) as f32)).unwrap();
            let mut range: f64 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.block_interaction_range()).unwrap();
            let mut vec3d3: Vec3 = eye_pos.add((rotation_vec.x * range), (rotation_vec.y * range), (rotation_vec.z * range));
            // `player.level()` is the client's one level.
            let chest = blocks::blocks().get("CHEST");
            let is_chest = |block_state: &BlockState| block_state.is_block(&chest);
            let mut raycast: BlockHitResult = mcrust::mcrust::modding::game::level().unwrap().is_block_in_line(&mcrust::net::minecraft::world::level::clip_block_state_context::ClipBlockStateContext::new(eye_pos.clone(), vec3d3.clone(), &is_chest));
            if raycast.get_type() != mcrust::net::minecraft::world::phys::hit_result::Type::Miss {
                compat::set_state(&CURRENT_LOCK_COUNT, (compat::get_state(&CURRENT_LOCK_COUNT)).wrapping_add(1));
                ACTIVE_PARTICLES.get_mut().clear();
            }
        } else if sound.location() == mcrust::net::minecraft::sounds::sound_events::villager_no().location() {
            compat::set_state(&CURRENT_LOCK_COUNT, 0);
            ACTIVE_PARTICLES.get_mut().clear();
        } else if sound.location() == mcrust::net::minecraft::sounds::sound_events::chest_open().location() {
            compat::set_state(&NEEDED_LOCK_COUNT, (compat::get_state(&CURRENT_LOCK_COUNT)).min(5));
            compat::set_state(&CURRENT_LOCK_COUNT, 0);
            ACTIVE_PARTICLES.get_mut().clear();
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(Utils::is_in_crystal_hollows()) || !(SkyblockerConfigManager::get().mining.crystal_hollows.chest_highlighter) {
            return;
        }
        let mut color: Vec<f32> = SkyblockerConfigManager::get().mining.crystal_hollows.chest_highlight_color.get_components(Some(&mut [0.0f32, 0.0, 0.0, 0.0])).to_vec();
        for chest in BlockPosSet::iterate_mut(ACTIVE_CHESTS.get_mut()) {
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_5()); __f(collector, &AABB::of_size(&vec3::at_center_of(&chest.immutable()).subtract((0) as f64, 0.0625, (0) as f64), 0.885, 0.885, 0.885), &color, color[(3) as usize], ((3) as f32) as f32, false) });
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let target: Option<&mcrust::net::minecraft::world::phys::hit_result::AnyHitResult> = compat::minecraft().hit_result.as_ref();
        if let Some(mcrust::net::minecraft::world::phys::hit_result::AnyHitResult::Block(block_hit_result)) = target && BlockPosSet::contains_block_pos(ACTIVE_CHESTS.get_mut(), &block_hit_result.get_block_pos()) {
            let mut chest_pos: Vec3 = vec3::at_center_of(&block_hit_result.get_block_pos());
            if !(ACTIVE_PARTICLES.get_mut().is_empty()) {
                let mut highlight_spot: Vec3 = mcrust::net::minecraft::world::phys::vec3::ZERO;
                //if to old remove particle
                ACTIVE_PARTICLES.get_mut().retain(|(_, time)| !(compat::millis().wrapping_sub(*time) > MAX_PARTICLE_LIFE_TIME));
                let mut added_particles: i32 = 0;
                for particle_pos in ACTIVE_PARTICLES.get().iter().map(|(pos, _)| *pos) {
                    if particle_pos.closer_than(&chest_pos, 0.8) {
                        highlight_spot = highlight_spot.add_vec(&particle_pos);
                        added_particles += 1;
                    }
                }
                highlight_spot = (highlight_spot.scale((((1) as f64) / (added_particles as f64)))).subtract_vec(&LOCK_HIGHLIGHT_SIZE.get_mut().scale(0.5));
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_5()); __f(collector, &highlight_spot, LOCK_HIGHLIGHT_SIZE.get_mut(), &color, color[(3) as usize], false) });
            }
            if compat::get_state(&NEEDED_LOCK_COUNT) <= 0 {
                return;
            }
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text()); __f(collector, &mcrust::net::minecraft::network::chat::component::literal(&format!("{}/{}", compat::get_state(&CURRENT_LOCK_COUNT).min(compat::get_state(&NEEDED_LOCK_COUNT)), compat::get_state(&NEEDED_LOCK_COUNT))).with_color(SkyblockerConfigManager::get().mining.crystal_hollows.chest_highlight_color.get_rgb()), &chest_pos, true) });
        }
    }

}
