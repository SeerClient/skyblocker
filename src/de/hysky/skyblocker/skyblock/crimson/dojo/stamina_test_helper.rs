//! Ported from de.hysky.skyblocker.skyblock.crimson.dojo.StaminaTestHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::vec3i::Vec3i;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::phys::aabb::AABB;

/// java: private static final int WALL_THRESHOLD_VALUE = 13;
pub const WALL_THRESHOLD_VALUE: i32 = 13;
/// java: private static final int WALL_HEIGHT = 5;
pub const WALL_HEIGHT: i32 = 5;
/// java: private static final float[] INCOMING_COLOR = new float[]{0f, 1f, 0f, 0f};
pub static INCOMING_COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0f32, 1.0f32, 0.0f32, 0.0f32]);
/// java: private static final float[] OUTGOING_COLOR = new float[]{1f, 0.64f, 0f, 0f};
pub static OUTGOING_COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 0.64f32, 0.0f32, 0.0f32]);
/// java: private static final List<AABB> wallHoles = new ArrayList<>();
pub static WALL_HOLES: compat::MainThread<Vec<AABB>> = compat::MainThread::new(|| Vec::new());
/// java: private static final List<AABB> lastHoles = new ArrayList<>();
pub static LAST_HOLES: compat::MainThread<Vec<AABB>> = compat::MainThread::new(|| Vec::new());
/// java: private static final Map<AABB, HoleDirection> holeDirections = new HashMap<>();
///
/// Keyed by `AabbKey`: `AABB` is a value class (`equals`/`hashCode` over its six doubles).
pub static HOLE_DIRECTIONS: compat::MainThread<std::collections::HashMap<AabbKey, HoleDirection>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static BlockPos middleBase;
pub static MIDDLE_BASE: compat::MainThread<Option<BlockPos>> = compat::MainThread::new(|| None);

/// java: public class StaminaTestHelper
#[derive(Clone)]
pub struct StaminaTestHelper {}

impl StaminaTestHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static void reset()
    pub fn reset() {
        WALL_HOLES.get_mut().clear();
        LAST_HOLES.get_mut().clear();
        HOLE_DIRECTIONS.get_mut().clear();
        *MIDDLE_BASE.get_mut() = None;
    }

    /// java: protected static void update()
    pub fn update() {
        let Some(mut current_bottom_wall_locations) = StaminaTestHelper::find_wall_blocks() else {
            return;
        };
        let mut walls: Vec<AABB> = StaminaTestHelper::find_walls(&current_bottom_wall_locations);
        LAST_HOLES.get_mut().clear();
        LAST_HOLES.get_mut().extend(WALL_HOLES.get_mut().iter().cloned());
        WALL_HOLES.get_mut().clear();
        for wall in walls.iter() {
            WALL_HOLES.get_mut().extend(StaminaTestHelper::find_holes_in_box(wall).into_iter());
        }
        let mut last_hole_directions: std::collections::HashMap<AabbKey, HoleDirection> = HOLE_DIRECTIONS.get_mut().clone();
        HOLE_DIRECTIONS.get_mut().clear();
        for hole in WALL_HOLES.get_mut().iter() {
            let mut hole_direction: HoleDirection = StaminaTestHelper::get_whole_direction(hole);
            if hole_direction == HoleDirection::Unchanged {
                HOLE_DIRECTIONS.get_mut().insert(AabbKey::of(hole), last_hole_directions.get(&AabbKey::of(hole)).cloned().unwrap());
                continue;
            }
            HOLE_DIRECTIONS.get_mut().insert(AabbKey::of(hole), hole_direction);
        }
    }

    /// java: private static List<BlockPos> findWallBlocks()
    pub fn find_wall_blocks() -> Option<Vec<BlockPos>> {
        if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return None;
        }
        let mut player_pos: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap();
        if MIDDLE_BASE.get_mut().is_none() {
            for x in player_pos.get_x().wrapping_sub(10)..player_pos.get_x().wrapping_add(10) {
                for y in player_pos.get_y().wrapping_sub(5)..player_pos.get_y() {
                    for z in player_pos.get_z().wrapping_sub(10)..player_pos.get_z().wrapping_add(10) {
                        let mut pos: BlockPos = BlockPos::new(x, y, z);
                        let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos);
                        if state.is_block(&blocks::blocks().get("chiseled_stone_bricks")) {
                            *MIDDLE_BASE.get_mut() = Some(pos.clone());
                            return None;
                        }
                    }
                }
            }
            return None;
        }
        let mut current_bottom_wall_locations: Vec<BlockPos> = Vec::new();
        for x in (MIDDLE_BASE.get_mut().as_ref().unwrap().get_x()).wrapping_sub(15)..(MIDDLE_BASE.get_mut().as_ref().unwrap().get_x()).wrapping_add(15) {
            for z in (MIDDLE_BASE.get_mut().as_ref().unwrap().get_z()).wrapping_sub(15)..(MIDDLE_BASE.get_mut().as_ref().unwrap().get_z()).wrapping_add(15) {
                let mut pos: BlockPos = BlockPos::new(x, (MIDDLE_BASE.get_mut().as_ref().unwrap().get_y()).wrapping_add(1), z);
                let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos);
                if !state.is_air() {
                    current_bottom_wall_locations.push(pos.clone());
                }
            }
        }
        return Some(current_bottom_wall_locations);
    }

    /// java: private static List<AABB> findWalls(List<BlockPos> currentBottomWallLocations)
    pub fn find_walls(current_bottom_wall_locations: &Vec<BlockPos>) -> Vec<AABB> {
        let mut possible_walls_x: std::collections::HashMap<i32, Vec<BlockPos>> = std::collections::HashMap::new();
        let mut possible_walls_z: std::collections::HashMap<i32, Vec<BlockPos>> = std::collections::HashMap::new();
        for block in current_bottom_wall_locations.iter() {
            let mut x: i32 = block.get_x();
            if !(possible_walls_x.contains_key(&x)) {
                possible_walls_x.insert(x, Vec::new());
            }
            possible_walls_x.get(&x).cloned().unwrap().push(block.clone());
            let mut z: i32 = block.get_z();
            if !(possible_walls_z.contains_key(&z)) {
                possible_walls_z.insert(z, Vec::new());
            }
            possible_walls_z.get(&z).cloned().unwrap().push(block.clone());
        }
        let mut walls: Vec<Vec<BlockPos>> = Vec::new();
        for line in possible_walls_x.values().cloned() {
            if (line.len() as i32) >= WALL_THRESHOLD_VALUE {
                walls.push(line.clone());
            }
        }
        for line in possible_walls_z.values().cloned() {
            if (line.len() as i32) >= WALL_THRESHOLD_VALUE {
                walls.push(line.clone());
            }
        }
        let mut wall_boxes: Vec<AABB> = Vec::new();
        for wall in walls.iter() {
            let mut min_pos: BlockPos = wall[0].clone();
            let mut max_pos: BlockPos = wall[0].clone();
            for pos in wall.iter() {
                if pos.get_x() < min_pos.get_x() {
                    min_pos = BlockPos::new(pos.get_x(), min_pos.get_y(), min_pos.get_z());
                }
                if pos.get_z() < min_pos.get_z() {
                    min_pos = BlockPos::new(min_pos.get_x(), min_pos.get_y(), pos.get_z());
                }
                if pos.get_x() > max_pos.get_x() {
                    max_pos = BlockPos::new(pos.get_x(), max_pos.get_y(), max_pos.get_z());
                }
                if pos.get_z() > max_pos.get_z() {
                    max_pos = BlockPos::new(max_pos.get_x(), max_pos.get_y(), pos.get_z());
                }
            }
            max_pos = BlockPos::new(max_pos.get_x(), max_pos.get_y().wrapping_add(WALL_HEIGHT), max_pos.get_z());
            wall_boxes.push(AABB::encapsulating_full_blocks(&min_pos, &max_pos));
        }
        return wall_boxes;
    }

    /// java: private static List<AABB> findHolesInBox(AABB box)
    pub fn find_holes_in_box(r#box: &AABB) -> Vec<AABB> {
        let mut holes: Vec<AABB> = Vec::new();
        if false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return holes;
        }
        let mut wall_direction: Vec3i = (if (r#box).get_xsize() == (1 as f64) { Vec3i::new(0, 0, 1) } else { Vec3i::new(1, 0, 0) });
        let mut top_left: Vec<BlockPos> = Vec::new();
        let mut top_right: Vec<BlockPos> = Vec::new();
        let mut bottom_left: Vec<BlockPos> = Vec::new();
        for z in (((r#box).min_z) as i32)..((r#box).max_z) as i32 {
            for x in (((r#box).min_x) as i32)..((r#box).max_x) as i32 {
                for y in (((r#box).min_y) as i32)..((r#box).max_y) as i32 {
                    let mut pos: BlockPos = BlockPos::new(x, y, z);
                    let mut state: Arc<BlockState> = mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos);
                    if !state.is_air() {
                        continue;
                    }
                    let mut top: bool = ((y as f64) == ((r#box).max_y - (1 as f64)) || !((mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos.offset(0, 1, 0))).is_air()));
                    let mut bottom: bool = !((mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos.offset(0, -1, 0))).is_air());
                    let mut left: bool = !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos.offset_vec(&wall_direction)).is_air());
                    let mut right: bool = !(mcrust::mcrust::modding::game::level().unwrap().get_block_state_typed(&pos.subtract(&wall_direction)).is_air());
                    if top {
                        if left {
                            top_left.push(pos.clone());
                        }
                        if right {
                            top_right.push(pos.clone());
                        }
                    }
                    if bottom && left {
                        bottom_left.push(pos.clone());
                    }
                }
            }
        }
        for i in 0..(top_left.len() as i32) {
            if (top_right.len() as i32) <= i || (bottom_left.len() as i32) <= i {
                break;
            }
            let mut hole: AABB = AABB::encapsulating_full_blocks(&top_left[(i) as usize], &top_right[(i) as usize]);
            hole = hole.expand_towards((0) as f64, (((bottom_left[(i) as usize]).get_y()).wrapping_sub((top_left[(i) as usize]).get_y())) as f64, (0) as f64);
            holes.push(hole.clone());
        }
        return holes;
    }

    /// java: private static HoleDirection getWholeDirection(AABB hole)
    pub fn get_whole_direction(hole: &AABB) -> HoleDirection {
        if LAST_HOLES.get_mut().iter().any(|e| *e == hole.clone()) {
            return HoleDirection::Unchanged;
        }
        let mut pos_x: AABB = hole.move_by((1) as f64, (0) as f64, (0) as f64);
        if LAST_HOLES.get_mut().iter().any(|e| *e == pos_x.clone()) {
            return HoleDirection::PositiveX;
        }
        let mut neg_x: AABB = hole.move_by((-1) as f64, (0) as f64, (0) as f64);
        if LAST_HOLES.get_mut().iter().any(|e| *e == neg_x.clone()) {
            return HoleDirection::NegativeX;
        }
        let mut pos_z: AABB = hole.move_by((0) as f64, (0) as f64, (1) as f64);
        if LAST_HOLES.get_mut().iter().any(|e| *e == pos_z.clone()) {
            return HoleDirection::PositiveZ;
        }
        let mut neg_z: AABB = hole.move_by((0) as f64, (0) as f64, (-1) as f64);
        if LAST_HOLES.get_mut().iter().any(|e| *e == neg_z.clone()) {
            return HoleDirection::NegativeZ;
        }
        return HoleDirection::New;
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if WALL_HOLES.get_mut().is_empty() || false /* java: compat::minecraft() == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut player_pos: BlockPos = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()).unwrap();
        for hole in WALL_HOLES.get_mut().iter() {
            let mut color: Vec<f32> = (if StaminaTestHelper::is_hole_incoming(hole, HOLE_DIRECTIONS.get_mut().get(&AabbKey::of(hole)).cloned().unwrap(), &player_pos) { INCOMING_COLOR.clone() } else { OUTGOING_COLOR.clone() });
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, hole, &color, 0.3f32, false) });
        }
    }

    /// java: private static boolean isHoleIncoming(AABB holePos, HoleDirection holeDirection, BlockPos playerPos)
    pub fn is_hole_incoming(hole_pos: &AABB, mut hole_direction: HoleDirection, player_pos: &BlockPos) -> bool {
        return match hole_direction {
            HoleDirection::PositiveX => ((player_pos.get_x() as f64) < hole_pos.min_x),
            HoleDirection::PositiveZ => ((player_pos.get_z() as f64) < hole_pos.min_z),
            HoleDirection::NegativeX => ((player_pos.get_x() as f64) > hole_pos.max_x),
            HoleDirection::NegativeZ => ((player_pos.get_z() as f64) > hole_pos.max_z),
            _ => true,
        };
    }

}

/// java: private enum HoleDirection
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HoleDirection {
    PositiveX,
    PositiveZ,
    NegativeX,
    NegativeZ,
    New,
    Unchanged,
}

impl HoleDirection {
    pub const VALUES: [HoleDirection; 6] = [HoleDirection::PositiveX, HoleDirection::PositiveZ, HoleDirection::NegativeX, HoleDirection::NegativeZ, HoleDirection::New, HoleDirection::Unchanged];

    /// java: values()
    pub fn values() -> Vec<HoleDirection> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            HoleDirection::PositiveX => "POSITIVE_X",
            HoleDirection::PositiveZ => "POSITIVE_Z",
            HoleDirection::NegativeX => "NEGATIVE_X",
            HoleDirection::NegativeZ => "NEGATIVE_Z",
            HoleDirection::New => "NEW",
            HoleDirection::Unchanged => "UNCHANGED",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> HoleDirection {
        match name {
            "POSITIVE_X" => HoleDirection::PositiveX,
            "POSITIVE_Z" => HoleDirection::PositiveZ,
            "NEGATIVE_X" => HoleDirection::NegativeX,
            "NEGATIVE_Z" => HoleDirection::NegativeZ,
            "NEW" => HoleDirection::New,
            "UNCHANGED" => HoleDirection::Unchanged,
            _ => panic!("IllegalArgumentException: No enum constant HoleDirection.{name}"),
        }
    }

}

impl std::fmt::Display for HoleDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// `AABB` as a map key: Java's `AABB.equals` compares the six doubles with `Double.compare` and `hashCode` hashes
/// `Double.doubleToLongBits` of each - so the key is those bits (NaN canonicalised, as `doubleToLongBits` does).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AabbKey([u64; 6]);

impl AabbKey {
    pub fn of(aabb: &AABB) -> AabbKey {
        let bits = |d: f64| if d.is_nan() { 0x7ff8000000000000u64 } else { d.to_bits() };
        AabbKey([bits(aabb.min_x), bits(aabb.min_y), bits(aabb.min_z), bits(aabb.max_x), bits(aabb.max_y), bits(aabb.max_z)])
    }
}
