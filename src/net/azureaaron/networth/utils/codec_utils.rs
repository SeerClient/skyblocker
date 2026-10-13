//! java: public class CodecUtils (net.azureaaron.networth.utils)
//!
//! `OptionalInt` / `OptionalLong` / `OptionalDouble` are `Option<i32>` / `Option<i64>` / `Option<f64>` here, the same
//! type as the boxed `Optional<Integer>` etc., so the three `xmap`s convert between two spellings of one type - they
//! are kept, as the Java builds them, so each record codec reads like its Java declaration.
//! `Object2IntMap<K>` is `Vec<(K, i32)>`: what `Codec.unboundedMap` decodes to in the port (insertion order).

use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::map_codec::{self, MapCodecRef};

/// java: public class CodecUtils
pub struct CodecUtils;

impl CodecUtils {
    /// java: public static <K> Codec<Object2IntMap<K>> createObject2IntMapCodec(Codec<K> keyCodec)
    pub fn create_object2_int_map_codec<
        K: Clone + PartialEq + std::fmt::Debug + Send + Sync + 'static,
    >(
        key_codec: CodecRef<K>,
    ) -> CodecRef<Vec<(K, i32)>> {
        // java: Codec.unboundedMap(keyCodec, Codec.INT).xmap(Object2IntOpenHashMap::new, Object2IntOpenHashMap::new)
        codec::xmap(
            codec::unbounded_map(key_codec, codec::int_codec()),
            |map: Vec<(K, i32)>| map,
            |map: &Vec<(K, i32)>| map.clone(),
        )
    }

    /// java: public static MapCodec<OptionalInt> optionalInt(MapCodec<Optional<Integer>> codec)
    pub fn optional_int(codec: MapCodecRef<Option<i32>>) -> MapCodecRef<Option<i32>> {
        map_codec::xmap(
            codec,
            |opt: Option<i32>| opt,
            |opt_int: &Option<i32>| *opt_int,
        )
    }

    /// java: public static MapCodec<OptionalLong> optionalLong(MapCodec<Optional<Long>> codec)
    pub fn optional_long(codec: MapCodecRef<Option<i64>>) -> MapCodecRef<Option<i64>> {
        map_codec::xmap(
            codec,
            |opt: Option<i64>| opt,
            |opt_long: &Option<i64>| *opt_long,
        )
    }

    /// java: public static MapCodec<OptionalDouble> optionalDouble(MapCodec<Optional<Double>> codec)
    pub fn optional_double(codec: MapCodecRef<Option<f64>>) -> MapCodecRef<Option<f64>> {
        map_codec::xmap(
            codec,
            |opt: Option<f64>| opt,
            |opt_double: &Option<f64>| *opt_double,
        )
    }
}
