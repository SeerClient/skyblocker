//! Ported from net.azureaaron.hmapi.network.packet.s2c.ErrorS2CPacket.

use crate::net::azureaaron::hmapi::data::error::error_reason::ErrorReason;
use crate::net::azureaaron::hmapi::data::error::mod_api_error_reason::ModApiErrorReason;
use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBufReader;

/// java: public record ErrorS2CPacket(CustomPacketPayload.Type<HypixelS2CPacket> id, ErrorReason reason) implements HypixelS2CPacket
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorS2CPacket {
    /// the channel of the packet that failed
    pub id: &'static str,
    pub reason: ErrorReason,
}

impl ErrorS2CPacket {
    /// java: the record accessor `id()`
    pub fn id(&self) -> &'static str {
        self.id
    }

    /// java: the record accessor `reason()`
    pub fn reason(&self) -> ErrorReason {
        self.reason
    }

    /// java: PACKET_CODEC = payloadId -> StreamCodec.composite(ByteBufCodecs.idMapper(ModApiErrorReason::tryResolveReason, ErrorReason::id), ..)
    pub fn decode(payload_id: &'static str, buf: &mut FriendlyByteBufReader<'_>) -> Result<ErrorS2CPacket, String> {
        let reason = ModApiErrorReason::try_resolve_reason(buf.read_var_int()?);
        Ok(ErrorS2CPacket { id: payload_id, reason })
    }
}
