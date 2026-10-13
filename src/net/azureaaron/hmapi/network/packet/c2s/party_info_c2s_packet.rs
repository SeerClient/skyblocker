//! Ported from net.azureaaron.hmapi.network.packet.c2s.PartyInfoC2SPacket.

use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBuf;

/// java: public record PartyInfoC2SPacket(int version) implements HypixelC2SPacket
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartyInfoC2SPacket {
    pub version: i32,
}

impl PartyInfoC2SPacket {
    /// java: public static final CustomPacketPayload.Type<PartyInfoC2SPacket> ID = .. "hypixel:party_info"
    pub const ID: &'static str = "hypixel:party_info";

    pub fn new(version: i32) -> PartyInfoC2SPacket {
        PartyInfoC2SPacket { version }
    }

    /// java: PACKET_CODEC = StreamCodec.composite(ByteBufCodecs.VAR_INT, PartyInfoC2SPacket::version, ..)
    pub fn encode(&self, buf: &mut FriendlyByteBuf) {
        buf.write_var_int(self.version);
    }
}
