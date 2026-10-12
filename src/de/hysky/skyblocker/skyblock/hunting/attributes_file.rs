//! Ported from de.hysky.skyblocker.skyblock.hunting.AttributesFile by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::hunting::attribute;
use crate::de::hysky::skyblocker::skyblock::hunting::attribute::Attribute;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: public static final Codec<AttributesFile> CODEC = RecordCodecBuilder.create(instance -> instance.group( Attribute.LIST_CODEC.fieldOf("attributes").forGetter(AttributesFile::attributes) ).apply(instanc
pub static CODEC: compat::MainThread<CodecRef<AttributesFile>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply1(record_codec_builder::of(|o: &AttributesFile| o.attributes(), codec::field_of(attribute::LIST_CODEC.get_mut().clone(), "attributes")), move |__a0| AttributesFile::new(__a0.clone()))));

/// java: public record AttributesFile(List<Attribute> attributes)
#[derive(Clone, Debug, PartialEq)]
pub struct AttributesFile {
    /// java: List<Attribute> attributes
    pub attributes: Vec<Attribute>,
}

impl AttributesFile {
    /// java: the canonical record constructor
    pub fn new(mut attributes: Vec<Attribute>) -> Self {
        Self { attributes }
    }

    /// java: the record accessor `attributes()`
    pub fn attributes(&self) -> Vec<Attribute> {
        self.attributes.clone()
    }

}
