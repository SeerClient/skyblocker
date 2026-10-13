//! Ported from net.azureaaron.hmapi.network.packet.s2c.HelloS2CPacket.

use crate::net::azureaaron::hmapi::data::server::environment::Environment;
use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBufReader;

/// java: public record HelloS2CPacket(Environment environment) implements HypixelS2CPacket
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelloS2CPacket {
    pub environment: Environment,
}

impl HelloS2CPacket {
    /// java: public static final CustomPacketPayload.Type<HypixelS2CPacket> ID = .. "hypixel:hello"
    pub const ID: &'static str = "hypixel:hello";

    /// java: the record accessor `environment()`
    pub fn environment(&self) -> Environment {
        self.environment
    }

    /// java: PACKET_CODEC = StreamCodec.composite(ByteBufCodecs.idMapper(i -> Environment.values()[i], Environment::ordinal), ..)
    pub fn decode(buf: &mut FriendlyByteBufReader<'_>) -> Result<HelloS2CPacket, String> {
        let i = buf.read_var_int()?;
        let environment = *Environment::VALUES.get(i as usize).ok_or_else(|| format!("ArrayIndexOutOfBoundsException: Index {i} out of bounds for length 3"))?;
        Ok(HelloS2CPacket { environment })
    }
}
