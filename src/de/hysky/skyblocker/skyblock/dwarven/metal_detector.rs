//! Ported from de.hysky.skyblocker.skyblock.dwarven.MetalDetector by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::area::CrystalHollows;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::vec3i::Vec3i;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;

/// java: private static final float[] LIGHT_GRAY = {192 / 255f, 192 / 255f, 192 / 255f};
pub static LIGHT_GRAY: LazyLock<Vec<f32>> = LazyLock::new(|| vec![((192 as f32) / 255.0f32), ((192 as f32) / 255.0f32), ((192 as f32) / 255.0f32)]);
/// java: private static final Pattern TREASURE_PATTERN = Pattern.compile("(§3§lTREASURE: §b)(\\d+\\.?\\d?)m");
pub static TREASURE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(§3§lTREASURE: §b)(\\d+\\.?\\d?)m"));
/// java: private static final Pattern KEEPER_PATTERN = Pattern.compile("Keeper of (\\w+)");
pub static KEEPER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("Keeper of (\\w+)"));
/// java: private static final Map<String, Vec3i> keeperOffsets = Map.of( "Diamond", new Vec3i(33, 0, 3), "Lapis", new Vec3i(-33, 0, -3), "Emerald", new Vec3i(-3, 0, 33), "Gold", new Vec3i(3, 0, -33) );
pub static KEEPER_OFFSETS: compat::MainThread<std::collections::HashMap<String, Vec3i>> = compat::MainThread::new(|| std::collections::HashMap::from([("Diamond".to_string(), Vec3i::new(33, 0, 3)), ("Lapis".to_string(), Vec3i::new(-33, 0, -3)), ("Emerald".to_string(), Vec3i::new(-3, 0, 33)), ("Gold".to_string(), Vec3i::new(3, 0, -33))]));
/// java: private static final Set<Vec3i> knownChestOffsets = Set.of( new Vec3i(-38, -22, 26), // -38, -22, 26 new Vec3i(38, -22, -26), // 38, -22, -26 new Vec3i(-40, -22, 18), // -40, -22, 18 new Vec3i(-41, -2
pub static KNOWN_CHEST_OFFSETS: compat::MainThread<std::collections::HashSet<Vec3i>> = compat::MainThread::new(|| std::collections::HashSet::from([Vec3i::new(-38, -22, 26), Vec3i::new(38, -22, -26), Vec3i::new(-40, -22, 18), Vec3i::new(-41, -20, 22), Vec3i::new(-5, -21, 16), Vec3i::new(40, -22, -30), Vec3i::new(-42, -20, -28), Vec3i::new(-43, -22, -40), Vec3i::new(42, -19, -41), Vec3i::new(43, -21, -16), Vec3i::new(-1, -22, -20), Vec3i::new(6, -21, 28), Vec3i::new(7, -21, 11), Vec3i::new(7, -21, 22), Vec3i::new(-12, -21, -44), Vec3i::new(12, -22, 31), Vec3i::new(12, -22, -22), Vec3i::new(12, -21, 7), Vec3i::new(12, -21, -43), Vec3i::new(-14, -21, 43), Vec3i::new(-14, -21, 22), Vec3i::new(-17, -21, 20), Vec3i::new(-20, -22, 0), Vec3i::new(1, -21, 20), Vec3i::new(19, -22, 29), Vec3i::new(20, -22, 0), Vec3i::new(20, -21, -26), Vec3i::new(-23, -22, 40), Vec3i::new(22, -21, -14), Vec3i::new(-24, -22, 12), Vec3i::new(23, -22, 26), Vec3i::new(23, -22, -39), Vec3i::new(24, -22, 27), Vec3i::new(25, -22, 17), Vec3i::new(29, -21, -44), Vec3i::new(-31, -21, -12), Vec3i::new(-31, -21, -40), Vec3i::new(30, -21, -25), Vec3i::new(-32, -21, -40), Vec3i::new(-36, -20, 42), Vec3i::new(-37, -21, -14), Vec3i::new(-37, -21, -22)]));
/// java: protected static Vec3i minesCenter = null;
pub static MINES_CENTER: compat::MainThread<Option<Vec3i>> = compat::MainThread::new(|| None);
/// java: private static double previousDistance;
pub static PREVIOUS_DISTANCE: LazyLock<Mutex<f64>> = LazyLock::new(|| Mutex::new(0.0));
/// java: private static Vec3 previousPlayerPos;
pub static PREVIOUS_PLAYER_POS: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);
/// java: protected static boolean newTreasure = true;
pub static NEW_TREASURE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));
/// java: private static boolean startedLooking = false;
pub static STARTED_LOOKING: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: protected static List<Vec3i> possibleBlocks = new ArrayList<>();
pub static POSSIBLE_BLOCKS: compat::MainThread<Vec<Vec3i>> = compat::MainThread::new(|| Vec::new());

/// java: public class MetalDetector
#[derive(Clone)]
pub struct MetalDetector {}

impl MetalDetector {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(MetalDetector::getDistanceMessage);
        compat::register_allow_game(|text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent, overlay: bool| MetalDetector::get_distance_message(text, overlay));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { MetalDetector::extract_rendering(&__a0); }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            MetalDetector::reset();
        }));
    }

    /// java: private static boolean getDistanceMessage(Component text, boolean overlay)
    pub fn get_distance_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if !overlay || !(SkyblockerConfigManager::get().mining.crystal_hollows.metal_detector_helper) || !(Utils::is_in_crystal_hollows()) || !compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<CrystalHollows>().is_some_and(|__e| *__e == CrystalHollows::MinesOfDivan)) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            MetalDetector::check_chest_found(text);
            return true;
        }
        let mut treasure_distance_mature: compat::regex::Matcher = TREASURE_PATTERN.matcher(&text.get_string());
        if !treasure_distance_mature.find() {
            return true;
        }
        let mut distance: f64 = treasure_distance_mature.group(2).unwrap().trim().parse::<f64>().expect("NumberFormatException");
        let mut player_pos: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap();
        let mut previous_possible_block_count: i32 = (POSSIBLE_BLOCKS.get_mut().len() as i32);
        if !(compat::get_state(&STARTED_LOOKING)) {
            compat::set_state(&STARTED_LOOKING, true);
            mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.dwarvenMines.metalDetectorHelper.startTip"))); });
        }
        if MINES_CENTER.get_mut().is_none() {
            MetalDetector::find_center_of_mines();
        }
        if distance == compat::get_state(&PREVIOUS_DISTANCE) && player_pos == PREVIOUS_PLAYER_POS.get_mut().clone().unwrap() {
            MetalDetector::update_possible_blocks(distance, &player_pos);
        }
        if (POSSIBLE_BLOCKS.get_mut().len() as i32) != previous_possible_block_count {
            if (POSSIBLE_BLOCKS.get_mut().len() as i32) == 1 {
                mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.dwarvenMines.metalDetectorHelper.foundTreasureMessage").with_style_format(ChatFormatting::Green))); });
            } else {
                mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.dwarvenMines.metalDetectorHelper.possibleTreasureLocationsMessage").append(component::null_to_empty(Some(&(POSSIBLE_BLOCKS.get_mut().len() as i32).to_string()))))); });
            }
        }
        compat::set_state(&PREVIOUS_DISTANCE, distance);
        *PREVIOUS_PLAYER_POS.get_mut() = Some(player_pos.clone());
        return true;
    }

    /// java: private static void checkChestFound(Component text)
    pub fn check_chest_found(text: &MutableComponent) {
        if !(Utils::is_in_crystal_hollows()) || !compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<CrystalHollows>().is_some_and(|__e| *__e == CrystalHollows::MinesOfDivan)) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        if text.get_string().starts_with("You found") {
            compat::set_state(&NEW_TREASURE, true);
            *POSSIBLE_BLOCKS.get_mut() = Vec::new();
        }
    }

    /// java: protected static void updatePossibleBlocks(double distance, Vec3 playerPos)
    pub fn update_possible_blocks(mut distance: f64, player_pos: &Vec3) {
        if compat::get_state(&NEW_TREASURE) {
            *POSSIBLE_BLOCKS.get_mut() = Vec::new();
            compat::set_state(&NEW_TREASURE, false);
            if MINES_CENTER.get_mut().is_some() {
                for known_offset in KNOWN_CHEST_OFFSETS.get_mut().iter() {
                    let mut check_pos: Vec3i = MINES_CENTER.get_mut().as_ref().unwrap().offset_vec(known_offset).offset(0, 1, 0);
                    if (player_pos.distance_to(&at_lower_corner_of(&check_pos)) - distance).abs() < 0.25 {
                        POSSIBLE_BLOCKS.get_mut().push(check_pos.clone());
                    }
                }
            } else {
                for x in (((-distance)) as i32)..=(distance) as i32 {
                    for z in (((-distance)) as i32)..=(distance) as i32 {
                        let mut check_pos: Vec3i = Vec3i::new(((player_pos.x) as i32).wrapping_add(x), ((player_pos.y) as i32), ((player_pos.z) as i32).wrapping_add(z));
                        if (player_pos.distance_to(&at_lower_corner_of(&check_pos)) - distance).abs() < 0.25 {
                            POSSIBLE_BLOCKS.get_mut().push(check_pos.clone());
                        }
                    }
                }
            }
        } else {
            POSSIBLE_BLOCKS.get_mut().retain(|location| !(((player_pos.distance_to(&at_lower_corner_of(location)) - distance).abs() >= 0.25)));
        }
        if POSSIBLE_BLOCKS.get_mut().is_empty() {
            compat::set_state(&NEW_TREASURE, true);
            if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.dwarvenMines.metalDetectorHelper.somethingWentWrongMessage").with_style_format(ChatFormatting::Red))); });
            }
        }
    }

    /// java: private static void findCenterOfMines()
    pub fn find_center_of_mines() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let mut search_box: AABB = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_bounding_box().inflate(500.0)).unwrap();
        // `CLIENT.level.getEntitiesOfClass(ArmorStand.class, searchBox, ArmorStand::hasCustomName)`: the level's entities
        // in the box that are armor stands with a custom name, each read through the game's tracked lock.
        use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;
        let armor_stands = mcrust::mcrust::modding::game::level().unwrap().get_entities(None, &search_box, &|entity| {
            entity.as_any().and_then(|any| any.downcast_ref::<ArmorStand>()).is_some_and(|armor_stand| armor_stand.base.base.has_custom_name())
        });
        for armor_stand in armor_stands.iter() {
            let Some((name, armor_stand_position)) = compat::with_entity_handle(armor_stand, |armor_stand| (compat::component_of(armor_stand.get_name()).get_string(), armor_stand.base().block_position())) else { continue; };
            let mut name_matcher: compat::regex::Matcher = KEEPER_PATTERN.matcher(&name);
            if name_matcher.matches() {
                let mut offset: Vec3i = KEEPER_OFFSETS.get_mut().get(name_matcher.group(1).as_ref().unwrap()).cloned().unwrap();
                *MINES_CENTER.get_mut() = Some(*armor_stand_position.offset_vec(&offset).as_vec3i());
                mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.dwarvenMines.metalDetectorHelper.foundCenter").with_style_format(ChatFormatting::Green))); });
                return;
            }
        }
    }

    /// java: private static void reset()
    pub fn reset() {
        *MINES_CENTER.get_mut() = None;
        *POSSIBLE_BLOCKS.get_mut() = Vec::new();
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().mining.crystal_hollows.metal_detector_helper) || !(Utils::is_in_crystal_hollows()) || POSSIBLE_BLOCKS.get_mut().is_empty() || (POSSIBLE_BLOCKS.get_mut().len() as i32) > 8 || !compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<CrystalHollows>().is_some_and(|__e| *__e == CrystalHollows::MinesOfDivan)) {
            return;
        }
        if (POSSIBLE_BLOCKS.get_mut().len() as i32) == 1 {
            let mut block: Vec3i = POSSIBLE_BLOCKS.get_mut()[0].offset(0, -1, 0);
            let mut waypoint: compat::Shared<dyn NamedWaypoint> = NamedWaypointBase::new_3(BlockPos::new(block.get_x(), block.get_y(), block.get_z()), component::translatable("skyblocker.dwarvenMines.metalDetectorHelper.treasure").get_string(), mcrust::java::awt::color::yellow().get_color_components(None).to_vec()) as compat::Shared<dyn NamedWaypoint>;
            ({ let __f = compat::dispatch(&waypoint, |__t| __t.extract_rendering()); __f(&compat::view(&waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_line_from_cursor()); __f(collector, &vec3::at_center_of(&BlockPos::new(block.get_x(), block.get_y(), block.get_z())), &LIGHT_GRAY, 1.0f32, 5.0f32) });
            return;
        }
        for block in POSSIBLE_BLOCKS.get_mut().iter() {
            let mut waypoint: compat::Shared<dyn NamedWaypoint> = NamedWaypointBase::new_3(BlockPos::new(block.get_x(), block.get_y(), block.get_z()), component::translatable("skyblocker.dwarvenMines.metalDetectorHelper.possible").get_string(), mcrust::java::awt::color::white().get_color_components(None).to_vec()) as compat::Shared<dyn NamedWaypoint>;
            ({ let __f = compat::dispatch(&waypoint, |__t| __t.extract_rendering()); __f(&compat::view(&waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
        }
    }

}

/// `Vec3.atLowerCornerOf(Vec3i)`: the port's takes the `BlockPos` (Java's parameter is the `Vec3i` it extends).
fn at_lower_corner_of(pos: &Vec3i) -> Vec3 {
    vec3::at_lower_corner_of(&BlockPos::new(pos.get_x(), pos.get_y(), pos.get_z()))
}
