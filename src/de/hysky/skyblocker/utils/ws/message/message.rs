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
