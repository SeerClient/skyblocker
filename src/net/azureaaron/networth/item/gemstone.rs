//! java: @ApiStatus.Internal public record Gemstone(String quality) (net.azureaaron.networth.item)
//!
//! `Map<String, Either<String, Gemstone>>` is `Vec<(String, Either<String, Gemstone>)>`: what `Codec.unboundedMap`
//! decodes to in the port (one entry per key, in the order the input map lists them).

use std::sync::LazyLock;

use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::com::mojang::serialization::codec::{self, CodecRef};
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: private static final Codec<Gemstone> CODEC
static CODEC: LazyLock<CodecRef<Gemstone>> = LazyLock::new(|| {
    record_codec_builder::create(record_codec_builder::apply1(
        record_codec_builder::of(
            |o: &Gemstone| o.quality.clone(),
            codec::field_of(codec::string_codec(), "quality"),
        ),
        Gemstone::new,
    ))
});

/// java: private static final Codec<Either<String, Gemstone>> EITHER_CODEC = Codec.either(Codec.STRING, CODEC);
static EITHER_CODEC: LazyLock<CodecRef<Either<String, Gemstone>>> =
    LazyLock::new(|| codec::either(codec::string_codec(), CODEC.clone()));

/// java: static final Codec<Map<String, Either<String, Gemstone>>> MAP_EITHER_CODEC
/// = Codec.unboundedMap(Codec.STRING, EITHER_CODEC).xmap(Object2ObjectOpenHashMap::new, map -> new Object2ObjectOpenHashMap<>(map));
pub(crate) static MAP_EITHER_CODEC: LazyLock<CodecRef<Vec<(String, Either<String, Gemstone>)>>> =
    LazyLock::new(|| {
        codec::xmap(
            codec::unbounded_map(codec::string_codec(), EITHER_CODEC.clone()),
            |map: Vec<(String, Either<String, Gemstone>)>| map,
            |map: &Vec<(String, Either<String, Gemstone>)>| map.clone(),
        )
    });

/// java: @ApiStatus.Internal public record Gemstone(String quality)
#[derive(Clone, Debug, PartialEq)]
pub struct Gemstone {
    pub quality: String,
}

impl Gemstone {
    /// java: the canonical record constructor
    pub fn new(quality: String) -> Self {
        Self { quality }
    }

    /// java: the record accessor `quality()`
    pub fn quality(&self) -> &str {
        &self.quality
    }
}
