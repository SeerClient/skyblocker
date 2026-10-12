//! Ported from de.hysky.skyblocker.skyblock.foraging.torrhus.CritterCapsuleHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::entity_hit_result::EntityHitResult;
use mcrust::net::minecraft::world::phys::hit_result::AnyHitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: public static @Nullable Entity highlighted = null;
/// (the entity's id: the level keeps the entity; Java's `highlighted.equals(target)` is `Some(target id)`)
pub static HIGHLIGHTED: compat::MainThread<Option<i32>> = compat::MainThread::new(|| None);

/// java: public class CritterCapsuleHelper
#[derive(Clone)]
pub struct CritterCapsuleHelper {}

impl CritterCapsuleHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { CritterCapsuleHelper::extract_rendering(&__a0); }));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() || !(SkyblockerConfigManager::get().hunting.safari.critter_capsule_helper) || !(Utils::is_in_safari()) {
            return;
        }
        let mut held_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::mcrust::modding::game::with_player(|player| player.get_main_hand_item().clone()).unwrap();
        let mut item_id: String = held_item.get_skyblock_id();
        if !(item_id == "CRITTER_CAPSULE") && !(item_id == "MASTERFUL_CRITTER_CAPSULE") {
            return;
        }
        let mut tick_delta: f32 = RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false);
        let mut start: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_eye_position_partial(tick_delta)).unwrap();
        let mut look: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_view_vector(tick_delta)).unwrap();
        let mut hit_result: Option<AnyHitResult> = CritterCapsuleHelper::throw_line(collector, &mut start, &look);
        if let Some(AnyHitResult::Block(block)) = &hit_result {
            let __colour = SkyblockerConfigManager::get().hunting.safari.critter_capsule_helper_color.get_components(None).to_vec();
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box()); __f(collector, &block.get_block_pos(), &__colour, 0.5f32, false) });
        }
        if let Some(AnyHitResult::Entity(entity)) = &hit_result {
            *HIGHLIGHTED.get_mut() = Some(entity.entity_id);
        } else {
            *HIGHLIGHTED.get_mut() = None;
        }
    }

    /// java: private static @Nullable HitResult throwLine(PrimitiveCollector collector, Vec3 start, Vec3 look)
    pub fn throw_line(collector: &compat::Shared<dyn PrimitiveCollector>, start: &mut Vec3, look: &Vec3) -> Option<AnyHitResult> {
        if mcrust::mcrust::modding::game::level().is_none() {
            return None;
        }
        let mut max_distance: i32 = 50;
        let mut last_pos: Vec3 = start.clone();
        for i in 0..max_distance {
            let mut pos: Vec3 = start.add_vec(&look.scale((i) as f64));
            let mut accumulatedgravity: f64;
            accumulatedgravity = ((((-0.85614 * ((-0.29696 * 0.093157) * (i as f64)).exp()) * (((0.093157 * ((1 as f64) - (0.29696 * 0.29696)).sqrt()) * (i as f64)) + 3.6507).sin()) - 36.584) + (35.962 * ((-0.027059 * (i as f64)) + 14.087).sin()));
            pos = pos.add((0) as f64, accumulatedgravity, (0) as f64);
            if i > 1 {
                let mut offset: Vec3 = look.cross(&mcrust::net::minecraft::world::phys::vec3::Y_AXIS).normalize().scale(0.05);
                let __colour = SkyblockerConfigManager::get().hunting.safari.critter_capsule_helper_color.get_components(None).to_vec();
                // java: 1 + 4 * (max_distance - i) / ((float) max_distance)
                let __width: f32 = (1 as f32) + (4i32.wrapping_mul(max_distance.wrapping_sub(i)) as f32) / (max_distance as f32);
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_lines_from_points()); __f(collector, &vec![last_pos.add_vec(&offset), pos.add_vec(&offset)], &__colour, 0.5f32, __width, false) });
            }
            let mut block: BlockPos = BlockPos::containing_position(&pos);
            let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&block);
            if !state.is_air() && !(state.is_block(&blocks::blocks().get("water"))) {
                return Some(AnyHitResult::Block(BlockHitResult::new(pos.clone(), Direction::Down, block.clone(), true)));
            }
            let mut detection_box: AABB = (AABB::of_corners(&last_pos, &pos)).inflate(0.05);
            // java: CLIENT.level.getEntities(null, detectionBox) - `EntitySelector.NO_SPECTATORS`
            let __level = mcrust::mcrust::modding::game::level().unwrap();
            for entity in __level.get_entities_no_spectators(None, &detection_box) {
                // java: entity instanceof Player
                let Some((is_player, id, entity_position)) = compat::with_entity_handle(&entity, |entity| (entity.as_living_entity().is_some_and(|living| living.as_player().is_some()), entity.base().get_id(), entity.base().position())) else {
                    continue;
                };
                if is_player {
                    continue;
                }
                // java: return new EntityHitResult(entity, pos);
                return Some(AnyHitResult::Entity(EntityHitResult::new(id, entity_position, pos.clone())));
            }
            last_pos = pos.clone();
        }
        return None;
    }

}
