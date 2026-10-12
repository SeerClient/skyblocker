//! Ported from de.hysky.skyblocker.utils.waypoint.NamedWaypoint by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::color_utils::ColorUtils;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::net::minecraft::network::chat::component::Component as _;

//   java: public static final Codec<NamedWaypoint> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			BlockPos.CODEC.fieldOf("pos").forGetter(waypoint -> waypoint.pos),
//   java: 			ComponentSerialization.CODEC.fieldOf("name").forGetter(waypoint -> waypoint.name),
//   java: 			Codec.floatRange(0, 1).listOf().comapFlatMap(
//   java: 					colorComponentsList -> colorComponentsList.size() == 3 ? DataResult.success(Floats.toArray(colorComponentsList)) : DataResult.error(() -> "Expected 3 color components, got " + colorComponentsList.size() + " instead"),
//   java: 					Floats::asList
//   java: 			).fieldOf("colorComponents").forGetter(waypoint -> waypoint.colorComponents),
//   java: 			Codec.FLOAT.fieldOf("alpha").forGetter(waypoint -> waypoint.alpha),
//   java: 			Codec.BOOL.fieldOf("shouldRender").forGetter(Waypoint::isEnabled)
//   java: 	).apply(instance, NamedWaypoint::new));
/// java: public static final Codec<NamedWaypoint> CODEC = RecordCodecBuilder.create(instance -> instance.group( BlockPos.CODEC.fieldOf("pos").forGetter(waypoint -> waypoint.pos), ComponentSerialization.CODEC.f
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<dyn NamedWaypoint>>> = compat::MainThread::new(|| {
    // rust: `RecordCodecBuilder` builds a record of `Send + Sync` values and a waypoint handle is not one: the record is
    // over the five getters' values, `xmap`ped to the waypoint - `apply(instance, NamedWaypoint::new)` on decode, the
    // `forGetter`s on encode
    type Fields = (BlockPos, MutableComponent, Vec<f32>, f32, bool);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply5(
        // BlockPos.CODEC.fieldOf("pos").forGetter(waypoint -> waypoint.pos)
        record_codec_builder::of(|o: &Fields| o.0.clone(), codec::field_of(mcrust::net::minecraft::core::block_pos::codec(), "pos")),
        // ComponentSerialization.CODEC.fieldOf("name").forGetter(waypoint -> waypoint.name)
        record_codec_builder::of(|o: &Fields| o.1.clone(), codec::field_of(mcrust::net::minecraft::network::chat::component_serialization::component_codec(), "name")),
        // Codec.floatRange(0, 1).listOf().comapFlatMap(colorComponentsList -> colorComponentsList.size() == 3 ? DataResult.success(Floats.toArray(colorComponentsList))
        //   : DataResult.error(() -> "Expected 3 color components, got " + colorComponentsList.size() + " instead"), Floats::asList).fieldOf("colorComponents")
        record_codec_builder::of(|o: &Fields| o.2.clone(), codec::field_of(codec::comap_flat_map(
            codec::list(codec::float_range(0.0, 1.0)),
            |color_components_list: Vec<f32>| if color_components_list.len() == 3 {
                mcrust::com::mojang::serialization::data_result::DataResult::success(color_components_list)
            } else {
                let size = color_components_list.len();
                mcrust::com::mojang::serialization::data_result::DataResult::error(Arc::new(move || format!("Expected 3 color components, got {} instead", size)))
            },
            |color_components: &Vec<f32>| color_components.clone(),
        ), "colorComponents")),
        // Codec.FLOAT.fieldOf("alpha").forGetter(waypoint -> waypoint.alpha)
        record_codec_builder::of(|o: &Fields| o.3, codec::field_of(codec::float_codec(), "alpha")),
        // Codec.BOOL.fieldOf("shouldRender").forGetter(Waypoint::isEnabled)
        record_codec_builder::of(|o: &Fields| o.4, codec::field_of(codec::bool_codec(), "shouldRender")),
        |pos, name, color_components, alpha, enabled| (pos, name, color_components, alpha, enabled),
    ));
    codec::xmap(
        record,
        // NamedWaypoint::new - (BlockPos pos, Component name, float[] colorComponents, float alpha, boolean enabled)
        |(pos, name, color_components, alpha, enabled): Fields| NamedWaypointBase::new6(pos, name, color_components, alpha, enabled) as compat::Shared<dyn NamedWaypoint>,
        |waypoint: &compat::Shared<dyn NamedWaypoint>| compat::read(waypoint, |t| {
            let w = t.named_waypoint_base();
            (w.pos.clone().unwrap(), w.name.clone().unwrap(), w.color_components.clone(), w.alpha, w.enabled)
        }),
    )
});
//   java: public static final Codec<NamedWaypoint> SKYTILS_CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.INT.fieldOf("x").forGetter(waypoint -> waypoint.pos.getX()),
//   java: 			Codec.INT.fieldOf("y").forGetter(waypoint -> waypoint.pos.getY()),
//   java: 			Codec.INT.fieldOf("z").forGetter(waypoint -> waypoint.pos.getZ()),
//   java: 			Codec.either(Codec.STRING, Codec.INT).xmap(either -> either.map(Function.identity(), Object::toString), Either::left).fieldOf("name").forGetter(waypoint -> waypoint.name.getString()),
//   java: 			Codec.INT.optionalFieldOf("color", ARGB.color(128, 0, 255, 0)).forGetter(waypoint -> (int) (waypoint.alpha * 255) << 24 | (int) (waypoint.colorComponents[0] * 255) << 16 | (int) (waypoint.colorComponents[1] * 255) << 8 | (int) (waypoint.colorComponents[2] * 255)),
//   java: 			Codec.BOOL.fieldOf("enabled").forGetter(Waypoint::isEnabled)
//   java: 	).apply(instance, NamedWaypoint::fromSkytils));
/// java: public static final Codec<NamedWaypoint> SKYTILS_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("x").forGetter(waypoint -> waypoint.pos.getX()), Codec.INT.fieldOf("y")
pub static SKYTILS_CODEC: compat::MainThread<CodecRef<compat::Shared<dyn NamedWaypoint>>> = compat::MainThread::new(|| {
    // rust: the record is over the getters' values, `xmap`ped to the waypoint (see `CODEC`)
    type Fields = (i32, i32, i32, String, i32, bool);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply6(
        // Codec.INT.fieldOf("x").forGetter(waypoint -> waypoint.pos.getX()) (and y, z)
        record_codec_builder::of(|o: &Fields| o.0, codec::field_of(codec::int_codec(), "x")),
        record_codec_builder::of(|o: &Fields| o.1, codec::field_of(codec::int_codec(), "y")),
        record_codec_builder::of(|o: &Fields| o.2, codec::field_of(codec::int_codec(), "z")),
        // Codec.either(Codec.STRING, Codec.INT).xmap(either -> either.map(Function.identity(), Object::toString), Either::left).fieldOf("name")
        //   .forGetter(waypoint -> waypoint.name.getString())
        record_codec_builder::of(|o: &Fields| o.3.clone(), codec::field_of(codec::xmap(
            codec::either(codec::string_codec(), codec::int_codec()),
            |either: mcrust::com::mojang::datafixers::util::either::Either<String, i32>| either.map(|s| s, |i| i.to_string()),
            |name: &String| mcrust::com::mojang::datafixers::util::either::Either::left(name.clone()),
        ), "name")),
        // Codec.INT.optionalFieldOf("color", ARGB.color(128, 0, 255, 0)).forGetter(waypoint -> (int) (waypoint.alpha * 255) << 24
        //   | (int) (waypoint.colorComponents[0] * 255) << 16 | (int) (waypoint.colorComponents[1] * 255) << 8 | (int) (waypoint.colorComponents[2] * 255))
        record_codec_builder::of(|o: &Fields| o.4, codec::optional_field_of(codec::int_codec(), "color", mcrust::net::minecraft::util::argb::color_argb(128, 0, 255, 0))),
        // Codec.BOOL.fieldOf("enabled").forGetter(Waypoint::isEnabled)
        record_codec_builder::of(|o: &Fields| o.5, codec::field_of(codec::bool_codec(), "enabled")),
        |x, y, z, name, color, enabled| (x, y, z, name, color, enabled),
    ));
    codec::xmap(
        record,
        // NamedWaypoint::fromSkytils
        |(x, y, z, name, color, enabled): Fields| NamedWaypointBase::from_skytils(x, y, z, &name, color, enabled),
        |waypoint: &compat::Shared<dyn NamedWaypoint>| compat::read(waypoint, |t| {
            let w = t.named_waypoint_base();
            let pos = w.pos.clone().unwrap();
            let c = &w.color_components;
            let color: i32 = ((w.alpha * 255.0) as i32) << 24 | ((c[0] * 255.0) as i32) << 16 | ((c[1] * 255.0) as i32) << 8 | ((c[2] * 255.0) as i32);
            (pos.get_x(), pos.get_y(), pos.get_z(), w.name.as_ref().unwrap().get_string(), color, w.enabled)
        }),
    )
});
//   java: static final Codec<NamedWaypoint> COLEWEIGHT_CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.INT.fieldOf("x").forGetter(waypoint -> waypoint.pos.getX()),
//   java: 			Codec.INT.fieldOf("y").forGetter(waypoint -> waypoint.pos.getY()),
//   java: 			Codec.INT.fieldOf("z").forGetter(waypoint -> waypoint.pos.getZ()),
//   java: 			CodecUtils.optionalDouble(Codec.DOUBLE.optionalFieldOf("r")).forGetter(waypoint -> OptionalDouble.of(waypoint.colorComponents[0])),
//   java: 			CodecUtils.optionalDouble(Codec.DOUBLE.optionalFieldOf("g")).forGetter(waypoint -> OptionalDouble.of(waypoint.colorComponents[1])),
//   java: 			CodecUtils.optionalDouble(Codec.DOUBLE.optionalFieldOf("b")).forGetter(waypoint -> OptionalDouble.of(waypoint.colorComponents[2])),
//   java: 			ColeweightOptions.CODEC.optionalFieldOf("options").forGetter(waypoint -> Optional.of(new ColeweightOptions(Optional.of(waypoint.name.getString()))))
//   java: 	).apply(instance, NamedWaypoint::fromColeweight));
/// java: static final Codec<NamedWaypoint> COLEWEIGHT_CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("x").forGetter(waypoint -> waypoint.pos.getX()), Codec.INT.fieldOf("y").for
pub static COLEWEIGHT_CODEC: compat::MainThread<CodecRef<compat::Shared<dyn NamedWaypoint>>> = compat::MainThread::new(|| {
    // rust: the record is over the getters' values, `xmap`ped to the waypoint (see `CODEC`)
    type Fields = (i32, i32, i32, Option<f64>, Option<f64>, Option<f64>, Option<ColeweightOptions>);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply7(
        record_codec_builder::of(|o: &Fields| o.0, codec::field_of(codec::int_codec(), "x")),
        record_codec_builder::of(|o: &Fields| o.1, codec::field_of(codec::int_codec(), "y")),
        record_codec_builder::of(|o: &Fields| o.2, codec::field_of(codec::int_codec(), "z")),
        // CodecUtils.optionalDouble(Codec.DOUBLE.optionalFieldOf("r")).forGetter(waypoint -> OptionalDouble.of(waypoint.colorComponents[0])) (and g, b)
        record_codec_builder::of(|o: &Fields| o.3, crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils::optional_double(&codec::optional_field_of_opt(codec::double_codec(), "r"))),
        record_codec_builder::of(|o: &Fields| o.4, crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils::optional_double(&codec::optional_field_of_opt(codec::double_codec(), "g"))),
        record_codec_builder::of(|o: &Fields| o.5, crate::de::hysky::skyblocker::utils::codec_utils::CodecUtils::optional_double(&codec::optional_field_of_opt(codec::double_codec(), "b"))),
        // ColeweightOptions.CODEC.optionalFieldOf("options").forGetter(waypoint -> Optional.of(new ColeweightOptions(Optional.of(waypoint.name.getString()))))
        record_codec_builder::of(|o: &Fields| o.6.clone(), codec::optional_field_of_opt(COLEWEIGHT_OPTIONS_CODEC.get().clone(), "options")),
        |x, y, z, r, g, b, options| (x, y, z, r, g, b, options),
    ));
    codec::xmap(
        record,
        // NamedWaypoint::fromColeweight
        |(x, y, z, r, g, b, options): Fields| NamedWaypointBase::from_coleweight(x, y, z, &r, &g, &b, &options),
        |waypoint: &compat::Shared<dyn NamedWaypoint>| compat::read(waypoint, |t| {
            let w = t.named_waypoint_base();
            let pos = w.pos.clone().unwrap();
            let c = &w.color_components;
            (pos.get_x(), pos.get_y(), pos.get_z(), Some(c[0] as f64), Some(c[1] as f64), Some(c[2] as f64), Some(ColeweightOptions::new(Some(w.name.as_ref().unwrap().get_string()))))
        }),
    )
});
//   java: static final Codec<NamedWaypoint> SKYBLOCKER_LEGACY_ORDERED_CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			BlockPos.CODEC.fieldOf("pos").forGetter(waypoint -> waypoint.pos),
//   java: 			Codec.floatRange(0, 1).listOf().xmap(Floats::toArray, FloatArrayList::new).optionalFieldOf("colorComponents", new float[0]).forGetter(waypoint -> waypoint.colorComponents)
//   java: 	).apply(instance, (pos, _) -> new OrderedNamedWaypoint(pos, "", new float[]{0, 1, 0})));
/// java: static final Codec<NamedWaypoint> SKYBLOCKER_LEGACY_ORDERED_CODEC = RecordCodecBuilder.create(instance -> instance.group( BlockPos.CODEC.fieldOf("pos").forGetter(waypoint -> waypoint.pos), Codec.float
pub static SKYBLOCKER_LEGACY_ORDERED_CODEC: compat::MainThread<CodecRef<compat::Shared<dyn NamedWaypoint>>> = compat::MainThread::new(|| {
    // rust: the record is over the getters' values, `xmap`ped to the waypoint (see `CODEC`)
    type Fields = (BlockPos, Vec<f32>);
    let record: CodecRef<Fields> = record_codec_builder::create(record_codec_builder::apply2(
        // BlockPos.CODEC.fieldOf("pos").forGetter(waypoint -> waypoint.pos)
        record_codec_builder::of(|o: &Fields| o.0.clone(), codec::field_of(mcrust::net::minecraft::core::block_pos::codec(), "pos")),
        // Codec.floatRange(0, 1).listOf().xmap(Floats::toArray, FloatArrayList::new).optionalFieldOf("colorComponents", new float[0])
        //   .forGetter(waypoint -> waypoint.colorComponents)
        // (the default is a fresh array, which `optionalFieldOf`'s `Objects.equals(a, defaultValue)` - identity for an array
        // - never matches on encode: the field is always written, so this is the optional field read with that default)
        record_codec_builder::of(|o: &Fields| o.1.clone(), mcrust::com::mojang::serialization::map_codec::xmap(
            codec::optional_field("colorComponents", codec::list(codec::float_range(0.0, 1.0)), false),
            |color_components: Option<Vec<f32>>| color_components.unwrap_or_default(),
            |color_components: &Vec<f32>| Some(color_components.clone()),
        )),
        |pos, color_components| (pos, color_components),
    ));
    codec::xmap(
        record,
        // (pos, _) -> new OrderedNamedWaypoint(pos, "", new float[]{0, 1, 0})
        |(pos, _): Fields| crate::de::hysky::skyblocker::utils::waypoint::ordered_named_waypoint::OrderedNamedWaypoint::new_3(pos, "".to_string(), vec![0.0f32, 1.0f32, 0.0f32]) as compat::Shared<dyn NamedWaypoint>,
        |waypoint: &compat::Shared<dyn NamedWaypoint>| compat::read(waypoint, |t| {
            let w = t.named_waypoint_base();
            (w.pos.clone().unwrap(), w.color_components.clone())
        }),
    )
});
/// java: public static final Comparator<NamedWaypoint> NAME_COMPARATOR = new NameComparator();
pub static NAME_COMPARATOR: compat::MainThread<Arc<dyn Fn(&compat::Shared<dyn NamedWaypoint>, &compat::Shared<dyn NamedWaypoint>) -> std::cmp::Ordering>> = compat::MainThread::new(|| {
    // the comparator object, as the closure type the port's sorts take (its int answer as an Ordering)
    let comparator: NameComparator = NameComparator::new();
    Arc::new(move |o1: &compat::Shared<dyn NamedWaypoint>, o2: &compat::Shared<dyn NamedWaypoint>| comparator.compare(o1, o2).cmp(&0))
});
//   java: public static final Codec<ColeweightOptions> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.either(Codec.STRING, Codec.INT).xmap(either -> either.map(Function.identity(), Object::toString), Either::left).optionalFieldOf("name").forGetter(ColeweightOptions::name)
//   java: 		).apply(instance, ColeweightOptions::new));
/// java: public static final Codec<ColeweightOptions> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.either(Codec.STRING, Codec.INT).xmap(either -> either.map(Function.identity(), Object::
pub static COLEWEIGHT_OPTIONS_CODEC: compat::MainThread<CodecRef<ColeweightOptions>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply1(
    // Codec.either(Codec.STRING, Codec.INT).xmap(either -> either.map(Function.identity(), Object::toString), Either::left).optionalFieldOf("name")
    //   .forGetter(ColeweightOptions::name)
    record_codec_builder::of(|o: &ColeweightOptions| o.name(), codec::optional_field_of_opt(codec::xmap(
        codec::either(codec::string_codec(), codec::int_codec()),
        |either: mcrust::com::mojang::datafixers::util::either::Either<String, i32>| either.map(|s| s, |i| i.to_string()),
        |name: &String| mcrust::com::mojang::datafixers::util::either::Either::left(name.clone()),
    ), "name")),
    ColeweightOptions::new,
)));

/// java: public class NamedWaypoint extends Waypoint
#[derive(Clone)]
pub struct NamedWaypointBase {
    /// java: extends Waypoint
    pub base: WaypointBase,
    /// java: public final Component name;
    pub name: Option<MutableComponent>,
    /// java: public final Vec3 centerPos;
    pub center_pos: Option<Vec3>,
}

impl std::ops::Deref for NamedWaypointBase {
    type Target = WaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for NamedWaypointBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl NamedWaypointBase {
    /// java: public NamedWaypoint(NamedWaypoint namedWaypoint) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut named_waypoint: compat::Shared<dyn NamedWaypoint>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let (__this, __r) = Self::new_7_in(compat::read(&named_waypoint, |__t| __t.named_waypoint_base().pos.clone()).unwrap(), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().name.clone()).unwrap(), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().color_components.clone()), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().alpha), ({ let __f = compat::dispatch(&named_waypoint, |__t| __t.is_enabled()); __f(&compat::view(&named_waypoint, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().through_walls), __wrap);
        (__this, __r)
    }

    /// java: public NamedWaypoint(NamedWaypoint namedWaypoint)
    pub fn new(mut named_waypoint: compat::Shared<dyn NamedWaypoint>) -> compat::Shared<Self> {
        Self::new_in(named_waypoint, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public NamedWaypoint(BlockPos pos, String name, float[] colorComponents) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_3_in<__R>(mut pos: BlockPos, mut name: String, mut color_components: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let (__this, __r) = Self::new3_in(pos.clone(), name.clone(), color_components.clone(), true, __wrap);
        (__this, __r)
    }

    /// java: public NamedWaypoint(BlockPos pos, String name, float[] colorComponents)
    pub fn new_3(mut pos: BlockPos, mut name: String, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        Self::new_3_in(pos, name, color_components, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public NamedWaypoint(BlockPos pos, String name, float[] colorComponents, boolean enabled) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new3_in<__R>(mut pos: BlockPos, mut name: String, mut color_components: Vec<f32>, mut enabled: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let (__this, __r) = Self::new4_in(pos.clone(), name.clone(), color_components.clone(), waypoint::DEFAULT_HIGHLIGHT_ALPHA, enabled, __wrap);
        (__this, __r)
    }

    /// java: public NamedWaypoint(BlockPos pos, String name, float[] colorComponents, boolean enabled)
    pub fn new3(mut pos: BlockPos, mut name: String, mut color_components: Vec<f32>, mut enabled: bool) -> compat::Shared<Self> {
        Self::new3_in(pos, name, color_components, enabled, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public NamedWaypoint(BlockPos pos, String name, float[] colorComponents, float alpha, boolean enabled) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new4_in<__R>(mut pos: BlockPos, mut name: String, mut color_components: Vec<f32>, mut alpha: f32, mut enabled: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let (__this, __r) = Self::new6_in(pos.clone(), component::null_to_empty(Some(&name)), color_components.clone(), alpha, enabled, __wrap);
        (__this, __r)
    }

    /// java: public NamedWaypoint(BlockPos pos, String name, float[] colorComponents, float alpha, boolean enabled)
    pub fn new4(mut pos: BlockPos, mut name: String, mut color_components: Vec<f32>, mut alpha: f32, mut enabled: bool) -> compat::Shared<Self> {
        Self::new4_in(pos, name, color_components, alpha, enabled, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, float[] colorComponents, boolean enabled) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new5_in<__R>(mut pos: BlockPos, mut name: MutableComponent, mut color_components: Vec<f32>, mut enabled: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let (__this, __r) = Self::new_6_in(pos.clone(), name.clone(), Arc::new(move || SkyblockerConfigManager::get().ui_and_visuals.waypoints.waypoint_type), color_components.clone(), waypoint::DEFAULT_HIGHLIGHT_ALPHA, enabled, __wrap);
        (__this, __r)
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, float[] colorComponents, boolean enabled)
    pub fn new5(mut pos: BlockPos, mut name: MutableComponent, mut color_components: Vec<f32>, mut enabled: bool) -> compat::Shared<Self> {
        Self::new5_in(pos, name, color_components, enabled, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, float[] colorComponents, float alpha, boolean enabled) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new6_in<__R>(mut pos: BlockPos, mut name: MutableComponent, mut color_components: Vec<f32>, mut alpha: f32, mut enabled: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let (__this, __r) = Self::new_6_in(pos.clone(), name.clone(), Arc::new(move || SkyblockerConfigManager::get().ui_and_visuals.waypoints.waypoint_type), color_components.clone(), alpha, enabled, __wrap);
        (__this, __r)
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, float[] colorComponents, float alpha, boolean enabled)
    pub fn new6(mut pos: BlockPos, mut name: MutableComponent, mut color_components: Vec<f32>, mut alpha: f32, mut enabled: bool) -> compat::Shared<Self> {
        Self::new6_in(pos, name, color_components, alpha, enabled, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new7_in<__R>(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let (__this, __r) = Self::new_6_in(pos.clone(), name.clone(), type_supplier.clone(), color_components.clone(), waypoint::DEFAULT_HIGHLIGHT_ALPHA, true, __wrap);
        (__this, __r)
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents)
    pub fn new7(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        Self::new7_in(pos, name, type_supplier, color_components, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, boolean enabled) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_6_in<__R>(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut enabled: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let (__this, __r) = Self::new_7_in(pos.clone(), name.clone(), type_supplier.clone(), color_components.clone(), alpha, enabled, true, __wrap);
        (__this, __r)
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, boolean enabled)
    pub fn new_6(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut enabled: bool) -> compat::Shared<Self> {
        Self::new_6_in(pos, name, type_supplier, color_components, alpha, enabled, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, boolean enabled, boolean throughWalls) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_7_in<__R>(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut enabled: bool, mut through_walls: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn NamedWaypoint>, __R)) -> (compat::Shared<dyn NamedWaypoint>, __R) {
        let __init_name = Some(name.clone());
        let __init_center_pos = Some(vec3::at_center_of(&pos));
        let (_, (__this, __r)) = WaypointBase::new_7_in(pos.clone(), type_supplier.clone(), color_components.clone(), alpha, waypoint::DEFAULT_LINE_WIDTH, through_walls, enabled, move |__base| {
            let this = Self {
                base: __base,
                name: __init_name,
                center_pos: __init_center_pos,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: public NamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, boolean enabled, boolean throughWalls)
    pub fn new_7(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut enabled: bool, mut through_walls: bool) -> compat::Shared<Self> {
        Self::new_7_in(pos, name, type_supplier, color_components, alpha, enabled, through_walls, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        }).1
    }

    /// java: public static NamedWaypoint fromSkytils(int x, int y, int z, String name, int color, boolean enabled)
    pub fn from_skytils(mut x: i32, mut y: i32, mut z: i32, name: &str, mut color: i32, mut enabled: bool) -> compat::Shared<dyn NamedWaypoint> {
        let mut alpha: f32 = ((((((color & 0xFF000000u32 as i32)) as u32) >> 24) as i32 as f32) / 255.0f32);
        if alpha == (0 as f32) {
            alpha = waypoint::DEFAULT_HIGHLIGHT_ALPHA;
        }
        return (NamedWaypointBase::new4(BlockPos::new(x, y, z), name.to_string(), ColorUtils::get_float_components(color), alpha, enabled) as compat::Shared<dyn NamedWaypoint>);
    }

    /// java: @SuppressWarnings("OptionalUsedAsFieldOrParameterType") public static NamedWaypoint fromColeweight(int x, int y, int z, OptionalDouble r, OptionalDouble g, OptionalDouble b, Optional<ColeweightOptions
    pub fn from_coleweight(mut x: i32, mut y: i32, mut z: i32, r: &Option<f64>, g: &Option<f64>, b: &Option<f64>, options: &Option<ColeweightOptions>) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new4(BlockPos::new(x, y, z), options.clone().and_then(|__a0| __a0.name()).unwrap_or("New Waypoint".to_string()), vec![((r.clone().unwrap_or(((0) as f64) as f64)) as f32), ((g.clone().unwrap_or(((1) as f64) as f64)) as f32), ((b.clone().unwrap_or(((0) as f64) as f64)) as f32)], waypoint::DEFAULT_HIGHLIGHT_ALPHA, true) as compat::Shared<dyn NamedWaypoint>);
    }

}

/// java: the virtual methods of NamedWaypoint (implemented by NamedWaypointBase and every subclass)
pub trait NamedWaypoint: Waypoint {
    fn named_waypoint_base(&self) -> &NamedWaypointBase;
    fn named_waypoint_base_mut(&mut self) -> &mut NamedWaypointBase;

    /// java: public NamedWaypoint copy() (dispatch: the body for this class)
    fn copy(&self) -> fn(&compat::Shared<dyn NamedWaypoint>) -> compat::Shared<dyn NamedWaypoint> {
        <dyn NamedWaypoint>::copy_body
    }

    /// java: public Component getName() (dispatch: the body for this class)
    fn get_name(&self) -> fn(&compat::Shared<dyn NamedWaypoint>) -> MutableComponent {
        <dyn NamedWaypoint>::get_name_body
    }

    /// java: public NamedWaypoint withName(String name) (dispatch: the body for this class)
    fn with_name(&self) -> fn(&compat::Shared<dyn NamedWaypoint>, &str) -> compat::Shared<dyn NamedWaypoint> {
        <dyn NamedWaypoint>::with_name_body
    }

    /// java: protected boolean shouldRenderName() (dispatch: the body for this class)
    fn should_render_name(&self) -> fn(&compat::Shared<dyn NamedWaypoint>) -> bool {
        <dyn NamedWaypoint>::should_render_name_body
    }

}

impl dyn NamedWaypoint {
    /// java: public NamedWaypoint copy()
    pub fn copy_body(__this: &compat::Shared<dyn NamedWaypoint>) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new(__this.clone()) as compat::Shared<dyn NamedWaypoint>);
    }

    /// java: @Override public NamedWaypoint withX(int x)
    pub fn with_x_body(__this: &compat::Shared<dyn NamedWaypoint>, mut x: i32) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new_7(BlockPos::new(x, __this.lock().borrow().named_waypoint_base().pos.as_ref().unwrap().get_y(), __this.lock().borrow().named_waypoint_base().pos.as_ref().unwrap().get_z()), compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.named_waypoint_base().alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.named_waypoint_base().through_walls)) as compat::Shared<dyn NamedWaypoint>);
    }

    /// NamedWaypoint.withX as the dispatch of Waypoint.withX for the concrete class `__C`
    pub fn with_x_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>, __p0: i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_x_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>), __p0)
    }

    /// java: @Override public NamedWaypoint withY(int y)
    pub fn with_y_body(__this: &compat::Shared<dyn NamedWaypoint>, mut y: i32) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new_7(__this.lock().borrow().named_waypoint_base().pos.as_ref().unwrap().at_y(y), compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.named_waypoint_base().alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.named_waypoint_base().through_walls)) as compat::Shared<dyn NamedWaypoint>);
    }

    /// NamedWaypoint.withY as the dispatch of Waypoint.withY for the concrete class `__C`
    pub fn with_y_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>, __p0: i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_y_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>), __p0)
    }

    /// java: @Override public NamedWaypoint withZ(int z)
    pub fn with_z_body(__this: &compat::Shared<dyn NamedWaypoint>, mut z: i32) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new_7(BlockPos::new(__this.lock().borrow().named_waypoint_base().pos.as_ref().unwrap().get_x(), __this.lock().borrow().named_waypoint_base().pos.as_ref().unwrap().get_y(), z), compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.named_waypoint_base().alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.named_waypoint_base().through_walls)) as compat::Shared<dyn NamedWaypoint>);
    }

    /// NamedWaypoint.withZ as the dispatch of Waypoint.withZ for the concrete class `__C`
    pub fn with_z_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>, __p0: i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_z_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>), __p0)
    }

    /// java: @Override public NamedWaypoint withColor(float[] colorComponents, float alpha)
    pub fn with_color_body(__this: &compat::Shared<dyn NamedWaypoint>, color_components: &Vec<f32>, mut alpha: f32) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new_7(compat::read(__this, |__t| __t.named_waypoint_base().pos.clone()).unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().type_supplier.clone()).clone().unwrap(), color_components.clone(), alpha, ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.named_waypoint_base().through_walls)) as compat::Shared<dyn NamedWaypoint>);
    }

    /// NamedWaypoint.withColor as the dispatch of Waypoint.withColor for the concrete class `__C`
    pub fn with_color_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>, __p0: &Vec<f32>, __p1: f32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_color_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>), __p0, __p1)
    }

    /// java: @Override public NamedWaypoint withThroughWalls(boolean throughWalls)
    pub fn with_through_walls_body(__this: &compat::Shared<dyn NamedWaypoint>, mut through_walls: bool) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new_7(compat::read(__this, |__t| __t.named_waypoint_base().pos.clone()).unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.named_waypoint_base().alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), through_walls) as compat::Shared<dyn NamedWaypoint>);
    }

    /// NamedWaypoint.withThroughWalls as the dispatch of Waypoint.withThroughWalls for the concrete class `__C`
    pub fn with_through_walls_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>, __p0: bool) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_through_walls_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>), __p0)
    }

    /// java: @Override public NamedWaypoint withTypeSupplier(Supplier<Type> typeSupplier)
    pub fn with_type_supplier_body(__this: &compat::Shared<dyn NamedWaypoint>, type_supplier: Arc<dyn Fn() -> Type>) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new_7(compat::read(__this, |__t| __t.named_waypoint_base().pos.clone()).unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap(), type_supplier.clone(), compat::read(__this, |__t| __t.named_waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.named_waypoint_base().alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.named_waypoint_base().through_walls)) as compat::Shared<dyn NamedWaypoint>);
    }

    /// NamedWaypoint.withTypeSupplier as the dispatch of Waypoint.withTypeSupplier for the concrete class `__C`
    pub fn with_type_supplier_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>, __p0: Arc<dyn Fn() -> Type>) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_type_supplier_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>), __p0)
    }

    /// java: public Component getName()
    pub fn get_name_body(__this: &compat::Shared<dyn NamedWaypoint>) -> MutableComponent {
        return compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap();
    }

    /// java: public NamedWaypoint withName(String name)
    pub fn with_name_body(__this: &compat::Shared<dyn NamedWaypoint>, name: &str) -> compat::Shared<dyn NamedWaypoint> {
        return (NamedWaypointBase::new_6(compat::read(__this, |__t| __t.named_waypoint_base().pos.clone()).unwrap(), component::literal(name), compat::read(__this, |__t| __t.named_waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.named_waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.named_waypoint_base().alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) })) as compat::Shared<dyn NamedWaypoint>);
    }

    /// java: protected boolean shouldRenderName()
    pub fn should_render_name_body(__this: &compat::Shared<dyn NamedWaypoint>) -> bool {
        return ({ let __f = compat::dispatch(__this, |__t| __t.should_render()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) });
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_body(__this: &compat::Shared<dyn NamedWaypoint>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>), collector);
        if { let __f = compat::dispatch(__this, |__t| __t.should_render_name()); __f(__this) } {
            let mut scale: f32 = (((RenderHelper::get_camera().position().distance_to(&compat::read(__this, |__t| __t.named_waypoint_base().center_pos.clone()).unwrap())) as f32) / (10 as f32)).max((1 as f32));
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_4()); __f(collector, &compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap(), &{ let (__q0, __q1, __q2,) = ((0) as f64, (1) as f64, (0) as f64,); __this.lock().borrow().named_waypoint_base().center_pos.as_ref().unwrap().add(__q0, __q1, __q2) }, scale, true) });
        }
    }

    /// NamedWaypoint.extractRendering as the dispatch of Renderable.extractRendering for the concrete class `__C`
    pub fn extract_rendering_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        <dyn NamedWaypoint>::extract_rendering_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>), __p0)
    }

    /// java: @Override public int hashCode()
    pub fn hash_code_body(__this: &compat::Shared<dyn NamedWaypoint>) -> i32 {
        return compat::hash_code(&(&<dyn Waypoint>::hash_code_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)), &compat::read(__this, |__t| __t.named_waypoint_base().name.clone()).unwrap(),));
    }

    /// NamedWaypoint.hashCode as the dispatch of Waypoint.hashCode for the concrete class `__C`
    pub fn hash_code_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>) -> i32 {
        <dyn NamedWaypoint>::hash_code_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>))
    }

    /// java: @Override public boolean equals(Object obj)
    pub fn equals_body(__this: &compat::Shared<dyn NamedWaypoint>, obj: &compat::Shared<dyn Waypoint>) -> bool {
        // java: return this == obj || super.equals(obj) && obj instanceof NamedWaypoint waypoint && name.equals(waypoint.name);
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(__this), std::sync::Arc::as_ptr(obj)) {
            return true;
        }
        if !<dyn Waypoint>::equals_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>), obj) {
            return false;
        }
        let Some(other_name) = compat::read(obj, |o| o.as_named_waypoint().map(|waypoint| waypoint.name.clone())) else {
            return false;
        };
        return compat::read(__this, |t| t.named_waypoint_base().name == other_name);
    }

    /// java: `a.equals(b)` on two named waypoints (`List.indexOf` / `List.remove(Object)` / `List.equals` /
    /// `Set.contains` over them): the dispatch of `Waypoint.equals` on `a`, so a subclass's override runs.
    pub fn equals_shared(a: &compat::Shared<dyn NamedWaypoint>, b: &compat::Shared<dyn NamedWaypoint>) -> bool {
        let a = compat::view(a, |w| w as compat::Shared<dyn Waypoint>);
        let b = compat::view(b, |w| w as compat::Shared<dyn Waypoint>);
        let __f = compat::dispatch(&*a, |__t| __t.equals());
        __f(&a, &b)
    }

    /// NamedWaypoint.equals as the dispatch of Waypoint.equals for the concrete class `__C`
    pub fn equals_from<__C: NamedWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>, __p0: &compat::Shared<dyn Waypoint>) -> bool {
        <dyn NamedWaypoint>::equals_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn NamedWaypoint>), __p0)
    }

}

impl NamedWaypoint for NamedWaypointBase {
    fn named_waypoint_base(&self) -> &NamedWaypointBase {
        self
    }
    fn named_waypoint_base_mut(&mut self) -> &mut NamedWaypointBase {
        self
    }

}

impl Waypoint for NamedWaypointBase {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base
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
    fn as_named_waypoint(&self) -> Option<&NamedWaypointBase> {
        Some(self.named_waypoint_base())
    }

}

impl Renderable for NamedWaypointBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn NamedWaypoint>::extract_rendering_from::<Self>
    }

}

/// java: private record ColeweightOptions(Optional<String> name)
#[derive(Clone, Debug, PartialEq)]
pub struct ColeweightOptions {
    /// java: Optional<String> name
    pub name: Option<String>,
}

impl ColeweightOptions {
    /// java: the canonical record constructor
    pub fn new(mut name: Option<String>) -> Self {
        Self { name }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

}

/// java: private static class NameComparator implements Comparator<NamedWaypoint>
#[derive(Clone)]
pub struct NameComparator {}

impl NameComparator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public int compare(NamedWaypoint o1, NamedWaypoint o2)
    pub fn compare(&self, o1: &compat::Shared<dyn NamedWaypoint>, o2: &compat::Shared<dyn NamedWaypoint>) -> i32 {
        let mut string1: String = ({ let __f = compat::dispatch(o1, |__t| __t.get_name()); __f(o1) }).get_string();
        let mut string2: String = ({ let __f = compat::dispatch(o2, |__t| __t.get_name()); __f(o2) }).get_string();
        let mut prefix1: String = compat::regex::replace_first(&string1, "\\d+$", "");
        let mut prefix2: String = compat::regex::replace_first(&string2, "\\d+$", "");
        let mut i: i32 = compat::compare_to(&prefix1, &prefix2);
        if i != 0 {
            return i;
        }
        let mut num1: String = compat::substring(&string1, compat::str_len(&prefix1));
        let mut num2: String = compat::substring(&string2, compat::str_len(&prefix2));
        if num1.is_empty() || num2.is_empty() {
            return compat::compare_to(&string1, &string2);
        }
        // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut i1: i32 = num1.parse::<i32>().expect("NumberFormatException");
            let mut i2: i32 = num2.parse::<i32>().expect("NumberFormatException");
            return compat::ord_int(i1.cmp(&i2));
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return compat::compare_to(&string1, &string2);
            }
        }
    }

}
