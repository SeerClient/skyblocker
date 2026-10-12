//! Ported from de.hysky.skyblocker.skyblock.teleport.PredictiveSmoothAOTE by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_boss::DungeonBoss;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::entity::mob_glow::MobGlow;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::EstimatedResource;
use crate::de::hysky::skyblocker::skyblock::status_bar_tracker::StatusBarTracker;
use crate::de::hysky::skyblocker::utils::area::CrimsonIsle;
use crate::de::hysky::skyblocker::utils::area::CrystalHollows;
use crate::de::hysky::skyblocker::utils::area::PrivateIsland;
use crate::de::hysky::skyblocker::utils::item_ability::ItemAbility;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::client::camera_type::CameraType;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::client::options;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::entity::projectile::projectile_util;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::world::level::block::block::Block;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::clip_context::ClipContext;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::entity_hit_result::EntityHitResult;
use mcrust::net::minecraft::world::phys::shapes::voxel_shape::VoxelShape;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;

/// java: public static final Identifier SMOOTH_AOTE_BEFORE_PHASE = SkyblockerMod.id("smooth_aote");
pub static SMOOTH_AOTE_BEFORE_PHASE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("smooth_aote"));
/// java: private static final long MAX_TELEPORT_TIME = 2500;
pub const MAX_TELEPORT_TIME: i64 = ((2500) as i64) as i64;
/// java: private static long startTime;
pub static START_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: @Nullable private static Vec3 startPos;
pub static START_POS: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);
/// java: @Nullable private static Vec3 cameraStartPos;
pub static CAMERA_START_POS: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);
/// java: @Nullable private static Vec3 teleportVector;
pub static TELEPORT_VECTOR: compat::MainThread<Option<Vec3>> = compat::MainThread::new(|| None);
/// java: private static long lastPing;
pub static LAST_PING: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static long currentTeleportPing;
pub static CURRENT_TELEPORT_PING: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: private static int teleportsAhead;
pub static TELEPORTS_AHEAD: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static long lastTeleportTime;
pub static LAST_TELEPORT_TIME: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: public static boolean teleportDisabled;
pub static TELEPORT_DISABLED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class PredictiveSmoothAOTE
#[derive(Clone)]
pub struct PredictiveSmoothAOTE {}

impl PredictiveSmoothAOTE {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): UseItemCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseItemCallback.EVENT.register(SMOOTH_AOTE_BEFORE_PHASE, PredictiveSmoothAOTE::onItemInteract);
        // TODO(fabric2seer): static field UseItemCallback.EVENT
        //   java: UseItemCallback.EVENT.addPhaseOrdering(SMOOTH_AOTE_BEFORE_PHASE, Event.DEFAULT_PHASE);
        // TODO(fabric2seer): UseBlockCallback.EVENT.register: no SeerClient equivalent yet
        //   java: UseBlockCallback.EVENT.register(PredictiveSmoothAOTE::onBlockInteract);
    }

    /// java: public static void playerTeleported()
    pub fn player_teleported() {
        compat::set_state(&TELEPORTS_AHEAD, 0i32.max((compat::get_state(&TELEPORTS_AHEAD)).wrapping_sub(1)));
        compat::set_state(&LAST_TELEPORT_TIME, compat::millis());
        compat::set_state(&TELEPORT_DISABLED, false);
        if compat::get_state(&TELEPORTS_AHEAD) == 0 {
            let mut time_left: i64 = (compat::get_state(&CURRENT_TELEPORT_PING)).wrapping_sub((compat::millis()).wrapping_sub(compat::get_state(&START_TIME)));
            if time_left > (0 as i64) && time_left <= (SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.maximum_added_lag as i64) {
                return;
            }
            *START_POS.get_mut() = None;
            *TELEPORT_VECTOR.get_mut() = None;
        }
    }

    /// java: private static int extractTunedCustomData(CompoundTag customData, int baseRange)
    pub fn extract_tuned_custom_data(custom_data: &Tag, mut base_range: i32) -> i32 {
        return (if true /* java: custom_data != null: never null here */ && custom_data.contains("tuned_transmission") { base_range.wrapping_add(custom_data.get_int_or("tuned_transmission", 0)) } else { base_range });
    }

    /// java: private static InteractionResult onItemInteract(Player playerEntity, Level world, InteractionHand hand)
    pub fn on_item_interact(player_entity: &dyn Player, world: &Level, hand: &InteractionHand) -> Option<InteractionResult> {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return None;
        }
        PredictiveSmoothAOTE::calculate_teleport_use(hand);
        return Some(InteractionResult::Pass);
    }

    /// java: private static InteractionResult onBlockInteract(Player playerEntity, Level world, InteractionHand hand, BlockHitResult blockHitResult)
    pub fn on_block_interact(player_entity: &dyn Player, world: &Level, hand: &InteractionHand, block_hit_result: &BlockHitResult) -> InteractionResult {
        let item_stack = player_entity.get_item_in_hand(*hand);
        if PredictiveSmoothAOTE::is_shovel(item_stack) && PredictiveSmoothAOTE::can_shovel_act_on_block(&world.get_block_state(&block_hit_result.get_block_pos()).get_block()) {
            PredictiveSmoothAOTE::calculate_teleport_use(hand);
        }
        return InteractionResult::Pass;
    }

    /// java: private static boolean isShovel(ItemStack itemStack)
    pub fn is_shovel(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        return (item_stack.is_item(&items::items().get("wooden_shovel")) || item_stack.is_item(&items::items().get("stone_shovel")) || item_stack.is_item(&items::items().get("iron_shovel")) || item_stack.is_item(&items::items().get("golden_shovel")) || item_stack.is_item(&items::items().get("diamond_shovel")));
    }

    /// java: private static boolean canShovelActOnBlock(Block block)
    pub fn can_shovel_act_on_block(block: &Arc<Block>) -> bool {
        return (std::ptr::addr_eq(std::sync::Arc::as_ptr(block), std::sync::Arc::as_ptr(&blocks::blocks().get("grass_block"))) || std::ptr::addr_eq(std::sync::Arc::as_ptr(block), std::sync::Arc::as_ptr(&blocks::blocks().get("dirt"))) || std::ptr::addr_eq(std::sync::Arc::as_ptr(block), std::sync::Arc::as_ptr(&blocks::blocks().get("coarse_dirt"))) || std::ptr::addr_eq(std::sync::Arc::as_ptr(block), std::sync::Arc::as_ptr(&blocks::blocks().get("podzol"))));
    }

    /// java: private static void calculateTeleportUse(InteractionHand hand)
    pub fn calculate_teleport_use(hand: &InteractionHand) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        if !(SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.predictive) {
            return;
        }
        if compat::get_state(&TELEPORT_DISABLED) {
            return;
        }
        if options::get_camera_type() != CameraType::FirstPerson && !(SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.third_person) {
            return;
        }
        if !(PredictiveSmoothAOTE::is_allowed_location()) {
            return;
        }
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
        let mut item_id: String = held_item.get_skyblock_id();
        let mut custom_data: Tag = ItemUtils::get_custom_data(&held_item);
        let mut distance: i32 = PredictiveSmoothAOTE::get_item_distance(&item_id, &custom_data);
        if distance == -1 {
            return;
        }
        let mut abilities: Vec<ItemAbility> = held_item.skyblocker_get_abilities();
        if !abilities.is_empty() && abilities[0].mana_cost().is_some() {
            let mut mana_cost: i32 = abilities[0].mana_cost().unwrap();
            let mut predicted_mana: i32 = EstimatedResource::value(&StatusBarTracker::get_mana());
            if predicted_mana < mana_cost {
                return;
            }
        }
        if compat::get_state(&TELEPORTS_AHEAD) == 0 || START_POS.get_mut().is_none() || TELEPORT_VECTOR.get_mut().is_none() {
            *START_POS.get_mut() = mcrust::mcrust::modding::game::with_player(|player| player.position().add((0) as f64, (Utils::get_eye_height(player)) as f64, (0) as f64));
            *CAMERA_START_POS.get_mut() = Some(RenderHelper::get_camera().position());
            compat::set_state(&LAST_TELEPORT_TIME, compat::millis());
            compat::set_state(&CURRENT_TELEPORT_PING, compat::get_state(&LAST_PING));
        } else {
            *START_POS.get_mut() = Some(START_POS.get_mut().as_ref().unwrap().add_vec(TELEPORT_VECTOR.get_mut().as_ref().unwrap()));
            *CAMERA_START_POS.get_mut() = PredictiveSmoothAOTE::get_interpolated_pos();
            compat::set_state(&CURRENT_TELEPORT_PING, compat::get_state(&LAST_PING));
        }
        compat::set_state(&START_TIME, compat::millis());
        let mut pitch: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_x_rot()).unwrap();
        let mut yaw: f32 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_y_rot()).unwrap();
        let mut look: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.calculate_view_vector(pitch, yaw)).unwrap();
        if START_POS.get_mut().is_none() {
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|player| PredictiveSmoothAOTE::is_targeting_npc(player, ((4) as f64) as f64, START_POS.get_mut(), &look)).unwrap_or(false) {
            *START_POS.get_mut() = None;
            *TELEPORT_VECTOR.get_mut() = None;
            return;
        }
        *TELEPORT_VECTOR.get_mut() = PredictiveSmoothAOTE::raycast(distance, look.clone(), START_POS.get_mut(), false);
        if TELEPORT_VECTOR.get_mut().is_none() {
            *START_POS.get_mut() = None;
            return;
        }
        let mut predicted_end: Vec3 = START_POS.get_mut().as_ref().unwrap().add_vec(TELEPORT_VECTOR.get_mut().as_ref().unwrap());
        let mut offset_vec: Vec3 = Vec3::new((predicted_end.x - PredictiveSmoothAOTE::round_to_center(predicted_end.x)), (predicted_end.y - ((predicted_end.y.ceil() + (mcrust::mcrust::modding::game::with_player(|player| Utils::get_eye_height(player)).unwrap() as f64)) - (1 as f64))), (predicted_end.z - PredictiveSmoothAOTE::round_to_center(predicted_end.z)));
        *TELEPORT_VECTOR.get_mut() = Some(TELEPORT_VECTOR.get_mut().as_ref().unwrap().subtract_vec(&offset_vec));
        compat::set_state(&TELEPORTS_AHEAD, (compat::get_state(&TELEPORTS_AHEAD)).wrapping_add(1));
    }

    /// java: protected static int getItemDistance(String itemId, CompoundTag customData)
    pub fn get_item_distance(item_id: &str, custom_data: &Tag) -> i32 {
        let mut distance: i32;
        'switch_1: {
            match item_id.as_str() {
                "ASPECT_OF_THE_LEECH_1" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.enable_weird_transmission {
                        distance = 3;
                        break 'switch_1;
                    }
                    return -1;
                }
                "ASPECT_OF_THE_LEECH_2" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.enable_weird_transmission {
                        distance = 4;
                        break 'switch_1;
                    }
                    return -1;
                }
                "ASPECT_OF_THE_END" | "ASPECT_OF_THE_VOID" => {
                    // TODO(fabric2seer): `Options.keyShift` has no Rust field in the port (searched net.minecraft.client.Options and its supertypes)
                    // TODO(fabric2seer): `is_down` takes 1 argument(s) in the port (`name: &str`), Java passes 0
                    if key_mapping::is_down() && custom_data.get_int_or("ethermerge", 0) == 1 {
                        if SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.enable_ether_transmission {
                            distance = PredictiveSmoothAOTE::extract_tuned_custom_data(custom_data, 57);
                            break 'switch_1;
                        }
                    } else if SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.enable_instant_transmission {
                        distance = PredictiveSmoothAOTE::extract_tuned_custom_data(custom_data, 8);
                        break 'switch_1;
                    }
                    return -1;
                }
                "ETHERWARP_CONDUIT" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.enable_ether_transmission {
                        distance = PredictiveSmoothAOTE::extract_tuned_custom_data(custom_data, 57);
                        break 'switch_1;
                    }
                    return -1;
                }
                "SINSEEKER_SCYTHE" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.enable_sinrecall_transmission {
                        distance = PredictiveSmoothAOTE::extract_tuned_custom_data(custom_data, 4);
                        break 'switch_1;
                    }
                    return -1;
                }
                "NECRON_BLADE" | "ASTRAEA" | "HYPERION" | "SCYLLA" | "VALKYRIE" => {
                    if SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.enable_wither_impact {
                        distance = 10;
                        break 'switch_1;
                    }
                    return -1;
                }
                _ => {
                    return -1;
                }
            }
        }
        return distance;
    }

    /// java: private static Boolean isTargetingNPC(Player player, double maxDistance, Vec3 startPos, Vec3 look)
    pub fn is_targeting_npc(player: &dyn Player, mut max_distance: f64, start_pos: &mut Vec3, look: &Vec3) -> bool {
        if false /* java: start_pos == null: never null here */ {
            return false;
        }
        let mut end_pos: Vec3 = start_pos.add_vec(&look.scale(max_distance));
        // TODO(fabric2seer): `Player.level` has no Rust method in the port (looked for `level` in Player and its supertypes)
        // rust: let mut world: Level = player.level();
        let mut world: Level = todo!("fabric2seer: the initialiser of `world` (TODO above)");
        // TODO(fabric2seer): field `COLLIDER` of a value of unknown type
        //   java: ClipContext context = new ClipContext(startPos, endPos, ClipContext.Block.COLLIDER, ClipContext.Fluid.NONE, player);
        let mut context: ClipContext = todo!("fabric2seer: the initialiser of `context` (TODO above)");
        let mut block_hit_distance: f64 = world.clip(&context).get_location().distance_to(start_pos);
        let mut search_box: AABB = player.get_bounding_box().expand_towards_vec(&look.scale(max_distance)).inflate((1) as f64);
        // TODO(fabric2seer): the port's `get_entity_hit_result` ports this Java method with other parameters (level, except, vehicle_of, from, to, bb, matching, max_value for Java's except, from, to, box, matching, max_value)
        // TODO(fabric2seer): `get_entity_hit_result` takes 8 argument(s) in the port (`level: &dyn EntityGetter, except: i32, vehicle_of: &dyn Fn(i32) -> Option<i32>, from: &Vec3, to: &Vec3, bb: &AABB, matching: &dyn Fn(&(dyn Entity + Send + Sync)) -> bool, max_value: f64`), Java passes 6
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Predicate<net.minecraft.world.entity.Entity>`)
        // rust: let mut entity_hit: Option<EntityHitResult> = projectile_util::get_entity_hit_result(player, start_pos.clone(), &end_pos, &search_box, |entity| (!entity.is_spectator() && !std::ptr::eq(&entity, player)), &mth::square_f64(block_hit_distance));
        let mut entity_hit: EntityHitResult = todo!("fabric2seer: the initialiser of `entityHit` (TODO above)");
        if false /* java: entity_hit == null: never null here */ {
            return false;
        }
        let mut entity: Box<dyn Entity> = entity_hit.get_entity();
        let mut armor_stands: Vec<ArmorStand> = MobGlow::get_armor_stands(&entity);
        return armor_stands.iter().cloned().any(|armor_stand| (compat::component_of(armor_stand.get_name()).get_string() == "CLICK"));
    }

    /// java: private static double roundToCenter(double input)
    pub fn round_to_center(mut input: f64) -> f64 {
        return ((compat::java_round_f64((input - 0.5)) as f64) + 0.5);
    }

    /// java: private static boolean isAllowedLocation()
    pub fn is_allowed_location() -> bool {
        if Utils::get_map() == "Mineshaft" {
            return false;
        } else if compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<CrystalHollows>().is_some_and(|__e| *__e == CrystalHollows::JungleTemple)) {
            return false;
        } else if Utils::get_location() == Location::PrivateIsland && !compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<PrivateIsland>().is_some_and(|__e| *__e == PrivateIsland::YourIsland)) {
            return false;
        } else if compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<CrimsonIsle>().is_some_and(|__e| *__e == CrimsonIsle::Dojo)) {
            return false;
        } else if Utils::is_in_dungeons() {
            if DungeonManager::is_in_boss() && DungeonManager::get_boss() == DungeonBoss::Maxor {
                return false;
            }
            if !(DungeonManager::is_current_room_matched()) {
                return true;
            }
            if ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_name()); __f(__r) }).as_deref() == Some(&*("boxes-room")) {
                return false;
            }
            if ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_name()); __f(__r) }).as_deref() == Some(&*("teleport-pad-room")) {
                return false;
            }
            if ({ let __r = &DungeonManager::get_current_room().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_name()); __f(__r) }).unwrap().starts_with("trap") {
                return false;
            }
        }
        return true;
    }

    /// java: protected static Vec3 raycast(int distance, Vec3 direction, Vec3 startPos, boolean isEtherwarp)
    pub fn raycast(mut distance: i32, mut direction: Vec3, start_pos: &mut Vec3, mut is_etherwarp: bool) -> Option<Vec3> {
        if mcrust::mcrust::modding::game::level().is_none() || false /* java: direction == null: never null here */ || false /* java: start_pos == null: never null here */ {
            return None;
        }
        let mut x_diagonal_offset: BlockPos = (if direction.x() > (0 as f64) { BlockPos::new(1, 0, 0) } else { BlockPos::new(-1, 0, 0) });
        let mut z_diagonal_offset: BlockPos = (if direction.z() > (0 as f64) { BlockPos::new(0, 0, 1) } else { BlockPos::new(0, 0, -1) });
        let mut close_floor_y: i32 = i32::MAX;
        {
            let mut offset: f64 = ((0) as f64) as f64;
            while offset <= (distance as f64) {
                'continue_1: {
                    let mut pos: Vec3 = start_pos.add_vec(&direction.scale(offset));
                    let mut check_pos: BlockPos = BlockPos::containing_position(&pos);
                    if !(PredictiveSmoothAOTE::can_teleport_through(&check_pos)) {
                        if !is_etherwarp && offset == (0 as f64) {
                            return None;
                        }
                        if is_etherwarp {
                            return Some((direction.scale((offset - (1 as f64)))).add_vec(&direction));
                        }
                        return Some(direction.scale((offset - (1 as f64))));
                    }
                    if !(PredictiveSmoothAOTE::can_teleport_through(&check_pos.above())) && !is_etherwarp {
                        if offset == (0 as f64) {
                            let mut just_ahead: Vec3 = start_pos.add_vec(&direction.scale(0.2));
                            if (just_ahead.y() - just_ahead.y().floor()) <= 0.495 {
                                break 'continue_1;
                            }
                            return None;
                        }
                        return Some(direction.scale((offset - (1 as f64))));
                    }
                    if offset != (0 as f64) && direction.x() < (0 as f64) && PredictiveSmoothAOTE::is_block_floor(&check_pos.east()) && PredictiveSmoothAOTE::is_block_floor(&BlockPos::containing_position(&pos.subtract_vec(&direction)).offset_vec(z_diagonal_offset.as_vec3i())) {
                        return Some(direction.scale((offset - (1 as f64))));
                    }
                    if offset != (0 as f64) && direction.z() < (0 as f64) && direction.x() < (0 as f64) && PredictiveSmoothAOTE::is_block_floor(&check_pos.south()) && PredictiveSmoothAOTE::is_block_floor(&BlockPos::containing_position(&pos.subtract_vec(&direction)).offset_vec(x_diagonal_offset.as_vec3i())) {
                        return Some(direction.scale((offset - (1 as f64))));
                    }
                    if PredictiveSmoothAOTE::is_block_floor(&check_pos.below()) || PredictiveSmoothAOTE::is_block_floor(&check_pos.below().offset_vec(x_diagonal_offset.as_vec3i())) && PredictiveSmoothAOTE::is_block_floor(&check_pos.below().offset_vec(z_diagonal_offset.as_vec3i())) && (pos.y() - pos.y().floor()) < 0.31 {
                        close_floor_y = check_pos.get_y().wrapping_sub(1);
                    }
                    if close_floor_y == check_pos.get_y() {
                        return Some(direction.scale((offset - (1 as f64))));
                    }
                }
                offset += 1;
            }
        }
        if !(PredictiveSmoothAOTE::is_block_floor(&BlockPos::containing_position(&(start_pos.add_vec(&direction.scale((distance) as f64))).subtract((0) as f64, (1) as f64, (0) as f64)))) {
            return Some((direction.scale((distance) as f64)).subtract((0) as f64, (1) as f64, (0) as f64));
        }
        return Some(direction.scale((distance) as f64));
    }

    /// java: private static Boolean canTeleportThrough(BlockPos blockPos)
    pub fn can_teleport_through(block_pos: &BlockPos) -> bool {
        if mcrust::mcrust::modding::game::level().is_none() {
            return false;
        }
        let mut block_state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(block_pos);
        if block_state.is_air() {
            return true;
        }
        let mut block: Arc<Block> = block_state.get_block();
        let mut shape: VoxelShape = block_state.get_collision_shape(mcrust::mcrust::modding::game::level().unwrap().as_block_getter(), block_pos);
        return (shape.is_empty() || block.as_any().is::<Block>() || block.as_any().is::<Block>() || block.as_any().is::<Block>() || std::ptr::addr_eq(std::sync::Arc::as_ptr(&block), std::sync::Arc::as_ptr(&blocks::blocks().get("snow"))) && block_state.get_value(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::layers()) <= 3);
    }

    /// java: private static Boolean isBlockFloor(BlockPos blockPos)
    pub fn is_block_floor(block_pos: &BlockPos) -> bool {
        if mcrust::mcrust::modding::game::level().is_none() {
            return false;
        }
        let mut block_state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(block_pos);
        let mut shape: VoxelShape = block_state.get_collision_shape(mcrust::mcrust::modding::game::level().unwrap().as_block_getter(), block_pos);
        if shape.is_empty() {
            return false;
        }
        return (shape.bounds().unwrap().max_y >= (1 as f64) || std::ptr::addr_eq(std::sync::Arc::as_ptr(&block_state.get_block()), std::sync::Arc::as_ptr(&blocks::blocks().get("mud"))));
    }

    /// java: @Nullable public static Vec3 getInterpolatedPos()
    pub fn get_interpolated_pos() -> Option<Vec3> {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || TELEPORT_VECTOR.get_mut().is_none() || START_POS.get_mut().is_none() || compat::get_state(&TELEPORT_DISABLED) {
            return None;
        }
        let mut gap: i64 = (compat::millis()).wrapping_sub(compat::get_state(&START_TIME));
        if (compat::millis()).wrapping_sub(compat::get_state(&LAST_TELEPORT_TIME)) > (((compat::get_state(&LAST_PING)).max(compat::get_state(&CURRENT_TELEPORT_PING))).wrapping_add(((SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.maximum_added_lag) as i64).wrapping_mul(compat::get_state(&TELEPORTS_AHEAD) as i64))).min(MAX_TELEPORT_TIME) {
            compat::set_state(&TELEPORT_DISABLED, true);
            *START_POS.get_mut() = None;
            *TELEPORT_VECTOR.get_mut() = None;
            compat::set_state(&TELEPORTS_AHEAD, 0);
            return None;
        }
        let mut estimated_teleport_time: i64 = (compat::get_state(&CURRENT_TELEPORT_PING)).min(MAX_TELEPORT_TIME);
        let mut percentage: f64 = (((gap) as f64) / (estimated_teleport_time as f64)).clamp((0 as f64), (1 as f64));
        if compat::get_state(&TELEPORTS_AHEAD) == 0 && gap >= estimated_teleport_time.wrapping_add(SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.maximum_added_lag as i64) {
            *START_POS.get_mut() = None;
            *TELEPORT_VECTOR.get_mut() = None;
            return None;
        }
        return Some(CAMERA_START_POS.get_mut().as_ref().unwrap().add_vec(&TELEPORT_VECTOR.get_mut().as_ref().unwrap().scale(percentage)));
    }

    /// java: @Nullable public static Vec3 getInterpolatedPlayerPos()
    pub fn get_interpolated_player_pos() -> Option<Vec3> {
        if START_POS.get_mut().is_none() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || CAMERA_START_POS.get_mut().is_none() {
            return None;
        }
        let mut diff: Vec3 = (START_POS.get_mut().as_ref().unwrap().subtract((0) as f64, (mcrust::mcrust::modding::game::with_player(|player| Utils::get_eye_height(player)).unwrap()) as f64, (0) as f64)).subtract_vec(CAMERA_START_POS.get_mut().as_ref().unwrap());
        let Some(mut camara) = PredictiveSmoothAOTE::get_interpolated_pos() else {
            return None;
        };
        return Some(camara.add_vec(&diff));
    }

    /// java: public static void updatePing(long ping)
    pub fn update_ping(mut ping: i64) {
        compat::set_state(&LAST_PING, ping);
    }

}
