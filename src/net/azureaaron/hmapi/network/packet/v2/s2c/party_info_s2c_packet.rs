//! Ported from net.azureaaron.hmapi.network.packet.v2.s2c.PartyInfoS2CPacket.

use crate::net::azureaaron::hmapi::data::party::party_role::PartyRole;
use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBufReader;

/// java: public record PartyInfoS2CPacket(boolean inParty, @Nullable @Unmodifiable Map<UUID, PartyRole> members) implements HypixelS2CPacket
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartyInfoS2CPacket {
    pub in_party: bool,
    /// (an `Object2ReferenceOpenHashMap`, in the order the entries were read)
    pub members: Option<Vec<(mcrust::java::util::uuid::Uuid, PartyRole)>>,
}

impl PartyInfoS2CPacket {
    /// java: public static final CustomPacketPayload.Type<HypixelS2CPacket> ID = .. "hypixel:party_info"
    pub const ID: &'static str = "hypixel:party_info";

    /// java: PACKET_CODEC = PacketCodecUtils.dispatchConditionally(IN_PARTY_PACKET_CODEC, StreamCodec.unit(new PartyInfoS2CPacket(false, null)))
    pub fn decode(buf: &mut FriendlyByteBufReader<'_>) -> Result<PartyInfoS2CPacket, String> {
        if !buf.read_boolean()? {
            return Ok(PartyInfoS2CPacket { in_party: false, members: None });
        }
        // IN_PARTY_PACKET_CODEC: ByteBufCodecs.map(.., UUIDUtil.STREAM_CODEC, ByteBufCodecs.idMapper(i -> PartyRole.values()[i], ..))
        let members = buf.read_map(
            |b| b.read_uuid(),
            |b| {
                let i = b.read_var_int()?;
                PartyRole::VALUES.get(i as usize).copied().ok_or_else(|| format!("ArrayIndexOutOfBoundsException: Index {i} out of bounds for length 3"))
            },
        )?;
        Ok(PartyInfoS2CPacket { in_party: true, members: Some(members) })
    }
}
