//! Ported from de.hysky.skyblocker.utils.ws.message.DungeonRoomHideWaypointMessage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::uuid::Uuid;

/// java: public static final String TYPE = "room_hide_waypoint";
pub const TYPE: &str = "room_hide_waypoint";
/// java: public static final Codec<DungeonRoomHideWaypointMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("type").forGetter(DungeonRoomHideWaypointMessage::type), UU
pub static CODEC: compat::MainThread<CodecRef<DungeonRoomHideWaypointMessage>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply4(record_codec_builder::of(|o: &DungeonRoomHideWaypointMessage| o.r#type(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "type")), record_codec_builder::of(|o: &DungeonRoomHideWaypointMessage| o.sender(), codec::field_of(mcrust::net::minecraft::core::uuid_util::string_codec(), "uuid")), record_codec_builder::of(|o: &DungeonRoomHideWaypointMessage| o.room_name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "room")), record_codec_builder::of(|o: &DungeonRoomHideWaypointMessage| o.waypoint_hash(), codec::field_of(mcrust::com::mojang::serialization::codec::int_codec(), "waypointHash")), move |__a0, __a1, __a2, __a3| DungeonRoomHideWaypointMessage::new_canonical(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone()))));

/// java: public record DungeonRoomHideWaypointMessage(String type, UUID sender, String roomName, int waypointHash) implements Message<DungeonRoomHideWaypointMessage>
#[derive(Clone, Debug, PartialEq)]
pub struct DungeonRoomHideWaypointMessage {
    /// java: String type
    pub r#type: String,
    /// java: UUID sender
    pub sender: Uuid,
    /// java: String roomName
    pub room_name: String,
    /// java: int waypointHash
    pub waypoint_hash: i32,
}

impl DungeonRoomHideWaypointMessage {
    /// java: public DungeonRoomHideWaypointMessage(UUID uuid, String roomName, int waypointHash)
    pub fn new(mut uuid: Uuid, mut room_name: String, mut waypoint_hash: i32) -> Self {
        let mut this = Self::new_canonical(TYPE.to_string(), uuid.clone(), room_name.clone(), waypoint_hash);
        this
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut r#type: String, mut sender: Uuid, mut room_name: String, mut waypoint_hash: i32) -> Self {
        Self { r#type, sender, room_name, waypoint_hash }
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

    /// java: the record accessor `waypointHash()`
    pub fn waypoint_hash(&self) -> i32 {
        self.waypoint_hash
    }

}

impl Message<crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_hide_waypoint_message::DungeonRoomHideWaypointMessage> for DungeonRoomHideWaypointMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Codec<DungeonRoomHideWaypointMessage> getCodec()
    fn get_codec(&self) -> CodecRef<DungeonRoomHideWaypointMessage> {
        return CODEC.get_mut().clone();
    }

}
