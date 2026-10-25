//! Ported from de.hysky.skyblocker.skyblock.waypoint.FairySouls by fabric2seer.

#[allow(unused_imports)]
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
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
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use crate::de::hysky::skyblocker::utils::constants;
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
            return CompletableFuture::completed_future(());
        }
        // `fairySoulsLoaded.thenRunAsync(runnable)`: Java's default executor is the common pool. The mod's objects are
        // the render thread's (not `Send`), so the task is handed to the game's own task queue (`game::execute`), which
        // runs it on that thread once the souls have loaded.
        let runnable = compat::RenderThread::new(runnable);
        let executor: mcrust::java::util::concurrent::completable_future::Executor = Arc::new(|task| crate::compat::execute(task));
        return FAIRY_SOULS_LOADED.get_mut().as_ref().unwrap().then_run_async(move || (runnable.get())(), executor);
    }

    /// java: public static int getFairySoulsSize(@Nullable String location)
    pub fn get_fairy_souls_size(mut location: Option<String>) -> i32 {
        return (if location.is_none() { compat::get_state(&MAX_SOULS) } else { (FAIRY_SOULS.get_mut().get(location.as_ref().unwrap()).cloned().unwrap().len() as i32) });
    }

    /// java: @Init public static void init()
    pub fn init() {
        FairySouls::load_fairy_souls();
        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(FairySouls::saveFoundFairySouls); - HEAD of `Minecraft#stop`
        report(hook::inject("net.minecraft.client.Minecraft#stop", At::Head, |_: &mut CallbackInfo<'_>| {
            FairySouls::save_found_fairy_souls(compat::minecraft());
        }));
        // java: ClientCommandRegistrationCallback.EVENT.register(FairySouls::registerCommands);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                FairySouls::register_commands(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { FairySouls::extract_rendering(&__a0); }));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(FairySouls::onChatMessage);
        compat::batch_w_allow_game_register(Arc::new(|text: &MutableComponent, overlay: bool| FairySouls::on_chat_message(text, overlay)));
    }

    /// java: private static void loadFairySouls()
    pub fn load_fairy_souls() {
        *FAIRY_SOULS_LOADED.get_mut() = NEURepoManager::run_async_after_load(Arc::new(move || {
            //   java: maxSouls = NEURepoManager.getConstants().getFairySouls().getMaxSouls();
            let neu_fairy_souls = NEURepoManager::get_constants().get_fairy_souls();
            compat::set_state(&MAX_SOULS, neu_fairy_souls.get_max_souls());
            //   java: NEURepoManager.getConstants().getFairySouls().getSoulLocations().forEach((location, fairiesForLocation) -> fairySouls.put(location, fairiesForLocation.stream().map(coordinate -> new BlockPos(coordinate.getX(), coordinate.getY(), coordinate.getZ())).collect(Collectors.toUnmodifiableMap(pos -> pos, pos -> new FairySoul(pos, TYPE_SUPPLIER, ColorUtils.getFloatComponents(DyeColor.GREEN), ColorUtils.getFloatComponents(DyeColor.RED))))));
            // (`Collectors.toUnmodifiableMap` throws on a duplicate position)
            for (location, fairies_for_location) in neu_fairy_souls.get_soul_locations().iter() {
                let mut waypoints: std::collections::HashMap<BlockPos, compat::Shared<dyn ProfileAwareWaypoint>> = std::collections::HashMap::new();
                for coordinate in fairies_for_location.iter() {
                    let pos: BlockPos = BlockPos::new(coordinate.get_x(), coordinate.get_y(), coordinate.get_z());
                    if waypoints.contains_key(&pos) {
                        panic!("java.lang.IllegalStateException: Duplicate key {:?}", pos);
                    }
                    let soul = FairySoul::new(
                        pos.clone(),
                        TYPE_SUPPLIER.get().clone(),
                        crate::de::hysky::skyblocker::utils::color_utils::ColorUtils::get_float_components_dye_color(&mcrust::net::minecraft::world::item::dye_color::DyeColor::Green),
                        crate::de::hysky::skyblocker::utils::color_utils::ColorUtils::get_float_components_dye_color(&mcrust::net::minecraft::world::item::dye_color::DyeColor::Red),
                    );
                    waypoints.insert(pos, soul as compat::Shared<dyn ProfileAwareWaypoint>);
                }
                FAIRY_SOULS.get_mut().insert(location.clone(), waypoints);
            }
            //   java: LOGGER.debug("[Skyblocker] Loaded {} fairy souls across {} locations", fairySouls.values().stream().mapToInt(Map::size).sum(), fairySouls.size());
            let soul_count: i32 = FAIRY_SOULS.get().values().map(|souls| souls.len() as i32).sum::<i32>();
            let location_count: usize = FAIRY_SOULS.get().len();
            logger().debug(&format!("[Skyblocker] Loaded {} fairy souls across {} locations", soul_count, location_count));
            // try (BufferedReader reader = Files.newBufferedReader(CONFIG_DIR.resolve("found_fairy_souls.json"))) {..}
            // catch (NoSuchFileException _) {} catch (IOException e) {..}
            match std::fs::read_to_string(skyblocker_mod::CONFIG_DIR.get_mut().join("found_fairy_souls.json")) {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => logger().error(&format!("[Skyblocker] Failed to load found fairy souls {}", e)),
                Ok(text) => {
                    // `JsonParser.parseReader(reader).getAsJsonObject()`: the unchecked exceptions (syntax, cast) are not caught.
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                    let serde_json::Value::Object(profiles) = json else { panic!("IllegalStateException: Not a JSON Object: {}", json) };
                    for (profile, found_fairies_for_profile_json) in profiles.iter() {
                        let serde_json::Value::Object(locations) = found_fairies_for_profile_json else { panic!("IllegalStateException: Not a JSON Object: {}", found_fairies_for_profile_json) };
                        for (location, found_fairies_for_location_json) in locations.iter() {
                            // `fairySouls.get(location)`: a location the repo does not list is null, and `.get(..)` on it throws.
                            let fairies_for_location = FAIRY_SOULS.get().get(location).cloned().expect("NullPointerException");
                            let serde_json::Value::Array(found_fairies) = found_fairies_for_location_json else { panic!("IllegalStateException: Not a JSON Array: {}", found_fairies_for_location_json) };
                            for found_fairy in found_fairies.iter() {
                                let found_fairy = compat::json::get_as_string(found_fairy);
                                match fairies_for_location.get(&PosUtils::parse_pos_string(&found_fairy)) {
                                    None => logger().warn(&format!("[Skyblocker] Ignored found fairy soul at {}", found_fairy)),
                                    Some(waypoint) => ({ let __f = compat::dispatch(waypoint, |__t| __t.set_found_1()); __f(waypoint, profile.clone()) }),
                                }
                            }
                        }
                    }
                    logger().debug("[Skyblocker] Loaded found fairy souls");
                }
            }
            //   java: LOGGER.info("[Skyblocker] Loaded {} fairy souls across {} locations", fairySouls.values().stream().mapToInt(Map::size).sum(), fairySouls.size());
            let soul_count: i32 = FAIRY_SOULS.get().values().map(|souls| souls.len() as i32).sum::<i32>();
            let location_count: usize = FAIRY_SOULS.get().len();
            logger().info(&format!("[Skyblocker] Loaded {} fairy souls across {} locations", soul_count, location_count));
        }));
    }

    /// java: private static void saveFoundFairySouls(Minecraft client)
    pub fn save_found_fairy_souls(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let mut found_fairies: std::collections::HashMap<String, std::collections::HashMap<String, compat::Shared<BlockPosSet>>> = std::collections::HashMap::new();
        for (location, fairies_for_location) in FAIRY_SOULS.get().iter() {
            for fairy_soul in fairies_for_location.values() {
                let (profiles, pos) = compat::read(fairy_soul, |__t| (__t.profile_aware_waypoint_base().found_profiles.clone(), __t.profile_aware_waypoint_base().pos.clone().unwrap()));
                for profile in profiles {
                    // `foundFairies.computeIfAbsent(profile, ..)`, `.get(profile).computeIfAbsent(location, ..)`, `.add(pos)`: on
                    // the maps themselves (a copy of the inner map would lose the new set).
                    let set = found_fairies.entry(profile).or_default().entry(location.clone()).or_insert_with(BlockPosSet::new).clone();
                    BlockPosSet::add(&set, &pos);
                }
            }
        }
        // try (BufferedWriter writer = Files.newBufferedWriter(CONFIG_DIR.resolve("found_fairy_souls.json"))) {..} catch (IOException e) {..}
        let mut found_fairies_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        for (profile, found_fairies_for_profile) in found_fairies.iter() {
            let mut found_fairies_for_profile_json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
            for (location, found_fairies_for_location) in found_fairies_for_profile.iter() {
                let mut found_fairies_for_location_json: Vec<serde_json::Value> = Vec::new();
                for found_fairy in BlockPosSet::iterate_mut(found_fairies_for_location) {
                    found_fairies_for_location_json.push(serde_json::Value::String(PosUtils::get_pos_string(&found_fairy.immutable())));
                }
                found_fairies_for_profile_json.insert(location.clone(), serde_json::Value::Array(found_fairies_for_location_json));
            }
            found_fairies_json.insert(profile.clone(), serde_json::Value::Object(found_fairies_for_profile_json));
        }
        // `GSON.toJson(foundFairiesJson, writer)` (pretty printing: two-space indentation).
        match std::fs::write(skyblocker_mod::CONFIG_DIR.get_mut().join("found_fairy_souls.json"), serde_json::to_string_pretty(&serde_json::Value::Object(found_fairies_json)).unwrap()) {
            Ok(()) => logger().info("[Skyblocker] Saved found fairy souls"),
            Err(e) => logger().error(&format!("[Skyblocker] Failed to write found fairy souls to file {}", e)),
        }
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    /// (`registryAccess` is the port's `Option`: absent before the first join)
    pub fn register_commands(dispatcher: &CommandDispatcher<client_command_registration_callback::FabricClientCommandSource>, registry_access: Option<&CommandBuildContext>) {
        let _ = registry_access;
        // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("fairySouls")
        //   .then(literal("markAllInCurrentIslandFound").executes(context -> { FairySouls.markAllFairiesOnCurrentIslandFound();
        //     context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.fairySouls.markAllFound"))); return 1; }))
        //   .then(literal("markAllInCurrentIslandMissing").executes(context -> { FairySouls.markAllFairiesOnCurrentIslandMissing();
        //     context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.fairySouls.markAllMissing"))); return 1; }))));
        let mut mark_all_found = client_command_registration_callback::literal("markAllInCurrentIslandFound");
        mark_all_found.executes(Arc::new(|_context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
            FairySouls::mark_all_fairies_on_current_island_found();
            compat::batch_w_send_feedback(&(constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.fairySouls.markAllFound")));
            Ok(1)
        }));
        let mut mark_all_missing = client_command_registration_callback::literal("markAllInCurrentIslandMissing");
        mark_all_missing.executes(Arc::new(|_context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
            FairySouls::mark_all_fairies_on_current_island_missing();
            compat::batch_w_send_feedback(&(constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.fairySouls.markAllMissing")));
            Ok(1)
        }));
        let mut fairy_souls = client_command_registration_callback::literal("fairySouls");
        fairy_souls.then(&mark_all_found);
        fairy_souls.then(&mark_all_missing);
        let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
        root.then(&fairy_souls);
        dispatcher.register(&root);
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let fairy_souls_config = &SkyblockerConfigManager::get().helpers.fairy_souls;
        // PERF (port-only, same result): `getLocationRaw()` read once, and the island's souls taken as a list of
        // handles (same order) instead of cloning the whole `HashMap` every frame.
        let __souls: Option<Vec<compat::Shared<dyn ProfileAwareWaypoint>>> = if fairy_souls_config.enable_fairy_souls_helper && (fairy_souls_config.highlight_on_bingo || !(Utils::is_on_bingo())) && FAIRY_SOULS_LOADED.get_mut().as_ref().unwrap().is_done() {
            let __location_raw: String = Utils::get_location_raw();
            FAIRY_SOULS.get_mut().get(&__location_raw).map(|souls| souls.values().cloned().collect())
        } else {
            None
        };
        if let Some(__souls) = __souls {
            for fairy_soul in __souls {
                let fairy_soul_waypoint = compat::view(&fairy_soul, |a| a as compat::Shared<dyn Waypoint>);
                let mut fairy_soul_not_found: bool = ({ let __f = compat::dispatch(&fairy_soul_waypoint, |__t| __t.should_render()); __f(&fairy_soul_waypoint) });
                if !fairy_souls_config.highlight_found_souls && !fairy_soul_not_found || fairy_souls_config.highlight_only_nearby_souls && { let __q0 = RenderHelper::get_camera().position(); compat::read(&fairy_soul, |__t| __t.waypoint_base().pos.clone()).unwrap().as_vec3i().dist_to_center_sqr_pos(&__q0) } > (2500 as f64) {
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
        // `.filter(Waypoint::shouldRender).min(comparingDouble(f -> f.pos.distToCenterSqr(player.position())))
        // .filter(f -> f.pos.distToCenterSqr(player.position()) <= 16).ifPresent(Waypoint::setFound)`
        let player_position = mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap();
        let distance = |fairy_soul: &compat::Shared<dyn ProfileAwareWaypoint>| compat::read(fairy_soul, |__t| __t.profile_aware_waypoint_base().pos.clone()).unwrap().as_vec3i().dist_to_center_sqr_pos(&player_position);
        let closest = fairies_on_current_island.values().cloned().filter(|fairy_soul| { let w = compat::view(fairy_soul, |a| a as compat::Shared<dyn Waypoint>); ({ let __f = compat::dispatch(&w, |__t| __t.should_render()); __f(&w) }) }).min_by(|a, b| distance(a).total_cmp(&distance(b)));
        if let Some(fairy_soul) = closest.filter(|fairy_soul| distance(fairy_soul) <= (16 as f64)) {
            let w = compat::view(&fairy_soul, |a| a as compat::Shared<dyn Waypoint>);
            ({ let __f = compat::dispatch(&w, |__t| __t.set_found()); __f(&w) });
        }
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
