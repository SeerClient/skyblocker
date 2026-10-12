//! Ported from de.hysky.skyblocker.skyblock.rift.EnigmaSouls by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::other_locations_config::Rift;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::pos_utils::PosUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::profile_aware_waypoint::ProfileAwareWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::profile_aware_waypoint::ProfileAwareWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::command_dispatcher::CommandDispatcher;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;

/// java: private static final Supplier<Waypoint.Type> TYPE_SUPPLIER = () -> SkyblockerConfigManager.get().uiAndVisuals.waypoints.waypointType;
pub static TYPE_SUPPLIER: compat::MainThread<Arc<dyn Fn() -> Type>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().ui_and_visuals.waypoints.waypoint_type));
/// java: private static final Identifier WAYPOINTS_JSON = SkyblockerMod.id("rift/enigma_soul_waypoints.json");
pub static WAYPOINTS_JSON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("rift/enigma_soul_waypoints.json"));
/// java: private static final Map<BlockPos, ProfileAwareWaypoint> SOUL_WAYPOINTS = new HashMap<>(52);
pub static SOUL_WAYPOINTS: compat::MainThread<std::collections::HashMap<BlockPos, compat::Shared<dyn ProfileAwareWaypoint>>> = compat::MainThread::new(|| std::collections::HashMap::with_capacity((52) as usize));
/// java: private static final Path FOUND_SOULS_FILE = SkyblockerMod.CONFIG_DIR.resolve("found_enigma_souls.json");
pub static FOUND_SOULS_FILE: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| skyblocker_mod::CONFIG_DIR.get_mut().join("found_enigma_souls.json"));
/// java: private static final float[] GREEN = ColorUtils.getFloatComponents(DyeColor.GREEN);
pub static GREEN: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Green));
/// java: private static final float[] RED = ColorUtils.getFloatComponents(DyeColor.RED);
pub static RED: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));
/// java: private static CompletableFuture<Void> soulsLoaded;
pub static SOULS_LOADED: compat::MainThread<Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>> = compat::MainThread::new(|| None);

/// java: public class EnigmaSouls
#[derive(Clone)]
pub struct EnigmaSouls {}

impl EnigmaSouls {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: static void load(Minecraft client)
    pub fn load(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        *SOULS_LOADED.get_mut() = Some(CompletableFuture::<()>::run_async(move || (|| {
            {
                let mut reader: crate::unresolved::BufferedReader = client.get_resource_manager().open_as_reader(WAYPOINTS_JSON.get_mut()).unwrap();
                // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): static call JsonParser.parseReader(..)
                    //   java: JsonObject file = JsonParser.parseReader(reader).getAsJsonObject();
                    let mut file: serde_json::Map<String, serde_json::Value> = todo!("fabric2seer: the initialiser of `file` (TODO above)");
                    let mut waypoints: Vec<serde_json::Value> = compat::json::get_as_json_array(&file.get("waypoints").cloned().unwrap());
                    for i in 0..(waypoints.len() as i32) {
                        let mut waypoint: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(waypoints[(i) as usize]));
                        let mut pos: BlockPos = BlockPos::new(compat::json::get_as_int(&waypoint.get("x").cloned().unwrap()), compat::json::get_as_int(&waypoint.get("y").cloned().unwrap()), compat::json::get_as_int(&waypoint.get("z").cloned().unwrap()));
                        SOUL_WAYPOINTS.get_mut().insert(pos.clone(), (EnigmaSoul::new(pos.clone(), TYPE_SUPPLIER.get_mut().clone(), GREEN.clone(), RED.clone()) as compat::Shared<dyn ProfileAwareWaypoint>));
                    }
                }));
                if let Err(e) = __result {
                    logger().error(&format!("[Skyblocker] There was an error while loading enigma soul waypoints!"));
                }
            }
            {
                // TODO(fabric2seer): Files.newBufferedReader
                //   java: BufferedReader reader = Files.newBufferedReader(..);
                let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                // java: try {..} catch (NoSuchFileException _) catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): static call JsonParser.parseReader(..)
                    //   java: for (Map.Entry<String, JsonElement> profile : JsonParser.parseReader(reader).getAsJsonObject().asMap().entrySet()) {
                    //   java: 					for (JsonElement foundSoul : profile.getValue().getAsJsonArray().asList()) {
                    //   java: 						SOUL_WAYPOINTS.get(PosUtils.parsePosString(foundSoul.getAsString())).setFound(profile.getKey());
                    //   java: 					}
                    //   java: 				}
                }));
                if let Err(_) = __result {
                }
                // TODO(fabric2seer): the second `catch (IOException e)` clause (merged into the first)
                //   java: {
                //   java: 				LOGGER.error("[Skyblocker] There was an error while loading found enigma souls!", e);
                //   java: 			}
            }
        })(), compat::executor));
    }

    /// java: static void save(Minecraft client)
    pub fn save(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let mut found_souls: std::collections::HashMap<String, compat::Shared<BlockPosSet>> = std::collections::HashMap::new();
        for soul in SOUL_WAYPOINTS.get_mut().values().cloned() {
            for profile in compat::read(&soul, |__t| __t.profile_aware_waypoint_base().found_profiles.clone()) {
                found_souls.entry(profile.clone()).or_insert_with_key(|k| (|_| BlockPosSet::new())(k.clone()));
                BlockPosSet::add(&found_souls.get(&profile).cloned().unwrap(), &compat::read(&soul, |__t| __t.profile_aware_waypoint_base().pos.clone()).unwrap());
            }
        }
        let mut json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        for found_souls_for_profile in found_souls.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut found_souls_json: Vec<serde_json::Value> = Vec::new();
            for found_soul in found_souls_for_profile.1.lock().borrow().iter() {
                found_souls_json.push(compat::json::to_value(PosUtils::get_pos_string(found_soul)));
            }
            json.insert(found_souls_for_profile.0.clone(), compat::json::to_value(found_souls_json.clone()));
        }
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(json, writer);
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] There was an error while saving found enigma souls!"));
            }
        }
    }

    /// java: static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let config = &SkyblockerConfigManager::get().other_locations.rift;
        if Utils::is_in_the_rift() && config.enigma_soul_waypoints && SOULS_LOADED.get_mut().as_ref().unwrap().is_done() {
            for soul in SOUL_WAYPOINTS.get_mut().values().cloned() {
                if { let __f = compat::dispatch(&soul, |__t| __t.should_render()); __f(&soul) } || config.highlight_found_enigma_souls {
                    ({ let __f = compat::dispatch(&soul, |__t| __t.extract_rendering()); __f(&compat::view(&soul, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
            }
        }
    }

    /// java: static boolean onMessage(Component text, boolean overlay)
    pub fn on_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_in_the_rift() && !overlay {
            let mut message: String = text.get_string();
            if message == "You have already found that Enigma Soul!" || chat_formatting::strip_formatting(&message) == "SOUL! You unlocked an Enigma Soul!" {
                EnigmaSouls::mark_closest_soul(true);
            }
        }
        return true;
    }

    /// java: static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   java: 				.then(literal("rift")
        //   java: 						.then(literal("enigmaSouls")
        //   java: 								.then(literal("markAllFound").executes(context -> {
        //   java: 									SOUL_WAYPOINTS.values().forEach(Waypoint::setFound);
        //   java: 									context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.rift.enigmaSouls.markAllFound")));
        //   java:
        //   java: 									return Command.SINGLE_SUCCESS;
        //   java: 								}))
        //   java: 								.then(literal("markAllMissing").executes(context -> {
        //   java: 									SOUL_WAYPOINTS.values().forEach(Waypoint::setMissing);
        //   java: 									context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.rift.enigmaSouls.markAllMissing")));
        //   java:
        //   java: 									return Command.SINGLE_SUCCESS;
        //   java: 								}))
        //   java: 								.then(literal("markClosestFound").executes(context -> {
        //   java: 									markClosestSoul(true);
        //   java: 									context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.rift.enigmaSouls.markClosestFound")));
        //   java:
        //   java: 									return Command.SINGLE_SUCCESS;
        //   java: 								}))
        //   java: 								.then(literal("markClosestMissing").executes(context -> {
        //   java: 									markClosestSoul(false);
        //   java: 									context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.rift.enigmaSouls.markClosestMissing")));
        //   java:
        //   java: 									return Command.SINGLE_SUCCESS;
        //   java: 								})))));
    }

    /// java: private static void markClosestSoul(boolean asFound)
    pub fn mark_closest_soul(mut as_found: bool) {
        if !(SOULS_LOADED.get_mut().as_ref().unwrap().is_done()) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        // TODO(fabric2seer): Optional.ifPresent
        //   java: SOUL_WAYPOINTS.values().stream()
        //   java: 				.filter(Waypoint::shouldRender)
        //   java: 				.min(Comparator.comparingDouble(soul -> soul.pos.distToCenterSqr(player.position())))
        //   java: 				.filter(soul -> soul.pos.distToCenterSqr(player.position()) <= 16)
        //   java: 				.ifPresent(asFound ? Waypoint::setFound : Waypoint::setMissing);
    }

}

/// java: private static class EnigmaSoul extends ProfileAwareWaypoint
#[derive(Clone)]
pub struct EnigmaSoul {
    /// java: extends ProfileAwareWaypoint
    pub base: ProfileAwareWaypointBase,
}

impl std::ops::Deref for EnigmaSoul {
    type Target = ProfileAwareWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EnigmaSoul {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EnigmaSoul {
    /// java: private EnigmaSoul(BlockPos pos, Supplier<Type> typeSupplier, float[] missingColor, float[] foundColor)
    pub fn new(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut missing_color: Vec<f32>, mut found_color: Vec<f32>) -> compat::Shared<Self> {
        let (_, __this) = ProfileAwareWaypointBase::new_in(pos.clone(), type_supplier.clone(), missing_color.clone(), found_color.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ProfileAwareWaypoint>), __h)
        });
        __this
    }

}

impl ProfileAwareWaypoint for EnigmaSoul {
    fn profile_aware_waypoint_base(&self) -> &ProfileAwareWaypointBase {
        &self.base
    }
    fn profile_aware_waypoint_base_mut(&mut self) -> &mut ProfileAwareWaypointBase {
        &mut self.base
    }

}

impl Waypoint for EnigmaSoul {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        EnigmaSoul::should_render_dyn
    }

    /// java: @Override public void setFound() (dispatch: the body for this class)
    fn set_found(&self) -> fn(&compat::Shared<dyn Waypoint>) {
        <dyn ProfileAwareWaypoint>::set_found_from::<Self>
    }

    /// java: @Override public void setMissing() (dispatch: the body for this class)
    fn set_missing(&self) -> fn(&compat::Shared<dyn Waypoint>) {
        <dyn ProfileAwareWaypoint>::set_missing_from::<Self>
    }

    /// java: @Override public float[] getRenderColorComponents() (dispatch: the body for this class)
    fn get_render_color_components(&self) -> fn(&compat::Shared<dyn Waypoint>) -> Vec<f32> {
        <dyn ProfileAwareWaypoint>::get_render_color_components_from::<Self>
    }

}

impl Renderable for EnigmaSoul {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

impl EnigmaSoul {
    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn ProfileAwareWaypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn ProfileAwareWaypoint>)) || SkyblockerConfigManager::get().other_locations.rift.highlight_found_enigma_souls);
    }

    /// EnigmaSoul.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
