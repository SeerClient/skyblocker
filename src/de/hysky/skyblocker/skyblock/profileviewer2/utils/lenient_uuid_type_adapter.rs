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
        // java: return UUIDUtil.AUTHLIB_CODEC.parse(JsonOps.INSTANCE, json).getOrThrow(JsonParseException::new);
        let _ = (type_of_t, context);
        mcrust::net::minecraft::core::uuid_util::authlib_codec().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(json)).get_or_throw().unwrap_or_else(|e| panic!("JsonParseException: {e}"))
    }

    /// java: @Override public JsonElement serialize(UUID src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &Uuid, type_of_src: &crate::unresolved::Type, context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        return compat::json_from_port(&(mcrust::net::minecraft::core::uuid_util::authlib_codec().encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), src)).get_or_throw().unwrap());
    }

}
