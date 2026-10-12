//! Ported from de.hysky.skyblocker.utils.data.constants.MinionConstants by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: public static final Codec<MinionConstants> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 			Codec.unboundedMap(Codec.STRING, Codec.STRING.listOf()).fieldOf("categories").forGetter(MinionConstants::categories)
//   java: 			).apply(instance, MinionConstants::new));
/// java: public static final Codec<MinionConstants> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.unboundedMap(Codec.STRING, Codec.STRING.listOf()).fieldOf("categories").forGetter(MinionC
pub static CODEC: compat::MainThread<CodecRef<MinionConstants>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: public static final MinionConstants EMPTY = new MinionConstants(Map.of());
pub static EMPTY: LazyLock<MinionConstants> = LazyLock::new(|| MinionConstants::new(std::collections::HashMap::from([])));

/// java: public record MinionConstants(Map<String, List<String>> categories)
#[derive(Clone, Debug, PartialEq)]
pub struct MinionConstants {
    /// java: Map<String, List<String>> categories
    pub categories: std::collections::HashMap<String, Vec<String>>,
}

impl MinionConstants {
    /// java: the canonical record constructor
    pub fn new(mut categories: std::collections::HashMap<String, Vec<String>>) -> Self {
        Self { categories }
    }

    /// java: the record accessor `categories()`
    pub fn categories(&self) -> std::collections::HashMap<String, Vec<String>> {
        self.categories.clone()
    }

}
