//! Ported from de.hysky.skyblocker.utils.ws.Payload by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::ws::r#type;
use crate::de::hysky::skyblocker::utils::ws::r#type::Type;
use crate::de::hysky::skyblocker::utils::ws::service;
use crate::de::hysky::skyblocker::utils::ws::service::Service;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::com::mojang::serialization::dynamic::Dynamic;

/// java: static final Codec<Payload> CODEC = RecordCodecBuilder.create(instance -> instance.group( Type.CODEC.fieldOf("type").forGetter(Payload::type), Service.CODEC.fieldOf("service").forGetter(Payload::servi
pub static CODEC: compat::MainThread<CodecRef<Payload>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &Payload| o.r#type(), codec::field_of(r#type::CODEC.get_mut().clone(), "type")), record_codec_builder::of(|o: &Payload| o.service(), codec::field_of(service::CODEC.get_mut().clone(), "service")), record_codec_builder::of(|o: &Payload| o.server_id(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::string_codec(), "serverId")), record_codec_builder::of(|o: &Payload| o.message(), codec::optional_field_of_opt(mcrust::com::mojang::serialization::codec::passthrough(), "message")), move |__a0, __a1, __a2, __a3| Payload::new_canonical(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));

/// java: record Payload(Type type, Service service, Optional<String> serverId, Optional<Dynamic<?>> message)
#[derive(Clone, Debug, PartialEq)]
pub struct Payload {
    /// java: Type type
    pub r#type: Type,
    /// java: Service service
    pub service: Service,
    /// java: Optional<String> serverId
    pub server_id: Option<String>,
    /// java: Optional<Dynamic<?>> message
    pub message: Option<Dynamic>,
}

impl Payload {
    /// java: the canonical record constructor `Payload(Type type, Service service, Optional<String> serverId, Optional<Dynamic<?>> message)`
    pub fn new_canonical(r#type: Type, service: Service, server_id: Option<String>, message: Option<Dynamic>) -> Self {
        Self { r#type, service, server_id, message }
    }

    /// java: Payload(Type type, Service service, String serverId, Optional<Dynamic<?>> message)
    pub fn new(mut r#type: Type, mut service: Service, mut server_id: String, mut message: Option<Dynamic>) -> Self {
        // java: this(type, service, Optional.of(serverId), message);
        let mut this: Self = Self::new_canonical(r#type, service, Some(server_id), message);
        this
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> Type {
        self.r#type
    }

    /// java: the record accessor `service()`
    pub fn service(&self) -> Service {
        self.service
    }

    /// java: the record accessor `serverId()`
    pub fn server_id(&self) -> Option<String> {
        self.server_id.clone()
    }

    /// java: the record accessor `message()`
    pub fn message(&self) -> Option<Dynamic> {
        self.message.clone()
    }

}
