//! Ported from de.hysky.skyblocker.skyblock.dungeon.GoldorWaypointsManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::terminal_hud;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget::ElementBasedWidget;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;

/// java: private static final ObjectArrayList<GoldorWaypoint> TERMINALS = new ObjectArrayList<>();
pub static TERMINALS: compat::MainThread<Vec<compat::Shared<GoldorWaypoint>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final ObjectArrayList<GoldorWaypoint> DEVICES = new ObjectArrayList<>();
pub static DEVICES: compat::MainThread<Vec<compat::Shared<GoldorWaypoint>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final ObjectArrayList<GoldorWaypoint> LEVERS = new ObjectArrayList<>();
pub static LEVERS: compat::MainThread<Vec<compat::Shared<GoldorWaypoint>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final ObjectArrayList<GoldorWaypoint> ACTIVE_PHASE_WAYPOINTS = new ObjectArrayList<>();
pub static ACTIVE_PHASE_WAYPOINTS: compat::MainThread<Vec<compat::Shared<GoldorWaypoint>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final String TERMINALS_START = "[BOSS] Storm: I should have known that I stood no chance.";
pub const TERMINALS_START: &str = "[BOSS] Storm: I should have known that I stood no chance.";
/// java: private static final Pattern TERMINAL_ACTIVATED = Pattern.compile("^(?<name>\\w+) activated a terminal! \\(\\d/\\d\\)$");
pub static TERMINAL_ACTIVATED: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?<name>\\w+) activated a terminal! \\(\\d/\\d\\)$"));
/// java: private static final Pattern DEVICE_ACTIVATED = Pattern.compile("^(?<name>\\w+) completed a device! \\(\\d/\\d\\)$");
pub static DEVICE_ACTIVATED: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?<name>\\w+) completed a device! \\(\\d/\\d\\)$"));
/// java: private static final Pattern LEVER_ACTIVATED = Pattern.compile("^(?<name>\\w+) activated a lever! \\(\\d/\\d\\)$");
pub static LEVER_ACTIVATED: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?<name>\\w+) activated a lever! \\(\\d/\\d\\)$"));
/// java: private static final Pattern PHASE_COMPLETE = Pattern.compile("^(?<name>\\w+) (?:activated a (?:terminal|lever)|completed a device)! (?:\\(7/7\\)|\\(8/8\\))$");
pub static PHASE_COMPLETE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^(?<name>\\w+) (?:activated a (?:terminal|lever)|completed a device)! (?:\\(7/7\\)|\\(8/8\\))$"));
/// java: private static final String GATE_DESTROYED = "The gate has been destroyed!";
pub const GATE_DESTROYED: &str = "The gate has been destroyed!";
/// java: private static final String CORE_ENTRANCE = "The Core entrance is opening!";
pub const CORE_ENTRANCE: &str = "The Core entrance is opening!";
/// java: private static final Codec<List<GoldorWaypoint>> CODEC = GoldorWaypoint.CODEC.listOf();
pub static CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<GoldorWaypoint>>>> = compat::MainThread::new(|| codec::list(GOLDOR_WAYPOINT_CODEC.get_mut().clone()));
/// java: private static boolean loaded = false;
pub static LOADED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean active = false;
pub static ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static boolean gateDestroyed = false;
pub static GATE_DESTROYED_2: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
/// java: private static short currentPhase = 0;
pub static CURRENT_PHASE: LazyLock<Mutex<i16>> = LazyLock::new(|| Mutex::new((0) as i16));
// TODO(fabric2seer): field `kind` of a value of unknown type
//   java: public static final Codec<GoldorWaypoint> CODEC = RecordCodecBuilder.create(i -> i.group(
//   java: 				WaypointTargetKind.CODEC.fieldOf("kind").forGetter(w -> w.kind),
//   java: 				Codec.INT.fieldOf("phase").forGetter(customWaypoint -> customWaypoint.phase),
//   java: 				ComponentSerialization.CODEC.fieldOf("name").forGetter(NamedWaypoint::getName),
//   java: 				BlockPos.CODEC.fieldOf("pos").forGetter(customWaypoint -> customWaypoint.pos)
//   java: 		).apply(i, GoldorWaypoint::new));
/// java: public static final Codec<GoldorWaypoint> CODEC = RecordCodecBuilder.create(i -> i.group( WaypointTargetKind.CODEC.fieldOf("kind").forGetter(w -> w.kind), Codec.INT.fieldOf("phase").forGetter(customWa
pub static GOLDOR_WAYPOINT_CODEC: compat::MainThread<CodecRef<compat::Shared<GoldorWaypoint>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: private static final Supplier<Type> TYPE_SUPPLIER = () -> SkyblockerConfigManager.get().dungeons.goldor.waypointType;
pub static GOLDOR_WAYPOINT_TYPE_SUPPLIER: compat::MainThread<Arc<dyn Fn() -> Type>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.goldor.waypoint_type));
// TODO(fabric2seer): `from_values` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: private static final Codec<WaypointTargetKind> CODEC = StringRepresentable.fromValues(WaypointTargetKind::values);
/// java: private static final Codec<WaypointTargetKind> CODEC = StringRepresentable.fromValues(WaypointTargetKind::values);
pub static WAYPOINT_TARGET_KIND_CODEC: compat::MainThread<CodecRef<WaypointTargetKind>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public class GoldorWaypointsManager
#[derive(Clone)]
pub struct GoldorWaypointsManager {}

impl GoldorWaypointsManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { GoldorWaypointsManager::extract_rendering(&__a0); }));
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(GoldorWaypointsManager::load);
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(GoldorWaypointsManager::onChatMessage);
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            GoldorWaypointsManager::reset();
        }));
    }

    /// java: private static void load(Minecraft client)
    pub fn load(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        // TODO(fabric2seer): an assignment used as a value
        //   java: loadWaypoints(client, SkyblockerMod.id("dungeons/goldorwaypoints.json")).thenRun(() -> loaded = true);
    }

    /// java: private static CompletableFuture<Void> loadWaypoints(Minecraft client, Identifier file)
    pub fn load_waypoints(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, file: &Identifier) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        // TODO(fabric2seer): `.forEach(..)` on a value of unknown type
        //   java: return CompletableFuture.supplyAsync(() -> {
        //   java: 			try (BufferedReader reader = client.getResourceManager().openAsReader(file)) {
        //   java: 				JsonArray arr = JsonParser.parseReader(reader).getAsJsonArray();
        //   java:
        //   java: 				return CODEC.parse(JsonOps.INSTANCE, arr).getOrThrow();
        //   java: 			} catch (Exception e) {
        //   java: 				LOGGER.error("[Skyblocker Goldor Waypoints] Failed to load waypoints from: {}", file, e);
        //   java:
        //   java: 				return List.<GoldorWaypoint>of();
        //   java: 			}
        //   java: 		}, SkyblockerMod.VIRTUAL_THREAD_EXECUTOR).thenAcceptAsync(list -> list.forEach(waypoint -> {
        //   java: 			switch (waypoint.kind) {
        //   java: 				case TERMINAL -> TERMINALS.add(waypoint);
        //   java: 				case DEVICE -> DEVICES.add(waypoint);
        //   java: 				case LEVER -> LEVERS.add(waypoint);
        //   java: 			}
        //   java: 		}), client);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static boolean shouldProcess()
    pub fn should_process() -> bool {
        if !(compat::get_state(&LOADED)) || !(Utils::is_in_dungeons()) || !(DungeonManager::is_in_boss()) || !(DungeonManager::get_boss().is_floor(7)) {
            return false;
        }
        return (SkyblockerConfigManager::get().dungeons.goldor.enable_goldor_waypoints || compat::read(&SkyblockerConfigManager::get().dungeons.terminal_hud, |__t| __t.enable_terminal_hud));
    }

    /// java: private static void removeNearestWaypoint(List<GoldorWaypoint> waypoints, String playerName)
    pub fn remove_nearest_waypoint(waypoints: &Vec<compat::Shared<GoldorWaypoint>>, player_name: &str) {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::level().is_none() {
            return;
        }
        // client.level.players().stream().filter(player -> player.getGameProfile().name().equals(playerName)).findAny().map(Entity::position)
        let mut pos_optional: Option<Vec3> = {
            use mcrust::net::minecraft::world::entity::entity::Entity as _;
            let players = mcrust::mcrust::modding::game::level().unwrap().with_players(|players| players.to_vec());
            players.iter().find_map(|avatar| {
                if mcrust::mcrust::modding::game::is_held_here(&**avatar) {
                    return None;
                }
                let player = mcrust::mcrust::modding::game::lock_entity(&**avatar, "goldor_waypoints_manager.rs:GoldorWaypointsManager.removeNearestWaypoint");
                (player.get_game_profile_name() == player_name).then(|| player.base().position())
            })
        };
        if let Some(__a0) = pos_optional.clone().and_then(|pos| waypoints.iter().cloned().filter(|__a0| ({ let __f = compat::dispatch(__a0, |__t| __t.should_render()); __f(&compat::view(__a0, |a| a as compat::Shared<dyn Waypoint>)) })).min_by(|a, b| { let key = |waypoint: &compat::Shared<GoldorWaypoint>| waypoint.lock().borrow().center_pos.as_ref().unwrap().distance_to_sqr_vec(&pos); key(a).total_cmp(&key(b)) })) {
            ({ let __f = compat::dispatch(&__a0, |__t| __t.set_found()); __f(&compat::view(&__a0, |a| a as compat::Shared<dyn Waypoint>)) });
        }
        ({ let __r = terminal_hud::INSTANCE.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&ACTIVE, false);
        compat::set_state(&GATE_DESTROYED_2, false);
        compat::set_state(&CURRENT_PHASE, (0) as i16);
        GoldorWaypointsManager::enable_all(TERMINALS.get_mut());
        GoldorWaypointsManager::enable_all(DEVICES.get_mut());
        GoldorWaypointsManager::enable_all(LEVERS.get_mut());
        ACTIVE_PHASE_WAYPOINTS.get_mut().clear();
    }

    /// java: private static void enableAll(ObjectArrayList<GoldorWaypoint> waypoints)
    pub fn enable_all(waypoints: &Vec<compat::Shared<GoldorWaypoint>>) {
        for __a0 in waypoints.iter() {
            ({ let __f = compat::dispatch(__a0, |__t| __t.set_missing()); __f(&compat::view(__a0, |a| a as compat::Shared<dyn Waypoint>)) });
        }
    }

    /// java: private static @Nullable String getPlayerName(Matcher matcher)
    pub fn get_player_name(matcher: &mut compat::regex::Matcher) -> Option<String> {
        return (if matcher.matches() { matcher.group_named("name") } else { None });
    }

    /// java: @SuppressWarnings("SameReturnValue") private static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if overlay || !(GoldorWaypointsManager::should_process()) {
            return true;
        }
        let mut message: String = text.get_string();
        if compat::get_state(&ACTIVE) {
            if PHASE_COMPLETE.matcher(&message).matches() {
                compat::set_state(&CURRENT_PHASE, compat::get_state(&CURRENT_PHASE) + 1);
                compat::set_state(&GATE_DESTROYED_2, false);
                GoldorWaypointsManager::set_phase_waypoints();
                ({ let __r = terminal_hud::INSTANCE.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
            } else {
                if let Some(player_name) = GoldorWaypointsManager::get_player_name(&mut TERMINAL_ACTIVATED.matcher(&message)) {
                    GoldorWaypointsManager::remove_nearest_waypoint(TERMINALS.get_mut(), &player_name);
                } else if let Some(player_name) = GoldorWaypointsManager::get_player_name(&mut DEVICE_ACTIVATED.matcher(&message)) {
                    GoldorWaypointsManager::remove_nearest_waypoint(DEVICES.get_mut(), &player_name);
                } else if let Some(player_name) = GoldorWaypointsManager::get_player_name(&mut LEVER_ACTIVATED.matcher(&message)) {
                    GoldorWaypointsManager::remove_nearest_waypoint(LEVERS.get_mut(), &player_name);
                } else if message == CORE_ENTRANCE {
                    compat::set_state(&ACTIVE, false);
                    ACTIVE_PHASE_WAYPOINTS.get_mut().clear();
                } else if message == GATE_DESTROYED {
                    compat::set_state(&GATE_DESTROYED_2, true);
                    ({ let __r = terminal_hud::INSTANCE.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
                }
            }
        } else if message == TERMINALS_START {
            GoldorWaypointsManager::reset();
            GoldorWaypointsManager::set_phase_waypoints();
            compat::set_state(&ACTIVE, true);
            ({ let __r = terminal_hud::INSTANCE.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.update()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ElementBasedWidget>)) });
        }
        return true;
    }

    /// java: private static void setPhaseWaypoints()
    pub fn set_phase_waypoints() {
        ACTIVE_PHASE_WAYPOINTS.get_mut().clear();
        ACTIVE_PHASE_WAYPOINTS.get_mut().extend(TERMINALS.get_mut().iter().cloned().filter(|waypoint| (compat::read(waypoint, |__t| __t.phase) == (compat::get_state(&CURRENT_PHASE) as i32))).collect::<Vec<_>>().into_iter());
        ACTIVE_PHASE_WAYPOINTS.get_mut().extend(DEVICES.get_mut().iter().cloned().filter(|waypoint| (compat::read(waypoint, |__t| __t.phase) == (compat::get_state(&CURRENT_PHASE) as i32))).collect::<Vec<_>>().into_iter());
        ACTIVE_PHASE_WAYPOINTS.get_mut().extend(LEVERS.get_mut().iter().cloned().filter(|waypoint| (compat::read(waypoint, |__t| __t.phase) == (compat::get_state(&CURRENT_PHASE) as i32))).collect::<Vec<_>>().into_iter());
    }

    /// java: private static void extractRenderingForWaypoints(PrimitiveCollector collector, ObjectArrayList<GoldorWaypoint> waypoints)
    pub fn extract_rendering_for_waypoints(collector: &compat::Shared<dyn PrimitiveCollector>, waypoints: &Vec<compat::Shared<GoldorWaypoint>>) {
        for waypoint in waypoints.iter() {
            if { let __f = compat::dispatch(waypoint, |__t| __t.should_render()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>)) } {
                ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if compat::get_state(&ACTIVE) && SkyblockerConfigManager::get().dungeons.goldor.enable_goldor_waypoints {
            GoldorWaypointsManager::extract_rendering_for_waypoints(collector, ACTIVE_PHASE_WAYPOINTS.get_mut());
        }
    }

    /// java: public static boolean isActive()
    pub fn is_active() -> bool {
        return compat::get_state(&ACTIVE);
    }

    /// java: public static boolean isGateDestroyed()
    pub fn is_gate_destroyed() -> bool {
        return compat::get_state(&GATE_DESTROYED_2);
    }

    /// java: public static short getCurrentPhase()
    pub fn get_current_phase() -> i16 {
        return compat::get_state(&CURRENT_PHASE);
    }

    /// java: public static List<GoldorWaypoint> getPhaseWaypoints()
    pub fn get_phase_waypoints() -> Vec<compat::Shared<GoldorWaypoint>> {
        return ACTIVE_PHASE_WAYPOINTS.get_mut().clone();
    }

}

/// java: public static class GoldorWaypoint extends NamedWaypoint
#[derive(Clone)]
pub struct GoldorWaypoint {
    /// java: extends NamedWaypoint
    pub base: NamedWaypointBase,
    /// java: final WaypointTargetKind kind;
    pub kind: WaypointTargetKind,
    /// java: final int phase;
    pub phase: i32,
}

impl std::ops::Deref for GoldorWaypoint {
    type Target = NamedWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GoldorWaypoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GoldorWaypoint {
    /// java: GoldorWaypoint(WaypointTargetKind kind, int phase, Component name, BlockPos pos)
    pub fn new(mut kind: WaypointTargetKind, mut phase: i32, mut name: MutableComponent, mut pos: BlockPos) -> compat::Shared<Self> {
        let __init_kind = kind;
        let __init_phase = phase;
        let (_, __this) = NamedWaypointBase::new_6_in(pos.clone(), name.clone(), GOLDOR_WAYPOINT_TYPE_SUPPLIER.get_mut().clone(), kind.fields().color_components.clone(), 0.25f32, true, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                kind: __init_kind,
                phase: __init_phase,
            });
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        });
        __this
    }

}

impl NamedWaypoint for GoldorWaypoint {
    fn named_waypoint_base(&self) -> &NamedWaypointBase {
        &self.base
    }
    fn named_waypoint_base_mut(&mut self) -> &mut NamedWaypointBase {
        &mut self.base
    }

}

impl Waypoint for GoldorWaypoint {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base
    }

    /// java: @Override public NamedWaypoint withX(int x) (dispatch: the body for this class)
    fn with_x(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_x_from::<Self>
    }

    /// java: @Override public NamedWaypoint withY(int y) (dispatch: the body for this class)
    fn with_y(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_y_from::<Self>
    }

    /// java: @Override public NamedWaypoint withZ(int z) (dispatch: the body for this class)
    fn with_z(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_z_from::<Self>
    }

    /// java: @Override public NamedWaypoint withColor(float[] colorComponents, float alpha) (dispatch: the body for this class)
    fn with_color(&self) -> fn(&compat::Shared<dyn Waypoint>, &Vec<f32>, f32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_color_from::<Self>
    }

    /// java: @Override public NamedWaypoint withThroughWalls(boolean throughWalls) (dispatch: the body for this class)
    fn with_through_walls(&self) -> fn(&compat::Shared<dyn Waypoint>, bool) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_through_walls_from::<Self>
    }

    /// java: @Override public NamedWaypoint withTypeSupplier(Supplier<Type> typeSupplier) (dispatch: the body for this class)
    fn with_type_supplier(&self) -> fn(&compat::Shared<dyn Waypoint>, Arc<dyn Fn() -> Type>) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_type_supplier_from::<Self>
    }

    /// java: @Override public int hashCode() (dispatch: the body for this class)
    fn hash_code(&self) -> fn(&compat::Shared<dyn Waypoint>) -> i32 {
        <dyn NamedWaypoint>::hash_code_from::<Self>
    }

    /// java: @Override public boolean equals(Object obj) (dispatch: the body for this class)
    fn equals(&self) -> fn(&compat::Shared<dyn Waypoint>, &compat::Shared<dyn Waypoint>) -> bool {
        <dyn NamedWaypoint>::equals_from::<Self>
    }

    /// java: `instanceof NamedWaypoint` - this class is one
    fn as_named_waypoint(&self) -> Option<&crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase> {
        Some(self.named_waypoint_base())
    }

}

impl Renderable for GoldorWaypoint {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn NamedWaypoint>::extract_rendering_from::<Self>
    }

}

/// java: enum WaypointTargetKind implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WaypointTargetKind {
    Terminal,
    Device,
    Lever,
}

impl WaypointTargetKind {
    pub const VALUES: [WaypointTargetKind; 3] = [WaypointTargetKind::Terminal, WaypointTargetKind::Device, WaypointTargetKind::Lever];

    /// java: values()
    pub fn values() -> Vec<WaypointTargetKind> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            WaypointTargetKind::Terminal => "TERMINAL",
            WaypointTargetKind::Device => "DEVICE",
            WaypointTargetKind::Lever => "LEVER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> WaypointTargetKind {
        match name {
            "TERMINAL" => WaypointTargetKind::Terminal,
            "DEVICE" => WaypointTargetKind::Device,
            "LEVER" => WaypointTargetKind::Lever,
            _ => panic!("IllegalArgumentException: No enum constant WaypointTargetKind.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static WaypointTargetKindFields {
        static FIELDS: LazyLock<Vec<WaypointTargetKindFields>> = LazyLock::new(|| vec![
            WaypointTargetKindFields::new(0, 255, 0),
            WaypointTargetKindFields::new(0, 0, 255),
            WaypointTargetKindFields::new(255, 255, 0),
        ]);
        &FIELDS[self as usize]
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name().toLowerCase(Locale.ENGLISH);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for WaypointTargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each WaypointTargetKind constant
pub struct WaypointTargetKindFields {
    pub color_components: Vec<f32>,
}

impl WaypointTargetKindFields {
    fn new(mut r: i32, mut g: i32, mut b: i32) -> Self {
        let mut this = Self {
            color_components: Default::default(),
        };
        this.color_components = vec![((r as f32) / 255.0f32), ((g as f32) / 255.0f32), ((b as f32) / 255.0f32)];
        this
    }

}
