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

/// java: public static final Codec<Color> COLOR_CODEC = Codec.withAlternative( Codec.STRING.xmap(s -> Integer.parseUnsignedInt(s, 16), i -> Integer.toHexString(i).toUpperCase(Locale.ENGLISH)), Codec.INT ).xmap
/// (`LazyLock`: config codecs are read off the render thread)
pub static COLOR_CODEC: LazyLock<CodecRef<Color>> = LazyLock::new(|| {
    let hex: CodecRef<i32> = codec::flat_xmap(
        codec::string_codec(),
        // java: Integer.parseUnsignedInt(s, 16) (throws on a bad string: the decode fails)
        |s: String| match u32::from_str_radix(&s, 16) {
            Ok(v) if !s.is_empty() && !s.starts_with('+') && !s.starts_with('-') => mcrust::com::mojang::serialization::data_result::DataResult::success(v as i32),
            _ => { let __m = format!("NumberFormatException: For input string: \"{s}\" under radix 16"); mcrust::com::mojang::serialization::data_result::DataResult::error(std::sync::Arc::new(move || __m.clone())) }
        },
        // java: Integer.toHexString(i).toUpperCase(Locale.ENGLISH)
        |i: &i32| mcrust::com::mojang::serialization::data_result::DataResult::success(format!("{:X}", *i as u32)),
    );
    codec::xmap(
        codec::with_alternative(hex, codec::int_codec()),
        |argb: i32| Color::from_rgba(argb, true),
        |color: &Color| color.get_rgb(),
    )
});
///
/// `ExtraCodecs.JSON` is `converter(JsonOps.INSTANCE)`: the port's `converter` over its `JsonOps` (whose tree is
/// `lwjgl::json::Value`), read back as the mod's Gson tree (`compat::json_from_port`).
pub static JSON_OBJECT_CODEC: compat::MainThread<CodecRef<serde_json::Map<String, serde_json::Value>>> = compat::MainThread::new(|| {
    use mcrust::com::mojang::serialization::data_result::{message, DataResult};
    let json = mcrust::net::minecraft::util::extra_codecs::converter(mcrust::com::mojang::serialization::json_ops::JsonOps::instance());
    codec::flat_xmap(
        json,
        // element -> element.isJsonObject() ? DataResult.success(element.getAsJsonObject()) : DataResult.error(() -> "Not a json object.")
        |element: mcrust::com::mojang::serialization::Obj| match compat::json_from_port(&element) {
            serde_json::Value::Object(object) => DataResult::success(object),
            _ => DataResult::error(message("Not a json object.")),
        },
        // DataResult::success
        |object: &serde_json::Map<String, serde_json::Value>| DataResult::success(compat::json_to_port(&serde_json::Value::Object(object.clone()))),
    )
});
pub static MAP_PASSTHROUGH: compat::MainThread<MapCodecRef<Dynamic>> = compat::MainThread::new(|| assume_map_unsafe(codec::passthrough()));

/// DFU `MapCodec.assumeMapUnsafe(Codec<A> codec)` - the anonymous `MapCodec` it answers (the port's DFU has no
/// counterpart; GAPS_L.md). It reads the whole map (or, compressed, its one `value` key) with `codec`, and
/// writes `codec`'s encoded map into the prefix entry by entry.
fn assume_map_unsafe<A: 'static>(codec: CodecRef<A>) -> MapCodecRef<A> {
    std::sync::Arc::new(AssumeMapUnsafe { codec })
}

/// java: `private static final String COMPRESSED_VALUE_KEY = "value";`
const COMPRESSED_VALUE_KEY: &str = "value";

struct AssumeMapUnsafe<A: 'static> {
    codec: CodecRef<A>,
}

impl<A: 'static> mcrust::com::mojang::serialization::keyable::Keyable for AssumeMapUnsafe<A> {
    /// java: `return Stream.of(ops.createString(COMPRESSED_VALUE_KEY));`
    fn keys(&self, ops: &mcrust::com::mojang::serialization::dynamic_ops::Ops) -> Vec<mcrust::com::mojang::serialization::Obj> {
        vec![ops.create_string(COMPRESSED_VALUE_KEY)]
    }
}

impl<A: 'static> mcrust::com::mojang::serialization::map_decoder::MapDecoder<A> for AssumeMapUnsafe<A> {
    fn decode(&self, ops: &mcrust::com::mojang::serialization::dynamic_ops::Ops, input: &mcrust::com::mojang::serialization::map_like::MapLike) -> mcrust::com::mojang::serialization::data_result::DataResult<A> {
        use mcrust::com::mojang::serialization::data_result::{message, DataResult};
        use mcrust::com::mojang::serialization::decoder::Decoder as _;
        if ops.compress_maps() {
            let Some(value) = input.get(&ops.create_string(COMPRESSED_VALUE_KEY)) else {
                return DataResult::error(message("Missing value"));
            };
            return self.codec.parse(ops, value);
        }
        self.codec.parse(ops, &ops.create_map(input.entries().to_vec()))
    }
}

impl<A: 'static> mcrust::com::mojang::serialization::map_encoder::MapEncoder<A> for AssumeMapUnsafe<A> {
    fn encode(&self, input: &A, ops: &mcrust::com::mojang::serialization::dynamic_ops::Ops, prefix: &mut dyn mcrust::com::mojang::serialization::record_builder::RecordBuilder) {
        use mcrust::com::mojang::serialization::encoder::Encoder as _;
        let encoded = self.codec.encode_start(ops, input);
        if ops.compress_maps() {
            prefix.add_string_result(COMPRESSED_VALUE_KEY, encoded);
            return;
        }
        let encoded_map_result = encoded.flat_map(|e| ops.get_map(ops, &e));
        match encoded_map_result.result_ref() {
            Some(encoded_map) => {
                for (key, value) in encoded_map.entries() {
                    prefix.add(key, value);
                }
            }
            None => prefix.with_errors_from(encoded_map_result.erase()),
        }
    }
}
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
        // java: return codec.xmap(opt -> opt.map(OptionalInt::of).orElseGet(OptionalInt::empty), optInt -> ..);
        // (`OptionalInt` and `Optional<Integer>` are both `Option<i32>` here: the xmap is the identity)
        return mcrust::com::mojang::serialization::map_codec::xmap(codec.clone(), |opt: Option<i32>| opt, |opt_int: &Option<i32>| *opt_int);
    }

    /// java: public static MapCodec<OptionalDouble> optionalDouble(MapCodec<Optional<Double>> codec)
    pub fn optional_double(codec: &MapCodecRef<Option<f64>>) -> MapCodecRef<Option<f64>> {
        // java: return codec.xmap(opt -> opt.map(OptionalDouble::of).orElseGet(OptionalDouble::empty), optDouble -> ..);
        // (`OptionalDouble` and `Optional<Double>` are both `Option<f64>` here: the xmap is the identity)
        return mcrust::com::mojang::serialization::map_codec::xmap(codec.clone(), |opt: Option<f64>| opt, |opt_double: &Option<f64>| *opt_double);
    }

    /// java: public static <K> Codec<Object2BooleanMap<K>> object2BooleanMapCodec(Codec<K> keyCodec)
    pub fn object2_boolean_map_codec<K: Clone + Eq + std::hash::Hash + std::fmt::Debug + 'static>(key_codec: &CodecRef<K>) -> CodecRef<std::collections::HashMap<K, bool>> {
        // java: return Codec.unboundedMap(keyCodec, Codec.BOOL).xmap(Object2BooleanOpenHashMap::new, Function.identity());
        return CodecUtils::object2_object_map_codec(key_codec, &codec::bool_codec());
    }

    /// java: public static <K> Codec<Object2IntMap<K>> object2IntMapCodec(Codec<K> keyCodec)
    pub fn object2_int_map_codec<K: Clone + Eq + std::hash::Hash + std::fmt::Debug + 'static>(key_codec: &CodecRef<K>) -> CodecRef<std::collections::HashMap<K, i32>> {
        // java: return Codec.unboundedMap(keyCodec, Codec.INT).xmap(Object2IntOpenHashMap::new, Function.identity());
        return CodecUtils::object2_object_map_codec(key_codec, &codec::int_codec());
    }

    /// java: public static <K> Codec<Object2LongMap<K>> object2LongMapCodec(Codec<K> keyCodec)
    pub fn object2_long_map_codec<K: Clone + Eq + std::hash::Hash + std::fmt::Debug + 'static>(key_codec: &CodecRef<K>) -> CodecRef<std::collections::HashMap<K, i64>> {
        // java: return Codec.unboundedMap(keyCodec, Codec.LONG).xmap(Object2LongOpenHashMap::new, Function.identity());
        return CodecUtils::object2_object_map_codec(key_codec, &codec::long_codec());
    }

    /// java: public static <K> Codec<Object2DoubleMap<K>> object2DoubleMapCodec(Codec<K> keyCodec)
    pub fn object2_double_map_codec<K: Clone + Eq + std::hash::Hash + std::fmt::Debug + 'static>(key_codec: &CodecRef<K>) -> CodecRef<std::collections::HashMap<K, f64>> {
        // java: return Codec.unboundedMap(keyCodec, Codec.DOUBLE).xmap(Object2DoubleOpenHashMap::new, Function.identity());
        return CodecUtils::object2_object_map_codec(key_codec, &codec::double_codec());
    }

    /// java: public static <K, V> Codec<Object2ObjectMap<K, V>> object2ObjectMapCodec(Codec<K> keyCodec, Codec<V> valueCodec)
    pub fn object2_object_map_codec<K: Clone + Eq + std::hash::Hash + std::fmt::Debug + 'static, V: Clone + 'static>(key_codec: &CodecRef<K>, value_codec: &CodecRef<V>) -> CodecRef<std::collections::HashMap<K, V>> {
        // java: return Codec.unboundedMap(keyCodec, valueCodec).xmap(Object2ObjectOpenHashMap::new, Function.identity());
        // (the port's `unboundedMap` answers the entries as pairs)
        return codec::xmap(
            codec::unbounded_map(key_codec.clone(), value_codec.clone()),
            |entries: Vec<(K, V)>| entries.into_iter().collect::<std::collections::HashMap<K, V>>(),
            |map: &std::collections::HashMap<K, V>| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<(K, V)>>(),
        );
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
    pub fn mutable_optional<T: Clone + 'static>(codec: &MapCodecRef<T>, mutable_factory: Arc<dyn Fn(T) -> T + Send + Sync>) -> MapCodecRef<T> {
        // java: return codec.xmap(mutableFactory, Function.identity());
        return mcrust::com::mojang::serialization::map_codec::xmap(codec.clone(), move |t: T| (mutable_factory)(t), |t: &T| t.clone());
    }

    /// java: public static <E> Codec<List<E>> sortedListCodec(Codec<E> elementCodec, Comparator<E> comparator)
    pub fn sorted_list_codec<E: Clone + 'static>(element_codec: &CodecRef<E>, comparator: Arc<dyn Fn(&E, &E) -> std::cmp::Ordering + Send + Sync>) -> CodecRef<Vec<E>> {
        // java: return elementCodec.listOf().xmap(list -> { List<E> sortedList = new ArrayList<>(list); sortedList.sort(comparator); return List.copyOf(sortedList); }, Function.identity());
        return codec::xmap(
            codec::list(element_codec.clone()),
            move |list: Vec<E>| {
                let mut sorted_list: Vec<E> = list;
                sorted_list.sort_by(|a, b| (comparator)(a, b));
                sorted_list
            },
            |list: &Vec<E>| list.clone(),
        );
    }

}
