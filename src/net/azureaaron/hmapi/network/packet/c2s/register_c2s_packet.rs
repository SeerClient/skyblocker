//! Ported from net.azureaaron.hmapi.network.packet.c2s.RegisterC2SPacket.

use mcrust::net::minecraft::network::friendly_byte_buf::FriendlyByteBuf;

/// java: public record RegisterC2SPacket(int version, Object2IntMap<Identifier> eventsToRegister) implements HypixelC2SPacket
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterC2SPacket {
    pub version: i32,
    /// (an `Object2IntOpenHashMap`: written in its iteration order)
    pub events_to_register: Vec<(String, i32)>,
}

impl RegisterC2SPacket {
    /// java: public static final CustomPacketPayload.Type<RegisterC2SPacket> ID = .. "hypixel:register"
    pub const ID: &'static str = "hypixel:register";

    pub fn new(version: i32, events_to_register: Vec<(String, i32)>) -> RegisterC2SPacket {
        RegisterC2SPacket { version, events_to_register }
    }

    /// java: PACKET_CODEC = StreamCodec.composite(ByteBufCodecs.VAR_INT, ..version, ByteBufCodecs.map(Object2IntOpenHashMap::new,
    /// Identifier.STREAM_CODEC, ByteBufCodecs.VAR_INT, 5), ..eventsToRegister, ..)
    pub fn encode(&self, buf: &mut FriendlyByteBuf) -> Result<(), String> {
        buf.write_var_int(self.version);
        // ByteBufCodecs.map(.., maxSize = 5): `Too many entries` past the limit
        if self.events_to_register.len() > 5 {
            return Err(format!("Map is too big: {} > {}", self.events_to_register.len(), 5));
        }
        buf.write_var_int(self.events_to_register.len() as i32);
        for (identifier, version) in self.events_to_register.iter() {
            // Identifier.STREAM_CODEC - the id's string
            buf.write_utf(identifier)?;
            buf.write_var_int(*version);
        }
        Ok(())
    }
}
