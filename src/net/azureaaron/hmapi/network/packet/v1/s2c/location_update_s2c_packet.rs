//! Ported from net.azureaaron.hmapi.network.packet.v1.s2c.LocationUpdateS2CPacket.

use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBufReader;

/// java: public record LocationUpdateS2CPacket(String serverName, Optional<String> serverType, Optional<String> lobbyName,
/// Optional<String> mode, Optional<String> map) implements HypixelS2CPacket
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationUpdateS2CPacket {
    pub server_name: String,
    pub server_type: Option<String>,
    pub lobby_name: Option<String>,
    pub mode: Option<String>,
    pub map: Option<String>,
}

impl LocationUpdateS2CPacket {
    /// java: public static final CustomPacketPayload.Type<HypixelS2CPacket> ID = .. "hyevent:location"
    pub const ID: &'static str = "hyevent:location";

    /// java: PACKET_CODEC = StreamCodec.composite(ByteBufCodecs.STRING_UTF8, .., ByteBufCodecs.optional(ByteBufCodecs.STRING_UTF8) x4, ..)
    pub fn decode(buf: &mut FriendlyByteBufReader<'_>) -> Result<LocationUpdateS2CPacket, String> {
        let server_name = buf.read_utf()?;
        let server_type = buf.read_nullable(|b| b.read_utf())?;
        let lobby_name = buf.read_nullable(|b| b.read_utf())?;
        let mode = buf.read_nullable(|b| b.read_utf())?;
        let map = buf.read_nullable(|b| b.read_utf())?;
        Ok(LocationUpdateS2CPacket { server_name, server_type, lobby_name, mode, map })
    }
}
