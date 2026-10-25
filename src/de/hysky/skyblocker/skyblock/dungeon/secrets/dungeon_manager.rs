//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.DungeonManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::dungeon_events;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_boss::DungeonBoss;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map::DungeonMap;
use crate::de::hysky::skyblocker::skyblock::dungeon::preview::room_preview_server;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::debug_room::DebugRoom;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_map_utils;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_map_utils::DungeonMapUtils;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::ClearState;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::RoomBase;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Shape;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Type;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::SecretWaypoint;
use crate::de::hysky::skyblocker::utils::codec_utils;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_pos_argument::ClientPosArgument;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonDataBase;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::builder::literal_argument_builder::LiteralArgumentBuilder;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::arguments::integer_argument_type::IntegerArgumentType;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_block_pos_argument_type::ClientBlockPosArgumentType;
use mcrust::com::mojang::brigadier::builder::required_argument_builder::RequiredArgumentBuilder;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions::Suggestions;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::awt::color::Color;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::commands::shared_suggestion_provider;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::core::vec3i::Vec3i;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::server::packs::resources::resource::Resource;
use mcrust::net::minecraft::world::entity::ambient::ambient_creature::AmbientCreature;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::item::item_entity::ItemEntity;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::interaction_result::InteractionResult;
use mcrust::net::minecraft::world::item::map_item;
use mcrust::net::minecraft::world::level::level::Level;
use mcrust::net::minecraft::world::level::saveddata::maps::map_item_saved_data::MapItemSavedData;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::org::joml::vector2i::Vector2i;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;

/// java: @VisibleForTesting public static final String DUNGEONS_PATH = "dungeons";
pub const DUNGEONS_PATH: &str = "dungeons";
/// java: private static final Path CUSTOM_WAYPOINTS_DIR = SkyblockerMod.CONFIG_DIR.resolve("custom_secret_waypoints.json");
pub static CUSTOM_WAYPOINTS_DIR: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("custom_secret_waypoints.json"));
/// java: private static final Pattern KEY_FOUND = Pattern.compile("^RIGHT CLICK on (?:the BLOOD DOOR|a WITHER door) to open it. This key can only be used to open 1 door!$");
pub static KEY_FOUND: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^RIGHT CLICK on (?:the BLOOD DOOR|a WITHER door) to open it. This key can only be used to open 1 door!$"));
/// java: private static final Pattern WITHER_DOOR_OPENED = Pattern.compile("^\\w+ opened a WITHER door!$");
pub static WITHER_DOOR_OPENED: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^\\w+ opened a WITHER door!$"));
/// java: private static final String BLOOD_DOOR_OPENED = "The BLOOD DOOR has been opened!";
pub const BLOOD_DOOR_OPENED: &str = "The BLOOD DOOR has been opened!";
/// java: private static final Pattern TEAM_SCORE_PATTERN = Pattern.compile(" +Team Score: [0-9]+ \\([A-z+]+\\)");
pub static TEAM_SCORE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(" +Team Score: [0-9]+ \\([A-z+]+\\)"));
/// java: protected static final float[] RED_COLOR_COMPONENTS = {1, 0, 0};
pub static RED_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![((1) as f32) as f32, ((0) as f32) as f32, ((0) as f32) as f32]);
/// java: protected static final float[] GREEN_COLOR_COMPONENTS = {0, 1, 0};
pub static GREEN_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![((0) as f32) as f32, ((1) as f32) as f32, ((0) as f32) as f32]);
/// java: @SuppressWarnings("JavadocReference") public static final Object2ByteMap<String> NUMERIC_ID = Object2ByteMap.ofEntries( Object2ByteMap.entry("minecraft:stone", (byte) 1), Object2ByteMap.entry("minecra
pub static NUMERIC_ID: LazyLock<std::collections::HashMap<String, i8>> = LazyLock::new(|| std::collections::HashMap::from([
    ("minecraft:stone".to_string(), 1),
    ("minecraft:diorite".to_string(), 2),
    ("minecraft:polished_diorite".to_string(), 3),
    ("minecraft:andesite".to_string(), 4),
    ("minecraft:polished_andesite".to_string(), 5),
    ("minecraft:grass_block".to_string(), 6),
    ("minecraft:dirt".to_string(), 7),
    ("minecraft:coarse_dirt".to_string(), 8),
    ("minecraft:cobblestone".to_string(), 9),
    ("minecraft:bedrock".to_string(), 10),
    ("minecraft:oak_leaves".to_string(), 11),
    ("minecraft:gray_wool".to_string(), 12),
    ("minecraft:double_stone_slab".to_string(), 13),
    ("minecraft:mossy_cobblestone".to_string(), 14),
    ("minecraft:clay".to_string(), 15),
    ("minecraft:stone_bricks".to_string(), 16),
    ("minecraft:mossy_stone_bricks".to_string(), 17),
    ("minecraft:chiseled_stone_bricks".to_string(), 18),
    ("minecraft:gray_terracotta".to_string(), 19),
    ("minecraft:cyan_terracotta".to_string(), 20),
    ("minecraft:black_terracotta".to_string(), 21),
]));
/// java: @SuppressWarnings("JavadocReference") protected static final Map<String, Map<String, Map<String, int[]>>> ROOMS_DATA = new ConcurrentHashMap<>();
pub static ROOMS_DATA: LazyLock<Mutex<std::collections::HashMap<String, std::collections::HashMap<String, std::collections::HashMap<String, Vec<i32>>>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: private static final Map<String, RoomInfo> ROOMS_INFO = new ConcurrentHashMap<>();
pub static ROOMS_INFO: LazyLock<Mutex<std::collections::HashMap<String, RoomInfo>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: private static final Map<String, List<RoomWaypoint>> ROOMS_WAYPOINTS = new ConcurrentHashMap<>();
pub static ROOMS_WAYPOINTS: LazyLock<Mutex<std::collections::HashMap<String, Vec<RoomWaypoint>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
//   java: static final JsonData<Map<SecretWaypoint.Category, Color>> WAYPOINT_COLOR_DATA = new JsonData<>(
//   java: 			SkyblockerMod.CONFIG_DIR.resolve("secret_waypoint_color.json"),
//   java: 			Codec.unboundedMap(SecretWaypoint.Category.CODEC, CodecUtils.COLOR_CODEC).xmap(EnumMap::new, Function.identity()),
//   java: 			Arrays.stream(SecretWaypoint.Category.values()).collect(Maps.<SecretWaypoint.Category, SecretWaypoint.Category, Color>toImmutableEnumMap(Function.identity(), category -> {
//   java: 				float[] components = category.getColorComponents();
//   java: 				return new Color(components[0], components[1], components[2]);
//   java: 			})));
/// java: static final JsonData<Map<SecretWaypoint.Category, Color>> WAYPOINT_COLOR_DATA = new JsonData<>( SkyblockerMod.CONFIG_DIR.resolve("secret_waypoint_color.json"), Codec.unboundedMap(SecretWaypoint.Categ
pub static WAYPOINT_COLOR_DATA: compat::MainThread<compat::Shared<dyn JsonData<std::collections::HashMap<crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category, Color>>>> = compat::MainThread::new(|| { use secret_waypoint::Category; let __codec = mcrust::com::mojang::serialization::codec::xmap(mcrust::com::mojang::serialization::codec::unbounded_map(secret_waypoint::CATEGORY_CODEC.clone(), codec_utils::COLOR_CODEC.clone()), |entries: Vec<(Category, Color)>| entries.into_iter().collect::<std::collections::HashMap<Category, Color>>(), |map: &std::collections::HashMap<Category, Color>| { let mut __e: Vec<(Category, Color)> = map.iter().map(|(k, v)| (*k, v.clone())).collect(); __e.sort_by_key(|(k, _)| *k); __e } /* EnumMap iterates in ordinal order */); let __default: std::collections::HashMap<Category, Color> = Category::values().into_iter().map(|category| { let components = category.get_color_components(); (category, Color::new_f32(components[0], components[1], components[2])) }).collect(); compat::shared(JsonDataBase::new(skyblocker_mod::CONFIG_DIR.get_mut().join("secret_waypoint_color.json"), __codec, __default)) as compat::Shared<dyn JsonData<std::collections::HashMap<Category, Color>>> });
/// java: private static final Map<Vector2ic, Room> rooms = new HashMap<>();
pub static ROOMS: compat::MainThread<std::collections::HashMap<Vector2i, compat::Shared<dyn Room>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: private static final Table<String, BlockPos, SecretWaypoint> customWaypoints = HashBasedTable.create();
pub static CUSTOM_WAYPOINTS: compat::MainThread<crate::unresolved::HashBasedTable<String, BlockPos, compat::Shared<SecretWaypoint>>> = compat::MainThread::new(|| crate::unresolved::HashBasedTable::create());
/// java: private static final Component NOT_MATCHED_TEXT = Component.translatable("skyblocker.dungeons.secrets.notMatched").withColor(TextColor.RED);
pub static NOT_MATCHED_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("skyblocker.dungeons.secrets.notMatched").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED));
/// java: private static @Nullable CompletableFuture<Void> roomsLoaded;
pub static ROOMS_LOADED: compat::MainThread<Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>> = compat::MainThread::new(|| None);
/// java: private static @Nullable Vector2ic mapEntrancePos;
pub static MAP_ENTRANCE_POS: compat::MainThread<Option<Vector2i>> = compat::MainThread::new(|| None);
/// java: private static int mapRoomSize;
pub static MAP_ROOM_SIZE: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static @Nullable Vector2ic physicalEntrancePos;
pub static PHYSICAL_ENTRANCE_POS: compat::MainThread<Option<Vector2i>> = compat::MainThread::new(|| None);
/// java: private static @Nullable Room currentRoom;
pub static CURRENT_ROOM: compat::MainThread<Option<compat::Shared<dyn Room>>> = compat::MainThread::new(|| None);
/// java: private static DungeonBoss boss = DungeonBoss.NONE;
pub static BOSS: LazyLock<Mutex<DungeonBoss>> = LazyLock::new(|| Mutex::new(DungeonBoss::None));
/// java: private static @Nullable AABB bloodRushDoorBox;
pub static BLOOD_RUSH_DOOR_BOX: compat::MainThread<Option<AABB>> = compat::MainThread::new(|| None);
/// java: private static boolean bloodOpened;
pub static BLOOD_OPENED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean hasKey;
pub static HAS_KEY: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean runEnded;
pub static RUN_ENDED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: public static final Codec<RoomInfo> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(RoomInfo::name) ).apply(instance, RoomInfo::new));
pub static ROOM_INFO_CODEC: LazyLock<CodecRef<RoomInfo>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply1(record_codec_builder::of(|o: &RoomInfo| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), move |__a0| RoomInfo::new(__a0.clone()))));
/// java: private static final Codec<RoomWaypoint> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("secretName").forGetter(RoomWaypoint::secretName), SecretWaypoint.Category.C
pub static ROOM_WAYPOINT_CODEC: LazyLock<CodecRef<RoomWaypoint>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply5(record_codec_builder::of(|o: &RoomWaypoint| o.secret_name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "secretName")), record_codec_builder::of(|o: &RoomWaypoint| o.category(), codec::field_of((*secret_waypoint::CATEGORY_CODEC).clone(), "category")), record_codec_builder::of(|o: &RoomWaypoint| o.x(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "x")), record_codec_builder::of(|o: &RoomWaypoint| o.y(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "y")), record_codec_builder::of(|o: &RoomWaypoint| o.z(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "z")), move |__a0, __a1, __a2, __a3, __a4| RoomWaypoint::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone()))));
/// java: public static final Codec<List<RoomWaypoint>> LIST_CODEC = CODEC.listOf();
pub static ROOM_WAYPOINT_LIST_CODEC: LazyLock<CodecRef<Vec<RoomWaypoint>>> = LazyLock::new(|| codec::list((*ROOM_WAYPOINT_CODEC).clone()));
/// java: public static final Codec<RoomData> CODEC = RecordCodecBuilder.create(instance -> instance.group( RoomInfo.CODEC.fieldOf("info").forGetter(RoomData::info), RoomWaypoint.LIST_CODEC.fieldOf("secrets").f
pub static ROOM_DATA_CODEC: LazyLock<CodecRef<RoomData>> = LazyLock::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &RoomData| o.info(), codec::field_of((*ROOM_INFO_CODEC).clone(), "info")), record_codec_builder::of(|o: &RoomData| o.secrets(), codec::field_of((*ROOM_WAYPOINT_LIST_CODEC).clone(), "secrets")), move |__a0, __a1| RoomData::new(__a0.clone(), __a1.clone()))));

/// java: public class DungeonManager
#[derive(Clone)]
pub struct DungeonManager {}

impl DungeonManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isRoomsLoaded()
    pub fn is_rooms_loaded() -> bool {
        return (ROOMS_LOADED.get_mut().is_some() && ROOMS_LOADED.get_mut().as_ref().unwrap().is_done());
    }

    /// java: public static Stream<Room> getRoomsStream()
    pub fn get_rooms_stream() -> Box<dyn Iterator<Item = compat::Shared<dyn Room>>> {
        return Box::new(ROOMS.get_mut().values().cloned());
    }

    /// java: public static @Nullable RoomInfo getRoomMetadata(String room)
    pub fn get_room_metadata(room: &str) -> Option<RoomInfo> {
        return compat::lock(&ROOMS_INFO).get(room).cloned();
    }

    /// java: public static @Nullable List<RoomWaypoint> getRoomWaypoints(String room)
    pub fn get_room_waypoints(room: &str) -> Option<Vec<RoomWaypoint>> {
        return compat::lock(&ROOMS_WAYPOINTS).get(room).cloned();
    }

    /// java: public static Map<BlockPos, SecretWaypoint> getCustomWaypoints(String room)
    /// The row's cells in the table's order (Guava's row view, read once).
    pub fn get_custom_waypoints(room: &str) -> indexmap::IndexMap<BlockPos, compat::Shared<SecretWaypoint>> {
        // java: return customWaypoints.row(room);
        return CUSTOM_WAYPOINTS.get().row(&room.to_string()).into_iter().collect();
    }

    /// java: @SuppressWarnings("UnusedReturnValue") public static @Nullable SecretWaypoint addCustomWaypoint(String room, SecretWaypoint waypoint)
    pub fn add_custom_waypoint(mut room: String, mut waypoint: compat::Shared<SecretWaypoint>) -> Option<compat::Shared<SecretWaypoint>> {
        // java: return customWaypoints.put(room, waypoint.pos, waypoint);
        // (Guava's `put` rejects a null column: `checkNotNull`)
        let pos = compat::read(&waypoint, |__t| __t.pos.clone()).expect("NullPointerException");
        return CUSTOM_WAYPOINTS.get_mut().put(room, pos, waypoint);
    }

    /// java: private static int addCustomWaypoint(CommandContext<FabricClientCommandSource> context, BlockPos pos)
    pub fn add_custom_waypoint_command_context_block_pos(context: &CommandContext<FabricClientCommandSource>, pos: &BlockPos) -> i32 {
        let mut room: Option<compat::Shared<dyn Room>> = DungeonManager::get_room_at_physical_vec3i(pos.as_vec3i());
        if DungeonManager::is_room_matched(room.clone()) {
            let room: compat::Shared<dyn Room> = room.unwrap();
            ({ let __f = compat::dispatch(&room, |__t| __t.add_custom_waypoint()); __f(&room, context, &({ let __f = compat::dispatch(&room, |__t| __t.actual_to_relative()); __f(&room, pos) })) });
        } else {
            // java: context.getSource().sendError(Constants.PREFIX.get().append(NOT_MATCHED_TEXT));
            compat::batch_w_send_error(&(constants::PREFIX.get())().append(NOT_MATCHED_TEXT.get().clone()));
        }
        return 1;
    }

    /// java: public static void addCustomWaypoints(String room, Collection<SecretWaypoint> waypoints)
    pub fn add_custom_waypoints(room: &str, waypoints: &Vec<compat::Shared<SecretWaypoint>>) {
        for waypoint in waypoints.iter() {
            DungeonManager::add_custom_waypoint(room.to_string(), waypoint.clone());
        }
    }

    /// java: public static @Nullable SecretWaypoint removeCustomWaypoint(String room, BlockPos pos)
    pub fn remove_custom_waypoint(room: &str, pos: &BlockPos) -> Option<compat::Shared<SecretWaypoint>> {
        // java: return customWaypoints.remove(room, pos);
        return CUSTOM_WAYPOINTS.get_mut().remove(&room.to_string(), pos);
    }

    /// java: private static int removeCustomWaypoint(CommandContext<FabricClientCommandSource> context, BlockPos pos)
    pub fn remove_custom_waypoint_command_context_block_pos(context: &CommandContext<FabricClientCommandSource>, pos: &BlockPos) -> i32 {
        let mut room: Option<compat::Shared<dyn Room>> = DungeonManager::get_room_at_physical_vec3i(pos.as_vec3i());
        if DungeonManager::is_room_matched(room.clone()) {
            let room: compat::Shared<dyn Room> = room.unwrap();
            ({ let __f = compat::dispatch(&room, |__t| __t.remove_custom_waypoint()); __f(&room, context, &({ let __f = compat::dispatch(&room, |__t| __t.actual_to_relative()); __f(&room, pos) })) });
        } else {
            // java: context.getSource().sendError(Constants.PREFIX.get().append(NOT_MATCHED_TEXT));
            compat::batch_w_send_error(&(constants::PREFIX.get())().append(NOT_MATCHED_TEXT.get().clone()));
        }
        return 1;
    }

    /// java: public static @Nullable Vector2ic getMapEntrancePos()
    pub fn get_map_entrance_pos() -> Option<Vector2i> {
        return MAP_ENTRANCE_POS.get_mut().clone();
    }

    /// java: public static int getMapRoomSize()
    pub fn get_map_room_size() -> i32 {
        return compat::get_state(&MAP_ROOM_SIZE);
    }

    /// java: public static @Nullable Vector2ic getPhysicalEntrancePos()
    pub fn get_physical_entrance_pos() -> Option<Vector2i> {
        return PHYSICAL_ENTRANCE_POS.get_mut().clone();
    }

    /// java: public static @Nullable Room getCurrentRoom()
    pub fn get_current_room() -> Option<compat::Shared<dyn Room>> {
        return CURRENT_ROOM.get_mut().clone();
    }

    /// java: public static DungeonBoss getBoss()
    pub fn get_boss() -> DungeonBoss {
        return compat::get_state(&BOSS);
    }

    /// java: public static boolean isInBoss()
    pub fn is_in_boss() -> bool {
        return compat::lock(&BOSS).is_in_boss();
    }

    /// java: @Init public static void init()
    pub fn init() {
        WAYPOINT_COLOR_DATA.get_mut().lock().borrow_mut().init();
        (CompletableFuture::<()>::run_async(move || (|| { DungeonManager::load(); })(), |__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t))).exceptionally(|e| {
            logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to load dungeon secrets"));
            return Default::default(); // java: return null
        });
        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(DungeonManager::saveCustomWaypoints);
        // (`CLIENT_STOPPING` is Fabric's HEAD injector on `Minecraft#stop`)
        let stopping = mcrust::mcrust::modding::hook::inject("net.minecraft.client.Minecraft#stop", mcrust::mcrust::modding::hook::At::Head, |_: &mut mcrust::mcrust::modding::hook::CallbackInfo<'_>| {
            DungeonManager::save_custom_waypoints(compat::minecraft());
        });
        if let Err(e) = stopping {
            logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to hook the client stop: {:?}", e));
        }
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DungeonManager::update(); }), 5) });
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DungeonManager::update_all_room_checkmarks(); }), 20) });
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { DungeonManager::extract_rendering(&__a0); }));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(DungeonManager::onChatMessage);
        compat::register_allow_game(|text, overlay| DungeonManager::on_chat_message(text, overlay));
        // java: UseBlockCallback.EVENT.register((_, world, _, hitResult) -> onUseBlock(world, hitResult));
        mcrust::mcrust::modding::events::use_block_callback::register(|_, world, _, hit_result| DungeonManager::on_use_block(world, hit_result));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("dungeons").then(literal("secrets")
                //     .then(literal("markAsFound").then(markSecretsCommand(true)))
                //     .then(literal("markAsMissing").then(markSecretsCommand(false)).then(markAllSecretsAsMissingCommand()))
                //     .then(literal("getRelativePos").executes(DungeonManager::getRelativePos))
                //     .then(literal("getRelativeTargetPos").executes(DungeonManager::getRelativeTargetPos))
                //     .then(literal("addWaypoint").then(addCustomWaypointCommand(false, registryAccess)))
                //     .then(literal("addWaypointRelatively").then(addCustomWaypointCommand(true, registryAccess)))
                //     .then(literal("removeWaypoint").then(removeCustomWaypointCommand(false)))
                //     .then(literal("removeWaypointRelatively").then(removeCustomWaypointCommand(true))))));
                let mut secrets = client_command_registration_callback::literal("secrets");
                let mut mark_as_found = client_command_registration_callback::literal("markAsFound");
                mark_as_found.then(&DungeonManager::mark_secrets_command(true));
                secrets.then(&mark_as_found);
                let mut mark_as_missing = client_command_registration_callback::literal("markAsMissing");
                mark_as_missing.then(&DungeonManager::mark_secrets_command(false));
                mark_as_missing.then(&DungeonManager::mark_all_secrets_as_missing_command());
                secrets.then(&mark_as_missing);
                let mut get_relative_pos = client_command_registration_callback::literal("getRelativePos");
                get_relative_pos.executes(Arc::new(|context: &CommandContext<FabricClientCommandSource>| Ok(DungeonManager::get_relative_pos(context))));
                secrets.then(&get_relative_pos);
                let mut get_relative_target_pos = client_command_registration_callback::literal("getRelativeTargetPos");
                get_relative_target_pos.executes(Arc::new(|context: &CommandContext<FabricClientCommandSource>| Ok(DungeonManager::get_relative_target_pos(context))));
                secrets.then(&get_relative_target_pos);
                // `registryAccess` is the dispatcher's build context (the connection's registries)
                if let Some(registry_access) = dispatcher.build_context().cloned() {
                    let mut add_waypoint = client_command_registration_callback::literal("addWaypoint");
                    add_waypoint.then(&DungeonManager::add_custom_waypoint_command(false, &registry_access));
                    secrets.then(&add_waypoint);
                    let mut add_waypoint_relatively = client_command_registration_callback::literal("addWaypointRelatively");
                    add_waypoint_relatively.then(&DungeonManager::add_custom_waypoint_command(true, &registry_access));
                    secrets.then(&add_waypoint_relatively);
                }
                let mut remove_waypoint = client_command_registration_callback::literal("removeWaypoint");
                remove_waypoint.then(&DungeonManager::remove_custom_waypoint_command(false));
                secrets.then(&remove_waypoint);
                let mut remove_waypoint_relatively = client_command_registration_callback::literal("removeWaypointRelatively");
                remove_waypoint_relatively.then(&DungeonManager::remove_custom_waypoint_command(true));
                secrets.then(&remove_waypoint_relatively);
                let mut dungeons = client_command_registration_callback::literal("dungeons");
                dungeons.then(&secrets);
                let mut namespace = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                namespace.then(&dungeons);
                dispatcher.dispatcher().register(&namespace);
            },
        ));
        if crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled() {
            client_command_registration_callback::EVENT.register(Arc::new(
                |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                    let mut secrets = client_command_registration_callback::literal("secrets");
                    // java: .then(literal("matchAgainst").then(matchAgainstCommand()))
                    let mut match_against = client_command_registration_callback::literal("matchAgainst");
                    match_against.then(&DungeonManager::match_against_command());
                    secrets.then(&match_against);
                    // java: .then(literal("clearSubProcesses").executes(context -> {..}))
                    let mut clear_sub_processes = client_command_registration_callback::literal("clearSubProcesses");
                    clear_sub_processes.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
                        if let Some(current) = CURRENT_ROOM.get().clone() {
                            compat::write(&current, |__t| {
                                __t.room_base_mut().tickables.clear();
                                __t.room_base_mut().renderables.clear();
                            });
                            compat::batch_w_send_feedback(&(constants::PREFIX.get())().append_str("Cleared sub processes in the current room."));
                        } else {
                            compat::batch_w_send_error(&DungeonManager::error("Current room is null."));
                        }
                        Ok(1)
                    }));
                    secrets.then(&clear_sub_processes);
                    // java: .then(literal("getCheckmarkColour").executes(context -> {..}))
                    let mut get_checkmark_colour = client_command_registration_callback::literal("getCheckmarkColour");
                    get_checkmark_colour.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
                        let state = DungeonManager::get_map_state(compat::minecraft());
                        let current = CURRENT_ROOM.get().clone();
                        if let (Some(current), Some(state)) = (current, state) {
                            let checkmark_colour: i32 = {
                                let state = state.lock().unwrap_or_else(|p| p.into_inner());
                                DungeonManager::get_room_checkmark_colour(&state, &current, None)
                            };
                            let result: &str = if checkmark_colour == dungeon_map_utils::GREEN_COLOR as i32 {
                                "Green"
                            } else if checkmark_colour == dungeon_map_utils::WHITE_COLOR as i32 {
                                "White"
                            } else if checkmark_colour == dungeon_map_utils::RED_COLOR as i32 {
                                "Red"
                            } else {
                                "Unknown"
                            };
                            compat::batch_w_send_feedback(&(constants::PREFIX.get())().append_str(&format!("Checkmark colour: {}", result)));
                        } else {
                            compat::batch_w_send_error(&DungeonManager::error("Current room or map state is null."));
                        }
                        Ok(1)
                    }));
                    secrets.then(&get_checkmark_colour);
                    // java: .then(literal("clearRooms").executes(context -> { rooms.clear(); ..}))
                    let mut clear_rooms = client_command_registration_callback::literal("clearRooms");
                    clear_rooms.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
                        ROOMS.get_mut().clear();
                        compat::batch_w_send_feedback(&(constants::PREFIX.get())().append_str("Cleared all rooms!"));
                        Ok(1)
                    }));
                    secrets.then(&clear_rooms);
                    let mut dungeons = client_command_registration_callback::literal("dungeons");
                    dungeons.then(&secrets);
                    let mut namespace = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                    namespace.then(&dungeons);
                    dispatcher.dispatcher().register(&namespace);
                },
            ));
        }
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            DungeonManager::reset();
        }));
    }

    /// java: private static void load()
    pub fn load() {
        let mut start_time: i64 = compat::millis();
        let mut dungeon_futures: Vec<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>> = Vec::new();
        for resource_entry in (compat::minecraft().get_resource_manager().list_resources(DUNGEONS_PATH, &|id| (id.path().ends_with(".skeleton") && id.namespace() == skyblocker_mod::NAMESPACE))).iter().map(|(k, v)| (k.clone(), v.clone())) {
            DungeonManager::check_resource_source(&resource_entry.0, &resource_entry.1);
            let mut path: Vec<String> = compat::regex::split(&resource_entry.0.path(), "/");
            if (path.len() as i32) != 4 {
                logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to load dungeon room data, invalid resource identifier {}", compat::jstring(&resource_entry.0)));
                break;
            }
            let mut dungeon: String = path[(1) as usize].clone();
            let mut room_shape: String = path[(2) as usize].clone();
            let mut room: String = compat::substring2(&(path[(3) as usize]), 0, (compat::str_len(&(path[(3) as usize]))).wrapping_sub(compat::str_len(".skeleton")));
            // ROOMS_DATA.computeIfAbsent(dungeon, ..).computeIfAbsent(roomShape, ..) - on the maps themselves
            compat::lock(&ROOMS_DATA).entry(dungeon.clone()).or_default().entry(room_shape.clone()).or_default();
            let (__dungeon, __room_shape, __room) = (dungeon.clone(), room_shape.clone(), room.clone());
            dungeon_futures.push((CompletableFuture::<()>::run_async(move || (|| {
                // Map<String, int[]> roomsMap = ROOMS_DATA.get(dungeon).get(roomShape); roomsMap.put(room, readRoom(resource));
                let room_data: Vec<i32> = DungeonManager::read_room(&resource_entry.1);
                compat::lock(&ROOMS_DATA).get_mut(&dungeon).unwrap().get_mut(&room_shape).unwrap().insert(room.clone(), room_data);
                logger().debug(&format!("[Skyblocker Dungeon Secrets] Loaded dungeon room skeleton - dungeon={}, shape={}, room={}", dungeon, room_shape, room));
            })(), compat::executor)).exceptionally(move |e| {
                logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to load dungeon room skeleton - dungeon={}, shape={}, room={}: {}", __dungeon, __room_shape, __room, e));
                return Default::default(); // java: return null
            }));
        }
        for resource_entry in (compat::minecraft().get_resource_manager().list_resources(DUNGEONS_PATH, &|id| (id.path().ends_with(".json") && id.namespace() == skyblocker_mod::NAMESPACE))).iter().map(|(k, v)| (k.clone(), v.clone())) {
            DungeonManager::check_resource_source(&resource_entry.0, &resource_entry.1);
            let mut path: Vec<String> = compat::regex::split(&resource_entry.0.path(), "/");
            if (path.len() as i32) != 4 {
                continue;
            }
            let mut dungeon: String = path[(1) as usize].clone();
            let mut room_shape: String = path[(2) as usize].clone();
            let mut room: String = compat::substring2(&(path[(3) as usize]), 0, (compat::str_len(&(path[(3) as usize]))).wrapping_sub(compat::str_len(".json")));
            let (__dungeon, __room_shape, __room) = (dungeon.clone(), room_shape.clone(), room.clone());
            dungeon_futures.push((CompletableFuture::<()>::run_async(move || (|| {
                {
                    // java: try (BufferedReader roomJsonReader = resource.openAsReader()) {..} catch (Exception ex) - an exception
                    // here (the reader's IOException too) is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let mut room_json_reader: String = resource_entry.1.open_as_reader().expect("IOException");
                        // RoomData roomData = RoomData.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseReader(roomJsonReader)).getOrThrow();
                        let parsed: serde_json::Value = serde_json::from_str(&room_json_reader).expect("JsonParseException");
                        let mut room_data: RoomData = ROOM_DATA_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&parsed)).get_or_throw().expect("IllegalStateException");
                        compat::lock(&ROOMS_WAYPOINTS).insert(room.clone(), room_data.secrets.clone());
                        compat::lock(&ROOMS_INFO).insert(room.clone(), room_data.info.clone());
                    }));
                    if let Err(ex) = __result {
                        panic!("RuntimeException: {}", compat::panic_message(&ex));
                    }
                }
                logger().debug(&format!("[Skyblocker Dungeon Secrets] Loaded dungeon room secrets - dungeon={}, shape={}, room={}", dungeon, room_shape, room));
            })(), compat::executor)).exceptionally(move |e| {
                logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to load room secrets - dungeon={}, shape={}, room={}: {}", __dungeon, __room_shape, __room, e));
                return Default::default(); // java: return null
            }));
        }
        // dungeonFutures.add(CompletableFuture.runAsync(() -> {
        //   try (BufferedReader customWaypointsReader = Files.newBufferedReader(CUSTOM_WAYPOINTS_DIR)) {
        //     SkyblockerMod.GSON.fromJson(customWaypointsReader, JsonObject.class).asMap().forEach((room, waypointsJson) ->
        //       addCustomWaypoints(room, SecretWaypoint.LIST_CODEC.parse(JsonOps.INSTANCE, waypointsJson).resultOrPartial(LOGGER::error).orElseGet(ArrayList::new)));
        //     LOGGER.debug(..);
        //   } catch (NoSuchFileException _) {} catch (Exception e) { LOGGER.error(.., e); }
        // }, VIRTUAL_THREAD_EXECUTOR));
        // The file is read on the worker; the waypoints are this mod's render-thread objects (the table, the
        // codec), so they are built and added through the game's executor.
        let custom_waypoints_dir: std::path::PathBuf = CUSTOM_WAYPOINTS_DIR.get().clone();
        dungeon_futures.push(CompletableFuture::<()>::run_async(move || {
            match std::fs::read_to_string(&custom_waypoints_dir) {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to load custom dungeon secret waypoints: {}", e));
                }
                Ok(text) => crate::compat::execute(move || {
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let json: serde_json::Value = serde_json::from_str(&text).expect("JsonParseException");
                        for (room, waypoints_json) in compat::json::get_as_json_object(&json) {
                            let waypoints = secret_waypoint::LIST_CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&waypoints_json)).result_or_partial_with(|e| logger().error(&e)).unwrap_or_default();
                            DungeonManager::add_custom_waypoints(&room, &waypoints);
                        }
                        logger().debug("[Skyblocker Dungeon Secrets] Loaded custom dungeon secret waypoints");
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to load custom dungeon secret waypoints: {}", compat::panic_message(&e)));
                    }
                }),
            }
        }, compat::executor));
        // roomsLoaded = CompletableFuture.allOf(dungeonFutures..).thenRun(() -> LOGGER.info(.. ROOMS_DATA sizes, customWaypoints.size(), ..))
        //   .exceptionally(e -> { LOGGER.error("[Skyblocker Dungeon Secrets] Failed to load dungeon secrets", e); return null; });
        // (the log reads the custom waypoint table, a render-thread object: it is logged through the game's executor)
        let rooms_loaded = CompletableFuture::<()>::all_of(&dungeon_futures)
            .then_apply(move |_| {
                crate::compat::execute(move || {
                    let (dungeons, shapes, rooms) = {
                        let data = compat::lock(&ROOMS_DATA);
                        (data.len(), data.values().map(|m| m.len()).sum::<usize>(), data.values().flat_map(|m| m.values()).map(|m| m.len()).sum::<usize>())
                    };
                    logger().info(&format!("[Skyblocker Dungeon Secrets] Loaded dungeon secrets for {} dungeon(s), {} room shapes, {} rooms, and {} custom secret waypoints total in {} ms", dungeons, shapes, rooms, CUSTOM_WAYPOINTS.get().size(), (compat::millis()).wrapping_sub(start_time)));
                });
            })
            .exceptionally(|e| {
                logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to load dungeon secrets: {}", e));
            });
        *ROOMS_LOADED.get_mut() = Some(rooms_loaded);
        logger().info(&format!("[Skyblocker Dungeon Secrets] Started loading dungeon secrets in (blocked main thread for) {} ms", (compat::millis()).wrapping_sub(start_time)));
    }

    /// java: private static void saveCustomWaypoints(Minecraft client)
    pub fn save_custom_waypoints(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let _ = client;
        // java: try (BufferedWriter writer = Files.newBufferedWriter(CUSTOM_WAYPOINTS_DIR)) {..} catch (Exception e) - an
        // exception here (the writer's IOException too) is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // java: JsonObject customWaypointsJson = new JsonObject();
            let mut custom_waypoints_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
            // java: customWaypoints.rowMap().forEach((room, waypoints) -> customWaypointsJson.add(room,
            //     SecretWaypoint.LIST_CODEC.encodeStart(JsonOps.INSTANCE, new ArrayList<>(waypoints.values())).resultOrPartial(LOGGER::error).orElseGet(JsonArray::new)));
            for (room, waypoints) in CUSTOM_WAYPOINTS.get().row_map() {
                let values: Vec<compat::Shared<SecretWaypoint>> = waypoints.into_iter().map(|(_, waypoint)| waypoint).collect();
                let encoded: serde_json::Value = secret_waypoint::LIST_CODEC.get().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &values).result_or_partial_with(|e| logger().error(&e)).map(|o| compat::json_from_port(&o)).unwrap_or_else(|| serde_json::Value::Array(Vec::new()));
                custom_waypoints_json.insert(room, encoded);
            }
            // java: SkyblockerMod.GSON.toJson(customWaypointsJson, writer); - GSON pretty-prints (two-space indent)
            let text: String = serde_json::to_string_pretty(&serde_json::Value::Object(custom_waypoints_json)).expect("JsonIOException");
            std::fs::write(CUSTOM_WAYPOINTS_DIR.get(), text).expect("IOException");
            logger().info("[Skyblocker Dungeon Secrets] Saved custom dungeon secret waypoints");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to save custom dungeon secret waypoints: {}", compat::panic_message(&e)));
        }
    }

    /// java: private static int[] readRoom(Resource resource) throws RuntimeException
    pub fn read_room(resource: &Resource) -> Vec<i32> {
        {
            // java: try (ObjectInputStream in = new ObjectInputStream(new InflaterInputStream(resource.open()))) {
            //   return (int[]) in.readObject();
            // } catch (IOException | ClassNotFoundException e) { throw new RuntimeException(e); }
            // - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let compressed: Vec<u8> = resource.open().expect("IOException");
                read_serialized_int_array(&inflate_stream(&compressed))
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    panic!("RuntimeException: {}", compat::panic_message(&e));
                }
            }
        }
    }

    /// java: private static RequiredArgumentBuilder<FabricClientCommandSource, Integer> markSecretsCommand(boolean found)
    pub fn mark_secrets_command(found: bool) -> RequiredArgumentBuilder<FabricClientCommandSource> {
        // java: return argument("secretIndex", IntegerArgumentType.integer()).suggests((_, builder) -> {
        //     if (isCurrentRoomMatched()) IntStream.rangeClosed(1, currentRoom.getSecretCount()).forEach(builder::suggest);
        //     return builder.buildFuture(); }).executes(context -> {..});
        let mut secret_index = client_command_registration_callback::argument("secretIndex", Arc::new(IntegerArgumentType::integer()));
        secret_index.suggests(Some(Arc::new(|_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| {
            if DungeonManager::is_current_room_matched() {
                let current = CURRENT_ROOM.get().clone().unwrap();
                let secret_count: i32 = { let __f = compat::dispatch(&current, |__t| __t.get_secret_count()); __f(&current) };
                for i in 1..=secret_count {
                    builder.suggest_integer(i);
                }
            }
            Ok(builder.build_future())
        })));
        secret_index.executes(Arc::new(move |context: &CommandContext<FabricClientCommandSource>| {
            let secret_index: i32 = IntegerArgumentType::get_integer(context, "secretIndex");
            if DungeonManager::mark_secrets(secret_index, found) {
                compat::batch_w_send_feedback(&(constants::PREFIX.get())().append(component::translatable_args(if found { "skyblocker.dungeons.secrets.markSecretFound" } else { "skyblocker.dungeons.secrets.markSecretMissing" }, vec![FormattedText::of(secret_index.to_string())])));
            } else {
                compat::batch_w_send_error(&(constants::PREFIX.get())().append(component::translatable_args(if found { "skyblocker.dungeons.secrets.markSecretFoundUnable" } else { "skyblocker.dungeons.secrets.markSecretMissingUnable" }, vec![FormattedText::of(secret_index.to_string())]).with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED)));
            }
            Ok(1)
        }));
        return secret_index;
    }

    /// java: private static LiteralArgumentBuilder<FabricClientCommandSource> markAllSecretsAsMissingCommand()
    pub fn mark_all_secrets_as_missing_command() -> LiteralArgumentBuilder<FabricClientCommandSource> {
        // java: return literal("all").executes(context -> {..});
        let mut all = client_command_registration_callback::literal("all");
        all.executes(Arc::new(|_: &CommandContext<FabricClientCommandSource>| {
            if DungeonManager::is_current_room_matched() {
                // java: currentRoom.markAllSecrets(false);
                let current = CURRENT_ROOM.get().clone().unwrap();
                { let __f = compat::dispatch(&current, |__t| __t.mark_all_secrets()); __f(&current, false) };
                compat::batch_w_send_feedback(&(constants::PREFIX.get())().append(component::translatable("skyblocker.dungeons.secrets.markSecretsMissing")));
            } else {
                compat::batch_w_send_feedback(&(constants::PREFIX.get())().append(component::translatable("skyblocker.dungeons.secrets.markSecretsMissingUnable").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED)));
            }
            Ok(1)
        }));
        return all;
    }

    /// java: private static int getRelativePos(CommandContext<FabricClientCommandSource> context)
    pub fn get_relative_pos(context: &CommandContext<FabricClientCommandSource>) -> i32 {
        // java: return getRelativePos(context.getSource(), context.getSource().getPlayer().blockPosition());
        let Some(pos) = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.block_position()) else {
            panic!("NullPointerException: getPlayer()");
        };
        return DungeonManager::get_relative_pos_2(context.get_source(), &pos);
    }

    /// java: private static int getRelativePos(FabricClientCommandSource source, BlockPos pos)
    pub fn get_relative_pos_2(source: &FabricClientCommandSource, pos: &BlockPos) -> i32 {
        let _ = source;
        let room: Option<compat::Shared<dyn Room>> = DungeonManager::get_room_at_physical_vec3i(pos.as_vec3i());
        if DungeonManager::is_room_matched(room.clone()) {
            // java: BlockPos relativePos = currentRoom.actualToRelative(pos);
            let current = CURRENT_ROOM.get().clone().unwrap();
            let relative_pos: BlockPos = { let __f = compat::dispatch(&current, |__t| __t.actual_to_relative()); __f(&current, pos) };
            let name: String = { let __f = compat::dispatch(&current, |__t| __t.get_name()); __f(&current) }.unwrap_or_else(|| "null".to_string());
            let direction: String = { let __f = compat::dispatch(&current, |__t| __t.get_direction()); __f(&current) }.expect("NullPointerException: getDirection()").get_serialized_name();
            // java: source.sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.dungeons.secrets.posMessage", currentRoom.getName(), currentRoom.getDirection().getSerializedName(), relativePos.getX(), relativePos.getY(), relativePos.getZ())));
            compat::batch_w_send_feedback(&(constants::PREFIX.get())().append(component::translatable_args("skyblocker.dungeons.secrets.posMessage", vec![FormattedText::of(name), FormattedText::of(direction), FormattedText::of(relative_pos.get_x().to_string()), FormattedText::of(relative_pos.get_y().to_string()), FormattedText::of(relative_pos.get_z().to_string())])));
        } else {
            // java: source.sendError(Constants.PREFIX.get().append(NOT_MATCHED_TEXT));
            compat::batch_w_send_error(&(constants::PREFIX.get())().append(NOT_MATCHED_TEXT.get().clone()));
        }
        return 1;
    }

    /// java: private static int getRelativeTargetPos(CommandContext<FabricClientCommandSource> context)
    pub fn get_relative_target_pos(context: &CommandContext<FabricClientCommandSource>) -> i32 {
        // java: if (CLIENT.hitResult instanceof BlockHitResult blockHitResult && blockHitResult.getType() == HitResult.Type.BLOCK) {
        let target: Option<BlockPos> = match compat::minecraft().hit_result.as_ref() {
            Some(mcrust::net::minecraft::world::phys::hit_result::AnyHitResult::Block(block_hit_result)) if block_hit_result.get_type() == mcrust::net::minecraft::world::phys::hit_result::Type::Block => Some(block_hit_result.get_block_pos().clone()),
            _ => None,
        };
        if let Some(pos) = target {
            return DungeonManager::get_relative_pos_2(context.get_source(), &pos);
        } else {
            compat::batch_w_send_error(&(constants::PREFIX.get())().append(component::translatable("skyblocker.dungeons.secrets.noTarget").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED)));
        }
        return 1;
    }

    /// java: private static RequiredArgumentBuilder<FabricClientCommandSource, ClientPosArgument> addCustomWaypointCommand(boolean relative, CommandBuildContext registryAccess)
    pub fn add_custom_waypoint_command(relative: bool, registry_access: &CommandBuildContext) -> RequiredArgumentBuilder<FabricClientCommandSource> {
        // java: return argument("pos", ClientBlockPosArgumentType.blockPos())
        //     .then(argument("secretIndex", IntegerArgumentType.integer())
        //         .then(argument("category", SecretWaypoint.Category.CategoryArgumentType.category())
        //             .then(argument("name", ComponentArgument.textComponent(registryAccess)).executes(context -> {
        //                 BlockPos pos = context.getArgument("pos", ClientPosArgument.class).toAbsoluteBlockPos(context.getSource());
        //                 return relative ? addCustomWaypointRelative(context, pos) : addCustomWaypoint(context, pos); }))));
        let mut name = client_command_registration_callback::argument("name", Arc::new(mcrust::net::minecraft::commands::arguments::component_argument::ComponentArgument::text_component(registry_access)));
        name.executes(Arc::new(move |context: &CommandContext<FabricClientCommandSource>| {
            let pos: BlockPos = ClientBlockPosArgumentType::get_block_pos(context, "pos");
            Ok(if relative { DungeonManager::add_custom_waypoint_relative(context, &pos) } else { DungeonManager::add_custom_waypoint_command_context_block_pos(context, &pos) })
        }));
        let mut category = client_command_registration_callback::argument("category", Arc::new(secret_waypoint::CategoryArgumentType::category().base));
        category.then(&name);
        let mut secret_index = client_command_registration_callback::argument("secretIndex", Arc::new(IntegerArgumentType::integer()));
        secret_index.then(&category);
        let mut pos = client_command_registration_callback::argument("pos", Arc::new(ClientBlockPosArgumentType::block_pos()));
        pos.then(&secret_index);
        return pos;
    }

    /// java: private static int addCustomWaypointRelative(CommandContext<FabricClientCommandSource> context, BlockPos pos)
    pub fn add_custom_waypoint_relative(context: &CommandContext<FabricClientCommandSource>, pos: &BlockPos) -> i32 {
        if DungeonManager::is_current_room_matched() {
            let current = CURRENT_ROOM.get().clone().unwrap();
            { let __f = compat::dispatch(&current, |__t| __t.add_custom_waypoint()); __f(&current, context, pos) };
        } else {
            // java: context.getSource().sendError(Constants.PREFIX.get().append(NOT_MATCHED_TEXT));
            compat::batch_w_send_error(&(constants::PREFIX.get())().append(NOT_MATCHED_TEXT.get().clone()));
        }
        return 1;
    }

    /// java: private static RequiredArgumentBuilder<FabricClientCommandSource, ClientPosArgument> removeCustomWaypointCommand(boolean relative)
    pub fn remove_custom_waypoint_command(relative: bool) -> RequiredArgumentBuilder<FabricClientCommandSource> {
        // java: return argument("pos", ClientBlockPosArgumentType.blockPos()).executes(context -> {
        //     BlockPos pos = context.getArgument("pos", ClientPosArgument.class).toAbsoluteBlockPos(context.getSource());
        //     return relative ? removeCustomWaypointRelative(context, pos) : removeCustomWaypoint(context, pos); });
        let mut pos = client_command_registration_callback::argument("pos", Arc::new(ClientBlockPosArgumentType::block_pos()));
        pos.executes(Arc::new(move |context: &CommandContext<FabricClientCommandSource>| {
            let pos: BlockPos = ClientBlockPosArgumentType::get_block_pos(context, "pos");
            Ok(if relative { DungeonManager::remove_custom_waypoint_relative(context, &pos) } else { DungeonManager::remove_custom_waypoint_command_context_block_pos(context, &pos) })
        }));
        return pos;
    }

    /// java: private static int removeCustomWaypointRelative(CommandContext<FabricClientCommandSource> context, BlockPos pos)
    pub fn remove_custom_waypoint_relative(context: &CommandContext<FabricClientCommandSource>, pos: &BlockPos) -> i32 {
        if DungeonManager::is_current_room_matched() {
            let current = CURRENT_ROOM.get().clone().unwrap();
            { let __f = compat::dispatch(&current, |__t| __t.remove_custom_waypoint()); __f(&current, context, pos) };
        } else {
            // java: context.getSource().sendError(Constants.PREFIX.get().append(NOT_MATCHED_TEXT));
            compat::batch_w_send_error(&(constants::PREFIX.get())().append(NOT_MATCHED_TEXT.get().clone()));
        }
        return 1;
    }

    /// java: private static RequiredArgumentBuilder<FabricClientCommandSource, String> matchAgainstCommand()
    pub fn match_against_command() -> RequiredArgumentBuilder<FabricClientCommandSource> {
        // java: return argument("room", StringArgumentType.string()).suggests((_, builder) -> SharedSuggestionProvider.suggest(
        //     ROOMS_DATA.values().stream().map(Map::values).flatMap(Collection::stream).map(Map::keySet).flatMap(Collection::stream), builder))
        //     .then(argument("direction", Room.Direction.DirectionArgumentType.direction()).executes(context -> {..}));
        let mut direction = client_command_registration_callback::argument("direction", Arc::new(crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::DirectionArgumentType::direction().base));
        direction.executes(Arc::new(|context: &CommandContext<FabricClientCommandSource>| {
            if !DungeonManager::is_clearing_dungeon() {
                compat::batch_w_send_error(&DungeonManager::error("You are not in a dungeon."));
                return Ok(1);
            }
            // java: if (CLIENT.player == null || CLIENT.level == null)
            let Some(player_position) = mcrust::mcrust::modding::game::with_player(|player| player.position()) else {
                compat::batch_w_send_error(&DungeonManager::error("Failed to get player or world."));
                return Ok(1);
            };
            let Some(level) = mcrust::mcrust::modding::game::level() else {
                compat::batch_w_send_error(&DungeonManager::error("Failed to get player or world."));
                return Ok(1);
            };
            // java: ItemStack stack = CLIENT.player.getInventory().getNonEquipmentItems().get(8);
            let stack = mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().get_non_equipment_items()[8].clone()).unwrap();
            if !stack.is_item(&mcrust::net::minecraft::world::item::items::items().get("filled_map")) {
                compat::batch_w_send_error(&DungeonManager::error("Failed to get dungeon map."));
                return Ok(1);
            }
            // java: MapItemSavedData map = MapItem.getSavedData(stack.get(DataComponents.MAP_ID), CLIENT.level);
            let Some(map_handle) = map_item::get_saved_data(Some(&DungeonMap::get_map_id_component(Some(stack))), &level) else {
                compat::batch_w_send_error(&DungeonManager::error("Failed to get dungeon map state."));
                return Ok(1);
            };
            let room_name: String = StringArgumentType::get_string(context, "room");
            let direction: Direction = crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::DirectionArgumentType::get_direction(context, "direction");
            let room: Option<compat::Shared<DebugRoom>> = {
                let map = map_handle.lock().unwrap_or_else(|p| p.into_inner());
                DungeonManager::new_debug_room(&room_name, direction, &player_position, &map)
            };
            let Some(room) = room else {
                compat::batch_w_send_error(&DungeonManager::error(&format!("Failed to find room with name {}.", room_name)));
                return Ok(1);
            };
            if let Some(current) = CURRENT_ROOM.get().clone() {
                // java: currentRoom.addSubProcess(room);
                let (tickable, renderable) = (room.clone() as compat::Shared<dyn Tickable>, room.clone() as compat::Shared<dyn Renderable>);
                compat::write(&current, |__t| __t.add_sub_process(tickable, renderable));
                compat::batch_w_send_feedback(&(constants::PREFIX.get())().append_str(&format!("Matching room {} with direction {} against current room.", room_name, direction.name())));
            } else {
                compat::batch_w_send_error(&DungeonManager::error("Current room is null."));
            }
            Ok(1)
        }));
        let mut room = client_command_registration_callback::argument("room", Arc::new(StringArgumentType::string()));
        room.suggests(Some(Arc::new(|_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| {
            let names: Vec<String> = compat::lock(&ROOMS_DATA).values().flat_map(|shapes| shapes.values()).flat_map(|rooms| rooms.keys().cloned()).collect();
            Ok(shared_suggestion_provider::suggest(&names, builder))
        })));
        room.then(&direction);
        return room;
    }

    /// java: private static Component error(String message)
    pub fn error(message: &str) -> MutableComponent {
        return (constants::PREFIX.get_mut())().append(component::literal(message).with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::RED));
    }

    /// java: private static @Nullable Room newDebugRoom(String roomName, Room.Direction direction, Player player, MapItemSavedData map)
    /// (the player is read for its `position()` only, so the position is handed in; every room it builds is a `DebugRoom`)
    pub fn new_debug_room(room_name: &str, direction: Direction, player_position: &Vec3, map: &MapItemSavedData) -> Option<compat::Shared<DebugRoom>> {
        let mut room: Option<compat::Shared<DebugRoom>> = None;
        let catacombs: std::collections::HashMap<String, std::collections::HashMap<String, Vec<i32>>> = compat::lock(&ROOMS_DATA).get("catacombs").cloned().expect("NullPointerException: ROOMS_DATA.get(catacombs)");
        let by_shape = |shape: Shape| -> Option<Vec<i32>> { catacombs.get(&shape.fields().shape).expect("NullPointerException").get(room_name).cloned() };
        if let Some(room_data) = by_shape(Shape::Puzzle) {
            room = Some(DebugRoom::of_single_possible_room(Type::Puzzle, &DungeonMapUtils::get_physical_room_pos(player_position), room_name, &room_data, direction));
        } else if let Some(room_data) = by_shape(Shape::Trap) {
            room = Some(DebugRoom::of_single_possible_room(Type::Trap, &DungeonMapUtils::get_physical_room_pos(player_position), room_name, &room_data, direction));
        } else if let Some(room_data) = by_shape(Shape::Miniboss) {
            room = Some(DebugRoom::of_single_possible_room(Type::Miniboss, &DungeonMapUtils::get_physical_room_pos(player_position), room_name, &room_data, direction));
        } else if let Some(room_data) = catacombs.values().flat_map(|rooms| rooms.iter()).find(|(name, _)| name.as_str() == room_name).map(|(_, data)| data.clone()) {
            let (Some(map_entrance_pos), Some(physical_entrance_pos)) = (MAP_ENTRANCE_POS.get().clone(), PHYSICAL_ENTRANCE_POS.get().clone()) else {
                return None;
            };
            let map_room_size: i32 = compat::get_state(&MAP_ROOM_SIZE);
            let Some(map_room_pos) = DungeonMapUtils::get_map_room_pos(map, &map_entrance_pos, map_room_size) else {
                return None;
            };
            let mut segments: Vec<Vector2i> = DungeonMapUtils::get_room_segments(map, map_room_pos, map_room_size, Type::Room.fields().color);
            let physical_positions: Vec<Vector2i> = DungeonMapUtils::get_physical_pos_from_map(&map_entrance_pos, map_room_size, &physical_entrance_pos, &mut segments);
            room = Some(DebugRoom::of_single_possible_room_type_vector2ic_string_int_direction(Type::Room, &physical_positions, room_name, &room_data, direction));
        }
        return room;
    }

    /// java: public static @Nullable Vec3 getMortArmorStandPos()
    pub fn get_mort_armor_stand_pos() -> Option<Vec3> {
        if mcrust::mcrust::modding::game::level().is_none() {
            return None;
        }
        // for (var entity : CLIENT.level.entitiesForRendering()) - each entity read under its own lock (never one this
        // thread already holds)
        let entities = mcrust::mcrust::modding::game::level().unwrap().with_entities(|getter| getter.get_all());
        for entity in entities.iter() {
            if mcrust::mcrust::modding::game::is_held_here(&**entity) {
                continue;
            }
            let entity = mcrust::mcrust::modding::game::lock_entity(&**entity, "dungeon_manager.rs:DungeonManager.getMortArmorStandPos");
            if entity.as_any().is_some_and(|a| a.is::<ArmorStand>()) {
                let mut name: Option<mcrust::net::minecraft::network::chat::formatted_text::FormattedText> = entity.base().get_custom_name();
                if name.is_some() && compat::component_of(name.clone().unwrap()).get_string().contains("Mort") {
                    return Some(entity.base().position());
                }
            }
        }
        return None;
    }

    /// java: @SuppressWarnings(
    pub fn update() {
        if !(Utils::is_in_dungeons()) || DungeonManager::is_in_boss() || compat::get_state(&RUN_ENDED) {
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        if PHYSICAL_ENTRANCE_POS.get_mut().is_none() {
            let mut mort_pos: Option<Vec3> = DungeonManager::get_mort_armor_stand_pos();
            if let Some(mort_pos) = &mort_pos {
                logger().info(&format!("[Skyblocker Dungeon Secrets] Found Mort armor stand at position {}", compat::jstring(&mort_pos)));
            } else {
                logger().warn("[Skyblocker Dungeon Secrets] Failed to find Mort armor stand, retrying...");
                return;
            }
            *PHYSICAL_ENTRANCE_POS.get_mut() = Some(DungeonMapUtils::get_physical_room_pos(mort_pos.as_ref().unwrap()));
            *CURRENT_ROOM.get_mut() = DungeonManager::new_room(Type::Entrance, &vec![PHYSICAL_ENTRANCE_POS.get_mut().clone().unwrap()]);
            (dungeon_events::dungeon_loaded_invoker())();
        }
        let Some(map_handle) = DungeonManager::get_map_state(compat::minecraft()) else {
            return;
        };
        let map = map_handle.lock().unwrap_or_else(|p| p.into_inner());
        if MAP_ENTRANCE_POS.get_mut().is_none() || compat::get_state(&MAP_ROOM_SIZE) == 0 {
            let Some(mut map_entrance_pos_and_size) = DungeonMapUtils::get_map_entrance_pos_and_room_size(&map) else {
                return;
            };
            *MAP_ENTRANCE_POS.get_mut() = Some(map_entrance_pos_and_size.0.clone());
            compat::set_state(&MAP_ROOM_SIZE, map_entrance_pos_and_size.1);
            logger().info(&format!("[Skyblocker Dungeon Secrets] Started dungeon with map room size {}, map entrance pos {}, player pos {}, and physical entrance pos {}", compat::get_state(&MAP_ROOM_SIZE), MAP_ENTRANCE_POS.get().map_or("null".to_string(), |p| compat::joml_string(&[p.x() as f64, p.y() as f64])), compat::opt_str(&mcrust::mcrust::modding::game::with_player(|player| player.position())), PHYSICAL_ENTRANCE_POS.get().map_or("null".to_string(), |p| compat::joml_string(&[p.x() as f64, p.y() as f64]))));
        }
        DungeonManager::get_blood_rush_door_pos(&map);
        let mut physical_pos: Vector2i = DungeonMapUtils::get_physical_room_pos(&mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap());
        let mut map_pos: Vector2i = DungeonMapUtils::get_map_pos_from_physical(PHYSICAL_ENTRANCE_POS.get_mut().as_ref().unwrap(), MAP_ENTRANCE_POS.get_mut().clone().unwrap(), compat::get_state(&MAP_ROOM_SIZE), &physical_pos);
        let mut room: Option<compat::Shared<dyn Room>> = ROOMS.get_mut().get(&physical_pos).cloned();
        if room.is_none() {
            let mut r#type: Option<Type> = DungeonMapUtils::get_room_type(&map, &map_pos);
            if r#type.is_none() || r#type.clone().unwrap() == Type::Unknown {
                return;
            }
            match r#type {
                Some(Type::Entrance) | Some(Type::Puzzle) | Some(Type::Trap) | Some(Type::Miniboss) | Some(Type::Fairy) | Some(Type::Blood) => {
                    room = DungeonManager::new_room(r#type.clone().unwrap(), &vec![physical_pos.clone()]);
                }
                Some(Type::Room) => {
                    room = DungeonManager::new_room(r#type.clone().unwrap(), &DungeonMapUtils::get_physical_pos_from_map(MAP_ENTRANCE_POS.get_mut().as_ref().unwrap(), compat::get_state(&MAP_ROOM_SIZE), PHYSICAL_ENTRANCE_POS.get_mut().as_ref().unwrap(), &mut DungeonMapUtils::get_room_segments(&map, map_pos.clone(), compat::get_state(&MAP_ROOM_SIZE), (r#type.as_ref().unwrap()).fields().color)));
                }
                _ => {}
            }
        }
        if room.is_some() && !std::ptr::addr_eq(std::sync::Arc::as_ptr(CURRENT_ROOM.get_mut().as_ref().unwrap()), std::sync::Arc::as_ptr(room.as_ref().unwrap())) {
            *CURRENT_ROOM.get_mut() = room.clone();
        }
        if CURRENT_ROOM.get_mut().is_none() {
            return;
        }
        if { let __r = CURRENT_ROOM.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.is_matched()); __f(__r) } && compat::read(CURRENT_ROOM.get_mut().as_ref().unwrap(), |__t| __t.room_base().clear_state) != ClearState::GreenChecked {
            DungeonManager::update_room_checkmark(CURRENT_ROOM.get_mut().as_ref().unwrap(), &map);
        }
        ({ let __r = CURRENT_ROOM.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Tickable>), compat::minecraft()) });
    }

    /// java: private static void updateAllRoomCheckmarks()
    pub fn update_all_room_checkmarks() {
        if !(Utils::is_in_dungeons()) || DungeonManager::is_in_boss() || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        let mut map: Option<std::sync::Arc<std::sync::Mutex<MapItemSavedData>>> = DungeonManager::get_map_state(compat::minecraft());
        if map.is_none() || MAP_ENTRANCE_POS.get_mut().is_none() {
            return;
        }
        let map = map.as_ref().unwrap().lock().unwrap_or_else(|p| p.into_inner());
        for room in DungeonManager::get_rooms_stream().filter(|__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.is_matched()); __f(__a0) })) {
            DungeonManager::update_room_checkmark(&room, &map);
        }
    }

    /// java: private static void updateRoomCheckmark(Room room, MapItemSavedData map)
    pub fn update_room_checkmark(room: &compat::Shared<dyn Room>, map: &MapItemSavedData) {
        if compat::read(room, |__t| __t.room_base().clear_state) == ClearState::GreenChecked {
            return;
        }
        // the colour is a map byte widened to int (or -1), so narrowing it back loses nothing
        match DungeonManager::get_room_checkmark_colour(map, room, None) as i8 {
            dungeon_map_utils::GREEN_COLOR => {
                { let __v = ClearState::GreenChecked; compat::write(room, |__t| __t.room_base_mut().clear_state = __v); }
                { let __v = false; compat::write(room, |__t| __t.room_base_mut().secret_count_outdated = __v); }
                { let __v = ({ let __f = compat::dispatch(room, |__t| __t.get_max_secret_count()); __f(room) }); compat::write(room, |__t| __t.room_base_mut().secrets_found = __v); }
                ({ let __f = compat::dispatch(room, |__t| __t.mark_all_secrets()); __f(room, true) });
                (dungeon_events::secret_count_updated_invoker())(room.clone(), true);
            }
            dungeon_map_utils::WHITE_COLOR => {
                if compat::read(room, |__t| __t.room_base().clear_state) == ClearState::WhiteChecked {
                    return;
                }
                { let __v = ClearState::WhiteChecked; compat::write(room, |__t| __t.room_base_mut().clear_state = __v); }
                { let __v = true; compat::write(room, |__t| __t.room_base_mut().secret_count_outdated = __v); }
            }
            dungeon_map_utils::RED_COLOR => {
                { let __v = ClearState::Failed; compat::write(room, |__t| __t.room_base_mut().clear_state = __v); }
            }
            _ => {
                { let __v = ClearState::Uncleared; compat::write(room, |__t| __t.room_base_mut().clear_state = __v); }
            }
        }
    }

    /// java: private static @Nullable Room newRoom(Room.Type type, Vector2ic... physicalPositions)
    pub fn new_room(mut r#type: Type, physical_positions: &Vec<Vector2i>) -> Option<compat::Shared<dyn Room>> {
        // java: try {..} catch (IllegalArgumentException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut new_room: compat::Shared<dyn Room> = (RoomBase::new(r#type, physical_positions.clone()) as compat::Shared<dyn Room>);
            for physical_pos in physical_positions.iter() {
                ROOMS.get_mut().insert(physical_pos.clone(), new_room.clone());
            }
            return Some(new_room);
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Dungeon Secrets] Failed to create room"));
            }
        }
        return None;
    }

    /// java: protected static void addRoomFromWs(Room room)
    pub fn add_room_from_ws(mut room: compat::Shared<dyn Room>) {
        for physical_pos in compat::read(&room, |__t| __t.room_base().segments.clone()) {
            ROOMS.get_mut().insert(physical_pos.clone(), room.clone());
        }
    }

    /// java: protected static boolean checkIfSegmentsExist(List<Vector2ic> segments)
    pub fn check_if_segments_exist(segments: &Vec<Vector2i>) -> bool {
        return segments.iter().cloned().any(|__a0| ROOMS.get_mut().contains_key(&__a0));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if DungeonManager::should_process() && CURRENT_ROOM.get_mut().is_some() {
            ({ let __r = CURRENT_ROOM.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.extract_rendering()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Renderable>), collector) });
        }
        if BLOOD_RUSH_DOOR_BOX.get_mut().is_some() && !(compat::get_state(&BLOOD_OPENED)) && SkyblockerConfigManager::get().dungeons.door_highlight.enable_door_highlight {
            let mut color_components: Vec<f32> = (if compat::get_state(&HAS_KEY) { GREEN_COLOR_COMPONENTS.clone() } else { RED_COLOR_COMPONENTS.clone() });
            match SkyblockerConfigManager::get().dungeons.door_highlight.door_highlight_type {
                crate::de::hysky::skyblocker::config::configs::dungeons_config::Type::Highlight => {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, BLOOD_RUSH_DOOR_BOX.get_mut().as_ref().unwrap(), &color_components, 0.5f32, true) });
                }
                crate::de::hysky::skyblocker::config::configs::dungeons_config::Type::OutlinedHighlight => {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, BLOOD_RUSH_DOOR_BOX.get_mut().as_ref().unwrap(), &color_components, 0.5f32, true) });
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, BLOOD_RUSH_DOOR_BOX.get_mut().as_ref().unwrap(), &color_components, ((5) as f32) as f32, true) });
                }
                crate::de::hysky::skyblocker::config::configs::dungeons_config::Type::Outline => {
                    ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, BLOOD_RUSH_DOOR_BOX.get_mut().as_ref().unwrap(), &color_components, ((5) as f32) as f32, true) });
                }
                _ => {}
            }
        }
    }

    /// java: @SuppressWarnings("SameReturnValue") private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if !(DungeonManager::should_process()) {
            return true;
        }
        let mut message: String = chat_formatting::strip_formatting(&text.get_string());
        if DungeonManager::is_current_room_matched() {
            ({ let __r = CURRENT_ROOM.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.on_chat_message()); __f(__r, &message, overlay) });
        }
        if SkyblockerConfigManager::get().dungeons.door_highlight.enable_door_highlight && !(compat::get_state(&BLOOD_OPENED)) {
            if BLOOD_DOOR_OPENED == message {
                compat::set_state(&BLOOD_OPENED, true);
            }
            if KEY_FOUND.matcher(&message).matches() {
                compat::set_state(&HAS_KEY, true);
            }
            if WITHER_DOOR_OPENED.matcher(&message).matches() {
                compat::set_state(&HAS_KEY, false);
            }
        }
        if message == "[NPC] Mort: You should find it useful if you get lost." {
            (dungeon_events::dungeon_started_invoker())();
        }
        if TEAM_SCORE_PATTERN.matcher(&message).matches() {
            (dungeon_events::dungeon_ended_invoker())();
            DungeonManager::reset();
            compat::set_state(&RUN_ENDED, true);
        }
        let mut new_boss: DungeonBoss = DungeonBoss::from_message(&message);
        if !(DungeonManager::is_in_boss()) && new_boss.is_in_boss() {
            DungeonManager::reset();
            compat::set_state(&BOSS, new_boss);
        }
        return true;
    }

    /// java: @SuppressWarnings("JavadocReference") private static InteractionResult onUseBlock(Level world, BlockHitResult hitResult)
    pub fn on_use_block(world: &Level, hit_result: &BlockHitResult) -> InteractionResult {
        if DungeonManager::is_current_room_matched() {
            ({ let __r = CURRENT_ROOM.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.on_use_block()); __f(__r, world, hit_result.get_block_pos()) });
        }
        return InteractionResult::Pass;
    }

    /// java: public static void onChestOpened(BlockPos pos)
    pub fn on_chest_opened(pos: &BlockPos) {
        if DungeonManager::is_current_room_matched() {
            ({ let __r = CURRENT_ROOM.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.on_chest_opened()); __f(__r, pos) });
        }
    }

    /// java: @SuppressWarnings("JavadocReference") public static void onItemPickup(ItemEntity itemEntity)
    pub fn on_item_pickup(item_entity: &ItemEntity) {
        use mcrust::net::minecraft::world::entity::entity::Entity as _;
        let mut room: Option<compat::Shared<dyn Room>> = DungeonManager::get_room_at_physical(&item_entity.base().position());
        if DungeonManager::is_room_matched(room.clone()) {
            let room: compat::Shared<dyn Room> = room.unwrap();
            ({ let __f = compat::dispatch(&room, |__t| __t.on_item_pickup()); __f(&room, item_entity) });
        }
    }

    /// java: @SuppressWarnings("JavadocReference") public static void onBatRemoved(AmbientCreature bat)
    pub fn on_bat_removed(bat: &dyn AmbientCreature) {
        let mut room: Option<compat::Shared<dyn Room>> = DungeonManager::get_room_at_physical(&bat.base().position());
        if DungeonManager::is_room_matched(room.clone()) {
            let room: compat::Shared<dyn Room> = room.unwrap();
            ({ let __f = compat::dispatch(&room, |__t| __t.on_bat_removed()); __f(&room, bat) });
        }
    }

    /// java: public static boolean markSecrets(int secretIndex, boolean found)
    pub fn mark_secrets(mut secret_index: i32, mut found: bool) -> bool {
        if DungeonManager::is_current_room_matched() {
            return ({ let __r = CURRENT_ROOM.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.mark_secrets()); __f(__r, secret_index, found) });
        }
        return false;
    }

    /// java: private static @Nullable Room getRoomAtPhysical(Vec3 pos)
    pub fn get_room_at_physical(pos: &Vec3) -> Option<compat::Shared<dyn Room>> {
        if compat::get_state(&room_preview_server::IS_ACTIVE) {
            return CURRENT_ROOM.get_mut().clone();
        }
        return ROOMS.get_mut().get(&DungeonMapUtils::get_physical_room_pos(pos)).cloned();
    }

    /// java: private static @Nullable Room getRoomAtPhysical(Vec3i pos)
    pub fn get_room_at_physical_vec3i(pos: &Vec3i) -> Option<compat::Shared<dyn Room>> {
        if compat::get_state(&room_preview_server::IS_ACTIVE) {
            return CURRENT_ROOM.get_mut().clone();
        }
        return ROOMS.get_mut().get(&DungeonMapUtils::get_physical_room_pos_vec3i(pos)).cloned();
    }

    /// java: private static @Nullable MapItemSavedData getMapState(Minecraft client)
    /// The level's map data object (the port keeps it behind its `Mutex`; callers lock it where Java reads it).
    pub fn get_map_state(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) -> Option<std::sync::Arc<std::sync::Mutex<MapItemSavedData>>> {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return None;
        }
        return map_item::get_saved_data(Some(&DungeonMap::get_map_id_component(mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().get_non_equipment_items()[(8) as usize].clone()))), &mcrust::mcrust::modding::game::level().unwrap());
    }

    /// java: public static boolean isClearingDungeon()
    pub fn is_clearing_dungeon() -> bool {
        return (PHYSICAL_ENTRANCE_POS.get_mut().is_some() && MAP_ENTRANCE_POS.get_mut().is_some() && compat::get_state(&MAP_ROOM_SIZE) != 0);
    }

    /// java: public static boolean isCurrentRoomMatched()
    pub fn is_current_room_matched() -> bool {
        return DungeonManager::is_room_matched(CURRENT_ROOM.get_mut().clone());
    }

    /// java: @Contract("null -> false") private static boolean isRoomMatched(@Nullable Room room)
    pub fn is_room_matched(mut room: Option<compat::Shared<dyn Room>>) -> bool {
        return (DungeonManager::should_process() && room.is_some() && { let __r = room.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.is_matched()); __f(__r) });
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        return (Utils::is_in_dungeons() || compat::get_state(&room_preview_server::IS_ACTIVE));
    }

    /// java: private static void reset()
    pub fn reset() {
        *MAP_ENTRANCE_POS.get_mut() = None;
        compat::set_state(&MAP_ROOM_SIZE, 0);
        *PHYSICAL_ENTRANCE_POS.get_mut() = None;
        ROOMS.get_mut().clear();
        *CURRENT_ROOM.get_mut() = None;
        compat::set_state(&BOSS, DungeonBoss::None);
        *BLOOD_RUSH_DOOR_BOX.get_mut() = None;
        compat::set_state(&BLOOD_OPENED, false);
        compat::set_state(&HAS_KEY, false);
        compat::set_state(&RUN_ENDED, false);
    }

    /// java: private static void getBloodRushDoorPos(MapItemSavedData map)
    pub fn get_blood_rush_door_pos(map: &MapItemSavedData) {
        if MAP_ENTRANCE_POS.get_mut().is_none() || PHYSICAL_ENTRANCE_POS.get_mut().is_none() || compat::get_state(&MAP_ROOM_SIZE) == 0 {
            logger().error(&format!("[Skyblocker Dungeon Secrets] Dungeon map info missing with map entrance pos {} and map room size {}", MAP_ENTRANCE_POS.get().map_or("null".to_string(), |p| compat::joml_string(&[p.x() as f64, p.y() as f64])), compat::get_state(&MAP_ROOM_SIZE)));
            return;
        }
        let mut n_w_most_room: Vector2i = DungeonMapUtils::get_map_pos_for_nw_most_room(MAP_ENTRANCE_POS.get_mut().as_ref().unwrap(), compat::get_state(&MAP_ROOM_SIZE));
        {
            let mut x: i32 = n_w_most_room.x.wrapping_add((compat::get_state(&MAP_ROOM_SIZE)).wrapping_div(2));
            while x < 128 {
                {
                    let mut y: i32 = n_w_most_room.y.wrapping_add(compat::get_state(&MAP_ROOM_SIZE));
                    while y < 128 {
                        let mut color: i8 = DungeonMapUtils::get_color_3(map, x, y);
                        if (color as i32) == 119 || (color as i32) == 18 {
                            let mut door_pos: Vector2i = DungeonMapUtils::get_physical_pos_from_map_vector2ic_int_vector2ic_vector2ic(MAP_ENTRANCE_POS.get_mut().as_ref().unwrap(), compat::get_state(&MAP_ROOM_SIZE), PHYSICAL_ENTRANCE_POS.get_mut().clone().unwrap(), &Vector2i::new(x.wrapping_sub((compat::get_state(&MAP_ROOM_SIZE)).wrapping_div(2)), y.wrapping_sub(compat::get_state(&MAP_ROOM_SIZE))));
                            *BLOOD_RUSH_DOOR_BOX.get_mut() = Some(AABB::new((door_pos.x().wrapping_add(14)) as f64, (69) as f64, (door_pos.y().wrapping_add(30)) as f64, (door_pos.x().wrapping_add(17)) as f64, (73) as f64, (door_pos.y().wrapping_add(33)) as f64));
                            return;
                        }
                        y = y.wrapping_add((compat::get_state(&MAP_ROOM_SIZE)).wrapping_add(4));
                    }
                }
                x = x.wrapping_add((compat::get_state(&MAP_ROOM_SIZE)).wrapping_add(4));
            }
        }
        {
            let mut x: i32 = n_w_most_room.x.wrapping_add(compat::get_state(&MAP_ROOM_SIZE));
            while x < 128 {
                {
                    let mut y: i32 = n_w_most_room.y.wrapping_add((compat::get_state(&MAP_ROOM_SIZE)).wrapping_div(2));
                    while y < 128 {
                        let mut color: i8 = DungeonMapUtils::get_color_3(map, x, y);
                        if (color as i32) == 119 || (color as i32) == 18 {
                            let mut door_pos: Vector2i = DungeonMapUtils::get_physical_pos_from_map_vector2ic_int_vector2ic_vector2ic(MAP_ENTRANCE_POS.get_mut().as_ref().unwrap(), compat::get_state(&MAP_ROOM_SIZE), PHYSICAL_ENTRANCE_POS.get_mut().clone().unwrap(), &Vector2i::new(x.wrapping_sub(compat::get_state(&MAP_ROOM_SIZE)), y.wrapping_sub((compat::get_state(&MAP_ROOM_SIZE)).wrapping_div(2))));
                            *BLOOD_RUSH_DOOR_BOX.get_mut() = Some(AABB::new((door_pos.x().wrapping_add(30)) as f64, (69) as f64, (door_pos.y().wrapping_add(14)) as f64, (door_pos.x().wrapping_add(33)) as f64, (73) as f64, (door_pos.y().wrapping_add(17)) as f64));
                            return;
                        }
                        y = y.wrapping_add((compat::get_state(&MAP_ROOM_SIZE)).wrapping_add(4));
                    }
                }
                x = x.wrapping_add((compat::get_state(&MAP_ROOM_SIZE)).wrapping_add(4));
            }
        }
    }

    /// java: public static int getRoomCheckmarkColour(MapItemSavedData mapState, Room room, @Nullable Consumer<Vector2ic> onCheckmarkFound)
    pub fn get_room_checkmark_colour(map_state: &MapItemSavedData, room: &compat::Shared<dyn Room>, on_checkmark_found: Option<&dyn Fn(Vector2i)>) -> i32 {
        if PHYSICAL_ENTRANCE_POS.get_mut().is_none() || MAP_ENTRANCE_POS.get_mut().is_none() {
            return -1;
        }
        let mut half_room_size: i32 = (compat::get_state(&MAP_ROOM_SIZE)).wrapping_div(2);
        for segment_physical_pos in compat::read(room, |__t| __t.room_base().segments.clone()) {
            let mut top_left_corner: Vector2i = DungeonMapUtils::get_map_pos_from_physical(PHYSICAL_ENTRANCE_POS.get_mut().as_ref().unwrap(), MAP_ENTRANCE_POS.get_mut().clone().unwrap(), compat::get_state(&MAP_ROOM_SIZE), &segment_physical_pos);
            // java: Vector2ic middle = topLeftCorner.add(halfRoomSize, halfRoomSize, new Vector2i());
            let mut middle: Vector2i = Vector2i::zero();
            top_left_corner.add_dest(half_room_size, half_room_size, &mut middle);
            for offset in 0..half_room_size {
                let mut map_pos: Vector2i = Vector2i::new(middle.x(), middle.y().wrapping_add(offset));
                let mut colour: i32 = (DungeonMapUtils::get_color(map_state, Some(map_pos.clone()))) as i32;
                if DungeonManager::matches_checkmark_colour(colour) {
                    if let Some(on_checkmark_found) = on_checkmark_found {
                        on_checkmark_found(map_pos.clone());
                    }
                    return colour;
                }
            }
        }
        return -1;
    }

    /// java: public static boolean matchesCheckmarkColour(int colour)
    pub fn matches_checkmark_colour(mut colour: i32) -> bool {
        return (colour == (dungeon_map_utils::WHITE_COLOR as i32) || colour == (dungeon_map_utils::GREEN_COLOR as i32) || colour == (dungeon_map_utils::RED_COLOR as i32));
    }

    /// java: public static Optional<int[]> getRoomBlockData(String roomType, String roomName)
    pub fn get_room_block_data(room_type: &str, room_name: &str) -> Option<Vec<i32>> {
        let Some(mut type_data) = compat::lock(&ROOMS_DATA).get("catacombs").cloned().unwrap().get(room_type).cloned() else {
            return None;
        };
        return type_data.get(room_name).cloned();
    }

    /// java: public static void startFromRoomPreview(Room room)
    pub fn start_from_room_preview(mut room: compat::Shared<dyn Room>) {
        for segment in compat::read(&room, |__t| __t.room_base().segments.clone()) {
            ROOMS.get_mut().insert(segment.clone(), room.clone());
        }
        *CURRENT_ROOM.get_mut() = Some(room.clone());
        compat::set_state(&RUN_ENDED, true);
    }

    /// java: public static CompletableFuture<Suggestions> suggestRoomTypes(CommandContext<FabricClientCommandSource> ctx, SuggestionsBuilder suggestionsBuilder)
    pub fn suggest_room_types(ctx: &CommandContext<FabricClientCommandSource>, suggestions_builder: &mut SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        return shared_suggestion_provider::suggest(&compat::lock(&ROOMS_DATA).get("catacombs").cloned().unwrap().keys().cloned().collect::<Vec<String>>(), suggestions_builder);
    }

    /// java: public static CompletableFuture<Suggestions> suggestRooms(String roomType, SuggestionsBuilder suggestionsBuilder)
    pub fn suggest_rooms(room_type: &str, suggestions_builder: &mut SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        if compat::lock(&ROOMS_DATA).get("catacombs").cloned().unwrap().get(room_type).cloned().is_none() {
            return Suggestions::empty();
        }
        return shared_suggestion_provider::suggest(&compat::lock(&ROOMS_DATA).get("catacombs").cloned().unwrap().get(room_type).cloned().unwrap().keys().cloned().collect::<Vec<String>>(), suggestions_builder);
    }

    /// java: @VisibleForTesting public static int getLoadedRoomCount()
    pub fn get_loaded_room_count() -> i32 {
        return (compat::lock(&ROOMS_INFO).len() as i32);
    }

    /// java: private static void checkResourceSource(Identifier id, Resource resource)
    pub fn check_resource_source(id: &Identifier, resource: &Resource) {
        Utils::check_for_illegal_resource_modification(id, resource, "[Skyblocker] Modifying the resource {} with resource packs is NOT SUPPORTED! This asset is important and must remain unmodified. Contact the Skyblocker devs for more info, ****THE GAME WILL NOW BE LOST****. Evil Resource Pack: {}");
    }

}

/// java: public record RoomInfo(String name)
#[derive(Clone, Debug, PartialEq)]
pub struct RoomInfo {
    /// java: String name
    pub name: String,
}

impl RoomInfo {
    /// java: the canonical record constructor
    pub fn new(mut name: String) -> Self {
        Self { name }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

}

/// java: public record RoomWaypoint(String secretName, SecretWaypoint.Category category, int x, int y, int z)
#[derive(Clone, Debug, PartialEq)]
pub struct RoomWaypoint {
    /// java: String secretName
    pub secret_name: String,
    /// java: SecretWaypoint.Category category
    pub category: crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category,
    /// java: int x
    pub x: i32,
    /// java: int y
    pub y: i32,
    /// java: int z
    pub z: i32,
}

impl RoomWaypoint {
    /// java: the canonical record constructor
    pub fn new(mut secret_name: String, mut category: crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category, mut x: i32, mut y: i32, mut z: i32) -> Self {
        Self { secret_name, category, x, y, z }
    }

    /// java: the record accessor `secretName()`
    pub fn secret_name(&self) -> String {
        self.secret_name.clone()
    }

    /// java: the record accessor `category()`
    pub fn category(&self) -> crate::de::hysky::skyblocker::skyblock::dungeon::secrets::secret_waypoint::Category {
        self.category
    }

    /// java: the record accessor `x()`
    pub fn x(&self) -> i32 {
        self.x
    }

    /// java: the record accessor `y()`
    pub fn y(&self) -> i32 {
        self.y
    }

    /// java: the record accessor `z()`
    pub fn z(&self) -> i32 {
        self.z
    }

}

/// java: public record RoomData(RoomInfo info, List<RoomWaypoint> secrets)
#[derive(Clone, Debug, PartialEq)]
pub struct RoomData {
    /// java: RoomInfo info
    pub info: RoomInfo,
    /// java: List<RoomWaypoint> secrets
    pub secrets: Vec<RoomWaypoint>,
}

impl RoomData {
    /// java: the canonical record constructor
    pub fn new(mut info: RoomInfo, mut secrets: Vec<RoomWaypoint>) -> Self {
        Self { info, secrets }
    }

    /// java: the record accessor `info()`
    pub fn info(&self) -> RoomInfo {
        self.info.clone()
    }

    /// java: the record accessor `secrets()`
    pub fn secrets(&self) -> Vec<RoomWaypoint> {
        self.secrets.clone()
    }

}


/// `new InflaterInputStream(in)` read to its end: the zlib stream a `.skeleton` resource is (`Inflater` with the
/// zlib wrapper, as `InflaterInputStream`'s default `new Inflater()`).
fn inflate_stream(compressed: &[u8]) -> Vec<u8> {
    let mut inflater = mcrust::java::util::zip::inflater::Inflater::new();
    inflater.set_input(compressed);
    let mut out: Vec<u8> = Vec::new();
    let mut buffer = vec![0u8; 8192];
    while !inflater.finished() {
        let n = inflater.inflate(&mut buffer).unwrap_or_else(|e| panic!("ZipException: {}", e.to_string()));
        if n == 0 && !inflater.finished() {
            panic!("EOFException: Unexpected end of ZLIB input stream");
        }
        out.extend_from_slice(&buffer[..n]);
    }
    inflater.end();
    out
}

/// `(int[]) new ObjectInputStream(..).readObject()` - Java object serialization of one `int[]`: the stream header
/// (`STREAM_MAGIC`, `STREAM_VERSION`), `TC_ARRAY`, the class descriptor of `[I` (`TC_CLASSDESC`, name,
/// serialVersionUID, flags, no fields, `TC_ENDBLOCKDATA`, no superclass), the length and the big-endian ints.
/// Anything else is the `ClassCastException` / `StreamCorruptedException` Java would throw.
fn read_serialized_int_array(bytes: &[u8]) -> Vec<i32> {
    fn next<'a>(bytes: &'a [u8], at: &mut usize, n: usize) -> &'a [u8] {
        let slice = bytes.get(*at..*at + n).unwrap_or_else(|| panic!("EOFException"));
        *at += n;
        slice
    }
    let mut at = 0usize;
    let mut take = |n: usize| next(bytes, &mut at, n);
    let u16_of = |b: &[u8]| u16::from_be_bytes([b[0], b[1]]);
    if take(2) != [0xAC, 0xED] || u16_of(take(2)) != 5 {
        panic!("StreamCorruptedException: invalid stream header");
    }
    if take(1)[0] != 0x75 {
        panic!("ClassCastException: the serialized object is not an array");
    }
    if take(1)[0] != 0x72 {
        panic!("StreamCorruptedException: expected a class descriptor");
    }
    let name_len = u16_of(take(2)) as usize;
    if take(name_len) != b"[I" {
        panic!("ClassCastException: the serialized array is not an int[]");
    }
    take(8); // serialVersionUID
    take(1); // flags (SC_SERIALIZABLE)
    if u16_of(take(2)) != 0 {
        panic!("StreamCorruptedException: an int[] has no fields");
    }
    if take(1)[0] != 0x78 || take(1)[0] != 0x70 {
        panic!("StreamCorruptedException: expected TC_ENDBLOCKDATA and a null superclass");
    }
    let len_bytes = take(4);
    let length = i32::from_be_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]);
    if length < 0 {
        panic!("NegativeArraySizeException: {}", length);
    }
    let mut out = Vec::with_capacity(length as usize);
    for _ in 0..length {
        let b = take(4);
        out.push(i32::from_be_bytes([b[0], b[1], b[2], b[3]]));
    }
    out
}
