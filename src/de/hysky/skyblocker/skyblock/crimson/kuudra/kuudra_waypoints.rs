//! Ported from de.hysky.skyblocker.skyblock.crimson.kuudra.KuudraWaypoints by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::crimson_isle_config::Kuudra;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra;
use crate::de::hysky::skyblocker::skyblock::crimson::kuudra::kuudra_phase::KuudraPhase;
use crate::de::hysky::skyblocker::utils::pos_utils;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::world::entity::monster::giant::Giant;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final float[] SUPPLIES_COLOR = {255f / 255f, 0f, 0f};
pub static SUPPLIES_COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![(255.0f32 / 255.0f32), 0.0f32, 0.0f32]);
/// java: private static final float[] PEARL_COLOR = {57f / 255f, 117f / 255f, 125f / 255f};
pub static PEARL_COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![(57.0f32 / 255.0f32), (117.0f32 / 255.0f32), (125.0f32 / 255.0f32)]);
/// java: private static final float[] SAFE_SPOT_COLOR = {255f / 255f, 85f / 255f, 255f / 255f};
pub static SAFE_SPOT_COLOR: LazyLock<Vec<f32>> = LazyLock::new(|| vec![(255.0f32 / 255.0f32), (85.0f32 / 255.0f32), (255.0f32 / 255.0f32)]);
/// java: private static final Supplier<Type> SUPPLIES_AND_FUEL_TYPE = () -> SkyblockerConfigManager.get().crimsonIsle.kuudra.suppliesAndFuelWaypointType;
pub static SUPPLIES_AND_FUEL_TYPE: compat::MainThread<Arc<dyn Fn() -> Type>> = compat::MainThread::new(|| Arc::new(move || compat::read(&SkyblockerConfigManager::get().crimson_isle.kuudra, |__t| __t.supplies_and_fuel_waypoint_type)));
/// java: private static final ObjectArrayList<Waypoint> SAFE_SPOT_WAYPOINTS = new ObjectArrayList<>();
pub static SAFE_SPOT_WAYPOINTS: compat::MainThread<Vec<compat::Shared<dyn Waypoint>>> = compat::MainThread::new(|| Vec::new());
/// java: private static final ObjectArrayList<Waypoint> PEARL_WAYPOINTS = new ObjectArrayList<>();
pub static PEARL_WAYPOINTS: compat::MainThread<Vec<compat::Shared<dyn Waypoint>>> = compat::MainThread::new(|| Vec::new());
// TODO(fabric2seer): field `pos` of a S
//   java: private static final Function<float[], Codec<List<Waypoint>>> CODEC = cc -> PosUtils.ALT_BLOCK_POS_CODEC.xmap(
//   java: 			pos -> new Waypoint(pos, () -> Waypoint.Type.HIGHLIGHT, cc, false),
//   java: 			waypoint -> waypoint.pos)
//   java: 			.listOf();
/// java: private static final Function<float[], Codec<List<Waypoint>>> CODEC = cc -> PosUtils.ALT_BLOCK_POS_CODEC.xmap( pos -> new Waypoint(pos, () -> Waypoint.Type.HIGHLIGHT, cc, false), waypoint -> waypoint.
pub static CODEC: compat::MainThread<Arc<dyn Fn(Vec<f32>) -> CodecRef<Vec<compat::Shared<dyn Waypoint>>>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: private static ObjectArrayList<Waypoint> supplyWaypoints = ObjectArrayList.of();
pub static SUPPLY_WAYPOINTS: compat::MainThread<Vec<compat::Shared<dyn Waypoint>>> = compat::MainThread::new(|| vec![]);
/// java: private static ObjectArrayList<Waypoint> ballistaBuildWaypoints = ObjectArrayList.of();
pub static BALLISTA_BUILD_WAYPOINTS: compat::MainThread<Vec<compat::Shared<dyn Waypoint>>> = compat::MainThread::new(|| vec![]);
/// java: private static ObjectArrayList<Waypoint> fuelWaypoints = ObjectArrayList.of();
pub static FUEL_WAYPOINTS: compat::MainThread<Vec<compat::Shared<dyn Waypoint>>> = compat::MainThread::new(|| vec![]);
/// java: private static boolean loaded;
pub static LOADED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class KuudraWaypoints
#[derive(Clone)]
pub struct KuudraWaypoints {}

impl KuudraWaypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        level_render_extraction_callback::EVENT.register(Arc::new(move |__a0| { KuudraWaypoints::extract_rendering(&__a0); }));
        // TODO(fabric2seer): no SeerClient event yet: hook RETURN of Minecraft's constructor (`net.minecraft.client.Minecraft#<init>`: exists)
        //   java: ClientLifecycleEvents.CLIENT_STARTED.register(KuudraWaypoints::load);
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { KuudraWaypoints::tick(); }), 20) });
    }

    /// java: private static void load(Minecraft client)
    pub fn load(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        let mut safe_spots: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = KuudraWaypoints::load_waypoints(client, &crate::entry::skyblocker_mod::id("crimson/kuudra/safe_spot_waypoints.json"), SAFE_SPOT_WAYPOINTS.get_mut(), &SAFE_SPOT_COLOR);
        let mut pearls: mcrust::java::util::concurrent::completable_future::CompletableFuture<()> = KuudraWaypoints::load_waypoints(client, &crate::entry::skyblocker_mod::id("crimson/kuudra/pearl_waypoints.json"), PEARL_WAYPOINTS.get_mut(), &PEARL_COLOR);
        // TODO(fabric2seer): an assignment used as a value
        //   java: CompletableFuture.allOf(safeSpots, pearls).whenComplete((_, _) -> loaded = true);
    }

    /// java: private static CompletableFuture<Void> loadWaypoints(Minecraft client, Identifier file, ObjectArrayList<Waypoint> list, float[] colorComponents)
    pub fn load_waypoints(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, file: &Identifier, list: &Vec<compat::Shared<dyn Waypoint>>, color_components: &Vec<f32>) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        return (CompletableFuture::supply_async(move || Ok((|| {
            {
                let mut reader: crate::unresolved::BufferedReader = client.get_resource_manager().open_as_reader(file).unwrap();
                // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    return (CODEC.get_mut())(color_components.clone()).parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&KuudraWaypoints::get_waypoints(&reader))).get_or_throw().unwrap();
                })) {
                    Ok(__v) => return __v,
                    Err(e) => {
                        logger().error(&format!("[Skyblocker Kuudra Waypoints] Failed to load kuudra waypoints from: {}", compat::jstring(&file)));
                        return vec![];
                    }
                }
            }
        })()), compat::executor)).then_accept_async(|__a0| list.extend(__a0.iter().cloned()), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)));
    }

    /// java: private static JsonElement getWaypoints(BufferedReader reader)
    pub fn get_waypoints(reader: &crate::unresolved::BufferedReader) -> serde_json::Value {
        // TODO(fabric2seer): static call JsonParser.parseReader(..)
        //   java: return JsonParser.parseReader(reader).getAsJsonObject().getAsJsonArray("waypoints");
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static void tick()
    pub fn tick() {
        let client = compat::minecraft();
        let mut config: compat::Shared<Kuudra> = SkyblockerConfigManager::get().crimson_isle.kuudra.clone();
        if Utils::is_in_kuudra() && compat::read(&config, |__t| __t.supply_waypoints) || compat::read(&config, |__t| __t.fuel_waypoints) || compat::read(&config, |__t| __t.ballista_build_waypoints) && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            let mut search_box: AABB = mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.get_bounding_box().inflate(500.0)).unwrap();
            let mut supplies: Vec<compat::Shared<dyn Waypoint>> = Vec::new();
            let mut fuel_cells: Vec<compat::Shared<dyn Waypoint>> = Vec::new();
            if compat::read(&config, |__t| __t.supply_waypoints) || compat::read(&config, |__t| __t.fuel_waypoints) && mcrust::mcrust::modding::game::level().is_some() {
                // TODO(fabric2seer): `.getY(..)` on a T
                //   java: List<Giant> giants = client.level.getEntitiesOfClass(Giant.class, searchBox, giant -> giant.getY() < 67);
                let mut giants: Vec<Giant> = todo!("fabric2seer: the initialiser of `giants` (TODO above)");
                for giant in giants.iter() {
                    let mut yaw_offset: f64 = ((giant.base.base.base.base.base.get_y_rot() + (115 as f32))) as f64;
                    let mut x: f64 = (giant.base.base.base.base.base.get_x() + (4.5 * (yaw_offset * (mcrust::net::minecraft::util::mth::DEG_TO_RAD as f64)).cos()));
                    let mut y: f64 = ((75) as f64) as f64;
                    let mut z: f64 = (giant.base.base.base.base.base.get_z() + (4.5 * (yaw_offset * (mcrust::net::minecraft::util::mth::DEG_TO_RAD as f64)).sin()));
                    let mut waypoint: compat::Shared<dyn Waypoint> = (WaypointBase::new6(BlockPos::containing(x, y, z), SUPPLIES_AND_FUEL_TYPE.get_mut().clone(), SUPPLIES_COLOR.clone(), false) as compat::Shared<dyn Waypoint>);
                    if compat::get_state(&kuudra::PHASE) == KuudraPhase::Dps {
                        fuel_cells.push(waypoint.clone());
                    } else {
                        supplies.push(waypoint.clone());
                    }
                }
            }
            let mut ballista_build_spots: Vec<compat::Shared<dyn Waypoint>> = Vec::new();
            if compat::read(&config, |__t| __t.ballista_build_waypoints) && mcrust::mcrust::modding::game::level().is_some() {
                // TODO(fabric2seer): `.hasCustomName(..)` on a T
                //   java: List<ArmorStand> armorStands = client.level.getEntitiesOfClass(ArmorStand.class, searchBox, ArmorStand::hasCustomName);
                let mut armor_stands: Vec<ArmorStand> = todo!("fabric2seer: the initialiser of `armorStands` (TODO above)");
                for armor_stand in armor_stands.iter() {
                    let mut name: String = compat::component_of(armor_stand.get_name()).get_string();
                    if compat::read(&config, |__t| __t.ballista_build_waypoints) && name.contains("SNEAK + PUNCH") {
                        ballista_build_spots.push((WaypointBase::new5(armor_stand.base.base.block_position(), Arc::new(move || Type::Waypoint), SUPPLIES_COLOR.clone(), false) as compat::Shared<dyn Waypoint>));
                    }
                }
            }
            *SUPPLY_WAYPOINTS.get_mut() = supplies.clone();
            *BALLISTA_BUILD_WAYPOINTS.get_mut() = ballista_build_spots.clone();
            *FUEL_WAYPOINTS.get_mut() = fuel_cells.clone();
        }
    }

    /// java: private static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        let mut config: compat::Shared<Kuudra> = SkyblockerConfigManager::get().crimson_isle.kuudra.clone();
        if Utils::is_in_kuudra() && compat::get_state(&LOADED) {
            if compat::read(&config, |__t| __t.supply_waypoints) {
                for waypoint in SUPPLY_WAYPOINTS.get_mut().iter() {
                    ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
            }
            if compat::read(&config, |__t| __t.ballista_build_waypoints) {
                for waypoint in BALLISTA_BUILD_WAYPOINTS.get_mut().iter() {
                    ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
            }
            if compat::read(&config, |__t| __t.fuel_waypoints) {
                for waypoint in FUEL_WAYPOINTS.get_mut().iter() {
                    ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
            }
            if compat::read(&config, |__t| __t.safe_spot_waypoints) {
                for waypoint in SAFE_SPOT_WAYPOINTS.get_mut().iter() {
                    ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
            }
            if compat::read(&config, |__t| __t.pearl_waypoints) {
                for waypoint in PEARL_WAYPOINTS.get_mut().iter() {
                    ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
                }
            }
        }
    }

}
