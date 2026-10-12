//! Ported from de.hysky.skyblocker.skyblock.waypoint.FairySouls by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::neu_repo_manager::NEURepoManager;
use crate::de::hysky::skyblocker::utils::pos_utils::PosUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
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
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;

/// java: private static final Supplier<Waypoint.Type> TYPE_SUPPLIER = () -> SkyblockerConfigManager.get().uiAndVisuals.waypoints.waypointType;
pub static TYPE_SUPPLIER: compat::MainThread<Arc<dyn Fn() -> Type>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().ui_and_visuals.waypoints.waypoint_type));
/// java: private static CompletableFuture<Void> fairySoulsLoaded;
pub static FAIRY_SOULS_LOADED: compat::MainThread<Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>> = compat::MainThread::new(|| None);
/// java: private static int maxSouls = 0;
pub static MAX_SOULS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final Map<String, Map<BlockPos, ProfileAwareWaypoint>> fairySouls = new HashMap<>();
pub static FAIRY_SOULS: compat::MainThread<std::collections::HashMap<String, std::collections::HashMap<BlockPos, compat::Shared<dyn ProfileAwareWaypoint>>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class FairySouls
#[derive(Clone)]
pub struct FairySouls {}

impl FairySouls {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @SuppressWarnings("UnusedReturnValue") public static CompletableFuture<Void> runAsyncAfterFairySoulsLoad(Runnable runnable)
    pub fn run_async_after_fairy_souls_load(runnable: Arc<dyn Fn()>) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        if FAIRY_SOULS_LOADED.get_mut().is_none() {
            logger().error("[Skyblocker] Fairy Souls have not being initialized yet! Please ensure the Fairy Souls configs is initialized before modules calling this method in SkyblockerMod#onInitializeClient. This error can be safely ignore in a test environment.");
            return CompletableFuture::completed_future(None);
        }
        // TODO(fabric2seer): `then_run_async` takes 2 argument(s) in the port (`action: F, executor: Executor`), Java passes 1
        // rust: return FAIRY_SOULS_LOADED.get_mut().as_ref().unwrap().then_run_async(runnable.clone());
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public static int getFairySoulsSize(@Nullable String location)
    pub fn get_fairy_souls_size(mut location: Option<String>) -> i32 {
        return (if location.is_none() { compat::get_state(&MAX_SOULS) } else { (FAIRY_SOULS.get_mut().get(location.as_ref().unwrap()).cloned().unwrap().len() as i32) });
    }

    /// java: @Init public static void init()
    pub fn init() {
        FairySouls::load_fairy_souls();
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(FairySouls::saveFoundFairySouls);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
                //   java: 				.then(literal("fairySouls")
                //   java: 						.then(literal("markAllInCurrentIslandFound").executes(context -> {
                //   java: 							FairySouls.markAllFairiesOnCurrentIslandFound();
                //   java: 							context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.fairySouls.markAllFound")));
                //   java: 							return 1;
                //   java: 						}))
                //   java: 						.then(literal("markAllInCurrentIslandMissing").executes(context -> {
                //   java: 							FairySouls.markAllFairiesOnCurrentIslandMissing();
                //   java: 							context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.fairySouls.markAllMissing")));
                //   java: 							return 1;
                //   java: 						}))));
            },
        ));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { FairySouls::extract_rendering(&__a0); }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(FairySouls::onChatMessage);
    }

    /// java: private static void loadFairySouls()
    pub fn load_fairy_souls() {
        *FAIRY_SOULS_LOADED.get_mut() = NEURepoManager::run_async_after_load(Arc::new(move || {
            // TODO(fabric2seer): `.getFairySouls(..)` on a NEUConstants
            //   java: maxSouls = NEURepoManager.getConstants().getFairySouls().getMaxSouls();
            // TODO(fabric2seer): `.getFairySouls(..)` on a NEUConstants
            //   java: NEURepoManager.getConstants().getFairySouls().getSoulLocations().forEach((location, fairiesForLocation) -> fairySouls.put(location, fairiesForLocation.stream().map(coordinate -> new BlockPos(coordinate.getX(), coordinate.getY(), coordinate.getZ())).collect(Collectors.toUnmodifiableMap(pos -> pos, pos -> new FairySoul(pos, TYPE_SUPPLIER, ColorUtils.getFloatComponents(DyeColor.GREEN), ColorUtils.getFloatComponents(DyeColor.RED))))));
            // TODO(fabric2seer): static call Map.size(..)
            //   java: LOGGER.debug("[Skyblocker] Loaded {} fairy souls across {} locations", fairySouls.values().stream().mapToInt(Map::size).sum(), fairySouls.size());
            {
                // TODO(fabric2seer): Files.newBufferedReader
                //   java: BufferedReader reader = Files.newBufferedReader(..);
                let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                // java: try {..} catch (NoSuchFileException _) catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): static call JsonParser.parseReader(..)
                    //   java: for (Map.Entry<String, JsonElement> foundFairiesForProfileJson : JsonParser.parseReader(reader).getAsJsonObject().asMap().entrySet()) {
                    //   java: 					for (Map.Entry<String, JsonElement> foundFairiesForLocationJson : foundFairiesForProfileJson.getValue().getAsJsonObject().asMap().entrySet()) {
                    //   java: 						Map<BlockPos, ProfileAwareWaypoint> fairiesForLocation = fairySouls.get(foundFairiesForLocationJson.getKey());
                    //   java: 						for (JsonElement foundFairy : foundFairiesForLocationJson.getValue().getAsJsonArray().asList()) {
                    //   java: 							ProfileAwareWaypoint waypoint = fairiesForLocation.get(PosUtils.parsePosString(foundFairy.getAsString()));
                    //   java: 							if (waypoint == null) {
                    //   java: 								LOGGER.warn("[Skyblocker] Ignored found fairy soul at {}", foundFairy.getAsString());
                    //   java: 							} else {
                    //   java: 								waypoint.setFound(foundFairiesForProfileJson.getKey());
                    //   java: 							}
                    //   java: 						}
                    //   java: 					}
                    //   java: 				}
                    logger().debug("[Skyblocker] Loaded found fairy souls");
                }));
                if let Err(_) = __result {
                }
                // TODO(fabric2seer): the second `catch (IOException e)` clause (merged into the first)
                //   java: {
                //   java: 				LOGGER.error("[Skyblocker] Failed to load found fairy souls", e);
                //   java: 			}
            }
            // TODO(fabric2seer): static call Map.size(..)
            //   java: LOGGER.info("[Skyblocker] Loaded {} fairy souls across {} locations", fairySouls.values().stream().mapToInt(Map::size).sum(), fairySouls.size());
        }));
    }

    /// java: private static void saveFoundFairySouls(Minecraft client)
    pub fn save_found_fairy_souls(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let mut found_fairies: std::collections::HashMap<String, std::collections::HashMap<String, compat::Shared<BlockPosSet>>> = std::collections::HashMap::new();
        for fairies_for_location in FAIRY_SOULS.get_mut().iter().map(|(k, v)| (k.clone(), v.clone())) {
            for fairy_soul in fairies_for_location.1.values().cloned() {
                for profile in compat::read(&fairy_soul, |__t| __t.profile_aware_waypoint_base().found_profiles.clone()) {
                    found_fairies.entry(profile.clone()).or_insert_with_key(|k| (|_| std::collections::HashMap::new())(k.clone()));
                    found_fairies.get(&profile).cloned().unwrap().entry(fairies_for_location.0.clone()).or_insert_with_key(|k| (|_| BlockPosSet::new())(k.clone()));
                    BlockPosSet::add(&found_fairies.get(&profile).cloned().unwrap().get(&fairies_for_location.0).cloned().unwrap(), &compat::read(&fairy_soul, |__t| __t.profile_aware_waypoint_base().pos.clone()).unwrap());
                }
            }
        }
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut found_fairies_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                for found_fairies_for_profile in found_fairies.iter().map(|(k, v)| (k.clone(), v.clone())) {
                    let mut found_fairies_for_profile_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                    for found_fairies_for_location in found_fairies_for_profile.1.iter().map(|(k, v)| (k.clone(), v.clone())) {
                        let mut found_fairies_for_location_json: Vec<serde_json::Value> = Vec::new();
                        for found_fairy in BlockPosSet::iterate_mut(&found_fairies_for_location.1) {
                            found_fairies_for_location_json.push(compat::json::to_value(PosUtils::get_pos_string(&found_fairy)));
                        }
                        found_fairies_for_profile_json.insert(found_fairies_for_location.0.clone(), compat::json::to_value(found_fairies_for_location_json.clone()));
                    }
                    found_fairies_json.insert(found_fairies_for_profile.0.clone(), compat::json::to_value(found_fairies_for_profile_json.clone()));
                }
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(foundFairiesJson, writer);
                logger().info("[Skyblocker] Saved found fairy souls");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to write found fairy souls to file"));
            }
        }
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   java: 				.then(literal("fairySouls")
        //   java: 						.then(literal("markAllInCurrentIslandFound").executes(context -> {
        //   java: 							FairySouls.markAllFairiesOnCurrentIslandFound();
        //   java: 							context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.fairySouls.markAllFound")));
        //   java: 							return 1;
        //   java: 						}))
        //   java: 						.then(literal("markAllInCurrentIslandMissing").executes(context -> {
        //   java: 							FairySouls.markAllFairiesOnCurrentIslandMissing();
        //   java: 							context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.fairySouls.markAllMissing")));
        //   java: 							return 1;
        //   java: 						}))));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let fairy_souls_config = &SkyblockerConfigManager::get().helpers.fairy_souls;
        if fairy_souls_config.enable_fairy_souls_helper && fairy_souls_config.highlight_on_bingo || !(Utils::is_on_bingo()) && FAIRY_SOULS_LOADED.get_mut().as_ref().unwrap().is_done() && FAIRY_SOULS.get_mut().contains_key(&Utils::get_location_raw()) {
            for fairy_soul in FAIRY_SOULS.get_mut().get(&Utils::get_location_raw()).cloned().unwrap().values().cloned() {
                let mut fairy_soul_not_found: bool = ({ let __f = compat::dispatch(&fairy_soul, |__t| __t.should_render()); __f(&fairy_soul) });
                // TODO(fabric2seer): `dist_to_center_sqr` takes 3 argument(s) in the port (`x: f64, y: f64, z: f64`), Java passes 1
                if !fairy_souls_config.highlight_found_souls && !fairy_soul_not_found || fairy_souls_config.highlight_only_nearby_souls && { let (__q0,) = (RenderHelper::get_camera().position(),); fairy_soul.lock().borrow().waypoint_base().pos.as_ref().unwrap().dist_to_center_sqr(__q0) } > (2500 as f64) {
                    continue;
                }
                ({ let __f = compat::dispatch(&fairy_soul, |__t| __t.extract_rendering()); __f(&compat::view(&fairy_soul, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

    /// java: private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        let mut message: String = chat_formatting::strip_formatting(&text.get_string());
        if message == "You have already found that Fairy Soul!" || message == "SOUL! You found a Fairy Soul!" {
            FairySouls::mark_closest_fairy_found();
        }
        return true;
    }

    /// java: private static void markClosestFairyFound()
    pub fn mark_closest_fairy_found() {
        if !(FAIRY_SOULS_LOADED.get_mut().as_ref().unwrap().is_done()) {
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            logger().warn("[Skyblocker] Failed to mark closest fairy soul as found because player is null");
            return;
        }
        let Some(mut fairies_on_current_island) = FAIRY_SOULS.get_mut().get(&Utils::get_location_raw()).cloned() else {
            logger().warn("[Skyblocker] Failed to mark closest fairy soul as found because there are no fairy souls loaded on the current island. NEU repo probably failed to load.");
            return;
        };
        // TODO(fabric2seer): `dist_to_center_sqr` takes 3 argument(s) in the port (`x: f64, y: f64, z: f64`), Java passes 1
        // rust: if let Some(__a0) = fairies_on_current_island.values().cloned().filter(|__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.should_render()); __f(&compat::view(__a0, |a| a as compat::Shared<dyn Waypoint>)) })).min_by(|a, b| { let key = |fairy_soul| { let (__q0,) = (mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap(),); fairy_soul.lock().borrow().profile_aware_waypoint_base().pos.as_ref().unwrap().dist_to_center_sqr(__q0) }; key(a).total_cmp(&key(b)) }).filter(|fairy_soul| ({ let (__q0,) = (mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap(),); fairy_soul.lock().borrow().profile_aware_waypoint_base().pos.as_ref().unwrap().dist_to_center_sqr(__q0) } <= (16 as f64))) {
        // rust: ({ let __f = compat::dispatch(&__a0, |__t| __t.set_found()); __f(&compat::view(&__a0, |a| a as compat::Shared<dyn Waypoint>)) });
        // rust: }
    }

    /// java: public static void markAllFairiesOnCurrentIslandFound()
    pub fn mark_all_fairies_on_current_island_found() {
        let mut fairies_for_location: Option<std::collections::HashMap<BlockPos, compat::Shared<dyn ProfileAwareWaypoint>>> = FAIRY_SOULS.get_mut().get(&Utils::get_location_raw()).cloned();
        if let Some(fairies_for_location) = &fairies_for_location {
            for __a0 in fairies_for_location.values().cloned() {
                ({ let __f = compat::dispatch(&__a0, |__t| __t.set_found()); __f(&compat::view(&__a0, |a| a as compat::Shared<dyn Waypoint>)) });
            }
        }
    }

    /// java: public static void markAllFairiesOnCurrentIslandMissing()
    pub fn mark_all_fairies_on_current_island_missing() {
        let mut fairies_for_location: Option<std::collections::HashMap<BlockPos, compat::Shared<dyn ProfileAwareWaypoint>>> = FAIRY_SOULS.get_mut().get(&Utils::get_location_raw()).cloned();
        if let Some(fairies_for_location) = &fairies_for_location {
            for __a0 in fairies_for_location.values().cloned() {
                ({ let __f = compat::dispatch(&__a0, |__t| __t.set_missing()); __f(&compat::view(&__a0, |a| a as compat::Shared<dyn Waypoint>)) });
            }
        }
    }

}

/// java: private static class FairySoul extends ProfileAwareWaypoint
#[derive(Clone)]
pub struct FairySoul {
    /// java: extends ProfileAwareWaypoint
    pub base: ProfileAwareWaypointBase,
}

impl std::ops::Deref for FairySoul {
    type Target = ProfileAwareWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FairySoul {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FairySoul {
    /// java: private FairySoul(BlockPos pos, Supplier<Type> typeSupplier, float[] missingColor, float[] foundColor)
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

impl ProfileAwareWaypoint for FairySoul {
    fn profile_aware_waypoint_base(&self) -> &ProfileAwareWaypointBase {
        &self.base
    }
    fn profile_aware_waypoint_base_mut(&mut self) -> &mut ProfileAwareWaypointBase {
        &mut self.base
    }

}

impl Waypoint for FairySoul {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        FairySoul::should_render_dyn
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

impl Renderable for FairySoul {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

impl FairySoul {
    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn ProfileAwareWaypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn ProfileAwareWaypoint>)) || SkyblockerConfigManager::get().helpers.fairy_souls.highlight_found_souls);
    }

    /// FairySoul.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
