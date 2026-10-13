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
//   java: public static final Codec<WaypointGroup> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name),
//   java: 			Codec.STRING.fieldOf("island").xmap(Location::from, Location::id).forGetter(WaypointGroup::island),
//   java: 			NamedWaypoint.CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints),
//   java: 			Codec.BOOL.lenientOptionalFieldOf("ordered", false).forGetter(WaypointGroup::ordered),
//   java: 			Codec.BOOL.lenientOptionalFieldOf("renderThroughWalls", true).forGetter(WaypointGroup::renderThroughWalls),
//   java: 			Waypoint.Type.CODEC.lenientOptionalFieldOf("waypointType", DEFAULT_TYPE).forGetter(WaypointGroup::waypointType)
//   java: 	).apply(instance, WaypointGroup::new));
/// java: public static final Codec<WaypointGroup> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name), Codec.STRING.fieldOf("island").xmap(
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<WaypointGroup>>> = compat::MainThread::new(|| {
    // port shape: `RecordCodecBuilder` builds a record of `Send + Sync` values; a waypoint handle is not one, so the record is
    // over the getters' values (the waypoint list carried in a render-thread cell, where Java reads and writes it) and
    // `xmap`ped to the group - `apply(instance, WaypointGroup::new)` on decode, the `forGetter`s on encode
    type Fields = (String, Location, compat::RenderThread<Vec<compat::Shared<dyn NamedWaypoint>>>, bool, bool, Type);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply6(
        // Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name)
        record_codec_builder::of(|o: &Fields| o.0.clone(), codec::field_of(codec::string_codec(), "name")),
        // Codec.STRING.fieldOf("island").xmap(Location::from, Location::id).forGetter(WaypointGroup::island)
        record_codec_builder::of(|o: &Fields| o.1, codec::field_of(codec::xmap(codec::string_codec(), |id: String| Location::from(&id), |island: &Location| island.id()), "island")),
        // NamedWaypoint.CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints)
        record_codec_builder::of(|o: &Fields| o.2.clone(), codec::field_of(waypoints_codec(named_waypoint::CODEC.get().clone()), "waypoints")),
        // Codec.BOOL.lenientOptionalFieldOf("ordered", false).forGetter(WaypointGroup::ordered)
        record_codec_builder::of(|o: &Fields| o.3, codec::lenient_optional_field_of(codec::bool_codec(), "ordered", false)),
        // Codec.BOOL.lenientOptionalFieldOf("renderThroughWalls", true).forGetter(WaypointGroup::renderThroughWalls)
        record_codec_builder::of(|o: &Fields| o.4, codec::lenient_optional_field_of(codec::bool_codec(), "renderThroughWalls", true)),
        // Waypoint.Type.CODEC.lenientOptionalFieldOf("waypointType", DEFAULT_TYPE).forGetter(WaypointGroup::waypointType)
        record_codec_builder::of(|o: &Fields| o.5, codec::lenient_optional_field_of(crate::de::hysky::skyblocker::utils::waypoint::waypoint::TYPE_CODEC.get().clone(), "waypointType", *DEFAULT_TYPE)),
        |name, island, waypoints, ordered, render_through_walls, waypoint_type| (name, island, waypoints, ordered, render_through_walls, waypoint_type),
    ));
    codec::xmap(
        record,
        |(name, island, waypoints, ordered, render_through_walls, waypoint_type): Fields| WaypointGroup::new_6(name, island, waypoints.get().clone(), ordered, render_through_walls, waypoint_type),
        |group: &compat::Shared<WaypointGroup>| (WaypointGroup::name(group), WaypointGroup::island(group), compat::RenderThread::new(WaypointGroup::waypoints(group)), WaypointGroup::ordered(group), WaypointGroup::render_through_walls(group), WaypointGroup::waypoint_type(group)),
    )
});
//   java: public static final Codec<WaypointGroup> SKYTILS_CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name),
//   java: 			Codec.STRING.fieldOf("island").xmap(Location::from, Location::id).forGetter(WaypointGroup::island),
//   java: 			NamedWaypoint.SKYTILS_CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints)
//   java: 	).apply(instance, WaypointGroup::new));
/// java: public static final Codec<WaypointGroup> SKYTILS_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name), Codec.STRING.fieldOf("island
pub static SKYTILS_CODEC: compat::MainThread<CodecRef<compat::Shared<WaypointGroup>>> = compat::MainThread::new(|| {
    // port shape: the record is over the getters' values, `xmap`ped to the group (see `CODEC`)
    type Fields = (String, Location, compat::RenderThread<Vec<compat::Shared<dyn NamedWaypoint>>>);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply3(
        record_codec_builder::of(|o: &Fields| o.0.clone(), codec::field_of(codec::string_codec(), "name")),
        // Codec.STRING.fieldOf("island").xmap(Location::from, Location::id).forGetter(WaypointGroup::island)
        record_codec_builder::of(|o: &Fields| o.1, codec::field_of(codec::xmap(codec::string_codec(), |id: String| Location::from(&id), |island: &Location| island.id()), "island")),
        // NamedWaypoint.SKYTILS_CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints)
        record_codec_builder::of(|o: &Fields| o.2.clone(), codec::field_of(waypoints_codec(named_waypoint::SKYTILS_CODEC.get().clone()), "waypoints")),
        |name, island, waypoints| (name, island, waypoints),
    ));
    codec::xmap(
        record,
        // WaypointGroup::new - (String name, Location island, List<NamedWaypoint> waypoints)
        |(name, island, waypoints): Fields| WaypointGroup::new_3(name, island, waypoints.get().clone()),
        |group: &compat::Shared<WaypointGroup>| (WaypointGroup::name(group), WaypointGroup::island(group), compat::RenderThread::new(WaypointGroup::waypoints(group))),
    )
});
//   java: public static final Codec<WaypointGroup> COLEWEIGHT_CODEC = NamedWaypoint.COLEWEIGHT_CODEC.listOf().xmap(coleWeightWaypoints -> new WaypointGroup("Coleweight", Location.UNKNOWN, coleWeightWaypoints, true, true, DEFAULT_TYPE), WaypointGroup::waypoints);
/// java: public static final Codec<WaypointGroup> COLEWEIGHT_CODEC = NamedWaypoint.COLEWEIGHT_CODEC.listOf().xmap(coleWeightWaypoints -> new WaypointGroup("Coleweight", Location.UNKNOWN, coleWeightWaypoints, t
pub static COLEWEIGHT_CODEC: compat::MainThread<CodecRef<compat::Shared<WaypointGroup>>> = compat::MainThread::new(|| codec::xmap(
    // NamedWaypoint.COLEWEIGHT_CODEC.listOf().xmap(coleWeightWaypoints -> new WaypointGroup("Coleweight", Location.UNKNOWN, coleWeightWaypoints, true, true, DEFAULT_TYPE), WaypointGroup::waypoints)
    codec::list(named_waypoint::COLEWEIGHT_CODEC.get().clone()),
    |cole_weight_waypoints: Vec<compat::Shared<dyn NamedWaypoint>>| WaypointGroup::new_6("Coleweight".to_string(), Location::Unknown, cole_weight_waypoints, true, true, *DEFAULT_TYPE),
    |group: &compat::Shared<WaypointGroup>| WaypointGroup::waypoints(group),
));
//   java: public static final Codec<WaypointGroup> SKYBLOCKER_LEGACY_ORDERED_CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name),
//   java: 			Codec.BOOL.fieldOf("enabled").forGetter(group -> !group.waypoints().isEmpty() && group.waypoints().stream().allMatch(Waypoint::isEnabled)),
//   java: 			NamedWaypoint.SKYBLOCKER_LEGACY_ORDERED_CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints)
//   java: 	).apply(instance, (name, enabled, waypoints) -> {
//   java: 		waypoints.forEach(enabled ? Waypoint::setMissing : Waypoint::setFound);
//   java: 		return new WaypointGroup(name, Location.UNKNOWN, waypoints, true, true, DEFAULT_TYPE);
//   java: 	}));
/// java: public static final Codec<WaypointGroup> SKYBLOCKER_LEGACY_ORDERED_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name), Codec.BOOL
pub static SKYBLOCKER_LEGACY_ORDERED_CODEC: compat::MainThread<CodecRef<compat::Shared<WaypointGroup>>> = compat::MainThread::new(|| {
    // port shape: the record is over the getters' values, `xmap`ped to the group (see `CODEC`)
    type Fields = (String, bool, compat::RenderThread<Vec<compat::Shared<dyn NamedWaypoint>>>);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply3(
        // Codec.STRING.fieldOf("name").forGetter(WaypointGroup::name)
        record_codec_builder::of(|o: &Fields| o.0.clone(), codec::field_of(codec::string_codec(), "name")),
        // Codec.BOOL.fieldOf("enabled").forGetter(group -> !group.waypoints().isEmpty() && group.waypoints().stream().allMatch(Waypoint::isEnabled))
        record_codec_builder::of(|o: &Fields| o.1, codec::field_of(codec::bool_codec(), "enabled")),
        // NamedWaypoint.SKYBLOCKER_LEGACY_ORDERED_CODEC.listOf().fieldOf("waypoints").forGetter(WaypointGroup::waypoints)
        record_codec_builder::of(|o: &Fields| o.2.clone(), codec::field_of(waypoints_codec(named_waypoint::SKYBLOCKER_LEGACY_ORDERED_CODEC.get().clone()), "waypoints")),
        |name, enabled, waypoints| (name, enabled, waypoints),
    ));
    codec::xmap(
        record,
        // (name, enabled, waypoints) -> { waypoints.forEach(enabled ? Waypoint::setMissing : Waypoint::setFound);
        //   return new WaypointGroup(name, Location.UNKNOWN, waypoints, true, true, DEFAULT_TYPE); }
        |(name, enabled, waypoints): Fields| {
            let waypoints: Vec<compat::Shared<dyn NamedWaypoint>> = waypoints.get().clone();
            for waypoint in waypoints.iter() {
                let waypoint = compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>);
                if enabled {
                    let __f = compat::dispatch(&*waypoint, |__t| __t.set_missing());
                    __f(&waypoint);
                } else {
                    let __f = compat::dispatch(&*waypoint, |__t| __t.set_found());
                    __f(&waypoint);
                }
            }
            WaypointGroup::new_6(name, Location::Unknown, waypoints, true, true, *DEFAULT_TYPE)
        },
        |group: &compat::Shared<WaypointGroup>| {
            let waypoints: Vec<compat::Shared<dyn NamedWaypoint>> = WaypointGroup::waypoints(group);
            let enabled: bool = !waypoints.is_empty() && waypoints.iter().all(|w| compat::read(w, |t| t.waypoint_base().enabled));
            (WaypointGroup::name(group), enabled, compat::RenderThread::new(waypoints))
        },
    )
});

/// `elementCodec.listOf()` for the waypoint list of a group record: the list in a render-thread cell, which is the
/// `Send + Sync` value a record field must be (the waypoints themselves are not `Send`; Java reads and writes them on
/// the render thread, where every waypoint file is loaded and saved).
fn waypoints_codec(element_codec: CodecRef<compat::Shared<dyn NamedWaypoint>>) -> CodecRef<compat::RenderThread<Vec<compat::Shared<dyn NamedWaypoint>>>> {
    codec::xmap(
        codec::list(element_codec),
        |waypoints: Vec<compat::Shared<dyn NamedWaypoint>>| compat::RenderThread::new(waypoints),
        |waypoints: &compat::RenderThread<Vec<compat::Shared<dyn NamedWaypoint>>>| waypoints.get().clone(),
    )
}

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
        // java: this.waypoints = waypoints.stream().map(this::convertWaypoint).collect(Collectors.toList());
        { let __v = waypoints.iter().map(|__a0| WaypointGroup::convert_waypoint(&__this, __a0)).collect::<Vec<_>>(); compat::write(&__this, |__t| __t.waypoints = __v); }
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
        // java: return new WaypointGroup(name, island, waypoints.stream().filter(predicate).toList(), ordered, renderThroughWalls, waypointType);
        let waypoints: Vec<compat::Shared<dyn NamedWaypoint>> = compat::read(__this, |__t| __t.waypoints.clone()).into_iter().filter(|w| (predicate)(w.clone())).collect();
        return WaypointGroup::new_6(compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.island.clone()).unwrap(), waypoints, compat::read(__this, |__t| __t.ordered), compat::read(__this, |__t| __t.render_through_walls), compat::read(__this, |__t| __t.waypoint_type.clone()).unwrap());
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
        let mut name: String = format!("Waypoint {}", (compat::read(__this, |__t| __t.waypoints.len()) as i32).wrapping_add(1));
        // java: return (ordered ? new OrderedNamedWaypoint(pos, name, new float[]{0f, 1f, 0f}) : new NamedWaypoint(pos, name, new float[]{0f, 1f, 0f}))
        //           .withThroughWalls(renderThroughWalls).withTypeSupplier(this::waypointType);
        let waypoint: compat::Shared<dyn NamedWaypoint> = if compat::read(__this, |__t| __t.ordered) {
            OrderedNamedWaypoint::new_3(pos.clone(), name.clone(), vec![0.0f32, 1.0f32, 0.0f32]) as compat::Shared<dyn NamedWaypoint>
        } else {
            NamedWaypointBase::new_3(pos.clone(), name.clone(), vec![0.0f32, 1.0f32, 0.0f32]) as compat::Shared<dyn NamedWaypoint>
        };
        return WaypointGroup::with_group_settings(__this, &waypoint);
    }

    /// java: public NamedWaypoint convertWaypoint(NamedWaypoint waypoint)
    pub fn convert_waypoint(__this: &compat::Shared<Self>, waypoint: &compat::Shared<dyn NamedWaypoint>) -> compat::Shared<dyn NamedWaypoint> {
        let is_ordered_waypoint: bool = compat::read(waypoint, |__t| __t.dyn_any().is::<OrderedNamedWaypoint>());
        if compat::read(__this, |__t| __t.ordered) {
            // java: return (waypoint instanceof OrderedNamedWaypoint ? waypoint : new OrderedNamedWaypoint(waypoint)).withThroughWalls(renderThroughWalls()).withTypeSupplier(this::waypointType);
            let converted: compat::Shared<dyn NamedWaypoint> = if is_ordered_waypoint { waypoint.clone() } else { OrderedNamedWaypoint::new(waypoint.clone()) as compat::Shared<dyn NamedWaypoint> };
            return WaypointGroup::with_group_settings(__this, &converted);
        } else {
            // java: return (waypoint instanceof OrderedNamedWaypoint ? new NamedWaypoint(waypoint) : waypoint).withThroughWalls(renderThroughWalls()).withTypeSupplier(this::waypointType);
            let converted: compat::Shared<dyn NamedWaypoint> = if is_ordered_waypoint { NamedWaypointBase::new(waypoint.clone()) as compat::Shared<dyn NamedWaypoint> } else { waypoint.clone() };
            return WaypointGroup::with_group_settings(__this, &converted);
        }
    }

    /// `.withThroughWalls(renderThroughWalls()).withTypeSupplier(this::waypointType)` on a named waypoint: both are
    /// covariant overrides in Java (NamedWaypoint / OrderedNamedWaypoint answer their own class), which the port's
    /// `Waypoint` slots answer as `dyn Waypoint` - so the answer is taken back as the named waypoint it is.
    fn with_group_settings(__this: &compat::Shared<Self>, waypoint: &compat::Shared<dyn NamedWaypoint>) -> compat::Shared<dyn NamedWaypoint> {
        let render_through_walls: bool = WaypointGroup::render_through_walls(__this);
        let __this_1 = compat::RenderThread::new(__this.clone());
        let as_waypoint = compat::view(waypoint, |a| a as compat::Shared<dyn Waypoint>);
        let with_walls: compat::Shared<dyn Waypoint> = ({ let __f = compat::dispatch(&*as_waypoint, |__t| __t.with_through_walls()); __f(&as_waypoint, render_through_walls) });
        let with_type: compat::Shared<dyn Waypoint> = ({ let __f = compat::dispatch(&with_walls, |__t| __t.with_type_supplier()); __f(&with_walls, Arc::new(move || WaypointGroup::waypoint_type(__this_1.get()))) });
        return as_named_waypoint(&with_type);
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
            // java: if (waypoint instanceof OrderedNamedWaypoint orderedNamedWaypoint)
            if compat::read(&waypoint, |__t| __t.dyn_any().is::<OrderedNamedWaypoint>()) {
                let ordered_named_waypoint: compat::Shared<OrderedNamedWaypoint> = compat::handle_as::<OrderedNamedWaypoint, _>(&waypoint);
                let ordered_named_waypoint = &ordered_named_waypoint;
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
    /// The body the build's `ObjectProcessor` generates (`java.lang.runtime.ObjectMethods`' equals, as a record's):
    /// the same object, or a `WaypointGroup` whose every non-static, non-transient field is equal - `name`, `island`,
    /// `waypoints` (`List.equals`: the same size, each pair `NamedWaypoint.equals`), `ordered`, `renderThroughWalls`,
    /// `waypointType`; `currentIndex` is `transient`.
    pub fn equals(&self, o: &WaypointGroup) -> bool {
        if std::ptr::eq(self, o) {
            return true;
        }
        self.name == o.name
            && self.island == o.island
            && self.waypoints.len() == o.waypoints.len()
            && self.waypoints.iter().zip(o.waypoints.iter()).all(|(a, b)| <dyn NamedWaypoint>::equals_shared(a, b))
            && self.ordered == o.ordered
            && self.render_through_walls == o.render_through_walls
            && self.waypoint_type == o.waypoint_type
    }

    /// java: `a.equals(b)` on two group references (`List.indexOf`, `Multimap.remove(key, value)`, `List.equals`)
    pub fn equals_shared(a: &compat::Shared<WaypointGroup>, b: &compat::Shared<WaypointGroup>) -> bool {
        // read borrows only: a converted waypoint's type supplier reads its group (`this::waypointType`)
        compat::read(a, |a| compat::read(b, |b| a.equals(b)))
    }

    /// java: @Override @GenHashCode public native int hashCode();
    /// The body the build's `ObjectProcessor` generates (`ObjectMethods`' hashCode): `31 * h + hash(field)` from 0
    /// over the same fields. `Location` and `Waypoint.Type` are enums, whose `hashCode` is the JVM's identity hash -
    /// the port's own value stands in for it.
    pub fn hash_code(&self) -> i32 {
        // `List.hashCode()`: `31 * h + e.hashCode()` from 1, each element's own `hashCode()`
        let waypoints: i32 = self.waypoints.iter().fold(1i32, |h, w| {
            let w = compat::view(w, |a| a as compat::Shared<dyn Waypoint>);
            let __f = compat::dispatch(&*w, |__t| __t.hash_code());
            h.wrapping_mul(31).wrapping_add(__f(&w))
        });
        let boolean_hash_code = |b: bool| if b { 1231 } else { 1237 };
        [
            compat::string_hash_code(&self.name),
            self.island.as_ref().map_or(0, |island| compat::hash_code(island)),
            waypoints,
            boolean_hash_code(self.ordered),
            boolean_hash_code(self.render_through_walls),
            self.waypoint_type.as_ref().map_or(0, |waypoint_type| compat::hash_code(waypoint_type)),
        ]
        .iter()
        .fold(0i32, |h, e| h.wrapping_mul(31).wrapping_add(*e))
    }

    /// java: @Override @GenToString public native String toString();
    /// The body the build's `ObjectProcessor` generates (`ObjectMethods`' toString):
    /// `WaypointGroup[name=.., island=.., waypoints=[..], ordered=.., renderThroughWalls=.., waypointType=..]`.
    /// `island` prints as `Location.toString()` (its friendly name), `waypointType` as `Type.toString()` (translated),
    /// and each waypoint as `Object.toString()` (`NamedWaypoint` overrides none: the class name, `@`, and its
    /// `hashCode()` in hex).
    pub fn to_string(&self) -> String {
        let waypoints: Vec<String> = self.waypoints.iter().map(|w| {
            let class_name = compat::read(w, |t| {
                if t.dyn_any().is::<OrderedNamedWaypoint>() {
                    "de.hysky.skyblocker.utils.waypoint.OrderedNamedWaypoint"
                } else if t.dyn_any().is::<crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypointBase>() {
                    "de.hysky.skyblocker.utils.waypoint.DistancedNamedWaypoint"
                } else {
                    "de.hysky.skyblocker.utils.waypoint.NamedWaypoint"
                }
            });
            let w = compat::view(w, |a| a as compat::Shared<dyn Waypoint>);
            let __f = compat::dispatch(&*w, |__t| __t.hash_code());
            format!("{}@{:x}", class_name, __f(&w) as u32)
        }).collect();
        format!(
            "WaypointGroup[name={}, island={}, waypoints=[{}], ordered={}, renderThroughWalls={}, waypointType={}]",
            self.name,
            compat::opt_str(&self.island),
            waypoints.join(", "),
            self.ordered,
            self.render_through_walls,
            compat::opt_str(&self.waypoint_type),
        )
    }

}

impl std::fmt::Display for WaypointGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", WaypointGroup::to_string(self))
    }
}

/// `(NamedWaypoint) waypoint`: a waypoint the port's `Waypoint` slots hand back as `dyn Waypoint` while Java's static
/// type is `NamedWaypoint` - the same object, seen as its named waypoint class (Java's checked cast).
pub fn as_named_waypoint(waypoint: &compat::Shared<dyn Waypoint>) -> compat::Shared<dyn NamedWaypoint> {
    if compat::read(waypoint, |__t| __t.dyn_any().is::<OrderedNamedWaypoint>()) {
        return compat::handle_as::<OrderedNamedWaypoint, _>(waypoint) as compat::Shared<dyn NamedWaypoint>;
    }
    if compat::read(waypoint, |__t| __t.dyn_any().is::<NamedWaypointBase>()) {
        return compat::handle_as::<NamedWaypointBase, _>(waypoint) as compat::Shared<dyn NamedWaypoint>;
    }
    if compat::read(waypoint, |__t| __t.dyn_any().is::<crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypointBase>()) {
        return compat::handle_as::<crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypointBase, _>(waypoint) as compat::Shared<dyn NamedWaypoint>;
    }
    panic!("ClassCastException: {} is not a NamedWaypoint", compat::read(waypoint, |__t| std::any::type_name_of_val(__t.dyn_any())));
}
