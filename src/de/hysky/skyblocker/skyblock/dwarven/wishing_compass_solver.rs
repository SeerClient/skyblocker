//! Ported from de.hysky.skyblocker.skyblock.dwarven.WishingCompassSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::particle_events;
use crate::de::hysky::skyblocker::skyblock::dwarven::crystals_locations_manager::CrystalsLocationsManager;
use crate::de::hysky::skyblocker::skyblock::dwarven::mining_location_label::CrystalHollowsLocationsCategory;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::core::particles::particle_options::ParticleOptions as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;

/// java: private static final Map<Zone, AABB> ZONE_BOUNDING_BOXES = Map.of( Zone.CRYSTAL_NUCLEUS, new AABB(462, 63, 461, 564, 181, 565), Zone.JUNGLE, new AABB(201, 63, 201, 513, 189, 513), Zone.MITHRIL_DEPOSIT
pub static ZONE_BOUNDING_BOXES: compat::MainThread<std::collections::HashMap<Zone, AABB>> = compat::MainThread::new(|| std::collections::HashMap::from([(Zone::CrystalNucleus, AABB::new((462) as f64, (63) as f64, (461) as f64, (564) as f64, (181) as f64, (565) as f64)), (Zone::Jungle, AABB::new((201) as f64, (63) as f64, (201) as f64, (513) as f64, (189) as f64, (513) as f64)), (Zone::MithrilDeposits, AABB::new((512) as f64, (63) as f64, (201) as f64, (824) as f64, (189) as f64, (513) as f64)), (Zone::GoblinHoldout, AABB::new((201) as f64, (63) as f64, (512) as f64, (513) as f64, (189) as f64, (824) as f64)), (Zone::PrecursorRemnants, AABB::new((512) as f64, (63) as f64, (512) as f64, (824) as f64, (189) as f64, (824) as f64)), (Zone::MagmaFields, AABB::new((201) as f64, (30) as f64, (201) as f64, (824) as f64, (64) as f64, (824) as f64))]));
/// java: private static final Vec3 JUNGLE_TEMPLE_DOOR_OFFSET = new Vec3(-57, 36, -21);
pub static JUNGLE_TEMPLE_DOOR_OFFSET: compat::MainThread<Vec3> = compat::MainThread::new(|| Vec3::new((-57) as f64, (36) as f64, (-21) as f64));
/// java: private static final long PARTICLES_PER_LINE = 25;
pub const PARTICLES_PER_LINE: i64 = ((25) as i64) as i64;
/// java: private static final long PARTICLES_MAX_DELAY = 500;
pub const PARTICLES_MAX_DELAY: i64 = ((500) as i64) as i64;
/// java: private static final double PARTICLES_MAX_DISTANCE = 0.9;
pub const PARTICLES_MAX_DISTANCE: f64 = 0.9;
/// java: private static final long DISTANCE_BETWEEN_USES = 8;
pub const DISTANCE_BETWEEN_USES: i64 = ((8) as i64) as i64;
/// java: private static final double DISTANCE_TOLERANCE = 5.0;
pub const DISTANCE_TOLERANCE: f64 = 5.0;
/// java: private static SolverStates currentState = SolverStates.NOT_STARTED;
pub static CURRENT_STATE: LazyLock<Mutex<SolverStates>> = LazyLock::new(|| Mutex::new(SolverStates::NotStarted));
/// java: private static Vec3 startPosOne = Vec3.ZERO;
pub static START_POS_ONE: compat::MainThread<Vec3> = compat::MainThread::new(|| mcrust::net::minecraft::world::phys::vec3::ZERO);
/// java: private static Vec3 startPosTwo = Vec3.ZERO;
pub static START_POS_TWO: compat::MainThread<Vec3> = compat::MainThread::new(|| mcrust::net::minecraft::world::phys::vec3::ZERO);
/// java: private static Vec3 directionOne = Vec3.ZERO;
pub static DIRECTION_ONE: compat::MainThread<Vec3> = compat::MainThread::new(|| mcrust::net::minecraft::world::phys::vec3::ZERO);
/// java: private static Vec3 directionTwo = Vec3.ZERO;
pub static DIRECTION_TWO: compat::MainThread<Vec3> = compat::MainThread::new(|| mcrust::net::minecraft::world::phys::vec3::ZERO);
/// java: private static long particleUsedCountOne = 0;
pub static PARTICLE_USED_COUNT_ONE: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(((0) as i64) as i64));
/// java: private static long particleUsedCountTwo = 0;
pub static PARTICLE_USED_COUNT_TWO: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(((0) as i64) as i64));
/// java: private static long particleLastUpdate = System.currentTimeMillis();
pub static PARTICLE_LAST_UPDATE: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(compat::millis()));
/// java: private static Vec3 particleLastPos = Vec3.ZERO;
pub static PARTICLE_LAST_POS: compat::MainThread<Vec3> = compat::MainThread::new(|| mcrust::net::minecraft::world::phys::vec3::ZERO);

/// java: public class WishingCompassSolver
#[derive(Clone)]
pub struct WishingCompassSolver {}

impl WishingCompassSolver {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: UseItemCallback.EVENT.register(WishingCompassSolver::onItemInteract);
        compat::register_use_item_callback(|player: &dyn mcrust::net::minecraft::world::entity::player::player::Player, level: &mcrust::net::minecraft::world::level::level::Level, hand: mcrust::net::minecraft::world::interaction_hand::InteractionHand| WishingCompassSolver::on_item_interact(player, level, &hand).unwrap_or(InteractionResult::Pass));
        // java: UseBlockCallback.EVENT.register(WishingCompassSolver::onBlockInteract);
        compat::batch_w_register_use_block_callback(Arc::new(|player: &dyn mcrust::net::minecraft::world::entity::player::player::Player, level: &mcrust::net::minecraft::world::level::level::Level, hand: mcrust::net::minecraft::world::interaction_hand::InteractionHand, hit_result: &mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult| WishingCompassSolver::on_block_interact(player, level, &hand, hit_result).unwrap_or(InteractionResult::Pass)));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(WishingCompassSolver::failMessageListener);
        compat::register_allow_game(|text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent, overlay: bool| WishingCompassSolver::fail_message_listener(text, overlay));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            WishingCompassSolver::reset();
        }));
        particle_events::FROM_SERVER.register(Arc::new(move |__a0| { WishingCompassSolver::on_particle(&__a0); }));
    }

    /// java: private static boolean failMessageListener(Component text, boolean b)
    pub fn fail_message_listener(text: &MutableComponent, mut b: bool) -> bool {
        if !(Utils::is_in_crystal_hollows()) {
            return true;
        }
        if chat_formatting::strip_formatting(&text.get_string()) == "The Wishing Compass can't seem to locate anything!" {
            compat::set_state(&CURRENT_STATE, SolverStates::NotStarted);
        }
        return true;
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&CURRENT_STATE, SolverStates::NotStarted);
        *START_POS_ONE.get_mut() = mcrust::net::minecraft::world::phys::vec3::ZERO;
        *START_POS_TWO.get_mut() = mcrust::net::minecraft::world::phys::vec3::ZERO;
        *DIRECTION_ONE.get_mut() = mcrust::net::minecraft::world::phys::vec3::ZERO;
        *DIRECTION_TWO.get_mut() = mcrust::net::minecraft::world::phys::vec3::ZERO;
        compat::set_state(&PARTICLE_USED_COUNT_ONE, ((0) as i64) as i64);
        compat::set_state(&PARTICLE_USED_COUNT_TWO, ((0) as i64) as i64);
        compat::set_state(&PARTICLE_LAST_UPDATE, compat::millis());
        *PARTICLE_LAST_POS.get_mut() = mcrust::net::minecraft::world::phys::vec3::ZERO;
    }

    /// java: private static boolean isKingsScentPresent()
    pub fn is_kings_scent_present() -> bool {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        if !PlayerListManager::get_player_string_list().into_iter().any(|entry| entry.starts_with("Active Effects:")) {
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.enableTabEffectsMessage"))); });
            return false;
        }
        return PlayerListManager::get_player_string_list().into_iter().any(|entry| entry.starts_with("King's Scent"));
    }

    /// java: private static boolean isKeyInInventory()
    pub fn is_key_in_inventory() -> bool {
        return (mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().get_non_equipment_items().iter().cloned().any(|stack| (true /* java: stack != null: never null here */ && stack.get_skyblock_id() == "JUNGLE_KEY"))).unwrap_or(false));
    }

    /// java: private static Zone getZoneOfLocation(Vec3 location)
    pub fn get_zone_of_location(location: &Vec3) -> Zone {
        //   java: return ZONE_BOUNDING_BOXES.entrySet().stream().filter(zone -> zone.getValue().contains(location)).findFirst().map(Map.Entry::getKey).orElse(Zone.CRYSTAL_NUCLEUS);
        // (`Map.of` iterates in an unspecified order, as the `HashMap` here does.)
        return ZONE_BOUNDING_BOXES.get().iter().find(|(_, bounding_box)| bounding_box.contains_vec(location)).map(|(zone, _)| *zone).unwrap_or(Zone::CrystalNucleus);
    }

    /// java: private static Boolean isZoneComplete(Zone zone)
    pub fn is_zone_complete(mut zone: Zone) -> bool {
        if compat::get_connection(compat::minecraft()).is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        if !PlayerListManager::get_player_string_list().into_iter().any(|entry| (entry == "Crystals:")) {
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.enableTabMessage"))); });
            return false;
        }
        let mut display_name_stream = PlayerListManager::get_player_string_list().into_iter();
        return match zone {
            Zone::Jungle => !display_name_stream.clone().any(|entry| (entry == "Amethyst: ✖ Not Found")),
            Zone::MithrilDeposits => !display_name_stream.clone().any(|entry| (entry == "Jade: ✖ Not Found")),
            Zone::GoblinHoldout => !display_name_stream.clone().any(|entry| (entry == "Amber: ✖ Not Found")),
            Zone::PrecursorRemnants => !display_name_stream.clone().any(|entry| (entry == "Sapphire: ✖ Not Found")),
            Zone::MagmaFields => !display_name_stream.clone().any(|entry| (entry == "Topaz: ✖ Not Found")),
            _ => false,
        };
    }

    /// java: private static MiningLocationLabel.CrystalHollowsLocationsCategory getTargetLocation(Zone startingZone)
    pub fn get_target_location(mut starting_zone: Zone) -> CrystalHollowsLocationsCategory {
        if WishingCompassSolver::is_zone_complete(starting_zone) {
            return CrystalHollowsLocationsCategory::Unknown;
        }
        return match starting_zone {
            Zone::Jungle => (if WishingCompassSolver::is_key_in_inventory() { CrystalHollowsLocationsCategory::JungleTemple } else { CrystalHollowsLocationsCategory::Odawa }),
            Zone::MithrilDeposits => CrystalHollowsLocationsCategory::MinesOfDivan,
            Zone::GoblinHoldout => (if WishingCompassSolver::is_kings_scent_present() { CrystalHollowsLocationsCategory::GoblinQueensDen } else { CrystalHollowsLocationsCategory::KingYolkar }),
            Zone::PrecursorRemnants => CrystalHollowsLocationsCategory::LostPrecursorCity,
            Zone::MagmaFields => CrystalHollowsLocationsCategory::KhazadDum,
            _ => CrystalHollowsLocationsCategory::Unknown,
        };
    }

    /// java: private static Boolean verifyLocation(Zone startingZone, Vec3 pos)
    pub fn verify_location(mut starting_zone: Zone, pos: &Vec3) -> bool {
        return (ZONE_BOUNDING_BOXES.get_mut().get(&starting_zone).cloned().unwrap().inflate3((100) as f64, (0) as f64, (100) as f64)).contains_vec(pos);
    }

    /// java: @SuppressWarnings("incomplete-switch") private static void onParticle(ClientboundLevelParticlesPacket packet)
    pub fn on_particle(packet: &ClientboundLevelParticlesPacket) {
        if !(Utils::is_in_crystal_hollows()) || !packet.particle().is_some_and(|particle| compat::particle_type_is(particle, mcrust::net::minecraft::core::particles::particle_types::happy_villager())) {
            return;
        }
        let mut particle_pos: Vec3 = Vec3::new(packet.x(), packet.y(), packet.z());
        compat::set_state(&PARTICLE_LAST_UPDATE, compat::millis());
        if particle_pos.distance_to(PARTICLE_LAST_POS.get_mut()) > PARTICLES_MAX_DISTANCE {
            return;
        }
        *PARTICLE_LAST_POS.get_mut() = particle_pos.clone();
        match compat::get_state(&CURRENT_STATE) {
            SolverStates::ProcessingFirstUse => {
                let mut particle_direction: Vec3 = particle_pos.subtract_vec(START_POS_ONE.get_mut()).normalize();
                *DIRECTION_ONE.get_mut() = DIRECTION_ONE.get_mut().add_vec(&particle_direction.scale((((1) as f64) / (PARTICLES_PER_LINE as f64))));
                compat::set_state(&PARTICLE_USED_COUNT_ONE, (compat::get_state(&PARTICLE_USED_COUNT_ONE)).wrapping_add(1 as i64));
                if compat::get_state(&PARTICLE_USED_COUNT_ONE) >= PARTICLES_PER_LINE {
                    compat::set_state(&CURRENT_STATE, SolverStates::WaitingForSecond);
                    if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                        mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.wishingCompassUsedMessage").with_style_format(ChatFormatting::Green))); });
                    }
                }
            }
            SolverStates::ProcessingSecondUse => {
                let mut particle_direction: Vec3 = particle_pos.subtract_vec(START_POS_TWO.get_mut()).normalize();
                *DIRECTION_TWO.get_mut() = DIRECTION_TWO.get_mut().add_vec(&particle_direction.scale((((1) as f64) / (PARTICLES_PER_LINE as f64))));
                compat::set_state(&PARTICLE_USED_COUNT_TWO, (compat::get_state(&PARTICLE_USED_COUNT_TWO)).wrapping_add(1 as i64));
                if compat::get_state(&PARTICLE_USED_COUNT_TWO) >= PARTICLES_PER_LINE {
                    WishingCompassSolver::process_solution();
                }
            }
            _ => {}
        }
    }

    /// java: private static void processSolution()
    pub fn process_solution() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            WishingCompassSolver::reset();
            return;
        }
        let mut target_location: Option<Vec3> = WishingCompassSolver::solve(START_POS_ONE.get_mut(), START_POS_TWO.get_mut(), DIRECTION_ONE.get_mut(), DIRECTION_TWO.get_mut());
        if target_location.is_none() {
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.somethingWentWrongMessage").with_style_format(ChatFormatting::Red))); });
        } else {
            let mut player_zone: Zone = WishingCompassSolver::get_zone_of_location(START_POS_ONE.get_mut());
            let mut location: CrystalHollowsLocationsCategory = WishingCompassSolver::get_target_location(player_zone);
            if !(WishingCompassSolver::verify_location(player_zone, target_location.as_ref().unwrap())) {
                location = CrystalHollowsLocationsCategory::Unknown;
                mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.targetLocationToFar").with_style_format(ChatFormatting::Red))); });
            }
            if location == CrystalHollowsLocationsCategory::JungleTemple {
                target_location = Some(target_location.as_ref().unwrap().add_vec(JUNGLE_TEMPLE_DOOR_OFFSET.get_mut()));
            }
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.foundMessage").with_style_format(ChatFormatting::Green)).append(component::literal(&location.get_name()).with_color(location.get_color())).append(component::literal(&format!(": {} {} {}", ((target_location.as_ref().unwrap().x()) as i32), ((target_location.as_ref().unwrap().y()) as i32), ((target_location.as_ref().unwrap().z()) as i32))))); });
            CrystalsLocationsManager::add_custom_waypoint(location.get_name(), &BlockPos::containing_position(target_location.as_ref().unwrap()));
        }
        WishingCompassSolver::reset();
    }

    /// java: protected static Vec3 solve(Vec3 startPosOne, Vec3 startPosTwo, Vec3 directionOne, Vec3 directionTwo)
    pub fn solve(start_pos_one: &Vec3, start_pos_two: &Vec3, direction_one: &Vec3, direction_two: &Vec3) -> Option<Vec3> {
        if direction_one == direction_two {
            return None;
        }
        let line_one_start: Vector3D = Vector3D::new(start_pos_one.x, start_pos_one.y, start_pos_one.z);
        let line_one_end: Vector3D = Vector3D::new(direction_one.x, direction_one.y, direction_one.z).add(&line_one_start);
        let line_two_start: Vector3D = Vector3D::new(start_pos_two.x, start_pos_two.y, start_pos_two.z);
        let line_two_end: Vector3D = Vector3D::new(direction_two.x, direction_two.y, direction_two.z).add(&line_two_start);
        let line: Line = Line::new(&line_one_start, &line_one_end, 1.0);
        let line_two: Line = Line::new(&line_two_start, &line_two_end, 1.0);
        let close: Vector3D = line.closest_point(&line_two);
        let close_two: Vector3D = line_two.closest_point(&line);
        let distance: f64 = close.distance(&close_two);
        let mut intersection: Option<Vec3> = None;
        if distance < DISTANCE_TOLERANCE {
            let c1: Vec3 = Vec3::new(close.get_x(), close.get_y(), close.get_z());
            // (Java builds `c2` from `close` too, not `closeTwo`.)
            let c2: Vec3 = Vec3::new(close.get_x(), close.get_y(), close.get_z());
            intersection = Some(c1.add_vec(&c2).scale(0.5));
        }
        return intersection;
    }

    /// java: private static InteractionResult onBlockInteract(Player playerEntity, Level world, InteractionHand hand, BlockHitResult blockHitResult)
    pub fn on_block_interact(player_entity: &dyn Player, world: &Level, hand: &InteractionHand, block_hit_result: &BlockHitResult) -> Option<InteractionResult> {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return None;
        }
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_item_in_hand(*hand).clone()).unwrap();
        if !(Utils::is_in_crystal_hollows()) || !(SkyblockerConfigManager::get().mining.crystals_waypoints.wishing_compass_solver) || !(stack.get_skyblock_id() == "WISHING_COMPASS") {
            return Some(InteractionResult::Pass);
        }
        if WishingCompassSolver::use_compass() {
            return Some(InteractionResult::Fail);
        }
        return Some(InteractionResult::Pass);
    }

    /// java: private static InteractionResult onItemInteract(Player playerEntity, Level world, InteractionHand hand)
    pub fn on_item_interact(player_entity: &dyn Player, world: &Level, hand: &InteractionHand) -> Option<InteractionResult> {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return None;
        }
        let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_item_in_hand(*hand).clone()).unwrap();
        if !(Utils::is_in_crystal_hollows()) || !(SkyblockerConfigManager::get().mining.crystals_waypoints.wishing_compass_solver) || !(stack.get_skyblock_id() == "WISHING_COMPASS") {
            return Some(InteractionResult::Pass);
        }
        if WishingCompassSolver::use_compass() {
            return Some(InteractionResult::Fail);
        }
        return Some(InteractionResult::Pass);
    }

    /// java: private static boolean useCompass()
    pub fn use_compass() -> bool {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return true;
        }
        let mut player_pos: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_eye_position()).unwrap();
        let mut current_zone: Zone = WishingCompassSolver::get_zone_of_location(&player_pos);
        match compat::get_state(&CURRENT_STATE) {
            SolverStates::NotStarted => {
                if current_zone == Zone::CrystalNucleus {
                    mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.useOutsideNucleusMessage"))); });
                    return true;
                }
                WishingCompassSolver::start_new_state(SolverStates::ProcessingFirstUse);
            }
            SolverStates::WaitingForSecond => {
                if START_POS_ONE.get_mut().closer_than(&player_pos, (DISTANCE_BETWEEN_USES) as f64) {
                    mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.moveFurtherMessage"))); });
                    return true;
                } else {
                    if current_zone != WishingCompassSolver::get_zone_of_location(START_POS_ONE.get_mut()) {
                        mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.changingZoneMessage"))); });
                        WishingCompassSolver::reset();
                        WishingCompassSolver::start_new_state(SolverStates::ProcessingFirstUse);
                    } else {
                        WishingCompassSolver::start_new_state(SolverStates::ProcessingSecondUse);
                    }
                }
            }
            SolverStates::ProcessingFirstUse | SolverStates::ProcessingSecondUse => {
                if (compat::millis()).wrapping_sub(compat::get_state(&PARTICLE_LAST_UPDATE)) < PARTICLES_MAX_DELAY {
                    mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.waitLongerMessage").with_style_format(ChatFormatting::Red))); });
                    return true;
                } else {
                    mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.couldNotDetectLastUseMessage").with_style_format(ChatFormatting::Red))); });
                    WishingCompassSolver::reset();
                    WishingCompassSolver::start_new_state(SolverStates::ProcessingFirstUse);
                }
            }
            _ => {}
        }
        return false;
    }

    /// java: private static void startNewState(SolverStates newState)
    pub fn start_new_state(mut new_state: SolverStates) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut player_pos: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.position().add((0) as f64, 1.62, (0) as f64)).unwrap();
        if new_state == SolverStates::ProcessingFirstUse {
            compat::set_state(&CURRENT_STATE, SolverStates::ProcessingFirstUse);
            *START_POS_ONE.get_mut() = player_pos.clone();
            compat::set_state(&PARTICLE_LAST_UPDATE, compat::millis());
            *PARTICLE_LAST_POS.get_mut() = player_pos.clone();
        } else if new_state == SolverStates::ProcessingSecondUse {
            compat::set_state(&CURRENT_STATE, SolverStates::ProcessingSecondUse);
            *START_POS_TWO.get_mut() = player_pos.clone();
            compat::set_state(&PARTICLE_LAST_UPDATE, compat::millis());
            *PARTICLE_LAST_POS.get_mut() = player_pos.clone();
        }
    }

}

/// java: private enum SolverStates
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SolverStates {
    NotStarted,
    ProcessingFirstUse,
    WaitingForSecond,
    ProcessingSecondUse,
}

impl SolverStates {
    pub const VALUES: [SolverStates; 4] = [SolverStates::NotStarted, SolverStates::ProcessingFirstUse, SolverStates::WaitingForSecond, SolverStates::ProcessingSecondUse];

    /// java: values()
    pub fn values() -> Vec<SolverStates> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SolverStates::NotStarted => "NOT_STARTED",
            SolverStates::ProcessingFirstUse => "PROCESSING_FIRST_USE",
            SolverStates::WaitingForSecond => "WAITING_FOR_SECOND",
            SolverStates::ProcessingSecondUse => "PROCESSING_SECOND_USE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SolverStates {
        match name {
            "NOT_STARTED" => SolverStates::NotStarted,
            "PROCESSING_FIRST_USE" => SolverStates::ProcessingFirstUse,
            "WAITING_FOR_SECOND" => SolverStates::WaitingForSecond,
            "PROCESSING_SECOND_USE" => SolverStates::ProcessingSecondUse,
            _ => panic!("IllegalArgumentException: No enum constant SolverStates.{name}"),
        }
    }

}

impl std::fmt::Display for SolverStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: private enum Zone
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Zone {
    CrystalNucleus,
    Jungle,
    MithrilDeposits,
    GoblinHoldout,
    PrecursorRemnants,
    MagmaFields,
}

impl Zone {
    pub const VALUES: [Zone; 6] = [Zone::CrystalNucleus, Zone::Jungle, Zone::MithrilDeposits, Zone::GoblinHoldout, Zone::PrecursorRemnants, Zone::MagmaFields];

    /// java: values()
    pub fn values() -> Vec<Zone> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Zone::CrystalNucleus => "CRYSTAL_NUCLEUS",
            Zone::Jungle => "JUNGLE",
            Zone::MithrilDeposits => "MITHRIL_DEPOSITS",
            Zone::GoblinHoldout => "GOBLIN_HOLDOUT",
            Zone::PrecursorRemnants => "PRECURSOR_REMNANTS",
            Zone::MagmaFields => "MAGMA_FIELDS",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Zone {
        match name {
            "CRYSTAL_NUCLEUS" => Zone::CrystalNucleus,
            "JUNGLE" => Zone::Jungle,
            "MITHRIL_DEPOSITS" => Zone::MithrilDeposits,
            "GOBLIN_HOLDOUT" => Zone::GoblinHoldout,
            "PRECURSOR_REMNANTS" => Zone::PrecursorRemnants,
            "MAGMA_FIELDS" => Zone::MagmaFields,
            _ => panic!("IllegalArgumentException: No enum constant Zone.{name}"),
        }
    }

}

impl std::fmt::Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// `MathArrays.linearCombination(a1, b1, a2, b2, a3, b3)` (commons-math3): `a1 * b1 + a2 * b2 + a3 * b3` with each
/// product's rounding error kept (`mul_add` gives Dekker's exact low part) and the partial sums compensated; a NaN
/// answer falls back to the plain sum, as the library does.
fn linear_combination3(a1: f64, b1: f64, a2: f64, b2: f64, a3: f64, b3: f64) -> f64 {
    let prod1 = a1 * b1;
    let prod1_low = a1.mul_add(b1, -prod1);
    let prod2 = a2 * b2;
    let prod2_low = a2.mul_add(b2, -prod2);
    let prod3 = a3 * b3;
    let prod3_low = a3.mul_add(b3, -prod3);
    // accurate addition a1 * b1 + a2 * b2
    let s12_high = prod1 + prod2;
    let s12_prime = s12_high - prod2;
    let s12_low = (prod2 - (s12_high - s12_prime)) + (prod1 - s12_prime);
    // accurate addition a1 * b1 + a2 * b2 + a3 * b3
    let s123_high = s12_high + prod3;
    let s123_prime = s123_high - prod3;
    let s123_low = (prod3 - (s123_high - s123_prime)) + (s12_high - s123_prime);
    let result = s123_high + (prod1_low + prod2_low + prod3_low + s12_low + s123_low);
    if result.is_nan() { a1 * b1 + a2 * b2 + a3 * b3 } else { result }
}

/// `MathArrays.linearCombination(a1, b1, a2, b2)` (commons-math3) - the two-term form of `linear_combination3`.
fn linear_combination2(a1: f64, b1: f64, a2: f64, b2: f64) -> f64 {
    let prod1 = a1 * b1;
    let prod1_low = a1.mul_add(b1, -prod1);
    let prod2 = a2 * b2;
    let prod2_low = a2.mul_add(b2, -prod2);
    let s12_high = prod1 + prod2;
    let s12_prime = s12_high - prod2;
    let s12_low = (prod2 - (s12_high - s12_prime)) + (prod1 - s12_prime);
    let result = s12_high + (prod1_low + prod2_low + s12_low);
    if result.is_nan() { prod1 + prod2 } else { result }
}

/// `org.apache.commons.math3.geometry.euclidean.threed.Vector3D` (commons-math3, unported - `crate::unresolved`): the
/// part this class uses. rust: belongs in its own file with the library's other classes (as batch D's 2D `Vector2D` /
/// `Line` in `waypoint/mythological_ritual.rs`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    /// `new Vector3D(double x, double y, double z)`
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    /// `new Vector3D(double a, Vector3D u)` - `a * u`
    fn scaled(a: f64, u: &Vector3D) -> Vector3D {
        Vector3D::new(a * u.x, a * u.y, a * u.z)
    }

    /// `new Vector3D(double a1, Vector3D u1, double a2, Vector3D u2)` - `a1 * u1 + a2 * u2`
    fn linear(a1: f64, u1: &Vector3D, a2: f64, u2: &Vector3D) -> Vector3D {
        Vector3D::new(linear_combination2(a1, u1.x, a2, u2.x), linear_combination2(a1, u1.y, a2, u2.y), linear_combination2(a1, u1.z, a2, u2.z))
    }

    /// `getX()`
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// `getY()`
    pub fn get_y(&self) -> f64 {
        self.y
    }

    /// `getZ()`
    pub fn get_z(&self) -> f64 {
        self.z
    }

    /// `add(Vector<Euclidean3D> v)`
    pub fn add(&self, v: &Vector3D) -> Vector3D {
        Vector3D::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }

    /// `subtract(Vector<Euclidean3D> v)`
    pub fn subtract(&self, v: &Vector3D) -> Vector3D {
        Vector3D::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    /// `getNormSq()`
    pub fn get_norm_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// `dotProduct(Vector<Euclidean3D> v)`
    pub fn dot_product(&self, v: &Vector3D) -> f64 {
        linear_combination3(self.x, v.x, self.y, v.y, self.z, v.z)
    }

    /// `distance(Vector<Euclidean3D> v)`
    pub fn distance(&self, v: &Vector3D) -> f64 {
        let dx = v.x - self.x;
        let dy = v.y - self.y;
        let dz = v.z - self.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// `org.apache.commons.math3.geometry.euclidean.threed.Line` (commons-math3, unported): `Line(p1, p2, tolerance)`
/// (`reset(p1, p2)`) and `closestPoint(Line)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    direction: Vector3D,
    zero: Vector3D,
    tolerance: f64,
}

impl Line {
    /// `public Line(Vector3D p1, Vector3D p2, double tolerance)` - `reset(p1, p2)`; a zero-length line throws
    /// `MathIllegalArgumentException(ZERO_NORM)`.
    pub fn new(p1: &Vector3D, p2: &Vector3D, tolerance: f64) -> Line {
        let delta = p2.subtract(p1);
        let norm2 = delta.get_norm_sq();
        if norm2 == 0.0 {
            panic!("MathIllegalArgumentException: zero norm");
        }
        let direction = Vector3D::scaled(1.0 / norm2.sqrt(), &delta);
        let zero = Vector3D::linear(1.0, p1, -p1.dot_product(&delta) / norm2, &delta);
        Line { direction, zero, tolerance }
    }

    /// `closestPoint(Line line)`: the point of this line closest to `line` (`zero` for parallel lines).
    pub fn closest_point(&self, line: &Line) -> Vector3D {
        let cos = self.direction.dot_product(&line.direction);
        let n = 1.0 - cos * cos;
        // `Precision.EPSILON` = 2^-53
        if n < f64::EPSILON / 2.0 {
            // the lines are parallel
            return self.zero;
        }
        let delta0 = line.zero.subtract(&self.zero);
        let a = delta0.dot_product(&self.direction);
        let b = delta0.dot_product(&line.direction);
        Vector3D::linear(1.0, &self.zero, (a - b * cos) / n, &self.direction)
    }
}
