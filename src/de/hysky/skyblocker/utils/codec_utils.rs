//! Ported from de.hysky.skyblocker.utils.CodecUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::java::awt::color::Color;
use mcrust::org::joml::vector2i::Vector2i;

// TODO(fabric2seer): Integer.parseUnsignedInt
//   java: public static final Codec<Color> COLOR_CODEC = Codec.withAlternative(
//   java: 			Codec.STRING.xmap(s -> Integer.parseUnsignedInt(s, 16), i -> Integer.toHexString(i).toUpperCase(Locale.ENGLISH)),
//   java: 			Codec.INT
//   java: 	).xmap(argb -> new Color(argb, true), Color::getRGB);
/// java: public static final Codec<Color> COLOR_CODEC = Codec.withAlternative( Codec.STRING.xmap(s -> Integer.parseUnsignedInt(s, 16), i -> Integer.toHexString(i).toUpperCase(Locale.ENGLISH)), Codec.INT ).xmap
pub static COLOR_CODEC: compat::MainThread<CodecRef<Color>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `COLOR_CODEC` - see the TODO above"));
// TODO(fabric2seer): static `ExtraCodecs.JSON` has no Rust counterpart in the port's ExtraCodecs file
//   java: public static final Codec<JsonObject> JSON_OBJECT_CODEC = ExtraCodecs.JSON.flatXmap(
//   java: 			element -> element.isJsonObject() ? DataResult.success(element.getAsJsonObject()) : DataResult.error(() -> "Not a json object."),
//   java: 			DataResult::success
//   java: 	);
/// java: public static final Codec<JsonObject> JSON_OBJECT_CODEC = ExtraCodecs.JSON.flatXmap( element -> element.isJsonObject() ? DataResult.success(element.getAsJsonObject()) : DataResult.error(() -> "Not a j
pub static JSON_OBJECT_CODEC: compat::MainThread<CodecRef<serde_json::Map<String, serde_json::Value>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `JSON_OBJECT_CODEC` - see the TODO above"));
// TODO(fabric2seer): static `MapCodec.assumeMapUnsafe` has no Rust counterpart in the port
//   java: public static final MapCodec<Dynamic<?>> MAP_PASSTHROUGH = MapCodec.assumeMapUnsafe(Codec.PASSTHROUGH);
/// java: public static final MapCodec<Dynamic<?>> MAP_PASSTHROUGH = MapCodec.assumeMapUnsafe(Codec.PASSTHROUGH);
pub static MAP_PASSTHROUGH: compat::MainThread<MapCodecRef<Dynamic>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `MAP_PASSTHROUGH` - see the TODO above"));
/// java: public static final Codec<Vector2ic> VECTOR_2I = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("x").forGetter(Vector2ic::x), Codec.INT.fieldOf("y").forGetter(Vector2ic::y) ).
pub static VECTOR_2I: compat::MainThread<CodecRef<Vector2i>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &Vector2i| o.x(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "x")), record_codec_builder::of(|o: &Vector2i| o.y(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "y")), move |__a0, __a1| Vector2i::new(__a0.clone(), __a1.clone()))));

/// java: public final class CodecUtils
#[derive(Clone)]
pub struct CodecUtils {}

impl CodecUtils {
    /// java: private CodecUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        panic!("IllegalStateException: {}", "Uhhhh no? like just no. What are you trying to do? D- Do you think this will be useful to instantiate this? Like it's private, so you went through the effort of putting an accessor actually i'm not sure you can accessor a constructor. can you? so if not did you really put an access widener for that? like really? honestly this is just sad. Plus there aren't even any method in here that requires an instance. There's only static methods. like bruh. you know what i'm done typing shit for you to read, bye i'm leaving *voice lowers as I leave* I swear those modders think they can access all they want sheesh *comes back instantly* AND I SWEAR IF YOU INJECT SO THIS ERROR CANNOT BE THROWN I WILL SEND YOU TO HELL'S FREEZER");
        this
    }

    /// java: public static MapCodec<OptionalInt> optionalInt(MapCodec<Optional<Integer>> codec)
    pub fn optional_int(codec: &MapCodecRef<Option<i32>>) -> MapCodecRef<Option<i32>> {
        // TODO(fabric2seer): `.map(..)` on a A
        //   java: return codec.xmap(opt -> opt.map(OptionalInt::of).orElseGet(OptionalInt::empty), optInt -> optInt.isPresent() ? Optional.of(optInt.getAsInt()) : Optional.empty());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static MapCodec<OptionalDouble> optionalDouble(MapCodec<Optional<Double>> codec)
    pub fn optional_double(codec: &MapCodecRef<Option<f64>>) -> MapCodecRef<Option<f64>> {
        // TODO(fabric2seer): `.map(..)` on a A
        //   java: return codec.xmap(opt -> opt.map(OptionalDouble::of).orElseGet(OptionalDouble::empty), optDouble -> optDouble.isPresent() ? Optional.of(optDouble.getAsDouble()) : Optional.empty());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <K> Codec<Object2BooleanMap<K>> object2BooleanMapCodec(Codec<K> keyCodec)
    pub fn object2_boolean_map_codec<K: 'static>(key_codec: &CodecRef<K>) -> CodecRef<std::collections::HashMap<K, bool>> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return Codec.unboundedMap(keyCodec, Codec.BOOL).xmap(Object2BooleanOpenHashMap::new, Function.identity());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <K> Codec<Object2IntMap<K>> object2IntMapCodec(Codec<K> keyCodec)
    pub fn object2_int_map_codec<K: 'static>(key_codec: &CodecRef<K>) -> CodecRef<std::collections::HashMap<K, i32>> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return Codec.unboundedMap(keyCodec, Codec.INT).xmap(Object2IntOpenHashMap::new, Function.identity());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <K> Codec<Object2LongMap<K>> object2LongMapCodec(Codec<K> keyCodec)
    pub fn object2_long_map_codec<K: 'static>(key_codec: &CodecRef<K>) -> CodecRef<std::collections::HashMap<K, i64>> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return Codec.unboundedMap(keyCodec, Codec.LONG).xmap(Object2LongOpenHashMap::new, Function.identity());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <K> Codec<Object2DoubleMap<K>> object2DoubleMapCodec(Codec<K> keyCodec)
    pub fn object2_double_map_codec<K: 'static>(key_codec: &CodecRef<K>) -> CodecRef<std::collections::HashMap<K, f64>> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return Codec.unboundedMap(keyCodec, Codec.DOUBLE).xmap(Object2DoubleOpenHashMap::new, Function.identity());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <K, V> Codec<Object2ObjectMap<K, V>> object2ObjectMapCodec(Codec<K> keyCodec, Codec<V> valueCodec)
    pub fn object2_object_map_codec<K: 'static, V: 'static>(key_codec: &CodecRef<K>, value_codec: &CodecRef<V>) -> CodecRef<std::collections::HashMap<K, V>> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return Codec.unboundedMap(keyCodec, valueCodec).xmap(Object2ObjectOpenHashMap::new, Function.identity());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <E extends Enum<E>> Codec<EnumSet<E>> enumSetCodec(Codec<E> enumCodec, Class<E> enumClass)
    pub fn enum_set_codec<E: Clone + Eq + Ord + std::hash::Hash + 'static>(enum_codec: &CodecRef<E>, enum_class: &std::any::TypeId) -> CodecRef<std::collections::HashSet<E>> {
        // java: return enumCodec.listOf().xmap(list -> list.isEmpty() ? EnumSet.noneOf(enumClass) : EnumSet.copyOf(list), List::copyOf);
        // (an EnumSet iterates in ordinal order, so the encoded list is in that order: the enum's `Ord`)
        let _ = enum_class;
        return codec::xmap(
            codec::list(enum_codec.clone()),
            |list: Vec<E>| list.into_iter().collect::<std::collections::HashSet<E>>(),
            |set: &std::collections::HashSet<E>| {
                let mut list: Vec<E> = set.iter().cloned().collect();
                list.sort();
                list
            },
        );
    }

    /// java: public static <T> MapCodec<T> mutableOptional(MapCodec<T> codec, Function<? super T, ? extends T> mutableFactory)
    pub fn mutable_optional<T: 'static>(codec: &MapCodecRef<T>, mutable_factory: Arc<dyn Fn(T) -> T>) -> MapCodecRef<T> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return codec.xmap(mutableFactory, Function.identity());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <E> Codec<List<E>> sortedListCodec(Codec<E> elementCodec, Comparator<E> comparator)
    pub fn sorted_list_codec<E: 'static>(element_codec: &CodecRef<E>, comparator: Arc<dyn Fn(&E, &E) -> std::cmp::Ordering>) -> CodecRef<Vec<E>> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return elementCodec.listOf().xmap(list -> {
        //   java: 			List<E> sortedList = new ArrayList<>(list);
        //   java: 			sortedList.sort(comparator);
        //   java:
        //   java: 			return List.copyOf(sortedList);
        //   java: 		}, Function.identity());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
