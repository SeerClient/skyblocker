//! Ported from de.hysky.skyblocker.utils.ws.message.Message by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: public sealed interface Message<T extends Message<T>> permits CrystalsWaypointMessage, CrystalsWaypointSubscribeMessage, DungeonBatKilledMessage, DungeonMimicKilledMessage, DungeonPrinceKilledMessage, DungeonRoomHideWaypointMessage, Dungeon
pub trait Message<T> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Codec<T> getCodec();
    fn get_codec(&self) -> CodecRef<T>;

}

/// java: `Message<? extends Message<?>>` - a message whose `T` the use site does not know (the wildcard
/// `WsMessageHandler` and `WsStateManager` take). Every message is one. Its operations are the two things those
/// classes do with such a message: `WsMessageHandler.encodeMessage`'s unchecked
/// `(Codec<Message<?>>) message.getCodec()` + `codec.encodeStart(ops, message)`, and printing it (`{}` in a log line:
/// the record's `toString`).
pub trait AnyMessage {
    /// `((Codec<Message<?>>) getCodec()).encodeStart(ops, this)`
    fn encode_start(&self, ops: &mcrust::com::mojang::serialization::dynamic_ops::Ops) -> mcrust::com::mojang::serialization::data_result::DataResult<mcrust::com::mojang::serialization::Obj>;
    /// `String.valueOf(this)`
    fn message_to_string(&self) -> String;
}

impl<T: Message<T> + std::fmt::Debug + 'static> AnyMessage for T {
    fn encode_start(&self, ops: &mcrust::com::mojang::serialization::dynamic_ops::Ops) -> mcrust::com::mojang::serialization::data_result::DataResult<mcrust::com::mojang::serialization::Obj> {
        use mcrust::com::mojang::serialization::encoder::Encoder as _;
        self.get_codec().encode_start(ops, self)
    }

    fn message_to_string(&self) -> String {
        format!("{:?}", self)
    }
}
