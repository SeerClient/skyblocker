//! Ported from net.azureaaron.hmapi.network.packet.v1.s2c.PlayerInfoS2CPacket.

use crate::net::azureaaron::hmapi::data::rank::monthly_package_rank::MonthlyPackageRank;
use crate::net::azureaaron::hmapi::data::rank::package_rank::PackageRank;
use crate::net::azureaaron::hmapi::data::rank::player_rank::PlayerRank;
use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBufReader;

/// java: public record PlayerInfoS2CPacket(PlayerRank playerRank, PackageRank packageRank, MonthlyPackageRank monthlyPackageRank,
/// Optional<String> prefix) implements HypixelS2CPacket
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerInfoS2CPacket {
    pub player_rank: PlayerRank,
    pub package_rank: PackageRank,
    pub monthly_package_rank: MonthlyPackageRank,
    pub prefix: Option<String>,
}

impl PlayerInfoS2CPacket {
    /// java: public static final CustomPacketPayload.Type<HypixelS2CPacket> ID = .. "hypixel:player_info"
    pub const ID: &'static str = "hypixel:player_info";

    /// java: PACKET_CODEC = StreamCodec.composite(ByteBufCodecs.idMapper(PlayerRank.BY_ID, ..), ByteBufCodecs.idMapper(PackageRank.BY_ID, ..),
    /// ByteBufCodecs.idMapper(MonthlyPackageRank.BY_ID, ..), ByteBufCodecs.optional(ByteBufCodecs.STRING_UTF8), ..)
    pub fn decode(buf: &mut FriendlyByteBufReader<'_>) -> Result<PlayerInfoS2CPacket, String> {
        let player_rank = PlayerRank::by_id(buf.read_var_int()?);
        let package_rank = PackageRank::by_id(buf.read_var_int()?);
        let monthly_package_rank = MonthlyPackageRank::by_id(buf.read_var_int()?);
        let prefix = buf.read_nullable(|b| b.read_utf())?;
        Ok(PlayerInfoS2CPacket { player_rank, package_rank, monthly_package_rank, prefix })
    }
}
