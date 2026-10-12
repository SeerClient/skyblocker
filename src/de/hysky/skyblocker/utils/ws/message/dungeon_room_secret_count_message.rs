//! Ported from de.hysky.skyblocker.utils.ws.message.DungeonRoomSecretCountMessage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::uuid::Uuid;

/// java: public static final String TYPE = "room_secret_count";
pub const TYPE: &str = "room_secret_count";
/// java: public static final Codec<DungeonRoomSecretCountMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("type").forGetter(DungeonRoomSecretCountMessage::type), UUID
pub static CODEC: compat::MainThread<CodecRef<DungeonRoomSecretCountMessage>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &DungeonRoomSecretCountMessage| o.r#type(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "type")), record_codec_builder::of(|o: &DungeonRoomSecretCountMessage| o.sender(), codec::field_of(mcrust::net::minecraft::core::uuid_util::string_codec(), "uuid")), record_codec_builder::of(|o: &DungeonRoomSecretCountMessage| o.room_name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "room")), record_codec_builder::of(|o: &DungeonRoomSecretCountMessage| o.secret_count(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "secretCount")), move |__a0, __a1, __a2, __a3| DungeonRoomSecretCountMessage::new_canonical(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));

/// java: public record DungeonRoomSecretCountMessage(String type, UUID sender, String roomName, int secretCount) implements Message<DungeonRoomSecretCountMessage>
#[derive(Clone, Debug, PartialEq)]
pub struct DungeonRoomSecretCountMessage {
    /// java: String type
    pub r#type: String,
    /// java: UUID sender
    pub sender: Uuid,
    /// java: String roomName
    pub room_name: String,
    /// java: int secretCount
    pub secret_count: i32,
}

impl DungeonRoomSecretCountMessage {
    /// java: public DungeonRoomSecretCountMessage(UUID uuid, String roomName, int secretCount)
    pub fn new(mut uuid: Uuid, mut room_name: String, mut secret_count: i32) -> Self {
        let mut this = Self::new_canonical(TYPE.to_string(), uuid.clone(), room_name.clone(), secret_count);
        this
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut r#type: String, mut sender: Uuid, mut room_name: String, mut secret_count: i32) -> Self {
        Self { r#type, sender, room_name, secret_count }
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> String {
        self.r#type.clone()
    }

    /// java: the record accessor `sender()`
    pub fn sender(&self) -> Uuid {
        self.sender.clone()
    }

    /// java: the record accessor `roomName()`
    pub fn room_name(&self) -> String {
        self.room_name.clone()
    }

    /// java: the record accessor `secretCount()`
    pub fn secret_count(&self) -> i32 {
        self.secret_count
    }

}

impl Message<crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_secret_count_message::DungeonRoomSecretCountMessage> for DungeonRoomSecretCountMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Codec<DungeonRoomSecretCountMessage> getCodec()
    fn get_codec(&self) -> CodecRef<DungeonRoomSecretCountMessage> {
        return CODEC.get_mut().clone();
    }

}
