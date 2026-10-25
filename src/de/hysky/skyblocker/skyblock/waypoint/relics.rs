//! Ported from de.hysky.skyblocker.skyblock.waypoint.Relics by fabric2seer.

#[allow(unused_imports)]
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::block_pos_set::BlockPosSet;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
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
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use crate::de::hysky::skyblocker::utils::constants;
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
        // java: ClientLifecycleEvents.CLIENT_STARTED.register(Relics::loadRelics); - a RETURN injector registered from inside
        // `Minecraft#<init>` (the client entrypoints run there) is not in that call's injector snapshot, so it would never
        // fire; `compat::execute` runs it before the first client tick, where Fabric fires CLIENT_STARTED (as
        // `MuseumItemCache.init`)
        compat::execute(|| Relics::load_relics(compat::minecraft()));
        // java: ClientLifecycleEvents.CLIENT_STOPPING.register(Relics::saveFoundRelics); - HEAD of `Minecraft#stop`
        report(hook::inject("net.minecraft.client.Minecraft#stop", At::Head, |_: &mut CallbackInfo<'_>| {
            Relics::save_found_relics(compat::minecraft());
        }));
        // java: ClientCommandRegistrationCallback.EVENT.register(Relics::registerCommands);
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                Relics::register_commands(dispatcher.dispatcher(), dispatcher.build_context());
            },
        ));
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { Relics::extract_rendering(&__a0); }));
        // java: ClientReceiveMessageEvents.ALLOW_GAME.register(Relics::onChatMessage);
        compat::batch_w_allow_game_register(Arc::new(|text: &MutableComponent, overlay: bool| Relics::on_chat_message(text, overlay)));
    }

    /// java: private static void loadRelics(Minecraft client)
    pub fn load_relics(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        // `CompletableFuture.runAsync(.., SkyblockerMod.VIRTUAL_THREAD_EXECUTOR)`: the task fills `relics`, a map of the
        // mod's render-thread objects (not `Send`), so it runs on that thread through the game's task queue
        // (`game::execute`); `client` is `Minecraft.getInstance()` there.
        *RELICS_LOADED.get_mut() = Some(CompletableFuture::<()>::run_async(move || {
            // try (BufferedReader reader = client.getResourceManager().openAsReader(SkyblockerMod.id("spidersden/relics.json"))) {..}
            // catch (IOException e) {..}
            match compat::minecraft().get_resource_manager().open_as_reader(&crate::entry::skyblocker_mod::id("spidersden/relics.json")) {
                Err(e) => logger().error(&format!("[Skyblocker] Failed to load relics locations {}", e)),
                Ok(text) => {
                    // `JsonParser.parseReader(reader).getAsJsonObject()`: its unchecked exceptions are not caught.
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                    for (key, value) in compat::json::get_as_json_object(&json).iter() {
                        if key == "total" {
                            compat::set_state(&TOTAL_RELICS, compat::json::get_as_int(value));
                        } else if key == "locations" {
                            for location_json in compat::json::get_as_json_array(value).iter() {
                                let pos_data = compat::json::get_as_json_object(location_json);
                                let pos = BlockPos::new(compat::json::get_as_int_of(&pos_data, "x"), compat::json::get_as_int_of(&pos_data, "y"), compat::json::get_as_int_of(&pos_data, "z"));
                                let relic = Relic::new(pos.clone(), TYPE_SUPPLIER.get().clone(), ColorUtils::get_float_components_dye_color(&DyeColor::Yellow), ColorUtils::get_float_components_dye_color(&DyeColor::Brown)) as compat::Shared<dyn ProfileAwareWaypoint>;
                                RELICS.get_mut().insert(pos, relic);
                            }
                        }
                    }
                    logger().info("[Skyblocker] Loaded relics locations");
                }
            }
            // try (BufferedReader reader = Files.newBufferedReader(CONFIG_DIR.resolve("found_relics.json"))) {..}
            // catch (NoSuchFileException _) {} catch (IOException e) {..}
            match std::fs::read_to_string(skyblocker_mod::CONFIG_DIR.get_mut().join("found_relics.json")) {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => logger().error(&format!("[Skyblocker] Failed to load found relics {}", e)),
                Ok(text) => {
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                    for (profile, found_relics_json) in compat::json::get_as_json_object(&json).iter() {
                        for found_relic in compat::json::get_as_json_array(found_relics_json).iter() {
                            // `relics.get(pos).setFound(profile)`: an unknown position is null, and the call throws.
                            let relic = RELICS.get().get(&PosUtils::parse_pos_string(&compat::json::get_as_string(found_relic))).cloned().expect("NullPointerException");
                            ({ let __f = compat::dispatch(&relic, |__t| __t.set_found_1()); __f(&relic, profile.clone()) });
                        }
                    }
                    logger().debug("[Skyblocker] Loaded found relics");
                }
            }
        }, |task| crate::compat::execute(task)));
    }

    /// java: private static void saveFoundRelics(Minecraft client)
    pub fn save_found_relics(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let mut found_relics: std::collections::HashMap<String, compat::Shared<BlockPosSet>> = std::collections::HashMap::new();
        for relic in RELICS.get().values() {
            let (profiles, pos) = compat::read(relic, |__t| (__t.profile_aware_waypoint_base().found_profiles.clone(), __t.profile_aware_waypoint_base().pos.clone().unwrap()));
            for profile in profiles {
                let set = found_relics.entry(profile).or_insert_with(BlockPosSet::new).clone();
                BlockPosSet::add(&set, &pos);
            }
        }
        // try (BufferedWriter writer = Files.newBufferedWriter(CONFIG_DIR.resolve("found_relics.json"))) {..} catch (IOException e) {..}
        let mut json: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        for (profile, found_relics_for_profile) in found_relics.iter() {
            let mut found_relics_json: Vec<serde_json::Value> = Vec::new();
            for found_relic in BlockPosSet::iterate_mut(found_relics_for_profile) {
                found_relics_json.push(serde_json::Value::String(PosUtils::get_pos_string(&found_relic.immutable())));
            }
            json.insert(profile.clone(), serde_json::Value::Array(found_relics_json));
        }
        // `GSON.toJson(json, writer)` (pretty printing: two-space indentation).
        match std::fs::write(skyblocker_mod::CONFIG_DIR.get_mut().join("found_relics.json"), serde_json::to_string_pretty(&serde_json::Value::Object(json)).unwrap()) {
            Ok(()) => logger().debug("[Skyblocker] Saved found relics"),
            Err(e) => logger().error(&format!("[Skyblocker] Failed to write found relics to file {}", e)),
        }
    }

    /// java: private static void registerCommands(CommandDispatcher<FabricClientCommandSource> dispatcher, CommandBuildContext registryAccess)
    /// (`registryAccess` is the port's `Option`: absent before the first join)
    pub fn register_commands(dispatcher: &CommandDispatcher<client_command_registration_callback::FabricClientCommandSource>, registry_access: Option<&CommandBuildContext>) {
        let _ = registry_access;
        // java: dispatcher.register(literal(SkyblockerMod.NAMESPACE).then(literal("relics")
        //   .then(literal("markAllFound").executes(context -> { relics.values().forEach(ProfileAwareWaypoint::setFound);
        //     context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.relics.markAllFound"))); return 1; }))
        //   .then(literal("markAllMissing").executes(context -> { relics.values().forEach(ProfileAwareWaypoint::setMissing);
        //     context.getSource().sendFeedback(Constants.PREFIX.get().append(Component.translatable("skyblocker.relics.markAllMissing"))); return 1; }))));
        let mut mark_all_found = client_command_registration_callback::literal("markAllFound");
        mark_all_found.executes(Arc::new(|_context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
            let relics: Vec<compat::Shared<dyn ProfileAwareWaypoint>> = RELICS.get().values().cloned().collect();
            for relic in relics.iter() {
                ({ let __f = compat::dispatch(relic, |__t| __t.set_found()); __f(&compat::view(relic, |a| a as compat::Shared<dyn Waypoint>)) });
            }
            compat::batch_w_send_feedback(&(constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.relics.markAllFound")));
            Ok(1)
        }));
        let mut mark_all_missing = client_command_registration_callback::literal("markAllMissing");
        mark_all_missing.executes(Arc::new(|_context: &CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
            let relics: Vec<compat::Shared<dyn ProfileAwareWaypoint>> = RELICS.get().values().cloned().collect();
            for relic in relics.iter() {
                ({ let __f = compat::dispatch(relic, |__t| __t.set_missing()); __f(&compat::view(relic, |a| a as compat::Shared<dyn Waypoint>)) });
            }
            compat::batch_w_send_feedback(&(constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable("skyblocker.relics.markAllMissing")));
            Ok(1)
        }));
        let mut relics = client_command_registration_callback::literal("relics");
        relics.then(&mark_all_found);
        relics.then(&mark_all_missing);
        let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
        root.then(&relics);
        dispatcher.register(&root);
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
        // java: relics.values().stream().filter(Waypoint::shouldRender)
        //   .min(Comparator.comparingDouble(relic -> relic.pos.distToCenterSqr(player.position())))
        //   .filter(relic -> relic.pos.distToCenterSqr(player.position()) <= 16).ifPresent(Waypoint::setFound);
        let player_position = mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap();
        let relics: Vec<compat::Shared<dyn ProfileAwareWaypoint>> = RELICS.get().values().cloned().collect();
        // (`Stream.min` keeps the first of equal elements: a strictly smaller distance replaces it)
        let mut closest: Option<(f64, compat::Shared<dyn ProfileAwareWaypoint>)> = None;
        for relic in relics {
            let should_render = ({ let __f = compat::dispatch(&relic, |__t| __t.should_render()); __f(&compat::view(&relic, |a| a as compat::Shared<dyn Waypoint>)) });
            if !should_render {
                continue;
            }
            let distance: f64 = compat::read(&relic, |__t| __t.profile_aware_waypoint_base().pos.as_ref().unwrap().dist_to_center_sqr(player_position.x, player_position.y, player_position.z));
            if closest.as_ref().is_none_or(|(closest_distance, _)| distance < *closest_distance) {
                closest = Some((distance, relic));
            }
        }
        if let Some((distance, relic)) = closest
            && distance <= 16 as f64
        {
            ({ let __f = compat::dispatch(&relic, |__t| __t.set_found()); __f(&compat::view(&relic, |a| a as compat::Shared<dyn Waypoint>)) });
        }
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
