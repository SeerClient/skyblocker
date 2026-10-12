//! Ported from de.hysky.skyblocker.utils.ws.message.DungeonPrinceKilledMessage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::uuid::Uuid;

/// java: public static final String TYPE = "prince_killed";
pub const TYPE: &str = "prince_killed";
/// java: public static final Codec<DungeonPrinceKilledMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("type").forGetter(DungeonPrinceKilledMessage::type), UUIDUtil.S
pub static CODEC: compat::MainThread<CodecRef<DungeonPrinceKilledMessage>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &DungeonPrinceKilledMessage| o.r#type(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "type")), record_codec_builder::of(|o: &DungeonPrinceKilledMessage| o.sender(), codec::field_of(mcrust::net::minecraft::core::uuid_util::string_codec(), "uuid")), move |__a0, __a1| DungeonPrinceKilledMessage::new_canonical(__a0.clone(), __a1.clone()))));

/// java: public record DungeonPrinceKilledMessage(String type, UUID sender) implements Message<DungeonPrinceKilledMessage>
#[derive(Clone, Debug, PartialEq)]
pub struct DungeonPrinceKilledMessage {
    /// java: String type
    pub r#type: String,
    /// java: UUID sender
    pub sender: Uuid,
}

impl DungeonPrinceKilledMessage {
    /// java: public DungeonPrinceKilledMessage(UUID uuid)
    pub fn new(mut uuid: Uuid) -> Self {
        let mut this = Self::new_canonical(TYPE.to_string(), uuid.clone());
        this
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut r#type: String, mut sender: Uuid) -> Self {
        Self { r#type, sender }
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> String {
        self.r#type.clone()
    }

    /// java: the record accessor `sender()`
    pub fn sender(&self) -> Uuid {
        self.sender.clone()
    }

}

impl Message<crate::de::hysky::skyblocker::utils::ws::message::dungeon_prince_killed_message::DungeonPrinceKilledMessage> for DungeonPrinceKilledMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Codec<DungeonPrinceKilledMessage> getCodec()
    fn get_codec(&self) -> CodecRef<DungeonPrinceKilledMessage> {
        return CODEC.get_mut().clone();
    }

}
