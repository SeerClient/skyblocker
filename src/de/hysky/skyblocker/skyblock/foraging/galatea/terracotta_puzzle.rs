//! Ported from de.hysky.skyblocker.skyblock.foraging.galatea.TerracottaPuzzle by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::area::Galatea;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::direction::Direction;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::vec3;

/// java: private static final BlockPos WALL_TOP_LEFT = new BlockPos(-633, 66, 85);
pub static WALL_TOP_LEFT: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(-633, 66, 85));
/// java: private static final BlockPos WALL_BOTTOM_RIGHT = new BlockPos(-640, 65, 85);
pub static WALL_BOTTOM_RIGHT: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(-640, 65, 85));
/// java: private static final BlockPos FLOOR_TOP_LEFT = new BlockPos(-633, 59, 76);
pub static FLOOR_TOP_LEFT: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(-633, 59, 76));
/// java: private static final BlockPos FLOOR_BOTTOM_LEFT = new BlockPos(-633, 59, 75);
pub static FLOOR_BOTTOM_LEFT: compat::MainThread<BlockPos> = compat::MainThread::new(|| BlockPos::new(-633, 59, 75));
/// java: private static final Map<Direction, Direction> DIRECTION_MAPPINGS = Map.ofEntries( Map.entry(Direction.NORTH, Direction.WEST), Map.entry(Direction.EAST, Direction.SOUTH), Map.entry(Direction.SOUTH, Di
pub static DIRECTION_MAPPINGS: compat::MainThread<std::collections::HashMap<Direction, Direction>> = compat::MainThread::new(|| std::collections::HashMap::from([(Direction::North, Direction::West), (Direction::East, Direction::South), (Direction::South, Direction::East), (Direction::West, Direction::North)]));

/// java: public class TerracottaPuzzle
#[derive(Clone)]
pub struct TerracottaPuzzle {}

impl TerracottaPuzzle {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { TerracottaPuzzle::extract_rendering(&__a0); }));
    }

    /// java: private static List<Direction> solve()
    pub fn solve() -> Vec<Direction> {
        if mcrust::mcrust::modding::game::level().is_none() {
            return vec![];
        }
        let mut solutions: Vec<Direction> = Vec::new();
        {
            let mut x: i32 = WALL_TOP_LEFT.get_mut().get_x();
            while x >= WALL_BOTTOM_RIGHT.get_mut().get_x() {
                {
                    let mut y: i32 = WALL_TOP_LEFT.get_mut().get_y();
                    while y >= WALL_BOTTOM_RIGHT.get_mut().get_y() {
                        'continue_2: {
                            let mut pos: BlockPos = BlockPos::new(x, y, WALL_TOP_LEFT.get_mut().get_z());
                            let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos);
                            if !(state.is_block(mcrust::net::minecraft::world::level::block::blocks::glazed_terracotta().orange())) || !(state.has_property(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::horizontal_facing())) {
                                break 'continue_2;
                            }
                            let mut facing: Direction = state.get_value(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::horizontal_facing());
                            solutions.push(DIRECTION_MAPPINGS.get_mut().get(&facing).cloned().unwrap());
                        }
                        y -= 1;
                    }
                }
                x -= 1;
            }
        }
        return solutions;
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !(SkyblockerConfigManager::get().foraging.moonglade_marsh.solve_forest_temple_puzzle) || !(Utils::is_in_galatea()) || !compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<Galatea>().is_some_and(|__e| *__e == Galatea::ForestTemple)) || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let mut solutions: Vec<Direction> = TerracottaPuzzle::solve();
        for i in 0..(solutions.len() as i32) {
            let mut correct_direction: Direction = solutions[(i) as usize];
            let mut pos: BlockPos = TerracottaPuzzle::get_floor_block(i);
            let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos);
            if !(state.is_block(mcrust::net::minecraft::world::level::block::blocks::glazed_terracotta().orange())) || !(state.has_property(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::horizontal_facing())) {
                continue;
            }
            let mut floor_direction: Direction = state.get_value(&mcrust::net::minecraft::world::level::block::state::properties::block_state_properties::horizontal_facing());
            let mut clockwise_rotations: i32 = 0;
            let mut last_clockwise_rotation: Direction = floor_direction;
            let mut counterclockwise_rotations: i32 = 0;
            let mut last_counterclockwise_rotation: Direction = floor_direction;
            let mut rotations_needed: i32 = 0;
            for j in 0..4 {
                if floor_direction == correct_direction {
                    break;
                }
                clockwise_rotations += 1;
                last_clockwise_rotation = last_clockwise_rotation.get_clock_wise();
                if last_clockwise_rotation == correct_direction {
                    rotations_needed = clockwise_rotations;
                    break;
                }
                counterclockwise_rotations += 1;
                last_counterclockwise_rotation = last_counterclockwise_rotation.get_counter_clock_wise();
                if last_counterclockwise_rotation == correct_direction {
                    rotations_needed = (-counterclockwise_rotations);
                    break;
                }
            }
            if clockwise_rotations != 0 {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_text()); __f(collector, &component::literal(&rotations_needed.to_string()).with_style_format(ChatFormatting::DarkBlue), &vec3::at_center_of(&pos).add((0) as f64, (1) as f64, (0) as f64), false) });
            }
        }
    }

    /// java: private static BlockPos getFloorBlock(int index)
    pub fn get_floor_block(mut index: i32) -> BlockPos {
        let mut row_index: i32 = index.wrapping_div(2);
        return (if index.wrapping_rem(2) == 0 { FLOOR_TOP_LEFT.get_mut().offset((-row_index), 0, 0) } else { FLOOR_BOTTOM_LEFT.get_mut().offset((-row_index), 0, 0) });
    }

}
