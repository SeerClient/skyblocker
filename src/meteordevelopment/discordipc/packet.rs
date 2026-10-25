//! Ported from meteordevelopment.discordipc.Packet (discord-ipc 1.1).

use crate::meteordevelopment::discordipc::opcode::Opcode;

/// java: public record Packet(Opcode opcode, JsonObject data)
#[derive(Clone, Debug)]
pub struct Packet {
    opcode: Opcode,
    data: serde_json::Map<String, serde_json::Value>,
}

impl Packet {
    /// java: the canonical record constructor
    pub fn new(opcode: Opcode, data: serde_json::Map<String, serde_json::Value>) -> Packet {
        Packet { opcode, data }
    }

    /// java: opcode()
    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    /// java: data()
    pub fn data(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.data
    }
}
