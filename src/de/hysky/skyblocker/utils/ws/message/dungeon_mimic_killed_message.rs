//! Ported from de.hysky.skyblocker.utils.ws.message.DungeonMimicKilledMessage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::uuid::Uuid;

/// java: public static final String TYPE = "mimic_killed";
pub const TYPE: &str = "mimic_killed";
/// java: public static final Codec<DungeonMimicKilledMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("type").forGetter(DungeonMimicKilledMessage::type), UUIDUtil.STR
pub static CODEC: compat::MainThread<CodecRef<DungeonMimicKilledMessage>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &DungeonMimicKilledMessage| o.r#type(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "type")), record_codec_builder::of(|o: &DungeonMimicKilledMessage| o.sender(), codec::field_of(mcrust::net::minecraft::core::uuid_util::string_codec(), "uuid")), move |__a0, __a1| DungeonMimicKilledMessage::new_canonical(__a0.clone(), __a1.clone()))));

/// java: public record DungeonMimicKilledMessage(String type, UUID sender) implements Message<DungeonMimicKilledMessage>
#[derive(Clone, Debug, PartialEq)]
pub struct DungeonMimicKilledMessage {
    /// java: String type
    pub r#type: String,
    /// java: UUID sender
    pub sender: Uuid,
}

impl DungeonMimicKilledMessage {
    /// java: public DungeonMimicKilledMessage(UUID uuid)
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

impl Message<crate::de::hysky::skyblocker::utils::ws::message::dungeon_mimic_killed_message::DungeonMimicKilledMessage> for DungeonMimicKilledMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Codec<DungeonMimicKilledMessage> getCodec()
    fn get_codec(&self) -> CodecRef<DungeonMimicKilledMessage> {
        return CODEC.get_mut().clone();
    }

}
