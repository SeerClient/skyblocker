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
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::net::minecraft::world::entity::living_entity::LivingEntity as _;
use mcrust::net::minecraft::world::level::entity_getter::EntityGetter as _;

/// java: public static @Nullable Entity highlighted = null;
pub static HIGHLIGHTED: compat::MainThread<Option<Box<dyn Entity>>> = compat::MainThread::new(|| None);

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
        // TODO(fabric2seer): `DeltaTracker.getGameTimeDeltaPartialTick` has no Rust method in the port (looked for `get_game_time_delta_partial_tick` in DeltaTracker and its supertypes)
        // rust: let mut tick_delta: f32 = RenderHelper::get_tick_counter().get_game_time_delta_partial_tick(false);
        let mut tick_delta: f32 = todo!("fabric2seer: the initialiser of `tickDelta` (TODO above)");
        let mut start: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_eye_position_partial(tick_delta)).unwrap();
        let mut look: Vec3 = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_view_vector(tick_delta)).unwrap();
        let mut hit_result: HitResult = CritterCapsuleHelper::throw_line(collector, &mut start, &look).unwrap();
        if let Some(block) = hit_result.as_any().downcast_ref::<BlockHitResult>() {
            // TODO(fabric2seer): `.getComponents(..)` on a Color
            //   java: collector.submitFilledBox(block.getBlockPos(), SkyblockerConfigManager.get().hunting.safari.critterCapsuleHelperColor.getComponents(new float[4]), 0.5f, false);
        }
        if let Some(entity) = hit_result.as_any().downcast_ref::<EntityHitResult>() {
            *HIGHLIGHTED.get_mut() = Some(entity.get_entity());
        } else {
            *HIGHLIGHTED.get_mut() = None;
        }
    }

    /// java: private static @Nullable HitResult throwLine(PrimitiveCollector collector, Vec3 start, Vec3 look)
    pub fn throw_line(collector: &compat::Shared<dyn PrimitiveCollector>, start: &mut Vec3, look: &Vec3) -> Option<HitResult> {
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
                // TODO(fabric2seer): `.getComponents(..)` on a Color
                //   java: collector.submitLinesFromPoints(new Vec3[]{lastPos.add(offset), pos.add(offset)}, SkyblockerConfigManager.get().hunting.safari.critterCapsuleHelperColor.getComponents(new float[4]), 0.5f, 1 + 4 * (max_distance - i) / ((float) max_distance), false);
            }
            let mut block: BlockPos = BlockPos::containing_position(&pos);
            let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&block);
            if !state.is_air() && !(state.is_block(&blocks::blocks().get("water"))) {
                return Some(BlockHitResult::new(pos.clone(), Direction::Down, block.clone(), true));
            }
            let mut detection_box: AABB = (AABB::of_corners(&last_pos, &pos)).inflate(0.05);
            // TODO(fabric2seer): `get_entities` takes 3 argument(s) in the port (`except: Option<i32>, bb: &crate::net::minecraft::world::phys::aabb::AABB, selector: &dyn Fn( &(dyn crate::net::minecraft::world::entity::entity::Entity + Send + Sync), ) -> bool`), Java passes 2
            for entity in mcrust::mcrust::modding::game::level().unwrap().get_entities(None, &detection_box) {
                // TODO(fabric2seer): the condition: instanceof an interface (`Player`): Rust cannot test for a trait at run time
                //   java: entity instanceof Player
                if todo!("fabric2seer: the condition (TODO above)") {
                    continue;
                }
                // TODO(fabric2seer): the port's `new` ports this Java method with other parameters (entity_id, entity_position, location for Java's entity, location)
                // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`entity_id: i32, entity_position: Vec3, location: Vec3`), Java passes 2
                // rust: return Some(EntityHitResult::new(entity.clone(), pos.clone()));
                return todo!("fabric2seer: see the TODO above");
            }
            last_pos = pos.clone();
        }
        return None;
    }

}
