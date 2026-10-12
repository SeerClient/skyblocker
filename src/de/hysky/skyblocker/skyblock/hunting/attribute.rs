//! Ported from de.hysky.skyblocker.skyblock.hunting.Attribute by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;

/// java: private static final Codec<Attribute> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("abilityName").forGetter(Attribute::name), Codec.STRING.fieldOf("displayName").
pub static CODEC: compat::MainThread<CodecRef<Attribute>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply5(record_codec_builder::of(|o: &Attribute| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "abilityName")), record_codec_builder::of(|o: &Attribute| o.shard_name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "displayName")), record_codec_builder::of(|o: &Attribute| o.id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "shardId")), record_codec_builder::of(|o: &Attribute| o.api_id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "bazaarName")), record_codec_builder::of(|o: &Attribute| o.neu_id(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "internalName")), move |__a0, __a1, __a2, __a3, __a4| Attribute::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone()))));
/// java: public static final Codec<List<Attribute>> LIST_CODEC = CODEC.listOf();
pub static LIST_CODEC: compat::MainThread<CodecRef<Vec<Attribute>>> = compat::MainThread::new(|| codec::list(CODEC.get_mut().clone()));

/// java: public record Attribute(String name, String shardName, String id, String apiId, String neuId)
#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    /// java: String name
    pub name: String,
    /// java: String shardName
    pub shard_name: String,
    /// java: String id
    pub id: String,
    /// java: String apiId
    pub api_id: String,
    /// java: String neuId
    pub neu_id: String,
}

impl Attribute {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut shard_name: String, mut id: String, mut api_id: String, mut neu_id: String) -> Self {
        Self { name, shard_name, id, api_id, neu_id }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `shardName()`
    pub fn shard_name(&self) -> String {
        self.shard_name.clone()
    }

    /// java: the record accessor `id()`
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// java: the record accessor `apiId()`
    pub fn api_id(&self) -> String {
        self.api_id.clone()
    }

    /// java: the record accessor `neuId()`
    pub fn neu_id(&self) -> String {
        self.neu_id.clone()
    }

}
