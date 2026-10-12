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
        // TODO(fabric2seer): UseItemCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseItemCallback.EVENT.register(WishingCompassSolver::onItemInteract);
        // TODO(fabric2seer): UseBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseBlockCallback.EVENT.register(WishingCompassSolver::onBlockInteract);
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(WishingCompassSolver::failMessageListener);
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
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.enableTabEffectsMessage")).to_string()); });
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
        // TODO(fabric2seer): static field Map.Entry
        //   java: return ZONE_BOUNDING_BOXES.entrySet().stream().filter(zone -> zone.getValue().contains(location)).findFirst().map(Map.Entry::getKey).orElse(Zone.CRYSTAL_NUCLEUS);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static Boolean isZoneComplete(Zone zone)
    pub fn is_zone_complete(mut zone: Zone) -> bool {
        if minecraft::get_connection(compat::minecraft()).is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        if !PlayerListManager::get_player_string_list().into_iter().any(|entry| (entry == "Crystals:")) {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.enableTabMessage")).to_string()); });
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
        if !(Utils::is_in_crystal_hollows()) || !(mcrust::net::minecraft::core::particles::particle_types::happy_villager() == packet.get_particle().unwrap().get_type()) {
            return;
        }
        let mut particle_pos: Vec3 = Vec3::new(packet.get_x(), packet.get_y(), packet.get_z());
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
                        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.wishingCompassUsedMessage").with_style_format(ChatFormatting::Green)).to_string()); });
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
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.somethingWentWrongMessage").with_style_format(ChatFormatting::Red)).to_string()); });
        } else {
            let mut player_zone: Zone = WishingCompassSolver::get_zone_of_location(START_POS_ONE.get_mut());
            let mut location: CrystalHollowsLocationsCategory = WishingCompassSolver::get_target_location(player_zone);
            if !(WishingCompassSolver::verify_location(player_zone, target_location.as_ref().unwrap())) {
                location = CrystalHollowsLocationsCategory::Unknown;
                mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.targetLocationToFar").with_style_format(ChatFormatting::Red)).to_string()); });
            }
            if location == CrystalHollowsLocationsCategory::JungleTemple {
                target_location = Some(target_location.as_ref().unwrap().add_vec(JUNGLE_TEMPLE_DOOR_OFFSET.get_mut()));
            }
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.foundMessage").with_style_format(ChatFormatting::Green)).append(component::literal(&location.get_name()).with_color(location.get_color())).append(component::literal(&format!(": {} {} {}", ((target_location.as_ref().unwrap().x()) as i32), ((target_location.as_ref().unwrap().y()) as i32), ((target_location.as_ref().unwrap().z()) as i32)))).to_string()); });
            CrystalsLocationsManager::add_custom_waypoint(location.get_name(), &BlockPos::containing_position(target_location.as_ref().unwrap()));
        }
        WishingCompassSolver::reset();
    }

    /// java: protected static Vec3 solve(Vec3 startPosOne, Vec3 startPosTwo, Vec3 directionOne, Vec3 directionTwo)
    pub fn solve(start_pos_one: &Vec3, start_pos_two: &Vec3, direction_one: &Vec3, direction_two: &Vec3) -> Option<Vec3> {
        if direction_one == direction_two {
            return None;
        }
        // TODO(fabric2seer): `new Vector3D(..)`
        //   java: Vector3D lineOneStart = new Vector3D(startPosOne.x, startPosOne.y, startPosOne.z);
        let mut line_one_start: crate::unresolved::Vector3D = todo!("fabric2seer: the initialiser of `lineOneStart` (TODO above)");
        // TODO(fabric2seer): `new Vector3D(..)`
        //   java: Vector3D lineOneEnd = new Vector3D(directionOne.x, directionOne.y, directionOne.z).add(lineOneStart);
        let mut line_one_end: crate::unresolved::Vector3D = todo!("fabric2seer: the initialiser of `lineOneEnd` (TODO above)");
        // TODO(fabric2seer): `new Vector3D(..)`
        //   java: Vector3D lineTwoStart = new Vector3D(startPosTwo.x, startPosTwo.y, startPosTwo.z);
        let mut line_two_start: crate::unresolved::Vector3D = todo!("fabric2seer: the initialiser of `lineTwoStart` (TODO above)");
        // TODO(fabric2seer): `new Vector3D(..)`
        //   java: Vector3D lineTwoEnd = new Vector3D(directionTwo.x, directionTwo.y, directionTwo.z).add(lineTwoStart);
        let mut line_two_end: crate::unresolved::Vector3D = todo!("fabric2seer: the initialiser of `lineTwoEnd` (TODO above)");
        // TODO(fabric2seer): `new Line(..)`
        //   java: Line line = new Line(lineOneStart, lineOneEnd, 1);
        let mut line: crate::unresolved::Line = todo!("fabric2seer: the initialiser of `line` (TODO above)");
        // TODO(fabric2seer): `new Line(..)`
        //   java: Line lineTwo = new Line(lineTwoStart, lineTwoEnd, 1);
        let mut line_two: crate::unresolved::Line = todo!("fabric2seer: the initialiser of `lineTwo` (TODO above)");
        // TODO(fabric2seer): `.closestPoint(..)` on a Line
        //   java: Vector3D close = line.closestPoint(lineTwo);
        let mut close: crate::unresolved::Vector3D = todo!("fabric2seer: the initialiser of `close` (TODO above)");
        // TODO(fabric2seer): `.closestPoint(..)` on a Line
        //   java: Vector3D closeTwo = lineTwo.closestPoint(line);
        let mut close_two: crate::unresolved::Vector3D = todo!("fabric2seer: the initialiser of `closeTwo` (TODO above)");
        // TODO(fabric2seer): `.distance(..)` on a Vector3D
        //   java: double distance = close.distance(closeTwo);
        let mut distance: f64 = todo!("fabric2seer: the initialiser of `distance` (TODO above)");
        let mut intersection: Option<Vec3> = None;
        if distance < DISTANCE_TOLERANCE {
            // TODO(fabric2seer): `.getX(..)` on a Vector3D
            //   java: Vec3 c1 = new Vec3(close.getX(), close.getY(), close.getZ());
            let mut c1: Vec3 = todo!("fabric2seer: the initialiser of `c1` (TODO above)");
            // TODO(fabric2seer): `.getX(..)` on a Vector3D
            //   java: Vec3 c2 = new Vec3(close.getX(), close.getY(), close.getZ());
            let mut c2: Vec3 = todo!("fabric2seer: the initialiser of `c2` (TODO above)");
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
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.useOutsideNucleusMessage")).to_string()); });
                    return true;
                }
                WishingCompassSolver::start_new_state(SolverStates::ProcessingFirstUse);
            }
            SolverStates::WaitingForSecond => {
                if START_POS_ONE.get_mut().closer_than(&player_pos, (DISTANCE_BETWEEN_USES) as f64) {
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.moveFurtherMessage")).to_string()); });
                    return true;
                } else {
                    if current_zone != WishingCompassSolver::get_zone_of_location(START_POS_ONE.get_mut()) {
                        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.changingZoneMessage")).to_string()); });
                        WishingCompassSolver::reset();
                        WishingCompassSolver::start_new_state(SolverStates::ProcessingFirstUse);
                    } else {
                        WishingCompassSolver::start_new_state(SolverStates::ProcessingSecondUse);
                    }
                }
            }
            SolverStates::ProcessingFirstUse | SolverStates::ProcessingSecondUse => {
                if (compat::millis()).wrapping_sub(compat::get_state(&PARTICLE_LAST_UPDATE)) < PARTICLES_MAX_DELAY {
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.waitLongerMessage").with_style_format(ChatFormatting::Red)).to_string()); });
                    return true;
                } else {
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.mining.crystalsWaypoints.wishingCompassSolver.couldNotDetectLastUseMessage").with_style_format(ChatFormatting::Red)).to_string()); });
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
