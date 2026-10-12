//! Ported from de.hysky.skyblocker.utils.waypoint.WaypointGroup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::Waypoints;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::location::Location;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::ordered_named_waypoint::OrderedNamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::ordered_named_waypoint::RelativeIndex;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::entity::item_owner::ItemOwner as _;

/// java: public static final Waypoint.Type DEFAULT_TYPE = Waypoint.Type.WAYPOINT;
pub static DEFAULT_TYPE: LazyLock<Type> = LazyLock::new(|| Type::Waypoint);
// TODO(fabric2seer): `.id(..)` on a S
//   java: public static final Codec<WaypointGroup> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name),
//   java: 			Codec.STRING.fieldOf("island").xmap(Location::from, Location::id).forGetter(WaypointGroup::island),
//   java: 			NamedWaypoint.CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints),
//   java: 			Codec.BOOL.lenientOptionalFieldOf("ordered", false).forGetter(WaypointGroup::ordered),
//   java: 			Codec.BOOL.lenientOptionalFieldOf("renderThroughWalls", true).forGetter(WaypointGroup::renderThroughWalls),
//   java: 			Waypoint.Type.CODEC.lenientOptionalFieldOf("waypointType", DEFAULT_TYPE).forGetter(WaypointGroup::waypointType)
//   java: 	).apply(instance, WaypointGroup::new));
/// java: public static final Codec<WaypointGroup> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name), Codec.STRING.fieldOf("island").xmap(
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<WaypointGroup>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
// TODO(fabric2seer): `.id(..)` on a S
//   java: public static final Codec<WaypointGroup> SKYTILS_CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name),
//   java: 			Codec.STRING.fieldOf("island").xmap(Location::from, Location::id).forGetter(WaypointGroup::island),
//   java: 			NamedWaypoint.SKYTILS_CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints)
//   java: 	).apply(instance, WaypointGroup::new));
/// java: public static final Codec<WaypointGroup> SKYTILS_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name), Codec.STRING.fieldOf("island
pub static SKYTILS_CODEC: compat::MainThread<CodecRef<compat::Shared<WaypointGroup>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `SKYTILS_CODEC` - see the TODO above"));
// TODO(fabric2seer): `.waypoints(..)` on a S
//   java: public static final Codec<WaypointGroup> COLEWEIGHT_CODEC = NamedWaypoint.COLEWEIGHT_CODEC.listOf().xmap(coleWeightWaypoints -> new WaypointGroup("Coleweight", Location.UNKNOWN, coleWeightWaypoints, true, true, DEFAULT_TYPE), WaypointGroup::waypoints);
/// java: public static final Codec<WaypointGroup> COLEWEIGHT_CODEC = NamedWaypoint.COLEWEIGHT_CODEC.listOf().xmap(coleWeightWaypoints -> new WaypointGroup("Coleweight", Location.UNKNOWN, coleWeightWaypoints, t
pub static COLEWEIGHT_CODEC: compat::MainThread<CodecRef<compat::Shared<WaypointGroup>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `COLEWEIGHT_CODEC` - see the TODO above"));
// TODO(fabric2seer): `.waypoints(..)` on a value of unknown type
//   java: public static final Codec<WaypointGroup> SKYBLOCKER_LEGACY_ORDERED_CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name),
//   java: 			Codec.BOOL.fieldOf("enabled").forGetter(group -> !group.waypoints().isEmpty() && group.waypoints().stream().allMatch(Waypoint::isEnabled)),
//   java: 			NamedWaypoint.SKYBLOCKER_LEGACY_ORDERED_CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints)
//   java: 	).apply(instance, (name, enabled, waypoints) -> {
//   java: 		waypoints.forEach(enabled ? Waypoint::setMissing : Waypoint::setFound);
//   java: 		return new WaypointGroup(name, Location.UNKNOWN, waypoints, true, true, DEFAULT_TYPE);
//   java: 	}));
/// java: public static final Codec<WaypointGroup> SKYBLOCKER_LEGACY_ORDERED_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name), Codec.BOOL
pub static SKYBLOCKER_LEGACY_ORDERED_CODEC: compat::MainThread<CodecRef<compat::Shared<WaypointGroup>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `SKYBLOCKER_LEGACY_ORDERED_CODEC` - see the TODO above"));

/// java: public class WaypointGroup
#[derive(Clone)]
pub struct WaypointGroup {
    /// java: private final String name;
    pub name: String,
    /// java: private final Location island;
    pub island: Option<Location>,
    /// java: private final List<NamedWaypoint> waypoints;
    pub waypoints: Vec<compat::Shared<dyn NamedWaypoint>>,
    /// java: private final boolean ordered;
    pub ordered: bool,
    /// java: private final boolean renderThroughWalls;
    pub render_through_walls: bool,
    /// java: private final Waypoint.Type waypointType;
    pub waypoint_type: Option<Type>,
    /// java: private transient int currentIndex = 0;
    pub current_index: i32,
}

impl WaypointGroup {
    /// java: public WaypointGroup(String name, Location island)
    pub fn new(mut name: String, mut island: Location) -> compat::Shared<Self> {
        let __this = Self::new_6(name.clone(), island, vec![], false, true, SkyblockerConfigManager::get().ui_and_visuals.waypoints.waypoint_type);
        __this
    }

    /// java: public WaypointGroup(String name, Location island, List<NamedWaypoint> waypoints)
    pub fn new_3(mut name: String, mut island: Location, mut waypoints: Vec<compat::Shared<dyn NamedWaypoint>>) -> compat::Shared<Self> {
        let __this = Self::new_6(name.clone(), island, waypoints.clone(), false, true, *DEFAULT_TYPE);
        __this
    }

    /// java: public WaypointGroup(String name, Location island, List<NamedWaypoint> waypoints, boolean ordered, boolean renderThroughWalls, Waypoint.Type waypointType)
    pub fn new_6(mut name: String, mut island: Location, mut waypoints: Vec<compat::Shared<dyn NamedWaypoint>>, mut ordered: bool, mut render_through_walls: bool, mut waypoint_type: Type) -> compat::Shared<Self> {
        let __init_name = name.clone();
        let __init_island = Some(island);
        let __init_ordered = ordered;
        let __init_render_through_walls = render_through_walls;
        let __init_waypoint_type = Some(waypoint_type);
        let __this = compat::shared(Self {
            name: __init_name,
            island: __init_island,
            waypoints: Default::default(),
            ordered: __init_ordered,
            render_through_walls: __init_render_through_walls,
            waypoint_type: __init_waypoint_type,
            current_index: 0,
        });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = waypoints.iter().cloned().map(|__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; WaypointGroup::convert_waypoint(&__this_1, &__a0) }).collect::<Vec<_>>(); compat::write(&__this, |__t| __t.waypoints = __v); }
        __this
    }

    /// java: public String name()
    pub fn name(__this: &compat::Shared<Self>) -> String {
        return compat::read(__this, |__t| __t.name.clone());
    }

    /// java: public Location island()
    pub fn island(__this: &compat::Shared<Self>) -> Location {
        return compat::read(__this, |__t| __t.island.clone()).unwrap();
    }

    /// java: public List<NamedWaypoint> waypoints()
    pub fn waypoints(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn NamedWaypoint>> {
        return compat::read(__this, |__t| __t.waypoints.clone());
    }

    /// java: public boolean ordered()
    pub fn ordered(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.ordered);
    }

    /// java: public boolean renderThroughWalls()
    pub fn render_through_walls(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.render_through_walls);
    }

    /// java: public Waypoint.Type waypointType()
    pub fn waypoint_type(__this: &compat::Shared<Self>) -> Type {
        return compat::read(__this, |__t| __t.waypoint_type.clone()).unwrap();
    }

    /// java: public int currentIndex()
    pub fn current_index(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.current_index);
    }

    /// java: public void setCurrentIndex(int currentIndex)
    pub fn set_current_index(__this: &compat::Shared<Self>, mut current_index: i32) {
        { let __v = current_index; compat::write(__this, |__t| __t.current_index = __v); }
    }

    /// java: public void resetCurrentIndex()
    pub fn reset_current_index(__this: &compat::Shared<Self>) {
        WaypointGroup::set_current_index(__this, 0);
    }

    /// java: public WaypointGroup withName(String name)
    pub fn with_name(__this: &compat::Shared<Self>, name: &str) -> compat::Shared<WaypointGroup> {
        return WaypointGroup::new_6(name.to_string(), compat::read(__this, |__t| __t.island.clone()).unwrap(), compat::read(__this, |__t| __t.waypoints.clone()), compat::read(__this, |__t| __t.ordered), compat::read(__this, |__t| __t.render_through_walls), compat::read(__this, |__t| __t.waypoint_type.clone()).unwrap());
    }

    /// java: public WaypointGroup withIsland(Location island)
    pub fn with_island(__this: &compat::Shared<Self>, mut island: Location) -> compat::Shared<WaypointGroup> {
        return WaypointGroup::new_6(compat::read(__this, |__t| __t.name.clone()), island, compat::read(__this, |__t| __t.waypoints.clone()), compat::read(__this, |__t| __t.ordered), compat::read(__this, |__t| __t.render_through_walls), compat::read(__this, |__t| __t.waypoint_type.clone()).unwrap());
    }

    /// java: public WaypointGroup withOrdered(boolean ordered)
    pub fn with_ordered(__this: &compat::Shared<Self>, mut ordered: bool) -> compat::Shared<WaypointGroup> {
        return WaypointGroup::new_6(compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.island.clone()).unwrap(), compat::read(__this, |__t| __t.waypoints.clone()), ordered, compat::read(__this, |__t| __t.render_through_walls), compat::read(__this, |__t| __t.waypoint_type.clone()).unwrap());
    }

    /// java: public WaypointGroup withRenderThroughWalls(boolean renderThroughWalls)
    pub fn with_render_through_walls(__this: &compat::Shared<Self>, mut render_through_walls: bool) -> compat::Shared<WaypointGroup> {
        return WaypointGroup::new_6(compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.island.clone()).unwrap(), compat::read(__this, |__t| __t.waypoints.clone()), compat::read(__this, |__t| __t.ordered), render_through_walls, compat::read(__this, |__t| __t.waypoint_type.clone()).unwrap());
    }

    /// java: public WaypointGroup withWaypointType(Waypoint.Type waypointType)
    pub fn with_waypoint_type(__this: &compat::Shared<Self>, mut waypoint_type: Type) -> compat::Shared<WaypointGroup> {
        return WaypointGroup::new_6(compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.island.clone()).unwrap(), compat::read(__this, |__t| __t.waypoints.clone()), compat::read(__this, |__t| __t.ordered), compat::read(__this, |__t| __t.render_through_walls), waypoint_type);
    }

    /// java: public WaypointGroup filterWaypoints(Predicate<NamedWaypoint> predicate)
    pub fn filter_waypoints(__this: &compat::Shared<Self>, predicate: Arc<dyn Fn(compat::Shared<dyn NamedWaypoint>) -> bool>) -> compat::Shared<WaypointGroup> {
        // TODO(fabric2seer): not a lambda
        //   java: return new WaypointGroup(name, island, waypoints.stream().filter(predicate).toList(), ordered, renderThroughWalls, waypointType);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public WaypointGroup sortWaypoints(Comparator<NamedWaypoint> comparator)
    pub fn sort_waypoints(__this: &compat::Shared<Self>, comparator: Arc<dyn Fn(&compat::Shared<dyn NamedWaypoint>, &compat::Shared<dyn NamedWaypoint>) -> std::cmp::Ordering>) -> compat::Shared<WaypointGroup> {
        return WaypointGroup::new_6(compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.island.clone()).unwrap(), compat::sorted_by(compat::read(__this, |__t| __t.waypoints.clone()).into_iter(), |a, b| (comparator)(a, b)).collect::<Vec<_>>(), compat::read(__this, |__t| __t.ordered), compat::read(__this, |__t| __t.render_through_walls), compat::read(__this, |__t| __t.waypoint_type.clone()).unwrap());
    }

    /// java: public WaypointGroup deepCopy()
    pub fn deep_copy(__this: &compat::Shared<Self>) -> compat::Shared<WaypointGroup> {
        return WaypointGroup::new_6(compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.island.clone()).unwrap(), compat::read(__this, |__t| __t.waypoints.clone()).into_iter().map(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.copy()); __f(&__a0) })).collect::<Vec<_>>(), compat::read(__this, |__t| __t.ordered), compat::read(__this, |__t| __t.render_through_walls), compat::read(__this, |__t| __t.waypoint_type.clone()).unwrap());
    }

    /// java: public NamedWaypoint createWaypoint(BlockPos pos)
    pub fn create_waypoint(__this: &compat::Shared<Self>, pos: &BlockPos) -> compat::Shared<dyn NamedWaypoint> {
        let mut name: String = format!("Waypoint {}", (__this.lock().borrow().waypoints.len() as i32).wrapping_add(1));
        let __this_1 = compat::RenderThread::new(__this.clone());
        return (({ let __r = &({ let __r = &(if compat::read(__this, |__t| __t.ordered) { OrderedNamedWaypoint::new_3(pos.clone(), name.clone(), vec![0.0f32, 1.0f32, 0.0f32]) } else { (NamedWaypointBase::new_3(pos.clone(), name.clone(), vec![0.0f32, 1.0f32, 0.0f32]) as compat::Shared<dyn NamedWaypoint>) }); let __f = compat::dispatch(__r, |__t| __t.with_through_walls()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), compat::read(__this, |__t| __t.render_through_walls)) }); let __f = compat::dispatch(__r, |__t| __t.with_type_supplier()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), Arc::new(move || WaypointGroup::waypoint_type(__this_1.get()))) }) as compat::Shared<dyn NamedWaypoint>);
    }

    /// java: public NamedWaypoint convertWaypoint(NamedWaypoint waypoint)
    pub fn convert_waypoint(__this: &compat::Shared<Self>, waypoint: &compat::Shared<dyn NamedWaypoint>) -> compat::Shared<dyn NamedWaypoint> {
        if compat::read(__this, |__t| __t.ordered) {
            let __this_1 = compat::RenderThread::new(__this.clone());
            return ({ let __r = &({ let __r = &(if waypoint.lock().borrow().as_any().is::<compat::Shared<OrderedNamedWaypoint>>() { waypoint.clone() } else { OrderedNamedWaypoint::new(waypoint.clone()) }); let __f = compat::dispatch(__r, |__t| __t.with_through_walls()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), WaypointGroup::render_through_walls(__this)) }); let __f = compat::dispatch(__r, |__t| __t.with_type_supplier()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), Arc::new(move || WaypointGroup::waypoint_type(__this_1.get()))) });
        } else {
            let __this_2 = compat::RenderThread::new(__this.clone());
            return ({ let __r = &({ let __r = &(if waypoint.lock().borrow().as_any().is::<compat::Shared<OrderedNamedWaypoint>>() { (NamedWaypointBase::new(waypoint.clone()) as compat::Shared<dyn NamedWaypoint>) } else { waypoint.clone() }); let __f = compat::dispatch(__r, |__t| __t.with_through_walls()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), WaypointGroup::render_through_walls(__this)) }); let __f = compat::dispatch(__r, |__t| __t.with_type_supplier()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Waypoint>), Arc::new(move || WaypointGroup::waypoint_type(__this_2.get()))) });
        }
    }

    /// java: public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(compat::read(__this, |__t| __t.ordered)) || __this.lock().borrow().waypoints.is_empty() {
            return;
        }
        for i in 0..(__this.lock().borrow().waypoints.len() as i32) {
            let mut waypoint: compat::Shared<dyn NamedWaypoint> = __this.lock().borrow().waypoints[(i) as usize].clone();
            let config = &SkyblockerConfigManager::get().ui_and_visuals.waypoints;
            let mut next_index: i32 = ((compat::read(__this, |__t| __t.current_index)).wrapping_add(1)).wrapping_rem(__this.lock().borrow().waypoints.len() as i32);
            let mut previous_index: i32 = (((compat::read(__this, |__t| __t.current_index)).wrapping_sub(1)).wrapping_add(__this.lock().borrow().waypoints.len() as i32)).wrapping_rem(__this.lock().borrow().waypoints.len() as i32);
            let mut not_backwards: bool = (config.allow_going_backwards || i == next_index);
            let mut not_skipping: bool = (config.allow_skipping_waypoints || i == next_index || i == previous_index);
            if not_backwards && not_skipping && { let (__q0, __q1,) = (&mcrust::mcrust::modding::game::with_player(|player| player.position()).unwrap(), (config.waypoint_activation_radius) as f64,); waypoint.lock().borrow().named_waypoint_base().pos.as_ref().unwrap().closer_to_center_than(__q0, __q1) } {
                { let __v = i; compat::write(__this, |__t| __t.current_index = __v); }
            }
        }
        let mut previous_index: i32 = (((compat::read(__this, |__t| __t.current_index)).wrapping_sub(1)).wrapping_add(__this.lock().borrow().waypoints.len() as i32)).wrapping_rem(__this.lock().borrow().waypoints.len() as i32);
        let mut next_index: i32 = ((compat::read(__this, |__t| __t.current_index)).wrapping_add(1)).wrapping_rem(__this.lock().borrow().waypoints.len() as i32);
        for i in 0..(__this.lock().borrow().waypoints.len() as i32) {
            let mut waypoint: compat::Shared<dyn NamedWaypoint> = __this.lock().borrow().waypoints[(i) as usize].clone();
            if let Some(ordered_named_waypoint) = waypoint.lock().borrow().as_any().downcast_ref::<compat::Shared<OrderedNamedWaypoint>>() {
                { let __v = i; compat::write(ordered_named_waypoint, |__t| __t.index = __v); }
                if i == previous_index {
                    { let __v = RelativeIndex::Previous; compat::write(ordered_named_waypoint, |__t| __t.relative_index = __v); }
                } else if i == next_index {
                    { let __v = RelativeIndex::Next; compat::write(ordered_named_waypoint, |__t| __t.relative_index = __v); }
                } else if i == compat::read(__this, |__t| __t.current_index) {
                    { let __v = RelativeIndex::Current; compat::write(ordered_named_waypoint, |__t| __t.relative_index = __v); }
                } else {
                    { let __v = RelativeIndex::None; compat::write(ordered_named_waypoint, |__t| __t.relative_index = __v); }
                }
            }
        }
    }

    /// java: public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        for waypoint in compat::read(__this, |__t| __t.waypoints.clone()) {
            if { let __f = compat::dispatch(&waypoint, |__t| __t.should_render()); __f(&compat::view(&waypoint, |a| a as compat::Shared<dyn Waypoint>)) } {
                ({ let __f = compat::dispatch(&waypoint, |__t| __t.extract_rendering()); __f(&compat::view(&waypoint, |a| a as compat::Shared<dyn Renderable>), collector) });
            }
        }
    }

    /// java: @Override @GenEquals public native boolean equals(Object o);
    pub fn equals(&self, o: &compat::Object) -> bool {
        todo!("native method: its body is generated at build time by an annotation processor (@GenEquals)")
    }

    /// java: @Override @GenHashCode public native int hashCode();
    pub fn hash_code(&self) -> i32 {
        todo!("native method: its body is generated at build time by an annotation processor (@GenHashCode)")
    }

    /// java: @Override @GenToString public native String toString();
    pub fn to_string(&self) -> String {
        todo!("native method: its body is generated at build time by an annotation processor (@GenToString)")
    }

}

impl std::fmt::Display for WaypointGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WaypointGroup::to_string(self))
    }
}
