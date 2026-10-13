//! Ported from net.azureaaron.hmapi.network.packet.s2c.HypixelS2CPacket (with HypixelPacket and its `Unknown`).

use super::error_s2c_packet::ErrorS2CPacket;
use super::hello_s2c_packet::HelloS2CPacket;
use crate::net::azureaaron::hmapi::network::packet::v1::s2c::location_update_s2c_packet::LocationUpdateS2CPacket;
use crate::net::azureaaron::hmapi::network::packet::v1::s2c::player_info_s2c_packet::PlayerInfoS2CPacket;
use crate::net::azureaaron::hmapi::network::packet::v2::s2c::party_info_s2c_packet::PartyInfoS2CPacket;

/// java: public interface HypixelS2CPacket extends HypixelPacket - its implementations, and `NOP` (the anonymous packet
/// an unknown or unsupported version decodes to).
#[derive(Clone, Debug, PartialEq)]
pub enum HypixelS2CPacket {
    Hello(HelloS2CPacket),
    LocationUpdate(LocationUpdateS2CPacket),
    PlayerInfo(PlayerInfoS2CPacket),
    PartyInfo(PartyInfoS2CPacket),
    Error(ErrorS2CPacket),
    /// java: HypixelS2CPacket NOP = new HypixelS2CPacket() {..}
    Nop,
}
