//! Ported from de.hysky.skyblocker.utils.mayor.Perk by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Perk EMPTY = new Perk("", "");
pub static EMPTY: LazyLock<Perk> = LazyLock::new(|| Perk::new("".to_string(), "".to_string()));
/// java: public static final Codec<Perk> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(Perk::name), Codec.STRING.fieldOf("description").forGetter(Perk::de
pub static CODEC: compat::MainThread<CodecRef<Perk>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &Perk| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &Perk| o.description(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "description")), move |__a0, __a1| Perk::new(__a0.clone(), __a1.clone()))));

/// java: public record Perk(String name, String description)
#[derive(Clone, Debug, PartialEq)]
pub struct Perk {
    /// java: String name
    pub name: String,
    /// java: String description
    pub description: String,
}

impl Perk {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut description: String) -> Self {
        Self { name, description }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `description()`
    pub fn description(&self) -> String {
        self.description.clone()
    }

}
