//! Ported from de.hysky.skyblocker.skyblock.waypoint.Relics by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::pos_utils::PosUtils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
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
use mcrust::net::minecraft::commands::command_build_context::CommandBuildContext;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;

/// java: private static final Supplier<Waypoint.Type> TYPE_SUPPLIER = () -> SkyblockerConfigManager.get().uiAndVisuals.waypoints.waypointType;
pub static TYPE_SUPPLIER: compat::MainThread<Arc<dyn Fn() -> Type>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().ui_and_visuals.waypoints.waypoint_type));
/// java: private static CompletableFuture<Void> relicsLoaded;
pub static RELICS_LOADED: compat::MainThread<Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>> = compat::MainThread::new(|| None);
/// java: @SuppressWarnings({"unused", "FieldCanBeLocal"}) private static int totalRelics = 0;
pub static TOTAL_RELICS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));
/// java: private static final Map<BlockPos, ProfileAwareWaypoint> relics = new HashMap<>();
pub static RELICS: compat::MainThread<std::collections::HashMap<BlockPos, compat::Shared<dyn ProfileAwareWaypoint>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class Relics
#[derive(Clone)]
pub struct Relics {}

impl Relics {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(Relics::loadRelics);
        // TODO(fabric2seer): no SeerClient event yet: hook HEAD of Minecraft#stop (`net.minecraft.client.Minecraft#stop`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STOPPING.register(Relics::saveFoundRelics);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
                //   java: 				.then(literal("relics")
                //   java: 						.then(literal("markAllFound").executes(context -> {
                //   java: 							relics.values().forEach(ProfileAwareWaypoint::setFound);
                //   java: 							context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.relics.markAllFound")));
                //   java: 							return 1;
                //   java: 						}))
                //   java: 						.then(literal("markAllMissing").executes(context -> {
                //   java: 							relics.values().forEach(ProfileAwareWaypoint::setMissing);
                //   java: 							context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.relics.markAllMissing")));
                //   java: 							return 1;
                //   java: 						}))));
            },
        ));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { Relics::extract_rendering(&__a0); }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(Relics::onChatMessage);
    }

    /// java: private static void loadRelics(Minecraft client)
    pub fn load_relics(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        *RELICS_LOADED.get_mut() = Some(CompletableFuture::<()>::run_async(move || (|| {
            {
                let mut reader: crate::unresolved::BufferedReader = client.get_resource_manager().open_as_reader(&crate::entry::skyblocker_mod::id("spidersden/relics.json")).unwrap();
                // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): static call JsonParser.parseReader(..)
                    //   java: for (Map.Entry<String, JsonElement> json : JsonParser.parseReader(reader).getAsJsonObject().asMap().entrySet()) {
                    //   java: 					if (json.getKey().equals("total")) {
                    //   java: 						totalRelics = json.getValue().getAsInt();
                    //   java: 					} else if (json.getKey().equals("locations")) {
                    //   java: 						for (JsonElement locationJson : json.getValue().getAsJsonArray().asList()) {
                    //   java: 							JsonObject posData = locationJson.getAsJsonObject();
                    //   java: 							BlockPos pos = new BlockPos(posData.get("x").getAsInt(), posData.get("y").getAsInt(), posData.get("z").getAsInt());
                    //   java: 							relics.put(pos, new Relic(pos, TYPE_SUPPLIER, ColorUtils.getFloatComponents(DyeColor.YELLOW), ColorUtils.getFloatComponents(DyeColor.BROWN)));
                    //   java: 						}
                    //   java: 					}
                    //   java: 				}
                    logger().info("[Skyblocker] Loaded relics locations");
                }));
                if let Err(e) = __result {
                    logger().error(&format!("[Skyblocker] Failed to load relics locations"));
                }
            }
            {
                // TODO(fabric2seer): Files.newBufferedReader
                //   java: BufferedReader reader = Files.newBufferedReader(..);
                let mut reader: crate::unresolved::BufferedReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                // java: try {..} catch (NoSuchFileException _) catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): static call JsonParser.parseReader(..)
                    //   java: for (Map.Entry<String, JsonElement> profileJson : JsonParser.parseReader(reader).getAsJsonObject().asMap().entrySet()) {
                    //   java: 					for (JsonElement foundRelicsJson : profileJson.getValue().getAsJsonArray().asList()) {
                    //   java: 						relics.get(PosUtils.parsePosString(foundRelicsJson.getAsString())).setFound(profileJson.getKey());
                    //   java: 					}
                    //   java: 				}
                    logger().debug("[Skyblocker] Loaded found relics");
                }));
                if let Err(_) = __result {
                }
                // TODO(fabric2seer): the second `catch (IOException e)` clause (merged into the first)
                //   java: {
                //   java: 				LOGGER.error("[Skyblocker] Failed to load found relics", e);
                //   java: 			}
            }
        })(), compat::executor));
    }

    /// java: private static void saveFoundRelics(Minecraft client)
    pub fn save_found_relics(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let mut found_relics: std::collections::HashMap<String, compat::Shared<BlockPosSet>> = std::collections::HashMap::new();
        for relic in RELICS.get_mut().values().cloned() {
            for profile in compat::read(&relic, |__t| __t.profile_aware_waypoint_base().found_profiles.clone()) {
                found_relics.entry(profile.clone()).or_insert_with_key(|k| (|_| BlockPosSet::new())(k.clone()));
                BlockPosSet::add(&found_relics.get(&profile).cloned().unwrap(), &compat::read(&relic, |__t| __t.profile_aware_waypoint_base().pos.clone()).unwrap());
            }
        }
        {
            // TODO(fabric2seer): Files.newBufferedWriter
            //   java: BufferedWriter writer = Files.newBufferedWriter(..);
            let mut writer: crate::unresolved::BufferedWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                for found_relics_for_profile in found_relics.iter().map(|(k, v)| (k.clone(), v.clone())) {
                    let mut found_relics_json: Vec<serde_json::Value> = Vec::new();
                    for found_relic in BlockPosSet::iterate_mut(&found_relics_for_profile.1) {
                        found_relics_json.push(compat::json::to_value(PosUtils::get_pos_string(&found_relic)));
                    }
                    json.insert(found_relics_for_profile.0.clone(), compat::json::to_value(found_relics_json.clone()));
                }
                // TODO(fabric2seer): `.toJson(..)` on a Gson
                //   java: SkyblockerMod.GSON.toJson(json, writer);
                logger().debug("[Skyblocker] Saved found relics");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker] Failed to write found relics to file"));
            }
        }
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::unresolved::FabricClientCommandSource>, registry_access: &CommandBuildContext) {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE)
        //   java: 				.then(literal("relics")
        //   java: 						.then(literal("markAllFound").executes(context -> {
        //   java: 							relics.values().forEach(ProfileAwareWaypoint::setFound);
        //   java: 							context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.relics.markAllFound")));
        //   java: 							return 1;
        //   java: 						}))
        //   java: 						.then(literal("markAllMissing").executes(context -> {
        //   java: 							relics.values().forEach(ProfileAwareWaypoint::setMissing);
        //   java: 							context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.relics.markAllMissing")));
        //   java: 							return 1;
        //   java: 						}))));
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let config = &SkyblockerConfigManager::get().other_locations.spiders_den.relics;
        if config.enable_relics_helper && RELICS_LOADED.get_mut().as_ref().unwrap().is_done() && Utils::get_location_raw() == "combat_1" {
            for relic in RELICS.get_mut().values().cloned() {
                let mut is_relic_missing: bool = ({ let __f = compat::dispatch(&relic, |__t| __t.should_render()); __f(&compat::view(&relic, |a| a as compat::Shared<dyn Waypoint>)) });
                if !is_relic_missing && !config.highlight_found_relics {
                    continue;
                }
                ({ let __f = compat::dispatch(&relic, |__t| __t.extract_rendering()); __f(&compat::view(&relic, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

    /// java: private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        let mut message: String = text.get_string();
        if message == "You've already found this relic!" || message.starts_with("+10,000 Coins! (") && message.ends_with("/28 Relics)") {
            Relics::mark_closest_relic_found();
        }
        return true;
    }

    /// java: private static void markClosestRelicFound()
    pub fn mark_closest_relic_found() {
        if !(RELICS_LOADED.get_mut().as_ref().unwrap().is_done()) {
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            logger().warn("[Skyblocker] Failed to mark closest relic as found because player is null");
            return;
        }
        // TODO(fabric2seer): `dist_to_center_sqr` takes 3 argument(s) in the port (`x: f64, y: f64, z: f64`), Java passes 1
        // rust: if let Some(__a0) = RELICS.get_mut().values().cloned().filter(|__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.should_render()); __f(&compat::view(__a0, |a| a as compat::Shared<dyn Waypoint>)) })).min_by(|a, b| { let key = |relic| { let (__q0,) = (mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap(),); relic.lock().borrow().profile_aware_waypoint_base().pos.as_ref().unwrap().dist_to_center_sqr(__q0) }; key(a).total_cmp(&key(b)) }).filter(|relic| ({ let (__q0,) = (mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap(),); relic.lock().borrow().profile_aware_waypoint_base().pos.as_ref().unwrap().dist_to_center_sqr(__q0) } <= (16 as f64))) {
        // rust: ({ let __f = compat::dispatch(&__a0, |__t| __t.set_found()); __f(&compat::view(&__a0, |a| a as compat::Shared<dyn Waypoint>)) });
        // rust: }
    }

}

/// java: private static class Relic extends ProfileAwareWaypoint
#[derive(Clone)]
pub struct Relic {
    /// java: extends ProfileAwareWaypoint
    pub base: ProfileAwareWaypointBase,
}

impl std::ops::Deref for Relic {
    type Target = ProfileAwareWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Relic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Relic {
    /// java: private Relic(BlockPos pos, Supplier<Type> typeSupplier, float[] missingColor, float[] foundColor)
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

impl ProfileAwareWaypoint for Relic {
    fn profile_aware_waypoint_base(&self) -> &ProfileAwareWaypointBase {
        &self.base
    }
    fn profile_aware_waypoint_base_mut(&mut self) -> &mut ProfileAwareWaypointBase {
        &mut self.base
    }

}

impl Waypoint for Relic {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        Relic::should_render_dyn
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

impl Renderable for Relic {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

impl Relic {
    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn ProfileAwareWaypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn ProfileAwareWaypoint>)) || SkyblockerConfigManager::get().other_locations.spiders_den.relics.highlight_found_relics);
    }

    /// Relic.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
