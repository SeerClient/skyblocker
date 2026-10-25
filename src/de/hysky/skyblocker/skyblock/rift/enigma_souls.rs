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
    ///
    /// The reads run on the executor (Java's `runAsync`); the waypoints are render-thread objects, so they are built
    /// through `game::execute`, and `soulsLoaded` completes after them.
    pub fn load(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let manager = client.get_resource_manager();
        let waypoints_json: Identifier = WAYPOINTS_JSON.get().clone();
        let found_souls_file: std::path::PathBuf = FOUND_SOULS_FILE.get().clone();
        let souls_loaded: CompletableFuture<()> = CompletableFuture::new();
        *SOULS_LOADED.get_mut() = Some(souls_loaded.clone());
        compat::executor(Box::new(move || {
            // try (BufferedReader reader = client.getResourceManager().openAsReader(WAYPOINTS_JSON)) { JsonParser.parseReader(reader) ..
            let waypoints_file: Result<String, String> = manager.open_as_reader(&waypoints_json);
            // try (BufferedReader reader = Files.newBufferedReader(FOUND_SOULS_FILE)) {..} catch (NoSuchFileException _) {}
            let found_souls: Result<Option<String>, std::io::Error> = match std::fs::read_to_string(&found_souls_file) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(e),
            };
            crate::compat::execute(move || {
                match waypoints_file {
                    Err(e) => logger().error(&format!("[Skyblocker] There was an error while loading enigma soul waypoints! {}", e)),
                    Ok(reader) => {
                        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            let file: serde_json::Value = serde_json::from_str(&reader).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                            let file: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&file);
                            let mut waypoints: Vec<serde_json::Value> = compat::json::get_as_json_array(&file.get("waypoints").cloned().unwrap());
                            for i in 0..(waypoints.len() as i32) {
                                let mut waypoint: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(waypoints[(i) as usize]));
                                let mut pos: BlockPos = BlockPos::new(compat::json::get_as_int(&waypoint.get("x").cloned().unwrap()), compat::json::get_as_int(&waypoint.get("y").cloned().unwrap()), compat::json::get_as_int(&waypoint.get("z").cloned().unwrap()));
                                SOUL_WAYPOINTS.get_mut().insert(pos.clone(), (EnigmaSoul::new(pos.clone(), TYPE_SUPPLIER.get_mut().clone(), GREEN.clone(), RED.clone()) as compat::Shared<dyn ProfileAwareWaypoint>));
                            }
                        }));
                        if let Err(e) = __result {
                            logger().error(&format!("[Skyblocker] There was an error while loading enigma soul waypoints! {}", compat::panic_message(&e)));
                        }
                    }
                }
                match found_souls {
                    // catch (NoSuchFileException _) {}
                    Ok(None) => {}
                    Err(e) => logger().error(&format!("[Skyblocker] There was an error while loading found enigma souls! {}", e)),
                    Ok(Some(reader)) => {
                        // not under the IOException catch in Java: a parse failure ends the task (the future completes)
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            let json: serde_json::Value = serde_json::from_str(&reader).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                            for (profile, found) in compat::json::get_as_json_object(&json) {
                                for found_soul in compat::json::get_as_json_array(&found) {
                                    let soul = SOUL_WAYPOINTS.get().get(&PosUtils::parse_pos_string(&compat::json::get_as_string(&found_soul))).cloned().expect("NullPointerException");
                                    let __f = compat::dispatch(&soul, |__t| __t.set_found_1());
                                    __f(&soul, profile.clone());
                                }
                            }
                        }));
                    }
                }
                souls_loaded.complete(());
            });
        }));
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
            // for (BlockPos foundSoul : foundSoulsForProfile.getValue())
            for found_soul in BlockPosSet::to_array_1::<BlockPos>(&found_souls_for_profile.1, Vec::new()).into_iter().flatten() {
                found_souls_json.push(serde_json::Value::String(PosUtils::get_pos_string(&found_soul)));
            }
            json.insert(found_souls_for_profile.0.clone(), serde_json::Value::Array(found_souls_json));
        }
        // java: try (BufferedWriter writer = Files.newBufferedWriter(FOUND_SOULS_FILE)) { SkyblockerMod.GSON.toJson(json, writer); }
        // catch (IOException e) {..} (GSON is `setPrettyPrinting()`)
        let text: String = serde_json::to_string_pretty(&serde_json::Value::Object(json)).unwrap_or_default();
        if let Err(e) = std::fs::write(FOUND_SOULS_FILE.get(), text) {
            logger().error(&format!("[Skyblocker] There was an error while saving found enigma souls! {}", e));
        }
    }

    /// java: static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let config = &SkyblockerConfigManager::get().other_locations.rift;
        if Utils::is_in_the_rift() && config.enigma_soul_waypoints && SOULS_LOADED.get_mut().as_ref().unwrap().is_done() {
            for soul in SOUL_WAYPOINTS.get_mut().values().cloned() {
                if { let __f = compat::dispatch(&soul, |__t| __t.should_render()); __f(&compat::view(&soul, |a| a as compat::Shared<dyn Waypoint>)) } || config.highlight_found_enigma_souls {
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
    pub fn register_commands(dispatcher: &CommandDispatcher<crate::prelude::client_command_registration_callback::FabricClientCommandSource>, registry_access: Option<&CommandBuildContext>) {
        //   java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("rift").then(literal("enigmaSouls")
        //   java:     .then(literal("markAllFound")..).then(literal("markAllMissing")..).then(literal("markClosestFound")..).then(literal("markClosestMissing")..))));
        use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
        use crate::prelude::client_command_registration_callback as ccr;
        type Ctx = mcrust::com::mojang::brigadier::context::command_context::CommandContext<ccr::FabricClientCommandSource>;
        let _ = registry_access;
        fn feedback(key: &str) {
            // java: context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable(key)));
            compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable(key)));
        }
        fn mark_all(found: bool) {
            // java: SOUL_WAYPOINTS.values().forEach(Waypoint::setFound / Waypoint::setMissing);
            let souls: Vec<compat::Shared<dyn ProfileAwareWaypoint>> = SOUL_WAYPOINTS.get().values().cloned().collect();
            for soul in souls {
                let soul: compat::Shared<dyn Waypoint> = compat::view(&soul, |a| a as compat::Shared<dyn Waypoint>).clone();
                let __f = if found { compat::dispatch(&soul, |__t| __t.set_found()) } else { compat::dispatch(&soul, |__t| __t.set_missing()) };
                __f(&soul);
            }
        }
        let mut mark_all_found = ccr::literal("markAllFound");
        mark_all_found.executes(Arc::new(|_: &Ctx| { mark_all(true); feedback("skyblocker.rift.enigmaSouls.markAllFound"); Ok(1) }));
        let mut mark_all_missing = ccr::literal("markAllMissing");
        mark_all_missing.executes(Arc::new(|_: &Ctx| { mark_all(false); feedback("skyblocker.rift.enigmaSouls.markAllMissing"); Ok(1) }));
        let mut mark_closest_found = ccr::literal("markClosestFound");
        mark_closest_found.executes(Arc::new(|_: &Ctx| { EnigmaSouls::mark_closest_soul(true); feedback("skyblocker.rift.enigmaSouls.markClosestFound"); Ok(1) }));
        let mut mark_closest_missing = ccr::literal("markClosestMissing");
        mark_closest_missing.executes(Arc::new(|_: &Ctx| { EnigmaSouls::mark_closest_soul(false); feedback("skyblocker.rift.enigmaSouls.markClosestMissing"); Ok(1) }));
        let mut enigma_souls = ccr::literal("enigmaSouls");
        enigma_souls.then(&mark_all_found);
        enigma_souls.then(&mark_all_missing);
        enigma_souls.then(&mark_closest_found);
        enigma_souls.then(&mark_closest_missing);
        let mut rift = ccr::literal("rift");
        rift.then(&enigma_souls);
        let mut root = ccr::literal(skyblocker_mod::NAMESPACE);
        root.then(&rift);
        dispatcher.register(&root);
    }

    /// java: private static void markClosestSoul(boolean asFound)
    pub fn mark_closest_soul(mut as_found: bool) {
        if !(SOULS_LOADED.get_mut().as_ref().unwrap().is_done()) || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let player_position = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.position()).unwrap();
        let dist = |soul: &compat::Shared<dyn ProfileAwareWaypoint>| compat::read(soul, |__t| __t.profile_aware_waypoint_base().pos.clone()).unwrap().dist_to_center_sqr(player_position.x, player_position.y, player_position.z);
        let souls: Vec<compat::Shared<dyn ProfileAwareWaypoint>> = SOUL_WAYPOINTS.get().values().cloned().collect();
        let closest = souls
            .into_iter()
            .filter(|soul| { let __f = compat::dispatch(soul, |__t| __t.should_render()); __f(&compat::view(soul, |a| a as compat::Shared<dyn Waypoint>)) })
            .min_by(|a, b| dist(a).total_cmp(&dist(b)))
            .filter(|soul| dist(soul) <= (16 as f64));
        if let Some(soul) = closest {
            let soul: compat::Shared<dyn Waypoint> = compat::view(&soul, |a| a as compat::Shared<dyn Waypoint>).clone();
            let __f = if as_found { compat::dispatch(&soul, |__t| __t.set_found()) } else { compat::dispatch(&soul, |__t| __t.set_missing()) };
            __f(&soul);
        }
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
