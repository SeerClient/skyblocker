//! Ported from meteordevelopment.discordipc.Opcode (discord-ipc 1.1).

/// java: public enum Opcode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Opcode {
    Handshake,
    Frame,
    Close,
    Ping,
    Pong,
}

/// java: private static final Opcode[] VALUES = values();
const VALUES: [Opcode; 5] = [Opcode::Handshake, Opcode::Frame, Opcode::Close, Opcode::Ping, Opcode::Pong];

impl Opcode {
    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: public static Opcode valueOf(int i) - `VALUES[i]`, an `ArrayIndexOutOfBoundsException` out of range
    pub fn value_of(i: i32) -> Opcode {
        *VALUES
            .get(usize::try_from(i).unwrap_or(usize::MAX))
            .unwrap_or_else(|| panic!("ArrayIndexOutOfBoundsException: Index {i} out of bounds for length 5"))
    }
}
