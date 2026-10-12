//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.DungeonMapUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::RoomWaypoint;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Type;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::position::Position;
use mcrust::net::minecraft::core::vec3i::Vec3i;
use mcrust::net::minecraft::world::level::saveddata::maps::map_item_saved_data::MapItemSavedData;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::vector2i::Vector2i;

// TODO(fabric2seer): static `MapColor.Brightness` has no Rust counterpart in the port's MapColor file
//   java: public static final byte BLACK_COLOR = MapColor.COLOR_BLACK.getPackedId(MapColor.Brightness.LOWEST);
/// java: public static final byte BLACK_COLOR = MapColor.COLOR_BLACK.getPackedId(MapColor.Brightness.LOWEST);
pub static BLACK_COLOR: LazyLock<i8> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `BLACK_COLOR` - see the TODO above"));
/// java: public static final byte RED_COLOR = 18;
pub const RED_COLOR: i8 = (18) as i8;
/// java: public static final byte WHITE_COLOR = 34;
pub const WHITE_COLOR: i8 = (34) as i8;
/// java: public static final byte GREEN_COLOR = 30;
pub const GREEN_COLOR: i8 = (30) as i8;

/// java: public class DungeonMapUtils
#[derive(Clone)]
pub struct DungeonMapUtils {}

impl DungeonMapUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static byte getColor(MapItemSavedData map, @Nullable Vector2ic pos)
    pub fn get_color(map: &MapItemSavedData, mut pos: Option<Vector2i>) -> i8 {
        return ((if pos.is_none() { -1 } else { (DungeonMapUtils::get_color_3(map, pos.as_ref().unwrap().x(), pos.as_ref().unwrap().y())) as i32 })) as i8;
    }

    /// java: public static byte getColor(MapItemSavedData map, int x, int z)
    pub fn get_color_3(map: &MapItemSavedData, mut x: i32, mut z: i32) -> i8 {
        if x < 0 || z < 0 || x >= 128 || z >= 128 {
            return (-1) as i8;
        }
        return map.colors[(x.wrapping_add(z.wrapping_shl((7) as u32))) as usize] as i8;
    }

    /// java: public static boolean isEntranceColor(MapItemSavedData map, int x, int z)
    pub fn is_entrance_color(map: &MapItemSavedData, mut x: i32, mut z: i32) -> bool {
        return (DungeonMapUtils::get_color_3(map, x, z) == Type::Entrance.fields().color);
    }

    /// java: public static boolean isEntranceColor(MapItemSavedData map, @Nullable Vector2ic pos)
    pub fn is_entrance_color_2(map: &MapItemSavedData, mut pos: Option<Vector2i>) -> bool {
        return (DungeonMapUtils::get_color(map, pos.clone()) == Type::Entrance.fields().color);
    }

    /// java: private static @Nullable Vector2i getMapPlayerPos(MapItemSavedData map)
    pub fn get_map_player_pos(map: &MapItemSavedData) -> Option<Vector2i> {
        for decoration in map.get_decorations() {
            // MapDecorationTypes.FRAME (the registry entry `minecraft:frame`)
            if decoration.type_().is_identifier(&mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace("frame")) {
                return Some(Vector2i::new(((decoration.x() as i32).wrapping_shr((1) as u32)).wrapping_add(64), ((decoration.y() as i32).wrapping_shr((1) as u32)).wrapping_add(64)));
            }
        }
        return None;
    }

    /// java: public static @Nullable ObjectIntPair<Vector2ic> getMapEntrancePosAndRoomSize(MapItemSavedData map)
    pub fn get_map_entrance_pos_and_room_size(map: &MapItemSavedData) -> Option<(Vector2i, i32)> {
        let Some(mut map_pos) = DungeonMapUtils::get_map_player_pos(map) else {
            return None;
        };
        let mut pos_to_check: std::collections::VecDeque<Vector2i> = std::collections::VecDeque::new();
        let mut checked: std::collections::HashSet<Vector2i> = std::collections::HashSet::new();
        pos_to_check.push_back(map_pos.clone());
        checked.insert(map_pos.clone());
        while true /* java: { map_pos = pos_to_check.pop_front().unwrap(); map_pos.clone() } != null: never null here */ {
            if DungeonMapUtils::is_entrance_color_2(map, Some(map_pos.clone())) {
                let mut map_entrance_and_room_size_pos: (Vector2i, i32) = DungeonMapUtils::get_map_entrance_pos_and_room_size_at(map, &map_pos);
                if map_entrance_and_room_size_pos.1 > 0 {
                    return Some(map_entrance_and_room_size_pos.clone());
                }
            }
            let mut pos: Vector2i = Vector2i::new_from(&map_pos).sub(10, 0).clone();
            if checked.insert(pos.clone()) {
                pos_to_check.push_back(pos.clone());
            }
            pos = Vector2i::new_from(&map_pos).sub(0, 10).clone();
            if checked.insert(pos.clone()) {
                pos_to_check.push_back(pos.clone());
            }
            pos = Vector2i::new_from(&map_pos).add(10, 0).clone();
            if checked.insert(pos.clone()) {
                pos_to_check.push_back(pos.clone());
            }
            pos = Vector2i::new_from(&map_pos).add(0, 10).clone();
            if checked.insert(pos.clone()) {
                pos_to_check.push_back(pos.clone());
            }
        }
        return None;
    }

    /// java: private static ObjectIntPair<Vector2ic> getMapEntrancePosAndRoomSizeAt(MapItemSavedData map, Vector2ic mapPosImmutable)
    pub fn get_map_entrance_pos_and_room_size_at(map: &MapItemSavedData, map_pos_immutable: &Vector2i) -> (Vector2i, i32) {
        let mut map_pos: Vector2i = Vector2i::new_from(map_pos_immutable);
        while DungeonMapUtils::is_entrance_color_2(map, Some(map_pos.sub(1, 0).clone())) {
        }
        map_pos.add(1, 0);
        while DungeonMapUtils::is_entrance_color_2(map, Some(map_pos.sub(0, 1).clone())) {
        }
        return (map_pos.add(0, 1).clone(), DungeonMapUtils::get_map_room_size(map, &map_pos));
    }

    /// java: public static int getMapRoomSize(MapItemSavedData map, Vector2ic mapEntrancePos)
    pub fn get_map_room_size(map: &MapItemSavedData, map_entrance_pos: &Vector2i) -> i32 {
        let mut i: i32 = -1;
        while DungeonMapUtils::is_entrance_color(map, map_entrance_pos.x().wrapping_add({ i += 1; i }), map_entrance_pos.y()) {
        }
        return (if i > 5 { i } else { 0 });
    }

    /// java: public static @Nullable Vector2ic getMapRoomPos(MapItemSavedData map, Vector2ic mapEntrancePos, int mapRoomSize)
    pub fn get_map_room_pos(map: &MapItemSavedData, map_entrance_pos: &Vector2i, mut map_room_size: i32) -> Option<Vector2i> {
        let mut map_room_size_with_gap: i32 = map_room_size.wrapping_add(4);
        let Some(mut map_pos) = DungeonMapUtils::get_map_player_pos(map) else {
            return None;
        };
        let mut offset: Vector2i = Vector2i::new(map_entrance_pos.x().wrapping_rem(map_room_size_with_gap), map_entrance_pos.y().wrapping_rem(map_room_size_with_gap));
        // Java evaluates the receiver chain `mapPos.add(2, 2).sub(offset)` before the arguments of the next
        // `sub`, which read the SAME (already moved) vector.
        map_pos.add(2, 2).sub_vector(&offset);
        let (floor_x, floor_y) = (compat::floor_mod_i32(map_pos.x(), map_room_size_with_gap), compat::floor_mod_i32(map_pos.y(), map_room_size_with_gap));
        return Some(map_pos.sub(floor_x, floor_y).add_vector(&offset).clone());
    }

    /// java: public static Vector2ic getMapPosFromPhysical(Vector2ic physicalEntrancePos, Vector2ic mapEntrancePos, int mapRoomSize, Vector2ic physicalPos)
    pub fn get_map_pos_from_physical(physical_entrance_pos: &Vector2i, mut map_entrance_pos: Vector2i, mut map_room_size: i32, physical_pos: &Vector2i) -> Vector2i {
        return ((Vector2i::new_from(physical_pos).sub_vector(physical_entrance_pos).div(32)).mul_scalar(map_room_size.wrapping_add(4))).add_vector(&map_entrance_pos).clone();
    }

    /// java: public static Vector2dc getMapPosFromPhysical(Vector2ic physicalEntrancePos, Vector2ic mapEntrancePos, int mapRoomSize, Position physicalPos)
    pub fn get_map_pos_from_physical_vector2ic_vector2ic_int_position(physical_entrance_pos: &Vector2i, map_entrance_pos: &Vector2i, mut map_room_size: i32, physical_pos: &dyn Position) -> crate::unresolved::Vector2dc {
        // java: return new Vector2d(physicalPos.x(), physicalPos.z()).sub(physicalEntrancePos.x(), physicalEntrancePos.y()).div(32).mul(mapRoomSize + 4).add(mapEntrancePos.x(), mapEntrancePos.y());
        let mut v = crate::unresolved::Vector2d::new(physical_pos.x(), physical_pos.z());
        v.sub(physical_entrance_pos.x() as f64, physical_entrance_pos.y() as f64).div(32.0).mul(map_room_size.wrapping_add(4) as f64).add(map_entrance_pos.x() as f64, map_entrance_pos.y() as f64);
        v
    }

    /// java: public static Vector2i getMapPosForNWMostRoom(Vector2ic mapEntrancePos, int mapRoomSize)
    pub fn get_map_pos_for_nw_most_room(map_entrance_pos: &Vector2i, mut map_room_size: i32) -> Vector2i {
        return Vector2i::new(compat::floor_mod_i32(map_entrance_pos.x(), map_room_size.wrapping_add(4)), compat::floor_mod_i32(map_entrance_pos.y(), map_room_size.wrapping_add(4)));
    }

    /// java: public static Vector2ic getPhysicalRoomPos(Vec3 pos)
    pub fn get_physical_room_pos(pos: &Vec3) -> Vector2i {
        return DungeonMapUtils::get_physical_room_pos_2(pos.x(), pos.z());
    }

    /// java: public static Vector2ic getPhysicalRoomPos(Vec3i pos)
    pub fn get_physical_room_pos_vec3i(pos: &Vec3i) -> Vector2i {
        return DungeonMapUtils::get_physical_room_pos_2(((pos.get_x()) as f64) as f64, ((pos.get_z()) as f64) as f64);
    }

    /// java: public static Vector2ic getPhysicalRoomPos(double x, double z)
    pub fn get_physical_room_pos_2(mut x: f64, mut z: f64) -> Vector2i {
        // TODO(fabric2seer): static field RoundingMode.TRUNCATE
        //   java: Vector2i physicalPos = new Vector2i(x + 8.5, z + 8.5, RoundingMode.TRUNCATE);
        let mut physical_pos: Vector2i = todo!("fabric2seer: the initialiser of `physicalPos` (TODO above)");
        return (physical_pos.sub(compat::floor_mod_i32(physical_pos.x(), 32), compat::floor_mod_i32(physical_pos.y(), 32))).sub(8, 8).clone();
    }

    /// java: public static Vector2ic[] getPhysicalPosFromMap(Vector2ic mapEntrancePos, int mapRoomSize, Vector2ic physicalEntrancePos, Vector2ic... mapPositions)
    pub fn get_physical_pos_from_map(map_entrance_pos: &Vector2i, mut map_room_size: i32, physical_entrance_pos: &Vector2i, map_positions: &mut Vec<Vector2i>) -> Vec<Vector2i> {
        for i in 0..(map_positions.len() as i32) {
            map_positions[(i) as usize] = DungeonMapUtils::get_physical_pos_from_map_vector2ic_int_vector2ic_vector2ic(map_entrance_pos, map_room_size, physical_entrance_pos.clone(), &map_positions[(i) as usize]);
        }
        return map_positions.clone();
    }

    /// java: public static Vector2ic getPhysicalPosFromMap(Vector2ic mapEntrancePos, int mapRoomSize, Vector2ic physicalEntrancePos, Vector2ic mapPos)
    pub fn get_physical_pos_from_map_vector2ic_int_vector2ic_vector2ic(map_entrance_pos: &Vector2i, mut map_room_size: i32, mut physical_entrance_pos: Vector2i, map_pos: &Vector2i) -> Vector2i {
        return ((Vector2i::new_from(map_pos).sub_vector(map_entrance_pos).div(map_room_size.wrapping_add(4))).mul_scalar(32)).add_vector(&physical_entrance_pos).clone();
    }

    /// java: public static Vector2ic getPhysicalCornerPos(Room.Direction direction, IntSortedSet segmentsX, IntSortedSet segmentsY)
    pub fn get_physical_corner_pos(mut direction: Direction, segments_x: &std::collections::BTreeSet<i32>, segments_y: &std::collections::BTreeSet<i32>) -> Vector2i {
        let (first_x, last_x) = (*segments_x.first().unwrap(), *segments_x.last().unwrap());
        let (first_y, last_y) = (*segments_y.first().unwrap(), *segments_y.last().unwrap());
        return match direction {
            Direction::Nw => Vector2i::new(first_x, first_y),
            Direction::Ne => Vector2i::new(last_x.wrapping_add(30), first_y),
            Direction::Sw => Vector2i::new(first_x, last_y.wrapping_add(30)),
            Direction::Se => Vector2i::new(last_x.wrapping_add(30), last_y.wrapping_add(30)),
        };
    }

    /// java: public static BlockPos actualToRelative(Room.Direction direction, Vector2ic physicalCornerPos, BlockPos pos)
    pub fn actual_to_relative(mut direction: Direction, physical_corner_pos: &Vector2i, pos: &BlockPos) -> BlockPos {
        return match direction {
            Direction::Nw => BlockPos::new(pos.get_x().wrapping_sub(physical_corner_pos.x()), pos.get_y(), pos.get_z().wrapping_sub(physical_corner_pos.y())),
            Direction::Ne => BlockPos::new(pos.get_z().wrapping_sub(physical_corner_pos.y()), pos.get_y(), (-pos.get_x()).wrapping_add(physical_corner_pos.x())),
            Direction::Sw => BlockPos::new((-pos.get_z()).wrapping_add(physical_corner_pos.y()), pos.get_y(), pos.get_x().wrapping_sub(physical_corner_pos.x())),
            Direction::Se => BlockPos::new((-pos.get_x()).wrapping_add(physical_corner_pos.x()), pos.get_y(), (-pos.get_z()).wrapping_add(physical_corner_pos.y())),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: public static Vec3 actualToRelative(Room.Direction direction, Vector2ic physicalCornerPos, Vec3 pos)
    pub fn actual_to_relative_direction_vector2ic_vec3(mut direction: Direction, physical_corner_pos: &Vector2i, pos: &Vec3) -> Vec3 {
        return match direction {
            Direction::Nw => Vec3::new((pos.x() - (physical_corner_pos.x() as f64)), pos.y(), (pos.z() - (physical_corner_pos.y() as f64))),
            Direction::Ne => Vec3::new((pos.z() - (physical_corner_pos.y() as f64)), pos.y(), ((-pos.x()) + (physical_corner_pos.x() as f64))),
            Direction::Sw => Vec3::new(((-pos.z()) + (physical_corner_pos.y() as f64)), pos.y(), (pos.x() - (physical_corner_pos.x() as f64))),
            Direction::Se => Vec3::new(((-pos.x()) + (physical_corner_pos.x() as f64)), pos.y(), ((-pos.z()) + (physical_corner_pos.y() as f64))),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: public static BlockPos relativeToActual(Room.Direction direction, Vector2ic physicalCornerPos, DungeonManager.RoomWaypoint waypoint)
    pub fn relative_to_actual(mut direction: Direction, physical_corner_pos: &Vector2i, waypoint: &RoomWaypoint) -> BlockPos {
        return DungeonMapUtils::relative_to_actual_direction_vector2ic_block_pos(direction, physical_corner_pos, &BlockPos::new(waypoint.x(), waypoint.y(), waypoint.z()));
    }

    /// java: public static BlockPos relativeToActual(Room.Direction direction, Vector2ic physicalCornerPos, BlockPos pos)
    pub fn relative_to_actual_direction_vector2ic_block_pos(mut direction: Direction, physical_corner_pos: &Vector2i, pos: &BlockPos) -> BlockPos {
        return match direction {
            Direction::Nw => BlockPos::new(pos.get_x().wrapping_add(physical_corner_pos.x()), pos.get_y(), pos.get_z().wrapping_add(physical_corner_pos.y())),
            Direction::Ne => BlockPos::new((-pos.get_z()).wrapping_add(physical_corner_pos.x()), pos.get_y(), pos.get_x().wrapping_add(physical_corner_pos.y())),
            Direction::Sw => BlockPos::new(pos.get_z().wrapping_add(physical_corner_pos.x()), pos.get_y(), (-pos.get_x()).wrapping_add(physical_corner_pos.y())),
            Direction::Se => BlockPos::new((-pos.get_x()).wrapping_add(physical_corner_pos.x()), pos.get_y(), (-pos.get_z()).wrapping_add(physical_corner_pos.y())),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: public static Vec3 relativeToActual(Room.Direction direction, Vector2ic physicalCornerPos, Vec3 pos)
    pub fn relative_to_actual_direction_vector2ic_vec3(mut direction: Direction, physical_corner_pos: &Vector2i, pos: &Vec3) -> Vec3 {
        return match direction {
            Direction::Nw => Vec3::new((pos.x() + (physical_corner_pos.x() as f64)), pos.y(), (pos.z() + (physical_corner_pos.y() as f64))),
            Direction::Ne => Vec3::new(((-pos.z()) + (physical_corner_pos.x() as f64)), pos.y(), (pos.x() + (physical_corner_pos.y() as f64))),
            Direction::Sw => Vec3::new((pos.z() + (physical_corner_pos.x() as f64)), pos.y(), ((-pos.x()) + (physical_corner_pos.y() as f64))),
            Direction::Se => Vec3::new(((-pos.x()) + (physical_corner_pos.x() as f64)), pos.y(), ((-pos.z()) + (physical_corner_pos.y() as f64))),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: public static @Nullable Type getRoomType(MapItemSavedData map, Vector2ic mapPos)
    pub fn get_room_type(map: &MapItemSavedData, map_pos: &Vector2i) -> Option<Type> {
        return match DungeonMapUtils::get_color(map, Some(map_pos.clone())) {
            GREEN_COLOR => Some(Type::Entrance),
            63 => Some(Type::Room),
            66 => Some(Type::Puzzle),
            62 => Some(Type::Trap),
            74 => Some(Type::Miniboss),
            82 => Some(Type::Fairy),
            18 => Some(Type::Blood),
            85 => Some(Type::Unknown),
            _ => None,
        };
    }

    /// java: public static Vector2ic[] getRoomSegments(MapItemSavedData map, Vector2ic mapPos, int mapRoomSize, byte color)
    pub fn get_room_segments(map: &MapItemSavedData, mut map_pos: Vector2i, mut map_room_size: i32, mut color: i8) -> Vec<Vector2i> {
        let mut segments: std::collections::HashSet<Vector2i> = std::collections::HashSet::new();
        let mut queue: std::collections::VecDeque<Vector2i> = std::collections::VecDeque::new();
        segments.insert(map_pos.clone());
        queue.push_back(map_pos.clone());
        while !queue.is_empty() {
            let mut cur_map_pos: Vector2i = queue.pop_front().unwrap();
            // TODO(fabric2seer): `new` takes 2 argument(s) in the port (`x: i32, y: i32`), Java passes 0
            // rust: let mut new_map_pos: Vector2i = Vector2i::new();
            let mut new_map_pos: Vector2i = todo!("fabric2seer: the initialiser of `newMapPos` (TODO above)");
            if DungeonMapUtils::get_color(map, Some(new_map_pos.set_vector(&cur_map_pos).sub(1, 0).clone())) == color && !(segments.contains(new_map_pos.sub(map_room_size.wrapping_add(3), 0))) {
                segments.insert(new_map_pos.clone());
                queue.push_back(new_map_pos.clone());
                // TODO(fabric2seer): `new` takes 2 argument(s) in the port (`x: i32, y: i32`), Java passes 0
                // rust: new_map_pos = Vector2i::new();
            }
            if DungeonMapUtils::get_color(map, Some(new_map_pos.set_vector(&cur_map_pos).sub(0, 1).clone())) == color && !(segments.contains(new_map_pos.sub(0, map_room_size.wrapping_add(3)))) {
                segments.insert(new_map_pos.clone());
                queue.push_back(new_map_pos.clone());
                // TODO(fabric2seer): `new` takes 2 argument(s) in the port (`x: i32, y: i32`), Java passes 0
                // rust: new_map_pos = Vector2i::new();
            }
            if DungeonMapUtils::get_color(map, Some(new_map_pos.set_vector(&cur_map_pos).add(map_room_size, 0).clone())) == color && !(segments.contains(new_map_pos.add(4, 0))) {
                segments.insert(new_map_pos.clone());
                queue.push_back(new_map_pos.clone());
                // TODO(fabric2seer): `new` takes 2 argument(s) in the port (`x: i32, y: i32`), Java passes 0
                // rust: new_map_pos = Vector2i::new();
            }
            if DungeonMapUtils::get_color(map, Some(new_map_pos.set_vector(&cur_map_pos).add(0, map_room_size).clone())) == color && !(segments.contains(new_map_pos.add(0, 4))) {
                segments.insert(new_map_pos.clone());
                queue.push_back(new_map_pos.clone());
            }
        }
        logger().debug(&format!("[Skyblocker] Found dungeon room segments: {}", format!("[{}]", segments.iter().map(|s| compat::joml_string(&[s.x() as f64, s.y() as f64])).collect::<Vec<_>>().join(", "))));
        return segments.iter().cloned().collect::<Vec<_>>();
    }

}
