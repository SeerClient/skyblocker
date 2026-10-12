//! Ported from de.hysky.skyblocker.utils.ws.message.DungeonRoomMatchMessage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Direction;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Shape;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Type;
use crate::de::hysky::skyblocker::utils::codec_utils;
use crate::de::hysky::skyblocker::utils::ws::message::message::Message;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::uuid::Uuid;
use mcrust::org::joml::vector2i::Vector2i;

/// java: public static final String TYPE = "room_match";
pub const TYPE: &str = "room_match";
/// java: public static final Codec<DungeonRoomMatchMessage> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("type").forGetter(DungeonRoomMatchMessage::type), UUIDUtil.STRING_
pub static CODEC: compat::MainThread<CodecRef<DungeonRoomMatchMessage>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply7(record_codec_builder::of(|o: &DungeonRoomMatchMessage| o.r#type(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "type")), record_codec_builder::of(|o: &DungeonRoomMatchMessage| o.sender(), codec::field_of(mcrust::net::minecraft::core::uuid_util::string_codec(), "uuid")), record_codec_builder::of(|o: &DungeonRoomMatchMessage| o.room_type(), codec::field_of(room::TYPE_CODEC.get_mut().clone(), "roomType")), record_codec_builder::of(|o: &DungeonRoomMatchMessage| o.shape(), codec::field_of(room::SHAPE_CODEC.get_mut().clone(), "shape")), record_codec_builder::of(|o: &DungeonRoomMatchMessage| o.direction(), codec::field_of(room::DIRECTION_CODEC.get_mut().clone(), "direction")), record_codec_builder::of(|o: &DungeonRoomMatchMessage| o.room(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "room")), record_codec_builder::of(|o: &DungeonRoomMatchMessage| o.pos(), codec::field_of(codec::list_sized(codec_utils::VECTOR_2I.get_mut().clone(), 1, 5), "pos")), move |__a0, __a1, __a2, __a3, __a4, __a5, __a6| DungeonRoomMatchMessage::new(__a0.clone(), __a1.clone(), __a2.clone(), __a3.clone(), __a4.clone(), __a5.clone(), __a6.clone()))));

/// java: public record DungeonRoomMatchMessage(String type, UUID sender, Room.Type roomType, Room.Shape shape, Room.Direction direction, String room, List<Vector2ic> pos) implements Message<DungeonRoomMatchMessage>
#[derive(Clone, Debug, PartialEq)]
pub struct DungeonRoomMatchMessage {
    /// java: String type
    pub r#type: String,
    /// java: UUID sender
    pub sender: Uuid,
    /// java: Room.Type roomType
    pub room_type: Type,
    /// java: Room.Shape shape
    pub shape: Shape,
    /// java: Room.Direction direction
    pub direction: Direction,
    /// java: String room
    pub room: String,
    /// java: List<Vector2ic> pos
    pub pos: Vec<Vector2i>,
}

impl DungeonRoomMatchMessage {
    /// java: public DungeonRoomMatchMessage(UUID uuid, Room.Type roomType, Room.Shape shape, Room.Direction direction, String room, List<Vector2ic> pos)
    pub fn new(mut uuid: Uuid, mut room_type: Type, mut shape: Shape, mut direction: Direction, mut room: String, mut pos: Vec<Vector2i>) -> Self {
        let mut this = Self::new(TYPE.to_string(), uuid.clone(), room_type, shape, direction, room.clone(), pos.clone());
        this
    }

    /// java: the canonical record constructor
    pub fn new(mut r#type: String, mut sender: Uuid, mut room_type: Type, mut shape: Shape, mut direction: Direction, mut room: String, mut pos: Vec<Vector2i>) -> Self {
        Self { r#type, sender, room_type, shape, direction, room, pos }
    }

    /// java: the record accessor `type()`
    pub fn r#type(&self) -> String {
        self.r#type.clone()
    }

    /// java: the record accessor `sender()`
    pub fn sender(&self) -> Uuid {
        self.sender.clone()
    }

    /// java: the record accessor `roomType()`
    pub fn room_type(&self) -> Type {
        self.room_type
    }

    /// java: the record accessor `shape()`
    pub fn shape(&self) -> Shape {
        self.shape
    }

    /// java: the record accessor `direction()`
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// java: the record accessor `room()`
    pub fn room(&self) -> String {
        self.room.clone()
    }

    /// java: the record accessor `pos()`
    pub fn pos(&self) -> Vec<Vector2i> {
        self.pos.clone()
    }

}

impl Message<crate::de::hysky::skyblocker::utils::ws::message::dungeon_room_match_message::DungeonRoomMatchMessage> for DungeonRoomMatchMessage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Codec<DungeonRoomMatchMessage> getCodec()
    fn get_codec(&self) -> CodecRef<DungeonRoomMatchMessage> {
        return CODEC.get_mut().clone();
    }

}
