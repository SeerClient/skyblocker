//! Ported from net.azureaaron.hmapi.network.packet.c2s.HypixelC2SPacket.

use super::party_info_c2s_packet::PartyInfoC2SPacket;
use super::player_info_c2s_packet::PlayerInfoC2SPacket;
use super::register_c2s_packet::RegisterC2SPacket;
use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBuf;

/// java: public sealed interface HypixelC2SPacket extends HypixelPacket permits PartyInfoC2SPacket, PlayerInfoC2SPacket, RegisterC2SPacket
#[derive(Clone, Debug)]
pub enum HypixelC2SPacket {
    PartyInfo(PartyInfoC2SPacket),
    PlayerInfo(PlayerInfoC2SPacket),
    Register(RegisterC2SPacket),
}

impl HypixelC2SPacket {
    /// java: CustomPacketPayload.type() - the channel the packet is sent on
    pub fn r#type(&self) -> &'static str {
        match self {
            HypixelC2SPacket::PartyInfo(_) => PartyInfoC2SPacket::ID,
            HypixelC2SPacket::PlayerInfo(_) => PlayerInfoC2SPacket::ID,
            HypixelC2SPacket::Register(_) => RegisterC2SPacket::ID,
        }
    }

    /// java: HypixelCustomPayloadCodecs.C2S_PACKET_CODEC.encode(buf, payload) - the packet's own codec
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        let mut buf = FriendlyByteBuf::new();
        match self {
            HypixelC2SPacket::PartyInfo(packet) => packet.encode(&mut buf),
            HypixelC2SPacket::PlayerInfo(packet) => packet.encode(&mut buf),
            HypixelC2SPacket::Register(packet) => packet.encode(&mut buf)?,
        }
        Ok(buf.data)
    }
}
