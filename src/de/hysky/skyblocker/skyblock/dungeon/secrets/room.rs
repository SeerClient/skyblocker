//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.Room by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::RoomWaypoint;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_map_utils::DungeonMapUtils;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::CategoryArgumentType;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::SecretWaypoint;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::com::mojang::brigadier::arguments::integer_argument_type::IntegerArgumentType;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::text::decimal_format_symbols::Locale;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::commands::arguments::string_representable_argument::StringRepresentableArgument;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::ambient::ambient_creature::AmbientCreature;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::entity::item::item_entity::ItemEntity;
use mcrust::net::minecraft::world::level::block::blocks;
use mcrust::net::minecraft::world::level::block::state::block_state::BlockState;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::vector2i::Vector2i;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::level::block_getter::BlockGetter as _;
use mcrust::net::minecraft::core::defaulted_registry::DefaultedRegistry as _;

/// java: public static final Pattern SECRET_INDEX = Pattern.compile("^(\\d+)");
pub static SECRET_INDEX: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(\\d+)"));
/// java: private static final Pattern SECRETS = Pattern.compile("(\\d{1,2})/(\\d{1,2}) Secrets");
pub static SECRETS: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(\\d{1,2})/(\\d{1,2}) Secrets"));
/// java: private static final String CHEST_ALREADY_OPENED = "This chest has already been searched!";
pub const CHEST_ALREADY_OPENED: &str = "This chest has already been searched!";
/// java: protected static final float[] RED_COLOR_COMPONENTS = {1, 0, 0};
pub static RED_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![((1) as f32) as f32, ((0) as f32) as f32, ((0) as f32) as f32]);
/// java: protected static final float[] GREEN_COLOR_COMPONENTS = {0, 1, 0};
pub static GREEN_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![((0) as f32) as f32, ((1) as f32) as f32, ((0) as f32) as f32]);
/// java: public static final Codec<Type> CODEC = StringRepresentable.fromEnum(Type::values);
pub static TYPE_CODEC: compat::MainThread<CodecRef<Type>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(Type::values(), |e: &Type| e.get_serialized_name()));
/// java: public static final Codec<Shape> CODEC = StringRepresentable.fromEnum(Shape::values);
pub static SHAPE_CODEC: compat::MainThread<CodecRef<Shape>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(Shape::values(), |e: &Shape| e.get_serialized_name()));
/// java: public static final Codec<Direction> CODEC = StringRepresentable.fromEnum(Direction::values);
pub static DIRECTION_CODEC: compat::MainThread<CodecRef<Direction>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(Direction::values(), |e: &Direction| e.get_serialized_name()));

/// java: public class Room implements Tickable, Renderable
pub struct RoomBase {
    /// java: private final Type type;
    pub r#type: Type,
    /// java: final Set<Vector2ic> segments;
    pub segments: std::collections::HashSet<Vector2i>,
    /// java: public ClearState clearState = ClearState.UNCLEARED;
    pub clear_state: ClearState,
    /// java: private int maxSecrets = -1;
    pub max_secrets: i32,
    /// java: protected int secretsFound = 0;
    pub secrets_found: i32,
    /// java: public boolean secretCountOutdated = true;
    pub secret_count_outdated: bool,
    /// java: private final Shape shape;
    pub shape: Option<Shape>,
    /// java: protected @Nullable Map<String, int[]> roomsData;
    pub rooms_data: Option<std::collections::HashMap<String, Vec<i32>>>,
    /// java: protected @Nullable List<MutableTriple<Direction, Vector2ic, List<String>>> possibleRooms;
    pub possible_rooms: Option<Vec<crate::unresolved::MutableTripleOf<Direction, Vector2i, Vec<String>>>>,
    /// java: private @Nullable BlockPosSet checkedBlocks = new BlockPosSet();
    pub checked_blocks: Option<compat::Shared<BlockPosSet>>,
    /// java: protected @Nullable CompletableFuture<Void> findRoom;
    /// (the search reads and writes this room, which is not `Send`: a `compat::LocalFuture`, run on the render thread)
    pub find_room: Option<compat::LocalFuture<()>>,
    /// java: private int doubleCheckBlocks;
    pub double_check_blocks: i32,
    /// java: protected MatchState matchState = MatchState.MATCHING;
    pub match_state: MatchState,
    /// java: protected Table<Integer, BlockPos, SecretWaypoint> secretWaypoints = HashBasedTable.create();
    pub secret_waypoints: crate::unresolved::HashBasedTable<i32, BlockPos, compat::Shared<SecretWaypoint>>,
    /// java: protected @Nullable String name;
    pub name: Option<String>,
    /// java: protected @Nullable Direction direction;
    pub direction: Option<Direction>,
    /// java: protected @Nullable Vector2ic physicalCornerPos;
    pub physical_corner_pos: Option<Vector2i>,
    /// java: protected List<Tickable> tickables = new ArrayList<>();
    pub tickables: Vec<compat::Shared<dyn Tickable>>,
    /// java: protected List<Renderable> renderables = new ArrayList<>();
    pub renderables: Vec<compat::Shared<dyn Renderable>>,
    /// java: private @Nullable BlockPos lastChestSecret;
    pub last_chest_secret: Option<BlockPos>,
    /// java: private long lastChestSecretTime;
    pub last_chest_secret_time: i64,
    /// java: boolean fromWebsocket = false;
    pub from_websocket: bool,
}

impl RoomBase {
    /// java: public Room(Type type, Vector2ic... physicalPositions) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut r#type: Type, mut physical_positions: Vec<Vector2i>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Room>, __R)) -> (compat::Shared<dyn Room>, __R) {
        let __init_type = r#type;
        // segments = Set.of(physicalPositions)
        let __init_segments: std::collections::HashSet<Vector2i> = physical_positions.iter().cloned().collect();
        let this = Self {
            r#type: __init_type,
            segments: __init_segments,
            clear_state: ClearState::Uncleared,
            max_secrets: -1,
            secrets_found: 0,
            secret_count_outdated: true,
            shape: None,
            rooms_data: None,
            possible_rooms: None,
            checked_blocks: Some(BlockPosSet::new()),
            find_room: None,
            double_check_blocks: 0,
            match_state: MatchState::Matching,
            // protected Table<Integer, BlockPos, SecretWaypoint> secretWaypoints = HashBasedTable.create();
            secret_waypoints: crate::unresolved::HashBasedTable::create(),
            name: None,
            direction: None,
            physical_corner_pos: None,
            tickables: Vec::new(),
            renderables: Vec::new(),
            last_chest_secret: None,
            last_chest_secret_time: 0,
            from_websocket: false,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        // IntSortedSet segmentsX = IntSortedSets.unmodifiable(new IntRBTreeSet(segments.stream().mapToInt(Vector2ic::x).toArray()));
        let mut segments_x: std::collections::BTreeSet<i32> = compat::read(&__this, |__t| __t.room_base().segments.iter().map(|s| s.x()).collect());
        // IntSortedSet segmentsY = IntSortedSets.unmodifiable(new IntRBTreeSet(segments.stream().mapToInt(Vector2ic::y).toArray()));
        let mut segments_y: std::collections::BTreeSet<i32> = compat::read(&__this, |__t| __t.room_base().segments.iter().map(|s| s.y()).collect());
        { let __v = Some(({ let __f = compat::dispatch(&__this, |__t| __t.determine_shape()); __f(&__this, &segments_x, &segments_y) })); compat::write(&__this, |__t| __t.room_base_mut().shape = __v); }
        { let __v = Some((compat::lock(&dungeon_manager::ROOMS_DATA).get("catacombs").cloned().unwrap_or_else(|| Default::default())).get(&__this.lock().borrow().room_base().shape.as_ref().unwrap().fields().shape.to_lowercase()).cloned().unwrap_or_else(|| Default::default())); compat::write(&__this, |__t| __t.room_base_mut().rooms_data = __v); }
        { let __v = Some(({ let __f = compat::dispatch(&__this, |__t| __t.get_possible_rooms()); __f(&__this, &segments_x, &segments_y) })); compat::write(&__this, |__t| __t.room_base_mut().possible_rooms = __v); }
        (__this, __r)
    }

    /// java: public Room(Type type, Vector2ic... physicalPositions)
    pub fn new(mut r#type: Type, mut physical_positions: Vec<Vector2i>) -> compat::Shared<Self> {
        Self::new_in(r#type, physical_positions, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Room>), __h)
        }).1
    }

    /// java: Room(Type type, Shape shape, Direction direction, String roomName, Set<Vector2ic> segments, IntSortedSet segmentsX, IntSortedSet segmentsY) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_7_in<__R>(mut r#type: Type, mut shape: Shape, mut direction: Direction, mut room_name: String, mut segments: std::collections::HashSet<Vector2i>, mut segments_x: std::collections::BTreeSet<i32>, mut segments_y: std::collections::BTreeSet<i32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Room>, __R)) -> (compat::Shared<dyn Room>, __R) {
        let __init_from_websocket = true;
        let __init_type = r#type;
        let __init_shape = Some(shape);
        let __init_segments = segments.clone();
        let __init_name = Some(room_name.clone());
        let __init_direction = Some(direction);
        let __init_physical_corner_pos = Some(DungeonMapUtils::get_physical_corner_pos(direction, &segments_x, &segments_y));
        let __init_rooms_data = Some((compat::lock(&dungeon_manager::ROOMS_DATA).get("catacombs").cloned().unwrap_or_else(|| Default::default())).get(&shape.fields().shape.to_lowercase()).cloned().unwrap_or_else(|| Default::default()));
        let this = Self {
            r#type: __init_type,
            segments: __init_segments,
            clear_state: ClearState::Uncleared,
            max_secrets: -1,
            secrets_found: 0,
            secret_count_outdated: true,
            shape: __init_shape,
            rooms_data: __init_rooms_data,
            possible_rooms: None,
            checked_blocks: Some(BlockPosSet::new()),
            find_room: None,
            double_check_blocks: 0,
            match_state: MatchState::Matching,
            // protected Table<Integer, BlockPos, SecretWaypoint> secretWaypoints = HashBasedTable.create();
            secret_waypoints: crate::unresolved::HashBasedTable::create(),
            name: __init_name,
            direction: __init_direction,
            physical_corner_pos: __init_physical_corner_pos,
            tickables: Vec::new(),
            renderables: Vec::new(),
            last_chest_secret: None,
            last_chest_secret_time: 0,
            from_websocket: __init_from_websocket,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        ({ let __f = compat::dispatch(&__this, |__t| __t.room_matched()); __f(&__this) });
        { let __v = MatchState::Matched; compat::write(&__this, |__t| __t.room_base_mut().match_state = __v); }
        (dungeon_events::room_matched_invoker())(__this.clone());
        ({ let __f = compat::dispatch(&__this, |__t| __t.discard()); __f(&__this) });
        (__this, __r)
    }

    /// java: Room(Type type, Shape shape, Direction direction, String roomName, Set<Vector2ic> segments, IntSortedSet segmentsX, IntSortedSet segmentsY)
    pub fn new_7(mut r#type: Type, mut shape: Shape, mut direction: Direction, mut room_name: String, mut segments: std::collections::HashSet<Vector2i>, mut segments_x: std::collections::BTreeSet<i32>, mut segments_y: std::collections::BTreeSet<i32>) -> compat::Shared<Self> {
        Self::new_7_in(r#type, shape, direction, room_name, segments, segments_x, segments_y, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Room>), __h)
        }).1
    }

    /// java: protected static Shape determineShape(Type type, Set<Vector2ic> segments, IntSortedSet segmentsX, IntSortedSet segmentsY)
    pub fn determine_shape_4(mut r#type: Type, segments: &std::collections::HashSet<Vector2i>, segments_x: &std::collections::BTreeSet<i32>, segments_y: &std::collections::BTreeSet<i32>) -> Shape {
        return match r#type {
            Type::Puzzle => Shape::Puzzle,
            Type::Trap => Shape::Trap,
            Type::Miniboss => Shape::Miniboss,
            _ => match segments.len() {
                1 => Shape::OneByOne,
                2 => Shape::OneByTwo,
                3 => if segments_x.len() == 2 && segments_y.len() == 2 { Shape::LShape } else { Shape::OneByThree },
                4 => if segments_x.len() == 2 && segments_y.len() == 2 { Shape::TwoByTwo } else { Shape::OneByFour },
                _ => panic!("IllegalArgumentException: There are no matching room shapes with this set of physical positions: [{}]", segments.iter().map(|s| compat::joml_string(&[s.x() as f64, s.y() as f64])).collect::<Vec<_>>().join(", ")),
            },
        };
    }

    /// `Arrays.toString(segments.toArray())`.
    pub fn segments_string(segments: &std::collections::HashSet<Vector2i>) -> String {
        format!("[{}]", segments.iter().map(|s| compat::joml_string(&[s.x() as f64, s.y() as f64])).collect::<Vec<_>>().join(", "))
    }

    /// java: private static boolean notInDoorway(BlockPos pos)
    pub fn not_in_doorway(pos: &BlockPos) -> bool {
        if pos.get_y() < 66 || pos.get_y() > 73 {
            return true;
        }
        let mut x: i32 = compat::floor_mod_i32(pos.get_x().wrapping_sub(8), 32);
        let mut z: i32 = compat::floor_mod_i32(pos.get_z().wrapping_sub(8), 32);
        return (x < 13 || x > 17 || z > 2 && z < 28) && (z < 13 || z > 17 || x > 2 && x < 28);
    }

}

impl std::fmt::Display for RoomBase {
    /// `toString()`: "Room{type=%s, segments=%s, shape=%s, matchState=%s, name=%s, direction=%s, physicalCornerPos=%s}"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let segments = format!("[{}]", self.segments.iter().map(|s| compat::joml_string(&[s.x() as f64, s.y() as f64])).collect::<Vec<_>>().join(", "));
        write!(
            f,
            "Room{{type={}, segments={}, shape={}, matchState={}, name={}, direction={}, physicalCornerPos={}}}",
            self.r#type,
            segments,
            compat::opt_str(&self.shape),
            self.match_state,
            compat::opt_str(&self.name),
            compat::opt_str(&self.direction),
            self.physical_corner_pos.map_or("null".to_string(), |p| compat::joml_string(&[p.x() as f64, p.y() as f64]))
        )
    }
}

/// java: the virtual methods of Room (implemented by RoomBase and every subclass)
pub trait Room: Tickable + Renderable {
    fn room_base(&self) -> &RoomBase;
    fn room_base_mut(&mut self) -> &mut RoomBase;

    /// java: public Type getType() (dispatch: the body for this class)
    fn get_type(&self) -> fn(&compat::Shared<dyn Room>) -> Type {
        <dyn Room>::get_type_body
    }

    /// java: public Set<Vector2ic> getSegments() (dispatch: the body for this class)
    fn get_segments(&self) -> fn(&compat::Shared<dyn Room>) -> std::collections::HashSet<Vector2i> {
        <dyn Room>::get_segments_body
    }

    /// java: public Shape getShape() (dispatch: the body for this class)
    fn get_shape(&self) -> fn(&compat::Shared<dyn Room>) -> Shape {
        <dyn Room>::get_shape_body
    }

    /// java: public @Nullable Vector2ic getPhysicalCornerPos() (dispatch: the body for this class)
    fn get_physical_corner_pos(&self) -> fn(&compat::Shared<dyn Room>) -> Option<Vector2i> {
        <dyn Room>::get_physical_corner_pos_body
    }

    /// java: public boolean isMatched() (dispatch: the body for this class)
    fn is_matched(&self) -> fn(&compat::Shared<dyn Room>) -> bool {
        <dyn Room>::is_matched_body
    }

    /// java: public @Nullable String getName() (dispatch: the body for this class)
    fn get_name(&self) -> fn(&compat::Shared<dyn Room>) -> Option<String> {
        <dyn Room>::get_name_body
    }

    /// java: public @Nullable Direction getDirection() (dispatch: the body for this class)
    fn get_direction(&self) -> fn(&compat::Shared<dyn Room>) -> Option<Direction> {
        <dyn Room>::get_direction_body
    }

    /// java: @Override public String toString() (dispatch: the body for this class)
    fn to_string(&self) -> fn(&compat::Shared<dyn Room>) -> String {
        <dyn Room>::to_string_body
    }

    /// java: private Shape determineShape(IntSortedSet segmentsX, IntSortedSet segmentsY) (dispatch: the body for this class)
    fn determine_shape(&self) -> fn(&compat::Shared<dyn Room>, &std::collections::BTreeSet<i32>, &std::collections::BTreeSet<i32>) -> Shape {
        <dyn Room>::determine_shape_body
    }

    /// java: private List<MutableTriple<Direction, Vector2ic, List<String>>> getPossibleRooms(IntSortedSet segmentsX, IntSortedSet segmentsY) (dispatch: the body for this class)
    fn get_possible_rooms(&self) -> fn(&compat::Shared<dyn Room>, &std::collections::BTreeSet<i32>, &std::collections::BTreeSet<i32>) -> Vec<crate::unresolved::MutableTripleOf<Direction, Vector2i, Vec<String>>> {
        <dyn Room>::get_possible_rooms_body
    }

    /// java: protected Direction[] getPossibleDirections(IntSortedSet segmentsX, IntSortedSet segmentsY) (dispatch: the body for this class)
    fn get_possible_directions(&self) -> fn(&compat::Shared<dyn Room>, &std::collections::BTreeSet<i32>, &std::collections::BTreeSet<i32>) -> Vec<Direction> {
        <dyn Room>::get_possible_directions_body
    }

    /// java: protected void addCustomWaypoint(CommandContext<FabricClientCommandSource> context, BlockPos pos) (dispatch: the body for this class)
    fn add_custom_waypoint(&self) -> fn(&compat::Shared<dyn Room>, &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, &BlockPos) {
        <dyn Room>::add_custom_waypoint_body
    }

    /// java: @SuppressWarnings("JavadocReference") private void addCustomWaypoint(int secretIndex, SecretWaypoint.Category category, Component waypointName, BlockPos pos) (dispatch: the body for this class)
    fn add_custom_waypoint_4(&self) -> fn(&compat::Shared<dyn Room>, i32, crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category, &MutableComponent, &BlockPos) {
        <dyn Room>::add_custom_waypoint_4_body
    }

    /// java: private void addCustomWaypoint(SecretWaypoint relativeWaypoint) (dispatch: the body for this class)
    fn add_custom_waypoint_1(&self) -> fn(&compat::Shared<dyn Room>, &compat::Shared<SecretWaypoint>) {
        <dyn Room>::add_custom_waypoint_1_body
    }

    /// java: protected void removeCustomWaypoint(CommandContext<FabricClientCommandSource> context, BlockPos pos) (dispatch: the body for this class)
    fn remove_custom_waypoint(&self) -> fn(&compat::Shared<dyn Room>, &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, &BlockPos) {
        <dyn Room>::remove_custom_waypoint_body
    }

    /// java: @SuppressWarnings("JavadocReference") private @Nullable SecretWaypoint removeCustomWaypoint(BlockPos pos) (dispatch: the body for this class)
    fn remove_custom_waypoint_1(&self) -> fn(&compat::Shared<dyn Room>, &BlockPos) -> Option<compat::Shared<SecretWaypoint>> {
        <dyn Room>::remove_custom_waypoint_1_body
    }

    /// java: private void removeCustomWaypoint(int secretIndex, BlockPos relativePos) (dispatch: the body for this class)
    fn remove_custom_waypoint_int_block_pos(&self) -> fn(&compat::Shared<dyn Room>, i32, &BlockPos) {
        <dyn Room>::remove_custom_waypoint_int_block_pos_body
    }

    /// java: public <T extends Tickable & Renderable> void addSubProcess(T process)
    /// `T extends Tickable & Renderable`: the one object, handed as both of its views (Rust has no
    /// intersection type for a trait object).
    fn add_sub_process(&mut self, mut process: compat::Shared<dyn Tickable>, process_renderable: compat::Shared<dyn Renderable>) {
        self.room_base_mut().tickables.push(process.clone());
        self.room_base_mut().renderables.push(process_renderable.clone());
    }

    /// java: protected boolean checkBlock(ClientLevel world, BlockPos pos) (dispatch: the body for this class)
    fn check_block(&self) -> fn(&compat::Shared<dyn Room>, &ClientLevel, &BlockPos) -> bool {
        <dyn Room>::check_block_body
    }

    /// java: protected int posIdToInt(BlockPos pos, byte id) (dispatch: the body for this class)
    fn pos_id_to_int(&self) -> fn(&compat::Shared<dyn Room>, &BlockPos, i8) -> i32 {
        <dyn Room>::pos_id_to_int_body
    }

    /// java: @SuppressWarnings("JavadocReference") private void roomMatched() (dispatch: the body for this class)
    fn room_matched(&self) -> fn(&compat::Shared<dyn Room>) {
        <dyn Room>::room_matched_body
    }

    /// java: protected void reset() (dispatch: the body for this class)
    fn reset(&self) -> fn(&compat::Shared<dyn Room>) {
        <dyn Room>::reset_body
    }

    /// java: private void discard() (dispatch: the body for this class)
    fn discard(&self) -> fn(&compat::Shared<dyn Room>) {
        <dyn Room>::discard_body
    }

    /// java: public BlockPos actualToRelative(BlockPos pos) (dispatch: the body for this class)
    fn actual_to_relative(&self) -> fn(&compat::Shared<dyn Room>, &BlockPos) -> BlockPos {
        <dyn Room>::actual_to_relative_body
    }

    /// java: public Vec3 actualToRelative(Vec3 pos) (dispatch: the body for this class)
    fn actual_to_relative_vec3(&self) -> fn(&compat::Shared<dyn Room>, &Vec3) -> Vec3 {
        <dyn Room>::actual_to_relative_vec3_body
    }

    /// java: public BlockPos relativeToActual(BlockPos pos) (dispatch: the body for this class)
    fn relative_to_actual(&self) -> fn(&compat::Shared<dyn Room>, &BlockPos) -> BlockPos {
        <dyn Room>::relative_to_actual_body
    }

    /// java: public Vec3 relativeToActual(Vec3 pos) (dispatch: the body for this class)
    fn relative_to_actual_vec3(&self) -> fn(&compat::Shared<dyn Room>, &Vec3) -> Vec3 {
        <dyn Room>::relative_to_actual_vec3_body
    }

    /// java: protected void onChatMessage(String message, boolean overlay) (dispatch: the body for this class)
    fn on_chat_message(&self) -> fn(&compat::Shared<dyn Room>, &str, bool) {
        <dyn Room>::on_chat_message_body
    }

    /// java: protected void updateSecretCount(String message) (dispatch: the body for this class)
    fn update_secret_count(&self) -> fn(&compat::Shared<dyn Room>, &str) {
        <dyn Room>::update_secret_count_body
    }

    /// java: protected void onUseBlock(Level world, BlockPos pos) (dispatch: the body for this class)
    fn on_use_block(&self) -> fn(&compat::Shared<dyn Room>, &Level, BlockPos) {
        <dyn Room>::on_use_block_body
    }

    /// java: protected void onChestOpened(BlockPos pos) (dispatch: the body for this class)
    fn on_chest_opened(&self) -> fn(&compat::Shared<dyn Room>, &BlockPos) {
        <dyn Room>::on_chest_opened_body
    }

    /// java: protected void onItemPickup(ItemEntity itemEntity) (dispatch: the body for this class)
    fn on_item_pickup(&self) -> fn(&compat::Shared<dyn Room>, &ItemEntity) {
        <dyn Room>::on_item_pickup_body
    }

    /// java: protected void onBatRemoved(AmbientCreature bat) (dispatch: the body for this class)
    fn on_bat_removed(&self) -> fn(&compat::Shared<dyn Room>, &dyn AmbientCreature) {
        <dyn Room>::on_bat_removed_body
    }

    /// java: private void markSecretsFoundAndLogInfo(SecretWaypoint secretWaypoint, String msg, Object... args) (dispatch: the body for this class)
    fn mark_secrets_found_and_log_info(&self) -> fn(&compat::Shared<dyn Room>, &compat::Shared<SecretWaypoint>, &str, &Vec<compat::Object>) {
        <dyn Room>::mark_secrets_found_and_log_info_body
    }

    /// java: private void markSecretsAndLogInfo(SecretWaypoint secretWaypoint, boolean found, String msg, Object... args) (dispatch: the body for this class)
    fn mark_secrets_and_log_info(&self) -> fn(&compat::Shared<dyn Room>, &compat::Shared<SecretWaypoint>, bool, &str, &Vec<compat::Object>) {
        <dyn Room>::mark_secrets_and_log_info_body
    }

    /// java: protected int getIndexByWaypointHash(int waypointHash) (dispatch: the body for this class)
    fn get_index_by_waypoint_hash(&self) -> fn(&compat::Shared<dyn Room>, i32) -> i32 {
        <dyn Room>::get_index_by_waypoint_hash_body
    }

    /// java: protected boolean markSecrets(int secretIndex, boolean found) (dispatch: the body for this class)
    fn mark_secrets(&self) -> fn(&compat::Shared<dyn Room>, i32, bool) -> bool {
        <dyn Room>::mark_secrets_body
    }

    /// java: protected void markAllSecrets(boolean found) (dispatch: the body for this class)
    fn mark_all_secrets(&self) -> fn(&compat::Shared<dyn Room>, bool) {
        <dyn Room>::mark_all_secrets_body
    }

    /// java: public int getMaxSecretCount() (dispatch: the body for this class)
    fn get_max_secret_count(&self) -> fn(&compat::Shared<dyn Room>) -> i32 {
        <dyn Room>::get_max_secret_count_body
    }

    /// java: protected int getSecretCount() (dispatch: the body for this class)
    fn get_secret_count(&self) -> fn(&compat::Shared<dyn Room>) -> i32 {
        <dyn Room>::get_secret_count_body
    }

    /// java: public int getFoundSecretCount() (dispatch: the body for this class)
    fn get_found_secret_count(&self) -> fn(&compat::Shared<dyn Room>) -> i32 {
        <dyn Room>::get_found_secret_count_body
    }

}

impl dyn Room {
    /// java: public Type getType()
    pub fn get_type_body(__this: &compat::Shared<dyn Room>) -> Type {
        return compat::read(__this, |__t| __t.room_base().r#type);
    }

    /// java: public Set<Vector2ic> getSegments()
    pub fn get_segments_body(__this: &compat::Shared<dyn Room>) -> std::collections::HashSet<Vector2i> {
        return compat::read(__this, |__t| __t.room_base().segments.clone());
    }

    /// java: public Shape getShape()
    pub fn get_shape_body(__this: &compat::Shared<dyn Room>) -> Shape {
        return compat::read(__this, |__t| __t.room_base().shape.clone()).unwrap();
    }

    /// java: public @Nullable Vector2ic getPhysicalCornerPos()
    pub fn get_physical_corner_pos_body(__this: &compat::Shared<dyn Room>) -> Option<Vector2i> {
        return compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone());
    }

    /// java: public boolean isMatched()
    pub fn is_matched_body(__this: &compat::Shared<dyn Room>) -> bool {
        return (compat::read(__this, |__t| __t.room_base().match_state) == MatchState::DoubleChecking || compat::read(__this, |__t| __t.room_base().match_state) == MatchState::Matched);
    }

    /// java: public @Nullable String getName()
    pub fn get_name_body(__this: &compat::Shared<dyn Room>) -> Option<String> {
        return compat::read(__this, |__t| __t.room_base().name.clone());
    }

    /// java: public @Nullable Direction getDirection()
    pub fn get_direction_body(__this: &compat::Shared<dyn Room>) -> Option<Direction> {
        return compat::read(__this, |__t| __t.room_base().direction.clone());
    }

    /// java: @Override public String toString()
    pub fn to_string_body(__this: &compat::Shared<dyn Room>) -> String {
        return compat::read(__this, |__t| std::string::ToString::to_string(__t.room_base()));
    }

    /// java: private Shape determineShape(IntSortedSet segmentsX, IntSortedSet segmentsY)
    pub fn determine_shape_body(__this: &compat::Shared<dyn Room>, segments_x: &std::collections::BTreeSet<i32>, segments_y: &std::collections::BTreeSet<i32>) -> Shape {
        return RoomBase::determine_shape_4(compat::read(__this, |__t| __t.room_base().r#type), &compat::read(__this, |__t| __t.room_base().segments.clone()), segments_x, segments_y);
    }

    /// java: private List<MutableTriple<Direction, Vector2ic, List<String>>> getPossibleRooms(IntSortedSet segmentsX, IntSortedSet segmentsY)
    pub fn get_possible_rooms_body(__this: &compat::Shared<dyn Room>, segments_x: &std::collections::BTreeSet<i32>, segments_y: &std::collections::BTreeSet<i32>) -> Vec<crate::unresolved::MutableTripleOf<Direction, Vector2i, Vec<String>>> {
        if compat::read(__this, |__t| __t.room_base().rooms_data.clone()).is_none() {
            return vec![];
        }
        let mut possible_direction_rooms: Vec<String> = __this.lock().borrow().room_base().rooms_data.as_ref().unwrap().keys().cloned().collect::<Vec<_>>();
        let mut possible_rooms: Vec<crate::unresolved::MutableTripleOf<Direction, Vector2i, Vec<String>>> = Vec::new();
        for direction in ({ let __f = compat::dispatch(__this, |__t| __t.get_possible_directions()); __f(__this, segments_x, segments_y) }) {
            possible_rooms.push(crate::unresolved::MutableTripleOf::of(direction, DungeonMapUtils::get_physical_corner_pos(direction, segments_x, segments_y), possible_direction_rooms.clone()));
        }
        return possible_rooms;
    }

    /// java: protected Direction[] getPossibleDirections(IntSortedSet segmentsX, IntSortedSet segmentsY)
    pub fn get_possible_directions_body(__this: &compat::Shared<dyn Room>, segments_x: &std::collections::BTreeSet<i32>, segments_y: &std::collections::BTreeSet<i32>) -> Vec<Direction> {
        return match compat::read(__this, |__t| __t.room_base().shape.clone()) {
            Some(Shape::OneByOne) | Some(Shape::TwoByTwo) | Some(Shape::Puzzle) | Some(Shape::Trap) | Some(Shape::Miniboss) => Direction::values(),
            Some(Shape::OneByTwo) | Some(Shape::OneByThree) | Some(Shape::OneByFour) => 'yield_1: {
                if segments_x.len() > 1 && segments_y.len() == 1 {
                    break 'yield_1 vec![Direction::Nw, Direction::Se];
                } else if segments_x.len() == 1 && segments_y.len() > 1 {
                    break 'yield_1 vec![Direction::Ne, Direction::Sw];
                }
                panic!("IllegalArgumentException: Shape {} does not match segments: {}", __this.lock().borrow().room_base().shape.as_ref().unwrap().fields().shape, RoomBase::segments_string(&compat::read(__this, |__t| __t.room_base().segments.clone())));
            }
            Some(Shape::LShape) => 'yield_2: {
                let segments = compat::read(__this, |__t| __t.room_base().segments.clone());
                let (first_x, last_x) = (*segments_x.first().unwrap(), *segments_x.last().unwrap());
                let (first_y, last_y) = (*segments_y.first().unwrap(), *segments_y.last().unwrap());
                if !segments.contains(&Vector2i::new(first_x, first_y)) {
                    break 'yield_2 vec![Direction::Sw];
                } else if !segments.contains(&Vector2i::new(first_x, last_y)) {
                    break 'yield_2 vec![Direction::Se];
                } else if !segments.contains(&Vector2i::new(last_x, first_y)) {
                    break 'yield_2 vec![Direction::Nw];
                } else if !segments.contains(&Vector2i::new(last_x, last_y)) {
                    break 'yield_2 vec![Direction::Ne];
                }
                panic!("IllegalArgumentException: Shape {} does not match segments: {}", __this.lock().borrow().room_base().shape.as_ref().unwrap().fields().shape, RoomBase::segments_string(&segments));
            }
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// java: protected void addCustomWaypoint(CommandContext<FabricClientCommandSource> context, BlockPos pos)
    pub fn add_custom_waypoint_body(__this: &compat::Shared<dyn Room>, context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, pos: &BlockPos) {
        let mut secret_index: i32 = IntegerArgumentType::get_integer(context, "secretIndex");
        let mut category: crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category = CategoryArgumentType::get_category(context, "category");
        // (`ComponentArgument` parses to a `Component`, the port's `FormattedText`)
        let mut waypoint_name: MutableComponent = compat::component_of(context.get_argument::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>("name").cloned().unwrap());
        ({ let __f = compat::dispatch(__this, |__t| __t.add_custom_waypoint_4()); __f(__this, secret_index, category, &waypoint_name, pos) });
        // java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatableEscape("skyblocker.dungeons.secrets.customWaypointAdded", pos.getX(), pos.getY(), pos.getZ(), name, secretIndex, category, waypointName)));
        let name: String = compat::read(__this, |__t| __t.room_base().name.clone()).unwrap_or_else(|| "null".to_string());
        compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable_args("skyblocker.dungeons.secrets.customWaypointAdded", vec![
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_x().to_string()),
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_y().to_string()),
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_z().to_string()),
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(name),
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(secret_index.to_string()),
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(category.to_string()),
            mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(waypoint_name.clone()),
        ])));
    }

    /// java: @SuppressWarnings("JavadocReference") private void addCustomWaypoint(int secretIndex, SecretWaypoint.Category category, Component waypointName, BlockPos pos)
    pub fn add_custom_waypoint_4_body(__this: &compat::Shared<dyn Room>, mut secret_index: i32, mut category: crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category, waypoint_name: &MutableComponent, pos: &BlockPos) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_matched()); __f(__this) }) {
            return;
        }
        let mut waypoint: compat::Shared<SecretWaypoint> = SecretWaypoint::new2(secret_index, category, waypoint_name.clone(), pos.clone());
        DungeonManager::add_custom_waypoint(compat::read(__this, |__t| __t.room_base().name.clone()).unwrap(), waypoint.clone());
        let __this_1 = compat::RenderThread::new(__this.clone());
        for r in DungeonManager::get_rooms_stream().filter(|r| (compat::read(__this_1.get(), |__t| __t.room_base().name.clone()).as_deref() == ({ let __f = compat::dispatch(r, |__t| __t.get_name()); __f(r) }).as_deref())) {
            ({ let __f = compat::dispatch(&r, |__t| __t.add_custom_waypoint_1()); __f(&r, &waypoint) });
        }
    }

    /// java: private void addCustomWaypoint(SecretWaypoint relativeWaypoint)
    pub fn add_custom_waypoint_1_body(__this: &compat::Shared<dyn Room>, relative_waypoint: &compat::Shared<SecretWaypoint>) {
        let mut actual_waypoint: compat::Shared<SecretWaypoint> = SecretWaypoint::relative_to_actual(relative_waypoint, &__this.clone());
        // secretWaypoints.put(actualWaypoint.secretIndex, actualWaypoint.pos, actualWaypoint);
        // (Guava's `put` rejects a null column: `checkNotNull`)
        let (secret_index, pos) = compat::read(&actual_waypoint, |__t| (__t.secret_index, __t.pos.clone().expect("NullPointerException")));
        compat::write(__this, |__t| __t.room_base_mut().secret_waypoints.put(secret_index, pos, actual_waypoint.clone()));
    }

    /// java: protected void removeCustomWaypoint(CommandContext<FabricClientCommandSource> context, BlockPos pos)
    pub fn remove_custom_waypoint_body(__this: &compat::Shared<dyn Room>, context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, pos: &BlockPos) {
        let mut waypoint: Option<compat::Shared<SecretWaypoint>> = ({ let __f = compat::dispatch(__this, |__t| __t.remove_custom_waypoint_1()); __f(__this, pos) });
        if let Some(waypoint) = &waypoint {
            // java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.dungeons.secrets.customWaypointRemoved", pos.getX(), pos.getY(), pos.getZ(), name, waypoint.secretIndex, waypoint.category.getSerializedName(), waypoint.getName())));
            let name: String = compat::read(__this, |__t| __t.room_base().name.clone()).unwrap_or_else(|| "null".to_string());
            let (secret_index, category) = compat::read(waypoint, |__t| (__t.secret_index, __t.category.clone().unwrap().get_serialized_name()));
            let named = compat::view(waypoint, |a| a as compat::Shared<dyn crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint>);
            let waypoint_name: MutableComponent = { let __f = compat::dispatch(&named, |__t| __t.get_name()); __f(&named) };
            compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable_args("skyblocker.dungeons.secrets.customWaypointRemoved", vec![
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_x().to_string()),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_y().to_string()),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_z().to_string()),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(name),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(secret_index.to_string()),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(category),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(waypoint_name),
            ])));
        } else {
            // java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.dungeons.secrets.customWaypointNotFound", pos.getX(), pos.getY(), pos.getZ(), name).withColor(TextColor.RED)));
            let name: String = compat::read(__this, |__t| __t.room_base().name.clone()).unwrap_or_else(|| "null".to_string());
            compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable_args("skyblocker.dungeons.secrets.customWaypointNotFound", vec![
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_x().to_string()),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_y().to_string()),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(pos.get_z().to_string()),
                mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(name),
            ]).with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED)));
        }
    }

    /// java: @SuppressWarnings("JavadocReference") private @Nullable SecretWaypoint removeCustomWaypoint(BlockPos pos)
    pub fn remove_custom_waypoint_1_body(__this: &compat::Shared<dyn Room>, pos: &BlockPos) -> Option<compat::Shared<SecretWaypoint>> {
        if compat::read(__this, |__t| __t.room_base().name.clone()).is_none() {
            return None;
        }
        let mut waypoint: Option<compat::Shared<SecretWaypoint>> = DungeonManager::remove_custom_waypoint(compat::read(__this, |__t| __t.room_base().name.clone()).as_deref().unwrap(), pos);
        if let Some(waypoint) = &waypoint {
            let __this_1 = compat::RenderThread::new(__this.clone());
            for r in DungeonManager::get_rooms_stream().filter(|r| (compat::read(__this_1.get(), |__t| __t.room_base().name.clone()).as_deref() == ({ let __f = compat::dispatch(r, |__t| __t.get_name()); __f(r) }).as_deref())) {
                ({ let __f = compat::dispatch(&r, |__t| __t.remove_custom_waypoint_int_block_pos()); __f(&r, compat::read(waypoint, |__t| __t.secret_index), pos) });
            }
        }
        return waypoint;
    }

    /// java: private void removeCustomWaypoint(int secretIndex, BlockPos relativePos)
    pub fn remove_custom_waypoint_int_block_pos_body(__this: &compat::Shared<dyn Room>, mut secret_index: i32, relative_pos: &BlockPos) {
        let mut actual_pos: BlockPos = ({ let __f = compat::dispatch(__this, |__t| __t.relative_to_actual()); __f(__this, relative_pos) });
        // secretWaypoints.remove(secretIndex, actualPos);
        compat::write(__this, |__t| __t.room_base_mut().secret_waypoints.remove(&secret_index, &actual_pos));
    }

    /// java: @SuppressWarnings("JavadocReference") @Override public void tick(Minecraft client)
    pub fn tick_body(__this: &compat::Shared<dyn Room>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        for tickable in compat::read(__this, |__t| __t.room_base().tickables.clone()) {
            ({ let __f = compat::dispatch(&tickable, |__t| __t.tick()); __f(&tickable, client) });
        }
        if !(__this.lock().borrow().room_base().r#type.needs_scanning()) || compat::read(__this, |__t| __t.room_base().match_state) != MatchState::Matching && compat::read(__this, |__t| __t.room_base().match_state) != MatchState::DoubleChecking || !(DungeonManager::is_rooms_loaded()) || compat::read(__this, |__t| __t.room_base().find_room.clone()).is_some() && !(__this.lock().borrow().room_base().find_room.as_ref().unwrap().is_done()) {
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let __this_w1 = std::sync::Arc::downgrade(__this);
        let __this_w2 = std::sync::Arc::downgrade(__this);
        // java: findRoom = CompletableFuture.runAsync(() -> {..}).exceptionally(e -> {..});
        // DIVERGENCE: the search reads and writes this room (not `Send`), so it runs deferred on the render thread
        // (`compat::LocalFuture::supply_local`) rather than on a worker
        { let __v = Some(compat::LocalFuture::<()>::supply_local(move || { let Some(__this_1) = __this_w1.upgrade() else { return Default::default(); }; {
            for pos in BlockPos::between_closed_pos(&mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position().offset(-5, -5, -5)).unwrap(), &mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position().offset(5, 5, 5)).unwrap()) {
                debug_assert!(compat::read((&__this_1), |__t| __t.room_base().checked_blocks.clone()).is_some());
                if (&__this_1).lock().borrow().room_base().segments.contains(&DungeonMapUtils::get_physical_room_pos_vec3i(pos.as_vec3i())) && RoomBase::not_in_doorway(&pos) && BlockPosSet::add(&compat::read((&__this_1), |__t| __t.room_base().checked_blocks.clone()).unwrap(), &pos) && { let __f = compat::dispatch((&__this_1), |__t| __t.check_block()); __f((&__this_1), &mcrust::mcrust::modding::game::level().unwrap(), &pos) } {
                    break;
                }
            }
        } }).exceptionally_async(move |e: String| { let Some(__this_2) = __this_w2.upgrade() else { return Default::default(); }; {
            logger().error(&format!("[Skyblocker Dungeon Secrets] Encountered an unknown exception while matching room {}", compat::read(&__this_2, |__t| std::string::ToString::to_string(__t.room_base()))));
            return Default::default(); // java: return null
        } })); compat::write(__this, |__t| __t.room_base_mut().find_room = __v); }
    }

    /// Room.tick as the dispatch of Tickable.tick for the concrete class `__C`
    pub fn tick_from<__C: Room + 'static>(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn Room>::tick_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Room>), __p0)
    }

    /// java: protected boolean checkBlock(ClientLevel world, BlockPos pos)
    pub fn check_block_body(__this: &compat::Shared<dyn Room>, world: &ClientLevel, pos: &BlockPos) -> bool {
        // byte id = DungeonManager.NUMERIC_ID.getByte(BuiltInRegistries.BLOCK.getKey(world.getBlockState(pos).getBlock()).toString());
        // (`getByte` of a missing key is the map's default return value, 0)
        let block_key: String = mcrust::net::minecraft::core::registries::built_in_registries::block().get_key_defaulted(&world.get_block_state_typed(pos).get_block().erased_obj()).to_string();
        let mut id: i8 = dungeon_manager::NUMERIC_ID.get(&block_key).copied().unwrap_or(0);
        if (id as i32) == 0 {
            return false;
        }
        debug_assert!(compat::read(__this, |__t| __t.room_base().possible_rooms.is_some()));
        // the triples are taken out for the loop: `posIdToInt` is virtual and reads the room again
        let mut possible_rooms = compat::write(__this, |__t| __t.room_base_mut().possible_rooms.take()).unwrap();
        let rooms_data = compat::read(__this, |__t| __t.room_base().rooms_data.clone());
        for direction_rooms in possible_rooms.iter_mut() {
            // int block = posIdToInt(DungeonMapUtils.actualToRelative(directionRooms.getLeft(), directionRooms.getMiddle(), pos), id);
            let mut block: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.pos_id_to_int()); __f(__this, &DungeonMapUtils::actual_to_relative(direction_rooms.get_left(), &direction_rooms.get_middle(), pos), id) });
            let mut possible_direction_rooms: Vec<String> = Vec::new();
            for room in direction_rooms.right.iter() {
                debug_assert!(rooms_data.is_some());
                // Arrays.binarySearch(roomsData.get(room), block) >= 0
                if rooms_data.as_ref().unwrap().get(room).expect("NullPointerException: roomsData.get(room)").binary_search(&block).is_ok() {
                    possible_direction_rooms.push(room.clone());
                }
            }
            direction_rooms.set_right(possible_direction_rooms);
        }
        // int matchingRoomsSize = possibleRooms.stream().map(Triple::getRight).mapToInt(Collection::size).sum();
        let mut matching_rooms_size: i32 = possible_rooms.iter().map(|t| t.right.len() as i32).sum();
        let one_room = possible_rooms.iter().find(|direction_rooms| direction_rooms.right.len() == 1).cloned();
        compat::write(__this, |__t| __t.room_base_mut().possible_rooms = Some(possible_rooms));
        if matching_rooms_size == 0 {
            { let __v = MatchState::Failed; compat::write(__this, |__t| __t.room_base_mut().match_state = __v); }
            debug_assert!(compat::read(__this, |__t| __t.room_base().checked_blocks.clone()).is_some());
            logger().warn(&format!("[Skyblocker Dungeon Secrets] No dungeon room matched after checking {} block(s) including double checking {} block(s)", BlockPosSet::size(&compat::read(__this, |__t| __t.room_base().checked_blocks.clone()).unwrap()), compat::read(__this, |__t| __t.room_base().double_check_blocks)));
            let __this_2 = compat::RenderThread::new(__this.clone());
            RenderHelper::run_on_render_thread(Arc::new(move || {
                let __this_1 = compat::RenderThread::new(__this_2.get().clone());
                ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { { let __v = MatchState::Matching; compat::write(__this_1.get(), |__t| __t.room_base_mut().match_state = __v); } }), 50) });
                ({ let __f = compat::dispatch(__this_2.get(), |__t| __t.reset()); __f(__this_2.get()) });
            }));
            return true;
        } else if matching_rooms_size == 1 {
            if compat::read(__this, |__t| __t.room_base().match_state) == MatchState::Matching {
                // If one room matches, load the secrets for that room and set state to double-checking.
                compat::write(__this, |__t| __t.room_base_mut().match_state = MatchState::DoubleChecking);
                let direction_room = one_room.expect("NoSuchElementException");
                compat::write(__this, |__t| {
                    let base = __t.room_base_mut();
                    base.name = Some(direction_room.right[0].clone());
                    base.direction = Some(direction_room.left);
                    base.physical_corner_pos = Some(direction_room.middle);
                });
                debug_assert!(compat::read(__this, |__t| __t.room_base().checked_blocks.is_some()));
                logger().info(&format!("[Skyblocker Dungeon Secrets] Room {} matched after checking {} block(s), starting double checking", compat::opt_str(&compat::read(__this, |__t| __t.room_base().name.clone())), BlockPosSet::size(&compat::read(__this, |__t| __t.room_base().checked_blocks.clone()).unwrap())));
                let __this_3 = compat::RenderThread::new(__this.clone());
                RenderHelper::run_on_render_thread(Arc::new(move || {
                    ({ let __f = compat::dispatch(__this_3.get(), |__t| __t.room_matched()); __f(__this_3.get()) });
                }));
                return false;
            } else if compat::read(__this, |__t| __t.room_base().match_state) == MatchState::DoubleChecking && {
                // ++doubleCheckBlocks >= 10
                let double_check_blocks = compat::write(__this, |__t| {
                    __t.room_base_mut().double_check_blocks += 1;
                    __t.room_base().double_check_blocks
                });
                double_check_blocks >= 10
            } {
                // If double-checked, set state to matched and discard the no longer needed fields.
                compat::write(__this, |__t| __t.room_base_mut().match_state = MatchState::Matched);
                debug_assert!(compat::read(__this, |__t| __t.room_base().checked_blocks.is_some()));
                logger().info(&format!("[Skyblocker Dungeon Secrets] Room {} confirmed after checking {} block(s) including double checking {} block(s)", compat::opt_str(&compat::read(__this, |__t| __t.room_base().name.clone())), BlockPosSet::size(&compat::read(__this, |__t| __t.room_base().checked_blocks.clone()).unwrap()), compat::read(__this, |__t| __t.room_base().double_check_blocks)));
                let __this_3 = compat::RenderThread::new(__this.clone());
                RenderHelper::run_on_render_thread(Arc::new(move || {
                    (dungeon_events::room_matched_invoker())(__this_3.get().clone());
                    ({ let __f = compat::dispatch(__this_3.get(), |__t| __t.discard()); __f(__this_3.get()) });
                }));
                return true;
            }
            return false;
        } else {
            debug_assert!(compat::read(__this, |__t| __t.room_base().checked_blocks.clone()).is_some());
            logger().debug(&format!("[Skyblocker Dungeon Secrets] {} room(s) remaining after checking {} block(s)", matching_rooms_size, BlockPosSet::size(&compat::read(__this, |__t| __t.room_base().checked_blocks.clone()).unwrap())));
            return false;
        }
    }

    /// java: protected int posIdToInt(BlockPos pos, byte id)
    pub fn pos_id_to_int_body(__this: &compat::Shared<dyn Room>, pos: &BlockPos, mut id: i8) -> i32 {
        return (((pos.get_x().wrapping_shl((24) as u32) | pos.get_y().wrapping_shl((16) as u32)) | pos.get_z().wrapping_shl((8) as u32)) | (id as i32));
    }

    /// java: @SuppressWarnings("JavadocReference") private void roomMatched()
    pub fn room_matched_body(__this: &compat::Shared<dyn Room>) {
        let mut name: Option<String> = compat::read(__this, |__t| __t.room_base().name.clone());
        let mut direction: Option<Direction> = compat::read(__this, |__t| __t.room_base().direction.clone());
        let mut physical_corner_pos: Option<Vector2i> = compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone());
        if name.is_none() || direction.is_none() || physical_corner_pos.is_none() {
            logger().warn(&format!("[Skyblocker Dungeon Secrets] Room matched called with invalid fields: matchState={}, name={}, direction={}, physicalCornerPos={}", compat::jstring(&compat::read(__this, |__t| __t.room_base().match_state)), compat::opt_str(&name), compat::opt_str(&direction), physical_corner_pos.map_or("null".to_string(), |p| compat::joml_string(&[p.x() as f64, p.y() as f64]))));
            return;
        }
        let mut room_waypoints: Option<Vec<RoomWaypoint>> = DungeonManager::get_room_waypoints(name.as_deref().unwrap());
        if let Some(room_waypoints) = &room_waypoints {
            for waypoint in room_waypoints.iter() {
                let mut secret_name: String = waypoint.secret_name();
                let mut secret_index_matcher: compat::regex::Matcher = SECRET_INDEX.matcher(&secret_name);
                let mut secret_index: i32 = (if secret_index_matcher.find() { secret_index_matcher.group(1).unwrap().parse::<i32>().expect("NumberFormatException") } else { 0 });
                let mut pos: BlockPos = DungeonMapUtils::relative_to_actual(direction.clone().unwrap(), physical_corner_pos.as_ref().unwrap(), waypoint);
                // secretWaypoints.put(secretIndex, pos, new SecretWaypoint(secretIndex, waypoint.category(), secretName, pos));
                let __w = SecretWaypoint::new(secret_index, waypoint.category(), secret_name.clone(), pos.clone());
                compat::write(__this, |__t| __t.room_base_mut().secret_waypoints.put(secret_index, pos.clone(), __w));
            }
        }
        for __a0 in DungeonManager::get_custom_waypoints(name.as_deref().unwrap()).values().cloned() {
            ({ let __f = compat::dispatch(__this, |__t| __t.add_custom_waypoint_1()); __f(__this, &__a0) });
        }
        { let __v = Utils::parse_int(&compat::regex::replace_all(&name.as_ref().unwrap(), "\\D", "")).unwrap_or(-1); compat::write(__this, |__t| __t.room_base_mut().max_secrets = __v); }
    }

    /// java: protected void reset()
    pub fn reset_body(__this: &compat::Shared<dyn Room>) {
        // IntSortedSet segmentsX = IntSortedSets.unmodifiable(new IntRBTreeSet(segments.stream().mapToInt(Vector2ic::x).toArray()));
        let mut segments_x: std::collections::BTreeSet<i32> = compat::read(&__this, |__t| __t.room_base().segments.iter().map(|s| s.x()).collect());
        // IntSortedSet segmentsY = IntSortedSets.unmodifiable(new IntRBTreeSet(segments.stream().mapToInt(Vector2ic::y).toArray()));
        let mut segments_y: std::collections::BTreeSet<i32> = compat::read(&__this, |__t| __t.room_base().segments.iter().map(|s| s.y()).collect());
        { let __v = Some(({ let __f = compat::dispatch(__this, |__t| __t.get_possible_rooms()); __f(__this, &segments_x, &segments_y) })); compat::write(__this, |__t| __t.room_base_mut().possible_rooms = __v); }
        { let __v = Some(BlockPosSet::new()); compat::write(__this, |__t| __t.room_base_mut().checked_blocks = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.room_base_mut().double_check_blocks = __v); }
        // secretWaypoints.clear();
        compat::write(__this, |__t| __t.room_base_mut().secret_waypoints.clear());
        { let __v = None; compat::write(__this, |__t| __t.room_base_mut().name = __v); }
        { let __v = None; compat::write(__this, |__t| __t.room_base_mut().direction = __v); }
        { let __v = None; compat::write(__this, |__t| __t.room_base_mut().physical_corner_pos = __v); }
    }

    /// java: private void discard()
    pub fn discard_body(__this: &compat::Shared<dyn Room>) {
        { let __v = None; compat::write(__this, |__t| __t.room_base_mut().rooms_data = __v); }
        { let __v = None; compat::write(__this, |__t| __t.room_base_mut().possible_rooms = __v); }
        { let __v = None; compat::write(__this, |__t| __t.room_base_mut().checked_blocks = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.room_base_mut().double_check_blocks = __v); }
    }

    /// java: public BlockPos actualToRelative(BlockPos pos)
    pub fn actual_to_relative_body(__this: &compat::Shared<dyn Room>, pos: &BlockPos) -> BlockPos {
        debug_assert!(compat::read(__this, |__t| __t.room_base().direction.clone()).is_some() && compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone()).is_some());
        return DungeonMapUtils::actual_to_relative(compat::read(__this, |__t| __t.room_base().direction.clone()).unwrap(), &compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone()).unwrap(), pos);
    }

    /// java: public Vec3 actualToRelative(Vec3 pos)
    pub fn actual_to_relative_vec3_body(__this: &compat::Shared<dyn Room>, pos: &Vec3) -> Vec3 {
        debug_assert!(compat::read(__this, |__t| __t.room_base().direction.clone()).is_some() && compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone()).is_some());
        return DungeonMapUtils::actual_to_relative_direction_vector2ic_vec3(compat::read(__this, |__t| __t.room_base().direction.clone()).unwrap(), &compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone()).unwrap(), pos);
    }

    /// java: public BlockPos relativeToActual(BlockPos pos)
    pub fn relative_to_actual_body(__this: &compat::Shared<dyn Room>, pos: &BlockPos) -> BlockPos {
        debug_assert!(compat::read(__this, |__t| __t.room_base().direction.clone()).is_some() && compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone()).is_some());
        return DungeonMapUtils::relative_to_actual_direction_vector2ic_block_pos(compat::read(__this, |__t| __t.room_base().direction.clone()).unwrap(), &compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone()).unwrap(), pos);
    }

    /// java: public Vec3 relativeToActual(Vec3 pos)
    pub fn relative_to_actual_vec3_body(__this: &compat::Shared<dyn Room>, pos: &Vec3) -> Vec3 {
        debug_assert!(compat::read(__this, |__t| __t.room_base().direction.clone()).is_some() && compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone()).is_some());
        return DungeonMapUtils::relative_to_actual_direction_vector2ic_vec3(compat::read(__this, |__t| __t.room_base().direction.clone()).unwrap(), &compat::read(__this, |__t| __t.room_base().physical_corner_pos.clone()).unwrap(), pos);
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_body(__this: &compat::Shared<dyn Room>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        for renderable in compat::read(__this, |__t| __t.room_base().renderables.clone()) {
            ({ let __f = compat::dispatch(&renderable, |__t| __t.extract_rendering()); __f(&renderable, collector) });
        }
        if compat::read(&SkyblockerConfigManager::get().dungeons.secret_waypoints, |__t| __t.enable_secret_waypoints) && { let __f = compat::dispatch(__this, |__t| __t.is_matched()); __f(__this) } {
            // java: for (SecretWaypoint secretWaypoint : secretWaypoints.values()) { if (secretWaypoint.shouldRender()) secretWaypoint.extractRendering(collector); }
            for secret_waypoint in compat::read(__this, |__t| __t.room_base().secret_waypoints.values()) {
                let waypoint = secret_as_waypoint(&secret_waypoint);
                if { let __f = compat::dispatch(&waypoint, |__t| __t.should_render()); __f(&waypoint) } {
                    let renderable = compat::view(&secret_waypoint, |a| a as compat::Shared<dyn Renderable>);
                    { let __f = compat::dispatch(&renderable, |__t| __t.extract_rendering()); __f(&renderable, collector) };
                }
            }
        }
    }

    /// Room.extractRendering as the dispatch of Renderable.extractRendering for the concrete class `__C`
    pub fn extract_rendering_from<__C: Room + 'static>(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Room>::extract_rendering_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Room>), __p0)
    }

    /// java: protected void onChatMessage(String message, boolean overlay)
    pub fn on_chat_message_body(__this: &compat::Shared<dyn Room>, message: &str, mut overlay: bool) {
        if CHEST_ALREADY_OPENED == message && (compat::read(__this, |__t| __t.room_base().last_chest_secret_time)).wrapping_add(1000 as i64) > compat::millis() && compat::read(__this, |__t| __t.room_base().last_chest_secret.clone()).is_some() {
            // java: secretWaypoints.column(lastChestSecret).values().stream().filter(SecretWaypoint::needsInteraction).findAny()
            //     .ifPresent(secretWaypoint -> markSecretsFoundAndLogInfo(secretWaypoint, "[Skyblocker Dungeon Secrets] Detected already searched chest interaction, setting secret #{} as found", secretWaypoint.secretIndex));
            let last_chest_secret: BlockPos = compat::read(__this, |__t| __t.room_base().last_chest_secret.clone()).unwrap();
            if let Some(secret_waypoint) = secrets_in_column(__this, &last_chest_secret).into_iter().find(|w| SecretWaypoint::needs_interaction(w)) {
                let secret_index = compat::read(&secret_waypoint, |__t| __t.secret_index);
                mark_found_and_log(__this, &secret_waypoint, "[Skyblocker Dungeon Secrets] Detected already searched chest interaction, setting secret #{} as found", vec![log_arg(secret_index)]);
            }
        }
        if compat::read(__this, |__t| __t.room_base().secret_count_outdated) && overlay {
            ({ let __f = compat::dispatch(__this, |__t| __t.update_secret_count()); __f(__this, message) });
        }
    }

    /// java: protected void updateSecretCount(String message)
    pub fn update_secret_count_body(__this: &compat::Shared<dyn Room>, message: &str) {
        let mut matcher: compat::regex::Matcher = SECRETS.matcher(message);
        if !matcher.find() {
            return;
        }
        { let __v = matcher.group(1).unwrap().parse::<i32>().expect("NumberFormatException"); compat::write(__this, |__t| __t.room_base_mut().secrets_found = __v); }
        { let __v = false; compat::write(__this, |__t| __t.room_base_mut().secret_count_outdated = __v); }
        (dungeon_events::secret_count_updated_invoker())(__this.clone(), false);
    }

    /// java: protected void onUseBlock(Level world, BlockPos pos)
    pub fn on_use_block_body(__this: &compat::Shared<dyn Room>, world: &Level, mut pos: BlockPos) {
        let mut state: Arc<BlockState> = world.get_block_state_typed(&pos);
        if state.is_block(&blocks::blocks().get("CHEST")) || state.is_block(&blocks::blocks().get("TRAPPED_CHEST")) {
            { let __v = Some(pos.clone()); compat::write(__this, |__t| __t.room_base_mut().last_chest_secret = __v); }
            { let __v = compat::millis(); compat::write(__this, |__t| __t.room_base_mut().last_chest_secret_time = __v); }
            // the lambda captures `world`: the client's level (the object `world` is), held by its handle
            let __world = mcrust::mcrust::modding::game::level();
            let __this_1 = compat::RenderThread::new(__this.clone());
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || {
                let Some(world) = __world.as_deref() else {
                    return;
                };
                if !(world.get_block_state_typed(&pos).is_air()) {
                    return;
                }
                // java: secretWaypoints.column(pos).values().stream().filter(SecretWaypoint::needsInteraction).filter(SecretWaypoint::isEnabled).findAny()
                //     .ifPresent(secretWaypoint -> markSecretsFoundAndLogInfo(secretWaypoint, "[Skyblocker Dungeon Secrets] Detected chest block removed, setting secret #{} as found", secretWaypoint.secretIndex));
                let __this = __this_1.get();
                if let Some(secret_waypoint) = secrets_in_column(__this, &pos).into_iter().filter(|w| SecretWaypoint::needs_interaction(w)).find(|w| secret_is_enabled(w)) {
                    let secret_index = compat::read(&secret_waypoint, |__t| __t.secret_index);
                    mark_found_and_log(__this, &secret_waypoint, "[Skyblocker Dungeon Secrets] Detected chest block removed, setting secret #{} as found", vec![log_arg(secret_index)]);
                }
            }), 5) });
        } else if state.is_block(&blocks::blocks().get("PLAYER_HEAD")) || state.is_block(&blocks::blocks().get("PLAYER_WALL_HEAD")) {
            // java: secretWaypoints.column(pos).values().stream().filter(SecretWaypoint::needsInteraction).filter(SecretWaypoint::isEnabled).findAny().ifPresent(secretWaypoint -> {..});
            if let Some(secret_waypoint) = secrets_in_column(__this, &pos).into_iter().filter(|w| SecretWaypoint::needs_interaction(w)).find(|w| secret_is_enabled(w)) {
                let (category, secret_index, name) = compat::read(&secret_waypoint, |__t| (__t.category.clone().unwrap(), __t.secret_index, __t.name.clone().map(|n| n.get_string()).unwrap_or_else(|| "null".to_string())));
                if category == secret_waypoint::Category::RedstoneKey {
                    logger().info(&format!("[Skyblocker Dungeon Secrets] Detected {} interaction, hiding secret #{} waypoint {}", category.to_string(), secret_index, name));
                    let waypoint = secret_as_waypoint(&secret_waypoint);
                    { let __f = compat::dispatch(&waypoint, |__t| __t.set_found()); __f(&waypoint) };
                } else {
                    mark_found_and_log(__this, &secret_waypoint, "[Skyblocker Dungeon Secrets] Detected {} interaction, setting secret #{} as found", vec![log_arg(category.to_string()), log_arg(secret_index)]);
                }
            }
        } else if state.is_block(&blocks::blocks().get("REDSTONE_BLOCK")) {
            // java: secretWaypoints.column(pos.above()).values().stream().filter(SecretWaypoint::needsInteraction).filter(SecretWaypoint::isEnabled).findAny()
            //     .ifPresent(secretWaypoint -> markSecretsFoundAndLogInfo(secretWaypoint, "[Skyblocker Dungeon Secrets] Detected {} interaction, setting secret #{} as found", secretWaypoint.category, secretWaypoint.secretIndex));
            if let Some(secret_waypoint) = secrets_in_column(__this, &pos.above()).into_iter().filter(|w| SecretWaypoint::needs_interaction(w)).find(|w| secret_is_enabled(w)) {
                let (category, secret_index) = compat::read(&secret_waypoint, |__t| (__t.category.clone().unwrap(), __t.secret_index));
                mark_found_and_log(__this, &secret_waypoint, "[Skyblocker Dungeon Secrets] Detected {} interaction, setting secret #{} as found", vec![log_arg(category.to_string()), log_arg(secret_index)]);
            }
        } else if state.is_block(&blocks::blocks().get("LEVER")) {
            // java: secretWaypoints.column(pos).values().stream().filter(SecretWaypoint::isLever).forEach(SecretWaypoint::setFound);
            for secret_waypoint in secrets_in_column(__this, &pos).into_iter().filter(|w| SecretWaypoint::is_lever(w)) {
                let waypoint = secret_as_waypoint(&secret_waypoint);
                { let __f = compat::dispatch(&waypoint, |__t| __t.set_found()); __f(&waypoint) };
            }
        }
    }

    /// java: protected void onChestOpened(BlockPos pos)
    pub fn on_chest_opened_body(__this: &compat::Shared<dyn Room>, pos: &BlockPos) {
        // java: secretWaypoints.column(pos).values().stream().filter(SecretWaypoint::needsInteraction).filter(SecretWaypoint::isEnabled).findAny()
        //     .ifPresent(secretWaypoint -> markSecretsFoundAndLogInfo(secretWaypoint, "[Skyblocker Dungeon Secrets] Detected chest opened, setting secret #{} as found", secretWaypoint.secretIndex));
        if let Some(secret_waypoint) = secrets_in_column(__this, pos).into_iter().filter(|w| SecretWaypoint::needs_interaction(w)).find(|w| secret_is_enabled(w)) {
            let secret_index = compat::read(&secret_waypoint, |__t| __t.secret_index);
            mark_found_and_log(__this, &secret_waypoint, "[Skyblocker Dungeon Secrets] Detected chest opened, setting secret #{} as found", vec![log_arg(secret_index)]);
        }
    }

    /// java: protected void onItemPickup(ItemEntity itemEntity)
    pub fn on_item_pickup_body(__this: &compat::Shared<dyn Room>, item_entity: &ItemEntity) {
        if !secret_waypoint::SECRET_ITEMS.iter().cloned().any(|__a0| compat::component_of(item_entity.get_item().get_hover_name()).get_string().contains(&__a0)) {
            return;
        }
        // java: secretWaypoints.values().stream().filter(SecretWaypoint::needsItemPickup).min(Comparator.comparingDouble(SecretWaypoint.getSquaredDistanceToFunction(itemEntity))).filter(SecretWaypoint.getRangePredicate(itemEntity))
        //     .ifPresent(secretWaypoint -> markSecretsFoundAndLogInfo(secretWaypoint, "[Skyblocker Dungeon Secrets] Detected item {} removed from a {} secret, setting secret #{} as found", itemEntity.getName().getString(), secretWaypoint.category, secretWaypoint.secretIndex));
        let entity: &dyn Entity = item_entity;
        let distance = SecretWaypoint::get_squared_distance_to_function(entity);
        let in_range = SecretWaypoint::get_range_predicate(entity);
        let candidates: Vec<compat::Shared<SecretWaypoint>> = compat::read(__this, |__t| __t.room_base().secret_waypoints.values()).into_iter().filter(|w| SecretWaypoint::needs_item_pickup(w)).collect();
        if let Some(secret_waypoint) = min_by_distance(candidates, &*distance).filter(|w| in_range(w.clone())) {
            let (category, secret_index) = compat::read(&secret_waypoint, |__t| (__t.category.clone().unwrap(), __t.secret_index));
            let item_name: String = compat::component_of(entity.get_name()).get_string();
            mark_found_and_log(__this, &secret_waypoint, "[Skyblocker Dungeon Secrets] Detected item {} removed from a {} secret, setting secret #{} as found", vec![log_arg(item_name), log_arg(category.to_string()), log_arg(secret_index)]);
        }
    }

    /// java: protected void onBatRemoved(AmbientCreature bat)
    pub fn on_bat_removed_body(__this: &compat::Shared<dyn Room>, bat: &dyn AmbientCreature) {
        // java: secretWaypoints.values().stream().filter(SecretWaypoint::isBat).min(Comparator.comparingDouble(SecretWaypoint.getSquaredDistanceToFunction(bat))).filter(SecretWaypoint.getRangePredicate(bat))
        //     .ifPresent(secretWaypoint -> markSecretsFoundAndLogInfo(secretWaypoint, "[Skyblocker Dungeon Secrets] Detected {} killed for a {} secret, setting secret #{} as found", bat.getName().getString(), secretWaypoint.category, secretWaypoint.secretIndex));
        let entity: &dyn Entity = bat;
        let distance = SecretWaypoint::get_squared_distance_to_function(entity);
        let in_range = SecretWaypoint::get_range_predicate(entity);
        let candidates: Vec<compat::Shared<SecretWaypoint>> = compat::read(__this, |__t| __t.room_base().secret_waypoints.values()).into_iter().filter(|w| SecretWaypoint::is_bat(w)).collect();
        if let Some(secret_waypoint) = min_by_distance(candidates, &*distance).filter(|w| in_range(w.clone())) {
            let (category, secret_index) = compat::read(&secret_waypoint, |__t| (__t.category.clone().unwrap(), __t.secret_index));
            let bat_name: String = compat::component_of(entity.get_name()).get_string();
            mark_found_and_log(__this, &secret_waypoint, "[Skyblocker Dungeon Secrets] Detected {} killed for a {} secret, setting secret #{} as found", vec![log_arg(bat_name), log_arg(category.to_string()), log_arg(secret_index)]);
        }
    }

    /// java: private void markSecretsFoundAndLogInfo(SecretWaypoint secretWaypoint, String msg, Object... args)
    pub fn mark_secrets_found_and_log_info_body(__this: &compat::Shared<dyn Room>, secret_waypoint: &compat::Shared<SecretWaypoint>, msg: &str, args: &Vec<compat::Object>) {
        ({ let __f = compat::dispatch(__this, |__t| __t.mark_secrets_and_log_info()); __f(__this, secret_waypoint, true, msg, args) });
    }

    /// java: private void markSecretsAndLogInfo(SecretWaypoint secretWaypoint, boolean found, String msg, Object... args)
    pub fn mark_secrets_and_log_info_body(__this: &compat::Shared<dyn Room>, secret_waypoint: &compat::Shared<SecretWaypoint>, mut found: bool, msg: &str, args: &Vec<compat::Object>) {
        if found {
            (dungeon_events::secret_found_invoker())(__this.clone(), secret_waypoint.clone());
            { let __v = true; compat::write(__this, |__t| __t.room_base_mut().secret_count_outdated = __v); }
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.mark_secrets()); __f(__this, compat::read(secret_waypoint, |__t| __t.secret_index), found) });
        // java: DungeonManager.LOGGER.info(msg, args); - SLF4J's `{}` placeholders, filled in order
        logger().info(&slf4j_format(msg, args));
    }

    /// java: protected int getIndexByWaypointHash(int waypointHash)
    pub fn get_index_by_waypoint_hash_body(__this: &compat::Shared<dyn Room>, mut waypoint_hash: i32) -> i32 {
        for i in 0..({ let __f = compat::dispatch(__this, |__t| __t.get_secret_count()); __f(__this) }) {
            // java: !secretWaypoints.containsRow(i)
            if !compat::read(__this, |__t| __t.room_base().secret_waypoints.contains_row(&i)) {
                continue;
            }
            //   java: for (SecretWaypoint waypoint : secretWaypoints.row(i).values()) {
            //   java: 				if (!waypoint.isEnabled()) continue;
            //   java: 				if (waypoint.hashCode() == waypointHash) return i;
            //   java: 			}
            for (_, waypoint) in compat::read(__this, |__t| __t.room_base().secret_waypoints.row(&i)) {
                let waypoint = compat::view(&waypoint, |a| a as compat::Shared<dyn crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint>);
                if !({ let __f = compat::dispatch(&waypoint, |__t| __t.is_enabled()); __f(&waypoint) }) {
                    continue;
                }
                if { let __f = compat::dispatch(&waypoint, |__t| __t.hash_code()); __f(&waypoint) } == waypoint_hash {
                    return i;
                }
            }
        }
        return -1;
    }

    /// java: protected boolean markSecrets(int secretIndex, boolean found)
    pub fn mark_secrets_body(__this: &compat::Shared<dyn Room>, mut secret_index: i32, mut found: bool) -> bool {
        // java: Map<BlockPos, SecretWaypoint> secret = secretWaypoints.row(secretIndex);
        let mut secret: Vec<(BlockPos, compat::Shared<SecretWaypoint>)> = compat::read(__this, |__t| __t.room_base().secret_waypoints.row(&secret_index));
        if secret.is_empty() {
            return false;
        } else {
            //   java: secret.values().forEach(found ? SecretWaypoint::setFound : SecretWaypoint::setMissing);
            for (_, waypoint) in secret.iter() {
                let waypoint = compat::view(waypoint, |a| a as compat::Shared<dyn crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint>);
                if found {
                    ({ let __f = compat::dispatch(&waypoint, |__t| __t.set_found()); __f(&waypoint) });
                } else {
                    ({ let __f = compat::dispatch(&waypoint, |__t| __t.set_missing()); __f(&waypoint) });
                }
            }
            return true;
        }
    }

    /// java: protected void markAllSecrets(boolean found)
    pub fn mark_all_secrets_body(__this: &compat::Shared<dyn Room>, mut found: bool) {
        // java: secretWaypoints.values().forEach(found ? SecretWaypoint::setFound : SecretWaypoint::setMissing);
        for secret_waypoint in compat::read(__this, |__t| __t.room_base().secret_waypoints.values()) {
            let waypoint = secret_as_waypoint(&secret_waypoint);
            if found {
                { let __f = compat::dispatch(&waypoint, |__t| __t.set_found()); __f(&waypoint) };
            } else {
                { let __f = compat::dispatch(&waypoint, |__t| __t.set_missing()); __f(&waypoint) };
            }
        }
    }

    /// java: public int getMaxSecretCount()
    pub fn get_max_secret_count_body(__this: &compat::Shared<dyn Room>) -> i32 {
        return compat::read(__this, |__t| __t.room_base().max_secrets);
    }

    /// java: protected int getSecretCount()
    pub fn get_secret_count_body(__this: &compat::Shared<dyn Room>) -> i32 {
        // java: return secretWaypoints.rowMap().size();
        compat::read(__this, |__t| __t.room_base().secret_waypoints.row_map().len() as i32)
    }

    /// java: public int getFoundSecretCount()
    pub fn get_found_secret_count_body(__this: &compat::Shared<dyn Room>) -> i32 {
        return compat::read(__this, |__t| __t.room_base().secrets_found);
    }

}

impl Room for RoomBase {
    fn room_base(&self) -> &RoomBase {
        &self
    }
    fn room_base_mut(&mut self) -> &mut RoomBase {
        self
    }

}

impl Tickable for RoomBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @SuppressWarnings("JavadocReference") @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn Room>::tick_from::<Self>
    }

}

impl Renderable for RoomBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Room>::extract_rendering_from::<Self>
    }

}

/// java: public enum Type implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Entrance,
    Room,
    Puzzle,
    Trap,
    Miniboss,
    Fairy,
    Blood,
    Unknown,
}

impl Type {
    pub const VALUES: [Type; 8] = [Type::Entrance, Type::Room, Type::Puzzle, Type::Trap, Type::Miniboss, Type::Fairy, Type::Blood, Type::Unknown];

    /// java: values()
    pub fn values() -> Vec<Type> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Type::Entrance => "ENTRANCE",
            Type::Room => "ROOM",
            Type::Puzzle => "PUZZLE",
            Type::Trap => "TRAP",
            Type::Miniboss => "MINIBOSS",
            Type::Fairy => "FAIRY",
            Type::Blood => "BLOOD",
            Type::Unknown => "UNKNOWN",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Type {
        match name {
            "ENTRANCE" => Type::Entrance,
            "ROOM" => Type::Room,
            "PUZZLE" => Type::Puzzle,
            "TRAP" => Type::Trap,
            "MINIBOSS" => Type::Miniboss,
            "FAIRY" => Type::Fairy,
            "BLOOD" => Type::Blood,
            "UNKNOWN" => Type::Unknown,
            _ => panic!("IllegalArgumentException: No enum constant Type.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static TypeFields {
        static FIELDS: LazyLock<Vec<TypeFields>> = LazyLock::new(|| vec![
            TypeFields::new(mcrust::net::minecraft::world::level::material::map_color::PLANT.get_packed_id(mcrust::net::minecraft::world::level::material::map_color::Brightness::High), "Entrance".to_string()),
            TypeFields::new(mcrust::net::minecraft::world::level::material::map_color::COLOR_ORANGE.get_packed_id(mcrust::net::minecraft::world::level::material::map_color::Brightness::Lowest), "Room".to_string()),
            TypeFields::new(mcrust::net::minecraft::world::level::material::map_color::COLOR_MAGENTA.get_packed_id(mcrust::net::minecraft::world::level::material::map_color::Brightness::High), "Puzzle".to_string()),
            TypeFields::new(mcrust::net::minecraft::world::level::material::map_color::COLOR_ORANGE.get_packed_id(mcrust::net::minecraft::world::level::material::map_color::Brightness::High), "Trap".to_string()),
            TypeFields::new(mcrust::net::minecraft::world::level::material::map_color::COLOR_YELLOW.get_packed_id(mcrust::net::minecraft::world::level::material::map_color::Brightness::High), "Miniboss".to_string()),
            TypeFields::new(mcrust::net::minecraft::world::level::material::map_color::COLOR_PINK.get_packed_id(mcrust::net::minecraft::world::level::material::map_color::Brightness::High), "Fairy".to_string()),
            TypeFields::new(mcrust::net::minecraft::world::level::material::map_color::FIRE.get_packed_id(mcrust::net::minecraft::world::level::material::map_color::Brightness::High), "Blood".to_string()),
            TypeFields::new(mcrust::net::minecraft::world::level::material::map_color::COLOR_GRAY.get_packed_id(mcrust::net::minecraft::world::level::material::map_color::Brightness::Normal), "Unknown".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: private boolean needsScanning()
    pub fn needs_scanning(&self) -> bool {
        return match *self {
            Type::Room | Type::Puzzle | Type::Trap | Type::Miniboss => true,
            _ => false,
        };
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().name.clone();
    }

}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Type constant
pub struct TypeFields {
    pub color: i8,
    pub name: String,
}

impl TypeFields {
    fn new(mut color: i8, mut name: String) -> Self {
        let mut this = Self {
            color: 0,
            name: Default::default(),
        };
        this.color = color;
        this.name = name.clone();
        this
    }

}

/// java: public enum Shape implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Shape {
    OneByOne,
    OneByTwo,
    OneByThree,
    OneByFour,
    LShape,
    TwoByTwo,
    Puzzle,
    Trap,
    Miniboss,
}

impl Shape {
    pub const VALUES: [Shape; 9] = [Shape::OneByOne, Shape::OneByTwo, Shape::OneByThree, Shape::OneByFour, Shape::LShape, Shape::TwoByTwo, Shape::Puzzle, Shape::Trap, Shape::Miniboss];

    /// java: values()
    pub fn values() -> Vec<Shape> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Shape::OneByOne => "ONE_BY_ONE",
            Shape::OneByTwo => "ONE_BY_TWO",
            Shape::OneByThree => "ONE_BY_THREE",
            Shape::OneByFour => "ONE_BY_FOUR",
            Shape::LShape => "L_SHAPE",
            Shape::TwoByTwo => "TWO_BY_TWO",
            Shape::Puzzle => "PUZZLE",
            Shape::Trap => "TRAP",
            Shape::Miniboss => "MINIBOSS",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Shape {
        match name {
            "ONE_BY_ONE" => Shape::OneByOne,
            "ONE_BY_TWO" => Shape::OneByTwo,
            "ONE_BY_THREE" => Shape::OneByThree,
            "ONE_BY_FOUR" => Shape::OneByFour,
            "L_SHAPE" => Shape::LShape,
            "TWO_BY_TWO" => Shape::TwoByTwo,
            "PUZZLE" => Shape::Puzzle,
            "TRAP" => Shape::Trap,
            "MINIBOSS" => Shape::Miniboss,
            _ => panic!("IllegalArgumentException: No enum constant Shape.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static ShapeFields {
        static FIELDS: LazyLock<Vec<ShapeFields>> = LazyLock::new(|| vec![
            ShapeFields::new("1x1".to_string()),
            ShapeFields::new("1x2".to_string()),
            ShapeFields::new("1x3".to_string()),
            ShapeFields::new("1x4".to_string()),
            ShapeFields::new("L-shape".to_string()),
            ShapeFields::new("2x2".to_string()),
            ShapeFields::new("puzzle".to_string()),
            ShapeFields::new("trap".to_string()),
            ShapeFields::new("miniboss".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return self.fields().shape.clone();
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().shape.clone();
    }

}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Shape::to_string(self))
    }
}

/// the constructor data of each Shape constant
pub struct ShapeFields {
    pub shape: String,
}

impl ShapeFields {
    fn new(mut shape: String) -> Self {
        let mut this = Self {
            shape: Default::default(),
        };
        this.shape = shape.clone();
        this
    }

}

/// java: public enum Direction implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Nw,
    Ne,
    Sw,
    Se,
}

impl Direction {
    pub const VALUES: [Direction; 4] = [Direction::Nw, Direction::Ne, Direction::Sw, Direction::Se];

    /// java: values()
    pub fn values() -> Vec<Direction> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Direction::Nw => "NW",
            Direction::Ne => "NE",
            Direction::Sw => "SW",
            Direction::Se => "SE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Direction {
        match name {
            "NW" => Direction::Nw,
            "NE" => Direction::Ne,
            "SW" => Direction::Sw,
            "SE" => Direction::Se,
            _ => panic!("IllegalArgumentException: No enum constant Direction.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static DirectionFields {
        static FIELDS: LazyLock<Vec<DirectionFields>> = LazyLock::new(|| vec![
            DirectionFields::new("northwest".to_string()),
            DirectionFields::new("northeast".to_string()),
            DirectionFields::new("southwest".to_string()),
            DirectionFields::new("southeast".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().name.clone();
    }

}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Direction constant
pub struct DirectionFields {
    pub name: String,
}

impl DirectionFields {
    fn new(mut name: String) -> Self {
        let mut this = Self {
            name: Default::default(),
        };
        this.name = name.clone();
        this
    }

}

/// java: static class DirectionArgumentType extends StringRepresentableArgument<Direction>
pub struct DirectionArgumentType {
    /// java: extends net.minecraft.commands.arguments.StringRepresentableArgument
    pub base: StringRepresentableArgument<Direction>,
}

impl std::ops::Deref for DirectionArgumentType {
    type Target = StringRepresentableArgument<Direction>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DirectionArgumentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DirectionArgumentType {
    /// java: DirectionArgumentType()
    pub fn new() -> Self {
        //   java: super(CODEC, Direction::values); - the two-argument constructor is `this(codec, values, s -> s)`; the
        // port's is handed the names the codec writes (`CODEC`'s `getSerializedName`) and that identity.
        let mut this: Self = Self {
            base: StringRepresentableArgument::new(Direction::values(), |value: &Direction| value.get_serialized_name(), |id: &str| id.to_string()),
        };
        this
    }

    /// java: static DirectionArgumentType direction()
    pub fn direction() -> DirectionArgumentType {
        return DirectionArgumentType::new();
    }

    /// java: static <S> Direction getDirection(CommandContext<S> context, String name)
    pub fn get_direction<S: 'static>(context: &CommandContext<S>, name: &str) -> Direction {
        return context.get_argument::<Direction>(name).cloned().unwrap();
    }

}

/// java: public enum ClearState
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClearState {
    GreenChecked,
    WhiteChecked,
    Failed,
    Uncleared,
}

impl ClearState {
    pub const VALUES: [ClearState; 4] = [ClearState::GreenChecked, ClearState::WhiteChecked, ClearState::Failed, ClearState::Uncleared];

    /// java: values()
    pub fn values() -> Vec<ClearState> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ClearState::GreenChecked => "GREEN_CHECKED",
            ClearState::WhiteChecked => "WHITE_CHECKED",
            ClearState::Failed => "FAILED",
            ClearState::Uncleared => "UNCLEARED",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ClearState {
        match name {
            "GREEN_CHECKED" => ClearState::GreenChecked,
            "WHITE_CHECKED" => ClearState::WhiteChecked,
            "FAILED" => ClearState::Failed,
            "UNCLEARED" => ClearState::Uncleared,
            _ => panic!("IllegalArgumentException: No enum constant ClearState.{name}"),
        }
    }

}

impl std::fmt::Display for ClearState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: protected enum MatchState
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MatchState {
    Matching,
    DoubleChecking,
    Matched,
    Failed,
}

impl MatchState {
    pub const VALUES: [MatchState; 4] = [MatchState::Matching, MatchState::DoubleChecking, MatchState::Matched, MatchState::Failed];

    /// java: values()
    pub fn values() -> Vec<MatchState> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            MatchState::Matching => "MATCHING",
            MatchState::DoubleChecking => "DOUBLE_CHECKING",
            MatchState::Matched => "MATCHED",
            MatchState::Failed => "FAILED",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> MatchState {
        match name {
            "MATCHING" => MatchState::Matching,
            "DOUBLE_CHECKING" => MatchState::DoubleChecking,
            "MATCHED" => MatchState::Matched,
            "FAILED" => MatchState::Failed,
            _ => panic!("IllegalArgumentException: No enum constant MatchState.{name}"),
        }
    }

}

impl std::fmt::Display for MatchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// A secret waypoint as the `Waypoint` it extends (`setFound`, `isEnabled`, `shouldRender` dispatch through it).
fn secret_as_waypoint(secret_waypoint: &compat::Shared<SecretWaypoint>) -> compat::Shared<dyn crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint> {
    (*compat::view(secret_waypoint, |a| a as compat::Shared<dyn crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint>)).clone()
}

/// `SecretWaypoint::isEnabled` (the `Waypoint` method, virtual).
fn secret_is_enabled(secret_waypoint: &compat::Shared<SecretWaypoint>) -> bool {
    let waypoint = secret_as_waypoint(secret_waypoint);
    let __f = compat::dispatch(&waypoint, |__t| __t.is_enabled());
    __f(&waypoint)
}

/// `secretWaypoints.column(pos).values()` - read out of the room once, so no guard is held while each is used.
fn secrets_in_column(room: &compat::Shared<dyn Room>, pos: &BlockPos) -> Vec<compat::Shared<SecretWaypoint>> {
    compat::read(room, |__t| __t.room_base().secret_waypoints.column(pos)).into_iter().map(|(_, waypoint)| waypoint).collect()
}

/// `stream.min(Comparator.comparingDouble(f))`: the first of the smallest (`Stream.min` keeps the earlier of two equal ones).
fn min_by_distance(candidates: Vec<compat::Shared<SecretWaypoint>>, distance: &dyn Fn(compat::Shared<SecretWaypoint>) -> f64) -> Option<compat::Shared<SecretWaypoint>> {
    let mut best: Option<(f64, compat::Shared<SecretWaypoint>)> = None;
    for candidate in candidates {
        let d = distance(candidate.clone());
        if best.as_ref().is_none_or(|(b, _)| d.total_cmp(b) == std::cmp::Ordering::Less) {
            best = Some((d, candidate));
        }
    }
    best.map(|(_, w)| w)
}

/// One `Object... args` value of a log call.
fn log_arg<T: std::fmt::Display>(value: T) -> compat::Object {
    Arc::new(value.to_string())
}

/// `markSecretsFoundAndLogInfo(secretWaypoint, msg, args)` through the room's virtual.
fn mark_found_and_log(room: &compat::Shared<dyn Room>, secret_waypoint: &compat::Shared<SecretWaypoint>, msg: &str, args: Vec<compat::Object>) {
    let __f = compat::dispatch(room, |__t| __t.mark_secrets_found_and_log_info());
    __f(room, secret_waypoint, msg, &args)
}

/// SLF4J's `MessageFormatter`: each `{}` takes the next argument's `toString()`; one left over prints as `{}`.
fn slf4j_format(msg: &str, args: &Vec<compat::Object>) -> String {
    let mut out = String::with_capacity(msg.len());
    let mut rest = msg;
    let mut next = args.iter();
    while let Some(at) = rest.find("{}") {
        out.push_str(&rest[..at]);
        match next.next() {
            Some(arg) => {
                if let Some(text) = arg.downcast_ref::<String>() {
                    out.push_str(text);
                } else if let Some(n) = arg.downcast_ref::<i32>() {
                    out.push_str(&n.to_string());
                } else if let Some(text) = arg.downcast_ref::<&'static str>() {
                    out.push_str(text);
                } else {
                    out.push_str("{}");
                }
            }
            None => out.push_str("{}"),
        }
        rest = &rest[at + 2..];
    }
    out.push_str(rest);
    out
}
