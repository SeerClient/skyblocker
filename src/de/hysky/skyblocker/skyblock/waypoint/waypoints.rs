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
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
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
//   java: private static final Codec<Collection<WaypointGroup>> SKYBLOCKER_LEGACY_ORDERED_CODEC = Codec.unboundedMap(Codec.STRING, WaypointGroup.SKYBLOCKER_LEGACY_ORDERED_CODEC).xmap(Map::values, groups -> groups.stream().collect(Collectors.toMap(WaypointGroup::name, Function.identity())));
/// java: private static final Codec<Collection<WaypointGroup>> SKYBLOCKER_LEGACY_ORDERED_CODEC = Codec.unboundedMap(Codec.STRING, WaypointGroup.SKYBLOCKER_LEGACY_ORDERED_CODEC).xmap(Map::values, groups -> grou
/// `Map::values` decodes (the map's order); `Collectors.toMap(WaypointGroup::name, identity())` encodes, throwing on
/// a repeated name. The map's order on encode is the list's (Java's `HashMap` order is unspecified).
pub static SKYBLOCKER_LEGACY_ORDERED_CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<WaypointGroup>>>> = compat::MainThread::new(|| codec::xmap(
    codec::unbounded_map(codec::string_codec(), waypoint_group::SKYBLOCKER_LEGACY_ORDERED_CODEC.get().clone()),
    |map: Vec<(String, compat::Shared<WaypointGroup>)>| map.into_iter().map(|(_, group)| group).collect::<Vec<_>>(),
    |groups: &Vec<compat::Shared<WaypointGroup>>| {
        let mut map: Vec<(String, compat::Shared<WaypointGroup>)> = Vec::with_capacity(groups.len());
        for group in groups.iter() {
            let name = WaypointGroup::name(group);
            if map.iter().any(|(key, _)| *key == name) {
                panic!("IllegalStateException: Duplicate key {}", name);
            }
            map.push((name, group.clone()));
        }
        map
    },
));
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
/// java: private static final Multimap<Location, WaypointGroup> waypoints = MultimapBuilder.enumKeys(Location.class).arrayListValues().build();
pub static WAYPOINTS: compat::MainThread<compat::Shared<LocationMultimap>> = compat::MainThread::new(|| compat::shared(LocationMultimap::new()));
/// java: private static final Codec<OrderedAction> CODEC = StringRepresentable.fromEnum(OrderedAction::values);
pub static ORDERED_ACTION_CODEC: compat::MainThread<CodecRef<OrderedAction>> = compat::MainThread::new(|| mcrust::net::minecraft::util::string_representable::from_enum(OrderedAction::values(), |value: &OrderedAction| value.get_serialized_name()));

/// `Multimap<Location, WaypointGroup>` as `MultimapBuilder.enumKeys(Location.class).arrayListValues().build()`
/// makes it: the keys in `Location`'s declaration order (an `EnumMap`; `Location`'s derived `Ord` is that order),
/// each key's values an `ArrayList` in insertion order. A key whose list is empty is not in the map (Guava drops
/// it), which `equals` and `values` both see. Java holds the map by reference (`WaypointsScreen` and the
/// `WaypointsShareScreen` it opens edit the same one), so every holder keeps a `compat::Shared` of it.
/// rust: belongs with `compat` (Guava's `Multimap` is unported - `crate::unresolved::Multimap`); kept here, the one
/// package that builds this map.
#[derive(Default)]
pub struct LocationMultimap {
    map: std::collections::BTreeMap<Location, Vec<compat::Shared<WaypointGroup>>>,
}

impl LocationMultimap {
    /// `MultimapBuilder.enumKeys(Location.class).arrayListValues().build()`
    pub fn new() -> LocationMultimap {
        LocationMultimap { map: std::collections::BTreeMap::new() }
    }

    /// `get(key)`, read: the groups of the key, in order (empty when there are none).
    pub fn get(&self, key: Location) -> Vec<compat::Shared<WaypointGroup>> {
        self.map.get(&key).cloned().unwrap_or_default()
    }

    /// `get(key)` as the live collection Guava hands out: a change to it is a change to the map.
    pub fn get_mut(&mut self, key: Location) -> &mut Vec<compat::Shared<WaypointGroup>> {
        self.map.entry(key).or_default()
    }

    /// `put(key, value)`: an `ArrayList` value collection always grows, so it answers `true`.
    pub fn put(&mut self, key: Location, value: compat::Shared<WaypointGroup>) -> bool {
        self.map.entry(key).or_default().push(value);
        true
    }

    /// `remove(key, value)`: the first group of the key that `equals` the value (`ArrayList.remove(Object)`).
    pub fn remove(&mut self, key: Location, value: &compat::Shared<WaypointGroup>) -> bool {
        let Some(list) = self.map.get_mut(&key) else { return false };
        let Some(index) = list.iter().position(|group| WaypointGroup::equals_shared(group, value)) else { return false };
        list.remove(index);
        true
    }

    /// `putAll(multimap)`: every entry of the other map, in its order; `true` when anything was added.
    pub fn put_all(&mut self, other: &LocationMultimap) -> bool {
        let mut changed = false;
        for (key, values) in other.map.iter() {
            for value in values.iter() {
                changed |= self.put(*key, value.clone());
            }
        }
        changed
    }

    /// `clear()`
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// `values()`: every group, key by key.
    pub fn values(&self) -> Vec<compat::Shared<WaypointGroup>> {
        self.map.values().flat_map(|values| values.iter().cloned()).collect()
    }

    /// The keys that hold at least one group, in key order (`asMap().keySet()`).
    pub fn keys(&self) -> Vec<Location> {
        self.map.iter().filter(|(_, values)| !values.is_empty()).map(|(key, _)| *key).collect()
    }

    /// `equals(Object)`: `asMap().equals(other.asMap())` - the same keys, and for each key lists that are equal
    /// element by element (`WaypointGroup.equals`).
    pub fn equals(&self, other: &LocationMultimap) -> bool {
        let keys = self.keys();
        if keys != other.keys() {
            return false;
        }
        keys.iter().all(|key| {
            let (a, b) = (&self.map[key], &other.map[key]);
            a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| WaypointGroup::equals_shared(a, b))
        })
    }
}

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
        // `waypoints.get(Utils.getLocation()).stream().filter(group -> group.ordered() && !group.waypoints().isEmpty()
        // && group.waypoints().stream().allMatch(Waypoint::isEnabled)).findFirst()`
        let mut group_optional: Option<compat::Shared<WaypointGroup>> = compat::read(WAYPOINTS.get(), |__t| __t.get(Utils::get_location())).into_iter().find(|group| {
            WaypointGroup::ordered(group) && !WaypointGroup::waypoints(group).is_empty() && WaypointGroup::waypoints(group).iter().all(|waypoint| {
                let waypoint = compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>);
                ({ let __f = compat::dispatch(&waypoint, |__t| __t.is_enabled()); __f(&waypoint) })
            })
        });
        if group_optional.is_none() {
            // TODO(batchD): cross-package - `FabricClientCommandSource` is `crate::unresolved` (no `sendFeedback`); the
            // command that calls this is not registered either (see `init`).
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
        }
        return 1;
    }

    /// java: public static void loadWaypoints()
    pub fn load_waypoints() {
        compat::write(WAYPOINTS.get(), |__t| __t.clear());
        // try (BufferedReader reader = Files.newBufferedReader(WAYPOINTS_FILE)) {..} catch (NoSuchFileException _) {} catch (Exception e) {..}
        match std::fs::read_to_string(WAYPOINTS_FILE.get()) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => logger().error(&format!("[Skyblocker Waypoints] Encountered exception while loading waypoints {}", e)),
            Ok(text) => {
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // `GSON.fromJson(reader, JsonArray.class)`: anything but an array is a `JsonSyntaxException`.
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                    if !json.is_array() {
                        panic!("JsonSyntaxException: Expected a com.google.gson.JsonArray but was {}", json);
                    }
                    let mut waypoint_groups: Vec<compat::Shared<WaypointGroup>> = CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json)).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present");
                    for __a0 in waypoint_groups.iter() {
                        Waypoints::put_waypoint_group(__a0.clone());
                    }
                }));
                if let Err(e) = __result {
                    logger().error(&format!("[Skyblocker Waypoints] Encountered exception while loading waypoints {}", compat::panic_message(&e)));
                }
            }
        }
        // try (BufferedReader reader = Files.newBufferedReader(SKYBLOCKER_LEGACY_ORDERED_FILE)) {..}
        // catch (NoSuchFileException | FileAlreadyExistsException _) {} catch (IOException e) {..}
        match std::fs::read_to_string(SKYBLOCKER_LEGACY_ORDERED_FILE.get()) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => logger().error(&format!("[Skyblocker Waypoints] Encountered exception while loading legacy ordered waypoints {}", e)),
            Ok(text) => {
                // `GSON.fromJson(reader, JsonObject.class)` and the codec throw unchecked exceptions, which Java does not catch here.
                let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                if !json.is_object() {
                    panic!("JsonSyntaxException: Expected a com.google.gson.JsonObject but was {}", json);
                }
                let mut waypoint_groups: Vec<compat::Shared<WaypointGroup>> = SKYBLOCKER_LEGACY_ORDERED_CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json)).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present");
                for group in waypoint_groups.iter() {
                    Waypoints::put_waypoint_group(WaypointGroup::deep_copy(&WaypointGroup::with_island(group, Location::DwarvenMines)));
                    Waypoints::put_waypoint_group(WaypointGroup::deep_copy(&WaypointGroup::with_island(group, Location::CrystalHollows)));
                }
                // `Files.move(source, target)` without REPLACE_EXISTING: a target that exists is `FileAlreadyExistsException`, caught silently.
                let target = skyblocker_mod::CONFIG_DIR.get_mut().join("legacy_ordered_waypoints.json");
                if target.exists() {
                    return;
                }
                if let Err(e) = std::fs::rename(SKYBLOCKER_LEGACY_ORDERED_FILE.get(), &target) {
                    logger().error(&format!("[Skyblocker Waypoints] Encountered exception while loading legacy ordered waypoints {}", e));
                    return;
                }
                logger().info(&format!("[Skyblocker Waypoints] Successfully migrated {} ordered waypoints from {} groups to waypoints!", waypoint_groups.iter().map(|group| WaypointGroup::waypoints(group).len() as i32).sum::<i32>(), (waypoint_groups.len() as i32)));
            }
        }
    }

    /// java: public static void saveWaypoints(Minecraft client)
    pub fn save_waypoints(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        // try (BufferedWriter writer = Files.newBufferedWriter(WAYPOINTS_FILE)) {..} catch (Exception e) {..}
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let values = compat::read(WAYPOINTS.get(), |__t| __t.values());
            let mut waypoints_json: serde_json::Value = compat::json_from_port(&CODEC.get().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &values).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present"));
            // `GSON.toJson(waypointsJson, writer)` with `setPrettyPrinting()`: two-space indentation, as serde's pretty printer.
            std::fs::write(WAYPOINTS_FILE.get(), serde_json::to_string_pretty(&waypoints_json).unwrap()).unwrap_or_else(|e| panic!("IOException: {e}"));
            logger().info("[Skyblocker Waypoints] Saved waypoints");
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Waypoints] Encountered exception while saving waypoints {}", compat::panic_message(&e)));
        }
    }

    /// java: public static @Nullable List<WaypointGroup> fromSkyblocker(String waypointsString, Location defaultIsland)
    pub fn from_skyblocker(mut waypoints_string: String, mut default_island: Location) -> Option<Vec<compat::Shared<WaypointGroup>>> {
        waypoints_string = compat::trim(&waypoints_string);
        if waypoints_string.starts_with(PREFIX) {
            // `Base64.getDecoder().decode(..)` throws `IllegalArgumentException` (not caught: it is no `IOException`).
            let compressed = base64::get_decoder().decode(&waypoints_string.replace(PREFIX, "")).unwrap_or_else(|e| panic!("IllegalArgumentException: {e}"));
            // try (GZIPInputStream reader = ..) {..} catch (IOException e) {..}
            let bytes = match mcrust::java::util::zip::gzip::gzip_decompress(&compressed) {
                Ok(bytes) => bytes,
                Err(e) => {
                    logger().error(&format!("[Skyblocker Waypoints] Encountered exception while parsing Skyblocker waypoint data {}", e));
                    return None;
                }
            };
            let json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&bytes)).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
            if !json.is_array() {
                panic!("JsonSyntaxException: Expected a com.google.gson.JsonArray but was {}", json);
            }
            return Some(CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json)).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present"));
        } else if waypoints_string.starts_with(SKYBLOCKER_LEGACY_ORDERED) {
            let compressed = base64::get_decoder().decode(&waypoints_string.replace(SKYBLOCKER_LEGACY_ORDERED, "")).unwrap_or_else(|e| panic!("IllegalArgumentException: {e}"));
            let bytes = match mcrust::java::util::zip::gzip::gzip_decompress(&compressed) {
                Ok(bytes) => bytes,
                Err(e) => {
                    logger().error(&format!("[Skyblocker Waypoints] Encountered exception while parsing Skyblocker legacy ordered waypoint data {}", e));
                    return None;
                }
            };
            let json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&bytes)).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
            if !json.is_object() {
                panic!("JsonSyntaxException: Expected a com.google.gson.JsonObject but was {}", json);
            }
            return Some(Waypoints::apply_default_location(&SKYBLOCKER_LEGACY_ORDERED_CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json)).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present"), default_island));
        }
        logger().error("[Skyblocker Waypoints] Unknown skyblocker waypoint data prefix");
        return None;
    }

    /// java: public static String toSkyblocker(List<WaypointGroup> waypointGroups)
    pub fn to_skyblocker(waypoint_groups: &Vec<compat::Shared<WaypointGroup>>) -> String {
        let mut waypoints_json: String = serde_json::to_string_pretty(&compat::json_from_port(&CODEC.get().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), waypoint_groups).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present"))).unwrap();
        // `new GZIPOutputStream(output)` written and closed: writing to memory raises no `IOException`.
        let output: Vec<u8> = mcrust::java::util::zip::gzip::gzip_compress(waypoints_json.as_bytes());
        return format!("{}{}", PREFIX, base64::get_encoder().encode_to_string(&output));
    }

    /// java: public static @Nullable List<WaypointGroup> fromSkytils(String waypointsString, Location defaultIsland)
    pub fn from_skytils(mut waypoints_string: String, mut default_island: Location) -> Option<Vec<compat::Shared<WaypointGroup>>> {
        waypoints_string = compat::trim(&waypoints_string);
        // try {..} catch (NumberFormatException e) {..} catch (Exception e) {..}: the body's `return` is the closure's
        // `Some`; falling out of it reaches `return Collections.emptyList()`.
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<Vec<compat::Shared<WaypointGroup>>> {
            if waypoints_string.starts_with("<Skytils-Waypoint-Data>(V") {
                // `Integer.parseInt` does not trim.
                let version_string = compat::substring2(&waypoints_string, 25, compat::index_of(&waypoints_string, ')'));
                let mut version: i32 = version_string.parse::<i32>().unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{}\"", version_string));
                let rest = compat::substring(&waypoints_string, (compat::index_of(&waypoints_string, ':')).wrapping_add(1));
                if version == 1 {
                    let compressed = base64::get_decoder().decode(&rest).unwrap_or_else(|e| panic!("IllegalArgumentException: {e}"));
                    let bytes = mcrust::java::util::zip::gzip::gzip_decompress(&compressed).unwrap_or_else(|e| panic!("IOException: {e}"));
                    // `IOUtils.toString(reader, StandardCharsets.UTF_8)`
                    return Some(Waypoints::from_skytils_json(&String::from_utf8_lossy(&bytes), default_island));
                } else {
                    logger().error(&format!("[Skyblocker Waypoints] Unknown Skytils waypoint data version: {}", version));
                }
            } else {
                let bytes = base64::get_decoder().decode(&waypoints_string).unwrap_or_else(|e| panic!("IllegalArgumentException: {e}"));
                return Some(Waypoints::from_skytils_json(&String::from_utf8_lossy(&bytes), default_island));
            }
            None
        }));
        match __result {
            Ok(Some(groups)) => return Some(groups),
            Ok(None) => {}
            Err(e) => {
                let message = compat::panic_message(&e);
                if message.starts_with("NumberFormatException") {
                    logger().error(&format!("[Skyblocker Waypoints] Encountered exception while parsing Skytils waypoint data version {}", message));
                } else {
                    logger().error(&format!("[Skyblocker Waypoints] Encountered exception while decoding Skytils waypoint data {}", message));
                }
                return None;
            }
        }
        return Some(Vec::new());
    }

    /// java: public static List<WaypointGroup> fromSkytilsJson(String waypointGroupsString, Location defaultIsland)
    pub fn from_skytils_json(waypoint_groups_string: &str, mut default_island: Location) -> Vec<compat::Shared<WaypointGroup>> {
        // try { waypointGroupsJson = GSON.fromJson(waypointGroupsString, JsonObject.class).getAsJsonArray("categories"); }
        // catch (JsonSyntaxException _) {..}: text that is not a JSON object is the syntax exception.
        let mut waypoint_groups_json: serde_json::Value = match serde_json::from_str::<serde_json::Value>(waypoint_groups_string) {
            Ok(serde_json::Value::Object(object)) => match object.get("categories") {
                Some(categories @ serde_json::Value::Array(_)) => categories.clone(),
                // `getAsJsonArray` casts: another element type is a `ClassCastException`.
                Some(other) => panic!("ClassCastException: {} is not a com.google.gson.JsonArray", other),
                // an absent member is null, which `SKYTILS_CODEC.parse(..)` reads as `JsonNull`.
                None => serde_json::Value::Null,
            },
            _ => {
                // Handle the case where there is only a single json list of waypoints and no group data.
                let mut waypoint_group_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                waypoint_group_json.insert("name".to_string(), serde_json::json!("New Group".to_string()));
                waypoint_group_json.insert("island".to_string(), serde_json::json!(default_island.id()));
                // `GSON.fromJson(waypointGroupsString, JsonArray.class)`: its own syntax exception is not caught.
                let waypoints: serde_json::Value = serde_json::from_str(waypoint_groups_string).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                if !waypoints.is_array() {
                    panic!("JsonSyntaxException: Expected a com.google.gson.JsonArray but was {}", waypoints);
                }
                waypoint_group_json.insert("waypoints".to_string(), waypoints);
                serde_json::Value::Array(vec![serde_json::Value::Object(waypoint_group_json)])
            }
        };
        let mut waypoint_groups: Vec<compat::Shared<WaypointGroup>> = (SKYTILS_CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&waypoint_groups_json))).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present");
        return Waypoints::apply_default_location(&waypoint_groups, default_island);
    }

    /// java: public static String toSkytilsBase64(List<WaypointGroup> waypointGroups)
    pub fn to_skytils_base64(waypoint_groups: &Vec<compat::Shared<WaypointGroup>>) -> String {
        return base64::get_encoder().encode_to_string(Waypoints::to_skytils_json(waypoint_groups).as_bytes());
    }

    /// java: public static String toSkytilsJson(List<WaypointGroup> waypointGroups)
    pub fn to_skytils_json(waypoint_groups: &Vec<compat::Shared<WaypointGroup>>) -> String {
        let mut waypoint_groups_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        waypoint_groups_json.insert("categories".to_string(), compat::json_from_port(&(SKYTILS_CODEC.get().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), waypoint_groups)).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present")));
        // `GSON_COMPACT.toJson(..)`: no whitespace, as serde's compact writer.
        return serde_json::to_string(&waypoint_groups_json).unwrap();
    }

    /// java: public static WaypointGroup fromColeweightJson(String waypointsJson, Location defaultIsland)
    pub fn from_coleweight_json(waypoints_json: &str, mut default_island: Location) -> compat::Shared<WaypointGroup> {
        // `JsonParser.parseString(waypointsJson)`
        let json: serde_json::Value = serde_json::from_str(waypoints_json).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
        let group = waypoint_group::COLEWEIGHT_CODEC.get().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&json)).result_or_partial_with(|__a0| { logger().error(&__a0); }).expect("NoSuchElementException: No value present");
        return WaypointGroup::with_island(&group, default_island);
    }

    /// java: public static List<WaypointGroup> applyDefaultLocation(Collection<WaypointGroup> waypointGroups, Location defaultIsland)
    pub fn apply_default_location(waypoint_groups: &Vec<compat::Shared<WaypointGroup>>, mut default_island: Location) -> Vec<compat::Shared<WaypointGroup>> {
        return waypoint_groups.iter().cloned().map(|waypoint_group| (if WaypointGroup::island(&waypoint_group) == Location::Unknown { WaypointGroup::with_island(&waypoint_group, default_island) } else { waypoint_group.clone() })).collect::<Vec<_>>();
    }

    /// java: public static Collection<WaypointGroup> getWaypointGroup(Location island)
    pub fn get_waypoint_group(mut island: Location) -> Vec<compat::Shared<WaypointGroup>> {
        return compat::read(WAYPOINTS.get(), |__t| __t.get(island));
    }

    /// java: public static boolean putWaypointGroup(WaypointGroup waypointGroup)
    pub fn put_waypoint_group(mut waypoint_group: compat::Shared<WaypointGroup>) -> bool {
        let island = WaypointGroup::island(&waypoint_group);
        return compat::write(WAYPOINTS.get(), |__t| __t.put(island, waypoint_group));
    }

    /// java: public static boolean clearAndPutAllWaypoints(Multimap<Location, WaypointGroup> newWaypoints)
    pub fn clear_and_put_all_waypoints(new_waypoints: &compat::Shared<LocationMultimap>) -> bool {
        compat::write(WAYPOINTS.get(), |__t| __t.clear());
        // `putAll` reads the other map after the clear (the same object would be empty by then).
        let other = compat::read(new_waypoints, |__t| { let mut copy = LocationMultimap::new(); copy.put_all(__t); copy });
        return compat::write(WAYPOINTS.get(), |__t| __t.put_all(&other));
    }

    /// java: public static boolean areWaypointsEqual(Multimap<Location, WaypointGroup> otherWaypoints)
    pub fn are_waypoints_equal(other_waypoints: &compat::Shared<LocationMultimap>) -> bool {
        if Arc::ptr_eq(WAYPOINTS.get(), other_waypoints) {
            return true;
        }
        return compat::read(WAYPOINTS.get(), |a| compat::read(other_waypoints, |b| a.equals(b)));
    }

    /// java: public static Multimap<Location, WaypointGroup> waypointsDeepCopy()
    pub fn waypoints_deep_copy() -> compat::Shared<LocationMultimap> {
        // `waypoints.values().stream().map(WaypointGroup::deepCopy).collect(Multimaps.toMultimap(WaypointGroup::island, identity(), ..))`
        let mut copy = LocationMultimap::new();
        for group in compat::read(WAYPOINTS.get(), |__t| __t.values()) {
            let group = WaypointGroup::deep_copy(&group);
            copy.put(WaypointGroup::island(&group), group);
        }
        return compat::shared(copy);
    }

    /// java: private static void forEachGroup(Consumer<WaypointGroup> consumer)
    pub fn for_each_group(consumer: &dyn Fn(compat::Shared<WaypointGroup>)) {
        // java: `group != null` - an `ArrayListMultimap` value can be null in Java; the port's list holds no null.
        for group in Waypoints::get_waypoint_group(Utils::get_location()) {
            (consumer)(group.clone());
        }
        if Utils::get_location_raw().is_empty() {
            return;
        }
        for group in Waypoints::get_waypoint_group(Location::Unknown) {
            (consumer)(group.clone());
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if SkyblockerConfigManager::get().ui_and_visuals.waypoints.enable_waypoints {
            Waypoints::for_each_group(&|group| { WaypointGroup::extract_rendering(&group, collector); });
        }
    }

    /// java: private static void tick()
    pub fn tick() {
        if !(SkyblockerConfigManager::get().ui_and_visuals.waypoints.enable_waypoints) {
            return;
        }
        Waypoints::for_each_group(&|__a0| { WaypointGroup::tick(&__a0); });
    }

    /// java: private static void reset()
    pub fn reset() {
        for group in compat::read(WAYPOINTS.get(), |__t| __t.values()) {
            WaypointGroup::reset_current_index(&group);
        }
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
        return self.name().to_lowercase();
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
    pub base: StringRepresentableArgument<OrderedAction>,
}

impl std::ops::Deref for ArgumentType {
    type Target = StringRepresentableArgument<OrderedAction>;
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
        // `super(CODEC, OrderedAction::values)`: the port's argument takes the values and the codec's naming
        // (`fromEnum`'s `getSerializedName`, no id conversion) in place of the codec itself.
        let mut this = Self { base: StringRepresentableArgument::new(OrderedAction::values(), |value: &OrderedAction| value.get_serialized_name(), |id: &str| id.to_string()) };
        this
    }

    /// java: static ArgumentType orderedAction()
    pub fn ordered_action() -> ArgumentType {
        return ArgumentType::new();
    }

    /// java: static <S> OrderedAction getOrderedAction(CommandContext<S> context, String name)
    pub fn get_ordered_action<S: 'static>(context: &CommandContext<S>, name: &str) -> OrderedAction {
        return *context.get_argument::<OrderedAction>(name).unwrap();
    }

}
