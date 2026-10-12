//! Ported from de.hysky.skyblocker.skyblock.waypoint.Waypoints by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint_group;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint_group::WaypointGroup;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::base64;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToastId;
use mcrust::net::minecraft::commands::arguments::string_representable_argument::StringRepresentableArgument;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: private static final Codec<List<WaypointGroup>> CODEC = WaypointGroup.CODEC.listOf();
pub static CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<WaypointGroup>>>> = compat::MainThread::new(|| codec::list(waypoint_group::CODEC.get_mut().clone()));
/// java: private static final Codec<List<WaypointGroup>> SKYTILS_CODEC = WaypointGroup.SKYTILS_CODEC.listOf();
pub static SKYTILS_CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<WaypointGroup>>>> = compat::MainThread::new(|| codec::list(waypoint_group::SKYTILS_CODEC.get_mut().clone()));
// TODO(fabric2seer): static call Map.values(..)
//   java: private static final Codec<Collection<WaypointGroup>> SKYBLOCKER_LEGACY_ORDERED_CODEC = Codec.unboundedMap(Codec.STRING, WaypointGroup.SKYBLOCKER_LEGACY_ORDERED_CODEC).xmap(Map::values, groups -> groups.stream().collect(Collectors.toMap(WaypointGroup::name, Function.identity())));
/// java: private static final Codec<Collection<WaypointGroup>> SKYBLOCKER_LEGACY_ORDERED_CODEC = Codec.unboundedMap(Codec.STRING, WaypointGroup.SKYBLOCKER_LEGACY_ORDERED_CODEC).xmap(Map::values, groups -> grou
pub static SKYBLOCKER_LEGACY_ORDERED_CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<WaypointGroup>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `SKYBLOCKER_LEGACY_ORDERED_CODEC` - see the TODO above"));
/// java: private static final String PREFIX = "[Skyblocker-Waypoint-Data-V1]";
pub const PREFIX: &str = "[Skyblocker-Waypoint-Data-V1]";
/// java: private static final String SKYBLOCKER_LEGACY_ORDERED = "[Skyblocker::OrderedWaypoints::v1]";
pub const SKYBLOCKER_LEGACY_ORDERED: &str = "[Skyblocker::OrderedWaypoints::v1]";
/// java: protected static final SystemToast.SystemToastId WAYPOINTS_TOAST_TYPE = new SystemToast.SystemToastId();
pub static WAYPOINTS_TOAST_TYPE: compat::MainThread<SystemToastId> = compat::MainThread::new(|| SystemToastId::new());
/// java: private static final Path WAYPOINTS_FILE = SkyblockerMod.CONFIG_DIR.resolve("waypoints.json");
pub static WAYPOINTS_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("waypoints.json"));
/// java: private static final Path SKYBLOCKER_LEGACY_ORDERED_FILE = SkyblockerMod.CONFIG_DIR.resolve("ordered_waypoints.json");
pub static SKYBLOCKER_LEGACY_ORDERED_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("ordered_waypoints.json"));
// TODO(fabric2seer): static call MultimapBuilder.enumKeys(..)
//   java: private static final Multimap<Location, WaypointGroup> waypoints = MultimapBuilder.enumKeys(Location.class).arrayListValues().build();
/// java: private static final Multimap<Location, WaypointGroup> waypoints = MultimapBuilder.enumKeys(Location.class).arrayListValues().build();
pub static WAYPOINTS: compat::MainThread<crate::unresolved::Multimap> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `waypoints` - see the TODO above"));
// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: private static final Codec<OrderedAction> CODEC = StringRepresentable.fromEnum(OrderedAction::values);
/// java: private static final Codec<OrderedAction> CODEC = StringRepresentable.fromEnum(OrderedAction::values);
pub static ORDERED_ACTION_CODEC: compat::MainThread<CodecRef<OrderedAction>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public class Waypoints
#[derive(Clone)]
pub struct Waypoints {}

impl Waypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        Waypoints::load_waypoints();
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(Waypoints::saveWaypoints);
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { Waypoints::extract_rendering(&__a0); }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
                //   java: 				.then(literal("waypoints").executes(Scheduler.queueOpenScreenCommand(() -> new WaypointsScreen(Minecraft.getInstance().gui.screen())))
                //   java: 						.then(literal("ordered").then(argument("action", OrderedAction.ArgumentType.orderedAction()).executes(Waypoints::executeOrderedWaypointAction)))
                //   java: 				));
            },
        ));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            Waypoints::reset();
        }));
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { Waypoints::tick(); }), 1) });
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext access)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   java: 				.then(literal("waypoints").executes(Scheduler.queueOpenScreenCommand(() -> new WaypointsScreen(Minecraft.getInstance().gui.screen())))
        //   java: 						.then(literal("ordered").then(argument("action", OrderedAction.ArgumentType.orderedAction()).executes(Waypoints::executeOrderedWaypointAction)))
        //   java: 				));
    }

    /// java: private static int executeOrderedWaypointAction(CommandContext<FabricClientCommandSource> context)
    pub fn execute_ordered_waypoint_action(context: &CommandContext<crate::unresolved::FabricClientCommandSource>) -> i32 {
        // TODO(fabric2seer): `.get(..)` on a Multimap
        //   java: Optional<WaypointGroup> groupOptional = waypoints.get(Utils.getLocation()).stream()
        //   java: 				.filter(group -> group.ordered() && !group.waypoints().isEmpty() && group.waypoints().stream().allMatch(Waypoint::isEnabled))
        //   java: 				.findFirst();
        let mut group_optional: Option<compat::Shared<WaypointGroup>> = todo!("fabric2seer: the initialiser of `groupOptional` (TODO above)");
        if group_optional.is_none() {
            // TODO(fabric2seer): `.sendFeedback(..)` on a FabricClientCommandSource
            //   java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.literal("No ordered group enabled here! (make sure all waypoints in the group are enabled)")));
            return 1;
        }
        let mut group: compat::Shared<WaypointGroup> = group_optional.clone().unwrap();
        let mut action: OrderedAction = ArgumentType::get_ordered_action(context, "action");
        let mut index: i32 = WaypointGroup::current_index(&group);
        let mut waypoint_count: i32 = (WaypointGroup::waypoints(&group).len() as i32);
        match action {
            OrderedAction::First | OrderedAction::Reset => {
                WaypointGroup::reset_current_index(&group);
            }
            OrderedAction::Next => {
                WaypointGroup::set_current_index(&group, (index.wrapping_add(1)).wrapping_rem(waypoint_count));
            }
            OrderedAction::Previous => {
                WaypointGroup::set_current_index(&group, ((index.wrapping_sub(1)).wrapping_add(waypoint_count)).wrapping_rem(waypoint_count));
            }
            _ => {}
        }
        return 1;
    }

    /// java: public static void loadWaypoints()
    pub fn load_waypoints() {
        // TODO(fabric2seer): `.clear(..)` on a Multimap
        //   java: waypoints.clear();
        {
            // TODO(fabric2seer): Files.newBufferedReader
            //   java: BufferedReader reader = Files.newBufferedReader(..);
            let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
            // java: try {..} catch (NoSuchFileException _) catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.fromJson(..)` on a Gson
                //   java: List<WaypointGroup> waypointGroups = CODEC.parse(JsonOps.INSTANCE, SkyblockerMod.GSON.fromJson(reader, JsonArray.class)).resultOrPartial(LOGGER::error).orElseThrow();
                let mut waypoint_groups: Vec<compat::Shared<WaypointGroup>> = todo!("fabric2seer: the initialiser of `waypointGroups` (TODO above)");
                for __a0 in waypoint_groups.iter() {
                    Waypoints::put_waypoint_group(__a0.clone());
                }
            }));
            if let Err(_) = __result {
            }
            // TODO(fabric2seer): the second `catch (Exception e)` clause (merged into the first)
            //   java: {
            //   java: 			LOGGER.error("[Skyblocker Waypoints] Encountered exception while loading waypoints", e);
            //   java: 		}
        }
        {
            // TODO(fabric2seer): Files.newBufferedReader
            //   java: BufferedReader reader = Files.newBufferedReader(..);
            let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
            // java: try {..} catch (NoSuchFileException | FileAlreadyExistsException _) catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.fromJson(..)` on a Gson
                //   java: Collection<WaypointGroup> waypointGroups = SKYBLOCKER_LEGACY_ORDERED_CODEC.parse(JsonOps.INSTANCE, SkyblockerMod.GSON.fromJson(reader, JsonObject.class)).resultOrPartial(LOGGER::error).orElseThrow();
                let mut waypoint_groups: Vec<compat::Shared<WaypointGroup>> = todo!("fabric2seer: the initialiser of `waypointGroups` (TODO above)");
                for group in waypoint_groups.iter() {
                    Waypoints::put_waypoint_group(WaypointGroup::deep_copy(&WaypointGroup::with_island(group, Location::DwarvenMines)));
                    Waypoints::put_waypoint_group(WaypointGroup::deep_copy(&WaypointGroup::with_island(group, Location::CrystalHollows)));
                }
                std::fs::rename(SKYBLOCKER_LEGACY_ORDERED_FILE.get_mut(), &skyblocker_mod::CONFIG_DIR.get_mut().join("legacy_ordered_waypoints.json")).expect("IOException");
                // TODO(fabric2seer): static `List.size` has no Rust counterpart in the port
                // rust: logger().info(&format!("[Skyblocker Waypoints] Successfully migrated {} ordered waypoints from {} groups to waypoints!", waypoint_groups.iter().cloned().map(|__a0| WaypointGroup::waypoints(&__a0)).map(|__a0| List::size(__a0.clone())).sum::<i32>(), (waypoint_groups.len() as i32)));
            }));
            if let Err(_) = __result {
            }
            // TODO(fabric2seer): the second `catch (IOException e)` clause (merged into the first)
            //   java: {
            //   java: 			LOGGER.error("[Skyblocker Waypoints] Encountered exception while loading legacy ordered waypoints", e);
            //   java: 		}
        }
    }

    /// java: public static void saveWaypoints(Minecraft client)
    pub fn save_waypoints(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.values(..)` on a Multimap
                //   java: JsonElement waypointsJson = CODEC.encodeStart(JsonOps.INSTANCE, List.copyOf(waypoints.values())).resultOrPartial(LOGGER::error).orElseThrow();
                let mut waypoints_json: serde_json::Value = todo!("fabric2seer: the initialiser of `waypointsJson` (TODO above)");
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(waypointsJson, writer);
                logger().info("[Skyblocker Waypoints] Saved waypoints");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Waypoints] Encountered exception while saving waypoints"));
            }
        }
    }

    /// java: public static @Nullable List<WaypointGroup> fromSkyblocker(String waypointsString, Location defaultIsland)
    pub fn from_skyblocker(mut waypoints_string: String, mut default_island: Location) -> Option<Vec<compat::Shared<WaypointGroup>>> {
        waypoints_string = compat::trim(&waypoints_string);
        if waypoints_string.starts_with(PREFIX) {
            {
                // TODO(fabric2seer): `new GZIPInputStream(..)`
                //   java: GZIPInputStream reader = new GZIPInputStream(new ByteArrayInputStream(Base64.getDecoder().decode(waypointsString.replace(PREFIX, ""))));
                let mut reader: crate::unresolved::GZIPInputStream = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): `.fromJson(..)` on a Gson
                    //   java: return CODEC.parse(JsonOps.INSTANCE, SkyblockerMod.GSON.fromJson(new String(reader.readAllBytes()), JsonArray.class)).resultOrPartial(LOGGER::error).orElseThrow();
                })) {
                    Ok(__v) => return __v,
                    Err(e) => {
                        logger().error(&format!("[Skyblocker Waypoints] Encountered exception while parsing Skyblocker waypoint data"));
                        return None;
                    }
                }
            }
        } else if waypoints_string.starts_with(SKYBLOCKER_LEGACY_ORDERED) {
            {
                // TODO(fabric2seer): `new GZIPInputStream(..)`
                //   java: GZIPInputStream reader = new GZIPInputStream(new ByteArrayInputStream(Base64.getDecoder().decode(waypointsString.replace(SKYBLOCKER_LEGACY_ORDERED, ""))));
                let mut reader: crate::unresolved::GZIPInputStream = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): `.fromJson(..)` on a Gson
                    //   java: return applyDefaultLocation(SKYBLOCKER_LEGACY_ORDERED_CODEC.parse(JsonOps.INSTANCE, SkyblockerMod.GSON.fromJson(new String(reader.readAllBytes()), JsonObject.class)).resultOrPartial(LOGGER::error).orElseThrow(), defaultIsland);
                })) {
                    Ok(__v) => return __v,
                    Err(e) => {
                        logger().error(&format!("[Skyblocker Waypoints] Encountered exception while parsing Skyblocker legacy ordered waypoint data"));
                        return None;
                    }
                }
            }
        }
        logger().error("[Skyblocker Waypoints] Unknown skyblocker waypoint data prefix");
        return None;
    }

    /// java: public static String toSkyblocker(List<WaypointGroup> waypointGroups)
    pub fn to_skyblocker(waypoint_groups: &Vec<compat::Shared<WaypointGroup>>) -> String {
        // TODO(fabric2seer): `.toJson(..)` on a Gson
        //   java: String waypointsJson = SkyblockerMod.GSON.toJson(CODEC.encodeStart(JsonOps.INSTANCE, waypointGroups).resultOrPartial(LOGGER::error).orElseThrow());
        let mut waypoints_json: String = todo!("fabric2seer: the initialiser of `waypointsJson` (TODO above)");
        // TODO(fabric2seer): `new ByteArrayOutputStream(..)`
        //   java: ByteArrayOutputStream output = new ByteArrayOutputStream();
        let mut output: crate::unresolved::ByteArrayOutputStream = todo!("fabric2seer: the initialiser of `output` (TODO above)");
        {
            // TODO(fabric2seer): `new GZIPOutputStream(..)`
            //   java: GZIPOutputStream gzip = new GZIPOutputStream(output);
            let mut gzip: crate::unresolved::GZIPOutputStream = todo!("fabric2seer: the initialiser of `gzip` (TODO above)");
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.write(..)` on a GZIPOutputStream
                //   java: gzip.write(waypointsJson.getBytes());
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Waypoints] Encountered exception while serializing Skyblocker waypoint data"));
            }
        }
        // TODO(fabric2seer): `.encode(..)` on a Encoder
        //   java: return PREFIX + new String(Base64.getEncoder().encode(output.toByteArray()));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static @Nullable List<WaypointGroup> fromSkytils(String waypointsString, Location defaultIsland)
    pub fn from_skytils(mut waypoints_string: String, mut default_island: Location) -> Option<Vec<compat::Shared<WaypointGroup>>> {
        waypoints_string = compat::trim(&waypoints_string);
        // java: try {..} catch (NumberFormatException e) catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            if waypoints_string.starts_with("<Skytils-Waypoint-Data>(V") {
                let mut version: i32 = (compat::substring2(&waypoints_string, 25, compat::index_of(&waypoints_string, ')'))).trim().parse::<i32>().expect("NumberFormatException");
                waypoints_string = compat::substring(&waypoints_string, (compat::index_of(&waypoints_string, ':')).wrapping_add(1));
                if version == 1 {
                    {
                        // TODO(fabric2seer): `new GZIPInputStream(..)`
                        //   java: GZIPInputStream reader = new GZIPInputStream(new ByteArrayInputStream(Base64.getDecoder().decode(waypointsString)));
                        let mut reader: crate::unresolved::GZIPInputStream = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                        // TODO(fabric2seer): static call IOUtils.toString(..)
                        //   java: return fromSkytilsJson(IOUtils.toString(reader, StandardCharsets.UTF_8), defaultIsland);
                    }
                } else {
                    logger().error(&format!("[Skyblocker Waypoints] Unknown Skytils waypoint data version: {}", version));
                }
            } else {
                // TODO(fabric2seer): `.decode(..)` on a Decoder
                //   java: return fromSkytilsJson(new String(Base64.getDecoder().decode(waypointsString)), defaultIsland);
            }
        }
        // TODO(fabric2seer): the `catch (NumberFormatException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker Waypoints] Encountered exception while parsing Skytils waypoint data version", e);
        //   java: 			return null;
        //   java: 		}
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java: 			LOGGER.error("[Skyblocker Waypoints] Encountered exception while decoding Skytils waypoint data", e);
        //   java: 			return null;
        //   java: 		}
        return Some(Default::default());
    }

    /// java: public static List<WaypointGroup> fromSkytilsJson(String waypointGroupsString, Location defaultIsland)
    pub fn from_skytils_json(waypoint_groups_string: &str, mut default_island: Location) -> Vec<compat::Shared<WaypointGroup>> {
        let mut waypoint_groups_json: Vec<serde_json::Value>;
        // java: try {..} catch (JsonSyntaxException _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.fromJson(..)` on a Gson
            //   java: waypointGroupsJson = SkyblockerMod.GSON.fromJson(waypointGroupsString, JsonObject.class).getAsJsonArray("categories");
        }));
        if let Err(_) = __result {
            let mut waypoint_group_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
            waypoint_group_json.insert("name".to_string(), serde_json::json!("New Group".to_string()));
            waypoint_group_json.insert("island".to_string(), serde_json::json!(default_island.id()));
            // TODO(fabric2seer): `.fromJson(..)` on a Gson
            //   java: waypointGroupJson.add("waypoints", SkyblockerMod.GSON.fromJson(waypointGroupsString, JsonArray.class));
            waypoint_groups_json = Vec::new();
            waypoint_groups_json.push(compat::json::to_value(waypoint_group_json.clone()));
        }
        let mut waypoint_groups: Vec<compat::Shared<WaypointGroup>> = (SKYTILS_CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&waypoint_groups_json))).result_or_partial_with(|__a0| { logger().error(&__a0); }).unwrap();
        return Waypoints::apply_default_location(&waypoint_groups, default_island);
    }

    /// java: public static String toSkytilsBase64(List<WaypointGroup> waypointGroups)
    pub fn to_skytils_base64(waypoint_groups: &Vec<compat::Shared<WaypointGroup>>) -> String {
        // TODO(fabric2seer): `.encodeToString(..)` on a Encoder
        //   java: return Base64.getEncoder().encodeToString(toSkytilsJson(waypointGroups).getBytes());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static String toSkytilsJson(List<WaypointGroup> waypointGroups)
    pub fn to_skytils_json(waypoint_groups: &Vec<compat::Shared<WaypointGroup>>) -> String {
        let mut waypoint_groups_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        waypoint_groups_json.insert("categories".to_string(), compat::json::to_value((SKYTILS_CODEC.get_mut().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), waypoint_groups)).result_or_partial_with(|__a0| { logger().error(&__a0); }).unwrap()));
        // TODO(fabric2seer): `.toJson(..)` on a Gson
        //   java: return SkyblockerMod.GSON_COMPACT.toJson(waypointGroupsJson);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static WaypointGroup fromColeweightJson(String waypointsJson, Location defaultIsland)
    pub fn from_coleweight_json(waypoints_json: &str, mut default_island: Location) -> compat::Shared<WaypointGroup> {
        // TODO(fabric2seer): Optional.withIsland
        //   java: return WaypointGroup.COLEWEIGHT_CODEC.parse(JsonOps.INSTANCE, JsonParser.parseString(waypointsJson)).resultOrPartial(LOGGER::error).orElseThrow().withIsland(defaultIsland);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static List<WaypointGroup> applyDefaultLocation(Collection<WaypointGroup> waypointGroups, Location defaultIsland)
    pub fn apply_default_location(waypoint_groups: &Vec<compat::Shared<WaypointGroup>>, mut default_island: Location) -> Vec<compat::Shared<WaypointGroup>> {
        return waypoint_groups.iter().cloned().map(|waypoint_group| (if WaypointGroup::island(&waypoint_group) == Location::Unknown { WaypointGroup::with_island(&waypoint_group, default_island) } else { waypoint_group.clone() })).collect::<Vec<_>>();
    }

    /// java: public static Collection<WaypointGroup> getWaypointGroup(Location island)
    pub fn get_waypoint_group(mut island: Location) -> Vec<compat::Shared<WaypointGroup>> {
        // TODO(fabric2seer): `.get(..)` on a Multimap
        //   java: return waypoints.get(island);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean putWaypointGroup(WaypointGroup waypointGroup)
    pub fn put_waypoint_group(mut waypoint_group: compat::Shared<WaypointGroup>) -> bool {
        // TODO(fabric2seer): `.put(..)` on a Multimap
        //   java: return waypoints.put(waypointGroup.island(), waypointGroup);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean clearAndPutAllWaypoints(Multimap<Location, WaypointGroup> newWaypoints)
    pub fn clear_and_put_all_waypoints(new_waypoints: &crate::unresolved::Multimap) -> bool {
        // TODO(fabric2seer): `.clear(..)` on a Multimap
        //   java: waypoints.clear();
        // TODO(fabric2seer): `.putAll(..)` on a Multimap
        //   java: return waypoints.putAll(newWaypoints);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean areWaypointsEqual(Multimap<Location, WaypointGroup> otherWaypoints)
    pub fn are_waypoints_equal(other_waypoints: &crate::unresolved::Multimap) -> bool {
        return (WAYPOINTS.get_mut() == other_waypoints);
    }

    /// java: public static Multimap<Location, WaypointGroup> waypointsDeepCopy()
    pub fn waypoints_deep_copy() -> crate::unresolved::Multimap {
        // TODO(fabric2seer): `.values(..)` on a Multimap
        //   java: return waypoints.values().stream().map(WaypointGroup::deepCopy).collect(Multimaps.toMultimap(WaypointGroup::island, Function.identity(), () -> MultimapBuilder.enumKeys(Location.class).arrayListValues().build()));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static void forEachGroup(Consumer<WaypointGroup> consumer)
    pub fn for_each_group(consumer: Arc<dyn Fn(compat::Shared<WaypointGroup>)>) {
        for group in Waypoints::get_waypoint_group(Utils::get_location()) {
            if true /* java: group != null: never null here */ {
                (consumer)(group.clone());
            }
        }
        if Utils::get_location_raw().is_empty() {
            return;
        }
        for group in Waypoints::get_waypoint_group(Location::Unknown) {
            if true /* java: group != null: never null here */ {
                (consumer)(group.clone());
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if SkyblockerConfigManager::get().ui_and_visuals.waypoints.enable_waypoints {
            Waypoints::for_each_group(Arc::new(move |group| { WaypointGroup::extract_rendering(&group, collector); }));
        }
    }

    /// java: private static void tick()
    pub fn tick() {
        if !(SkyblockerConfigManager::get().ui_and_visuals.waypoints.enable_waypoints) {
            return;
        }
        Waypoints::for_each_group(Arc::new(move |__a0| { WaypointGroup::tick(&__a0); }));
    }

    /// java: private static void reset()
    pub fn reset() {
        // TODO(fabric2seer): `.values(..)` on a Multimap
        //   java: waypoints.values().forEach(WaypointGroup::resetCurrentIndex);
    }

}

/// java: private enum OrderedAction implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OrderedAction {
    Next,
    Previous,
    First,
    Reset,
}

impl OrderedAction {
    pub const VALUES: [OrderedAction; 4] = [OrderedAction::Next, OrderedAction::Previous, OrderedAction::First, OrderedAction::Reset];

    /// java: values()
    pub fn values() -> Vec<OrderedAction> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            OrderedAction::Next => "NEXT",
            OrderedAction::Previous => "PREVIOUS",
            OrderedAction::First => "FIRST",
            OrderedAction::Reset => "RESET",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> OrderedAction {
        match name {
            "NEXT" => OrderedAction::Next,
            "PREVIOUS" => OrderedAction::Previous,
            "FIRST" => OrderedAction::First,
            "RESET" => OrderedAction::Reset,
            _ => panic!("IllegalArgumentException: No enum constant OrderedAction.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name().toLowerCase(Locale.ENGLISH);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for OrderedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: static class ArgumentType extends StringRepresentableArgument<OrderedAction>
pub struct ArgumentType {
    /// java: extends net.minecraft.commands.arguments.StringRepresentableArgument
    pub base: StringRepresentableArgument<compat::Object>,
}

impl std::ops::Deref for ArgumentType {
    type Target = StringRepresentableArgument<compat::Object>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ArgumentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ArgumentType {
    /// java: protected ArgumentType()
    pub fn new() -> Self {
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`values: Vec<T>, codec_name: fn(&T) -> String, convert_id: fn(&str) -> String`), Java passes 2
        //   java: super(CODEC, OrderedAction::values);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: static ArgumentType orderedAction()
    pub fn ordered_action() -> ArgumentType {
        return ArgumentType::new();
    }

    /// java: static <S> OrderedAction getOrderedAction(CommandContext<S> context, String name)
    pub fn get_ordered_action<S: 'static>(context: &CommandContext<S>, name: &str) -> OrderedAction {
        return context.get_argument::<OrderedAction>(name).unwrap();
    }

}
