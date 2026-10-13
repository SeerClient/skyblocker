//! Ported from net.azureaaron.hmapi.network.packet.c2s.PlayerInfoC2SPacket.

use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBuf;

/// java: public record PlayerInfoC2SPacket(int version) implements HypixelC2SPacket
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerInfoC2SPacket {
    pub version: i32,
}

impl PlayerInfoC2SPacket {
    /// java: public static final CustomPacketPayload.Type<PlayerInfoC2SPacket> ID = .. "hypixel:player_info"
    pub const ID: &'static str = "hypixel:player_info";

    pub fn new(version: i32) -> PlayerInfoC2SPacket {
        PlayerInfoC2SPacket { version }
    }

    /// java: PACKET_CODEC = StreamCodec.composite(ByteBufCodecs.VAR_INT, PlayerInfoC2SPacket::version, ..)
    pub fn encode(&self, buf: &mut FriendlyByteBuf) {
        buf.write_var_int(self.version);
    }
}
