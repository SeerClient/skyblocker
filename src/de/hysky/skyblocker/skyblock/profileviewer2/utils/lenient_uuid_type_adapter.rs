//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.LenientUuidTypeAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::util::uuid::Uuid;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: public class LenientUuidTypeAdapter implements JsonSerializer<UUID>, JsonDeserializer<UUID>
#[derive(Clone)]
pub struct LenientUuidTypeAdapter {}

impl LenientUuidTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public UUID deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value, type_of_t: &crate::unresolved::Type, context: &crate::unresolved::JsonDeserializationContext) -> Uuid {
        // TODO(fabric2seer): `new JsonParseException(..)`
        //   java: return UUIDUtil.AUTHLIB_CODEC.parse(JsonOps.INSTANCE, json).getOrThrow(JsonParseException::new);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public JsonElement serialize(UUID src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &Uuid, type_of_src: &crate::unresolved::Type, context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        return (mcrust::net::minecraft::core::uuid_util::authlib_codec().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), src)).get_or_throw().unwrap();
    }

}
