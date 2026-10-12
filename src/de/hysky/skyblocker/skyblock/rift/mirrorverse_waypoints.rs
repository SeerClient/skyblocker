//! Ported from de.hysky.skyblocker.skyblock.rift.MirrorverseWaypoints by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::area::TheRift;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::dye_color::DyeColor;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;

/// java: private static final Supplier<Waypoint.Type> WAYPOINT_TYPE = () -> Waypoint.Type.HIGHLIGHT;
pub static WAYPOINT_TYPE: compat::MainThread<Arc<dyn Fn() -> Type>> = compat::MainThread::new(|| Arc::new(move || Type::Highlight));
/// java: private static final Identifier WAYPOINTS_JSON = SkyblockerMod.id("rift/mirrorverse_waypoints.json");
pub static WAYPOINTS_JSON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("rift/mirrorverse_waypoints.json"));
/// java: private static Waypoint[] LAVA_PATH_WAYPOINTS;
pub static LAVA_PATH_WAYPOINTS: compat::MainThread<Vec<compat::Shared<dyn Waypoint>>> = compat::MainThread::new(|| Default::default());
/// java: private static Waypoint[] UPSIDE_DOWN_WAYPOINTS;
pub static UPSIDE_DOWN_WAYPOINTS: compat::MainThread<Vec<compat::Shared<dyn Waypoint>>> = compat::MainThread::new(|| Default::default());
/// java: private static Waypoint[] TURBULATOR_WAYPOINTS;
pub static TURBULATOR_WAYPOINTS: compat::MainThread<Vec<compat::Shared<dyn Waypoint>>> = compat::MainThread::new(|| Default::default());
/// java: private static final float[] COLOR_COMPONENTS = ColorUtils.getFloatComponents(DyeColor.RED);
pub static COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| ColorUtils::get_float_components_dye_color(&DyeColor::Red));
/// java: private static CompletableFuture<Void> waypointsLoaded;
pub static WAYPOINTS_LOADED: compat::MainThread<Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>> = compat::MainThread::new(|| None);

/// java: public class MirrorverseWaypoints
#[derive(Clone)]
pub struct MirrorverseWaypoints {}

impl MirrorverseWaypoints {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: static void load(Minecraft client)
    pub fn load(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        *WAYPOINTS_LOADED.get_mut() = Some(CompletableFuture::<()>::run_async(move || (|| {
            {
                let mut reader: crate::unresolved::BufferedReader = client.get_resource_manager().open_as_reader(WAYPOINTS_JSON.get_mut()).unwrap();
                // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // TODO(fabric2seer): static call JsonParser.parseReader(..)
                    //   java: JsonArray sections = JsonParser.parseReader(reader).getAsJsonObject().get("sections").getAsJsonArray();
                    let mut sections: Vec<serde_json::Value> = todo!("fabric2seer: the initialiser of `sections` (TODO above)");
                    *LAVA_PATH_WAYPOINTS.get_mut() = MirrorverseWaypoints::load_waypoints(&compat::json::get_as_json_array(&((compat::json::get_as_json_object(&(sections[(0) as usize]))).get("waypoints").cloned().unwrap())));
                    *UPSIDE_DOWN_WAYPOINTS.get_mut() = MirrorverseWaypoints::load_waypoints(&compat::json::get_as_json_array(&((compat::json::get_as_json_object(&(sections[(1) as usize]))).get("waypoints").cloned().unwrap())));
                    *TURBULATOR_WAYPOINTS.get_mut() = MirrorverseWaypoints::load_waypoints(&compat::json::get_as_json_array(&((compat::json::get_as_json_object(&(sections[(2) as usize]))).get("waypoints").cloned().unwrap())));
                }));
                if let Err(e) = __result {
                    logger().error(&format!("[Skyblocker] Mirrorverse Waypoints failed to load ;("));
                }
            }
        })(), compat::executor));
    }

    /// java: private static Waypoint[] loadWaypoints(JsonArray waypointsJson)
    pub fn load_waypoints(waypoints_json: &Vec<serde_json::Value>) -> Vec<compat::Shared<dyn Waypoint>> {
        let mut waypoints: Vec<compat::Shared<dyn Waypoint>> = (0..((waypoints_json.len() as i32)) as usize).map(|_| Default::default()).collect::<Vec<_>>();
        for i in 0..(waypoints_json.len() as i32) {
            let mut point: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(waypoints_json[(i) as usize]));
            waypoints[(i) as usize] = (WaypointBase::new6(BlockPos::new(compat::json::get_as_int(&point.get("x").cloned().unwrap()), compat::json::get_as_int(&point.get("y").cloned().unwrap()), compat::json::get_as_int(&point.get("z").cloned().unwrap())), WAYPOINT_TYPE.get_mut().clone(), COLOR_COMPONENTS.clone(), false) as compat::Shared<dyn Waypoint>);
        }
        return waypoints;
    }

    /// java: protected static void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(collector: &compat::Shared<dyn PrimitiveCollector>) {
        if Utils::is_in_the_rift() && compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<TheRift>().is_some_and(|__e| *__e == TheRift::Mirrorverse)) && SkyblockerConfigManager::get().other_locations.rift.mirrorverse_waypoints && WAYPOINTS_LOADED.get_mut().as_ref().unwrap().is_done() {
            for waypoint in LAVA_PATH_WAYPOINTS.get_mut().iter() {
                ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
            for waypoint in UPSIDE_DOWN_WAYPOINTS.get_mut().iter() {
                ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
            for waypoint in TURBULATOR_WAYPOINTS.get_mut().iter() {
                ({ let __f = compat::dispatch(waypoint, |__t| __t.extract_rendering()); __f(&compat::view(waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

}
