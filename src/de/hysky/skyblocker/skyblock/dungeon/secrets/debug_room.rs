//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.DebugRoom by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_map_utils::DungeonMapUtils;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::RoomBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Type;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::org::joml::vector2i::Vector2i;

/// java: public class DebugRoom extends Room
pub struct DebugRoom {
    /// java: extends Room
    pub base: RoomBase,
    /// java: private final List<Waypoint> checkedBlocks = Collections.synchronizedList(new ArrayList<>());
    pub checked_blocks: Vec<compat::Shared<dyn Waypoint>>,
}

impl std::ops::Deref for DebugRoom {
    type Target = RoomBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DebugRoom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DebugRoom {
    /// java: public DebugRoom(Type type, Vector2ic... physicalPositions)
    pub fn new(mut r#type: Type, mut physical_positions: Vec<Vector2i>) -> compat::Shared<Self> {
        let (_, __this) = RoomBase::new_in(r#type, physical_positions.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                checked_blocks: Vec::new(),
            });
            ((__h.clone() as compat::Shared<dyn Room>), __h)
        });
        __this
    }

    /// java: public static DebugRoom ofSinglePossibleRoom(Type type, Vector2ic physicalPositions, String roomName, int[] roomData, Direction direction)
    pub fn of_single_possible_room(mut r#type: Type, physical_positions: &Vector2i, room_name: &str, room_data: &Vec<i32>, mut direction: Direction) -> compat::Shared<DebugRoom> {
        return DebugRoom::of_single_possible_room_type_vector2ic_string_int_direction(r#type, &vec![physical_positions.clone()], room_name, room_data, direction);
    }

    /// java: public static DebugRoom ofSinglePossibleRoom(Type type, Vector2ic[] physicalPositions, String roomName, int[] roomData, Direction direction)
    pub fn of_single_possible_room_type_vector2ic_string_int_direction(mut r#type: Type, physical_positions: &Vec<Vector2i>, room_name: &str, room_data: &Vec<i32>, mut direction: Direction) -> compat::Shared<DebugRoom> {
        let mut room: compat::Shared<DebugRoom> = DebugRoom::new(r#type, physical_positions.clone());
        // IntSortedSet segmentsX = IntSortedSets.unmodifiable(new IntRBTreeSet(room.segments.stream().mapToInt(Vector2ic::x).toArray()));
        let mut segments_x: std::collections::BTreeSet<i32> = compat::read(&room, |__t| __t.segments.iter().map(|s| s.x()).collect());
        // IntSortedSet segmentsY = IntSortedSets.unmodifiable(new IntRBTreeSet(room.segments.stream().mapToInt(Vector2ic::y).toArray()));
        let mut segments_y: std::collections::BTreeSet<i32> = compat::read(&room, |__t| __t.segments.iter().map(|s| s.y()).collect());
        { let __v = Some(std::collections::HashMap::from([(room_name.to_string(), room_data.clone())])); compat::write(&room, |__t| __t.rooms_data = __v); }
        // room.possibleRooms = List.of(MutableTriple.of(direction, DungeonMapUtils.getPhysicalCornerPos(direction, segmentsX, segmentsY), List.of(roomName)));
        { let __v = Some(vec![crate::unresolved::MutableTripleOf::of(direction, DungeonMapUtils::get_physical_corner_pos(direction, &segments_x, &segments_y), vec![room_name.to_string()])]); compat::write(&room, |__t| __t.possible_rooms = __v); }
        return room;
    }

}

impl Room for DebugRoom {
    fn room_base(&self) -> &RoomBase {
        &self.base
    }
    fn room_base_mut(&mut self) -> &mut RoomBase {
        &mut self.base
    }

    /// java: @Override protected boolean checkBlock(ClientLevel world, BlockPos pos) (dispatch: the body for this class)
    fn check_block(&self) -> fn(&compat::Shared<dyn Room>, &ClientLevel, &BlockPos) -> bool {
        DebugRoom::check_block_dyn
    }

}

impl Tickable for DebugRoom {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @SuppressWarnings("JavadocReference") @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn Room>::tick_from::<Self>
    }

}

impl Renderable for DebugRoom {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        DebugRoom::extract_rendering_dyn
    }

}

impl DebugRoom {
    /// java: @Override protected boolean checkBlock(ClientLevel world, BlockPos pos)
    pub fn check_block_impl(__this: &compat::Shared<Self>, world: &ClientLevel, pos: &BlockPos) -> bool {
        // TODO(fabric2seer): Map.getByte(..)
        //   java: byte id = DungeonManager.NUMERIC_ID.getByte(BuiltInRegistries.BLOCK.getKey(world.getBlockState(pos).getBlock()).toString());
        let mut id: i8 = todo!("fabric2seer: the initialiser of `id` (TODO above)");
        if (id as i32) == 0 {
            return false;
        }
        for direction_rooms in __this.lock().borrow().possible_rooms.as_ref().unwrap().iter() {
            // TODO(fabric2seer): `.getLeft(..)` on a MutableTriple
            //   java: int block = posIdToInt(DungeonMapUtils.actualToRelative(directionRooms.getLeft(), directionRooms.getMiddle(), pos), id);
            let mut block: i32 = todo!("fabric2seer: the initialiser of `block` (TODO above)");
            // TODO(fabric2seer): `.getRight(..)` on a MutableTriple
            //   java: for (String room : directionRooms.getRight()) {
            //   java: 				checkedBlocks.add(new Waypoint(pos, SecretWaypoint.TYPE_SUPPLIER, Arrays.binarySearch(roomsData.get(room), block) >= 0 ? Room.GREEN_COLOR_COMPONENTS : Room.RED_COLOR_COMPONENTS));
            //   java: 			}
        }
        return false;
    }

    /// DebugRoom.checkBlock as the dispatch of Room.checkBlock
    pub fn check_block_dyn(__this: &compat::Shared<dyn Room>, __p0: &ClientLevel, __p1: &BlockPos) -> bool {
        Self::check_block_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Room>::extract_rendering_body(&compat::view(__this, |a| a as compat::Shared<dyn Room>), collector);
        // java: synchronized (checkedBlocks) - the shared state is behind a Mutex here
        {
            for checked_block in compat::read(__this, |__t| __t.checked_blocks.clone()) {
                ({ let __f = compat::dispatch(&checked_block, |__t| __t.extract_rendering()); __f(&compat::view(&checked_block, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

    /// DebugRoom.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
