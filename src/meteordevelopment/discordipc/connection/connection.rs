//! Ported from meteordevelopment.discordipc.connection.Connection (discord-ipc 1.1).
//!
//! `public abstract class Connection`: the two abstract methods are the trait; the concrete `write(Opcode, JsonObject)`
//! and the static `open` are below. Java chooses the transport at run time from `os.name` (`contains("win")`); the
//! Windows branch is the named pipe (`WinConnection`), every other OS the Unix domain socket (`UnixConnection`), which
//! here is the build target's OS.
//!
//! The Windows pipe path: the published 1.1 jar opens `\\?\pipe\discord-ipc-<i>`; upstream's fix #3 (2026-05-25,
//! "the ambiguous path definition here would cause the pipe to be defined incorrectly on certain machines, failing the
//! connection") changed it to the documented `\\.\pipe\discord-ipc-<i>`, which this opens.

use crate::meteordevelopment::discordipc::opcode::Opcode;
use crate::meteordevelopment::discordipc::packet::Packet;
use std::sync::Arc;

/// java: `Consumer<Packet> callback` - called on the read thread
pub type PacketCallback = Arc<dyn Fn(Packet) + Send + Sync>;

/// java: private final static String[] UNIX_TEMP_PATHS = { "XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP" };
pub const UNIX_TEMP_PATHS: [&str; 4] = ["XDG_RUNTIME_DIR", "TMPDIR", "TMP", "TEMP"];

/// java: public abstract class Connection
pub trait Connection: Send + Sync {
    /// java: protected abstract void write(ByteBuffer buffer);
    fn write_buffer(&self, buffer: &[u8]);

    /// java: public abstract void close();
    fn close(&self);
}

impl dyn Connection {
    /// java: public void write(Opcode opcode, JsonObject o)
    pub fn write(&self, opcode: Opcode, mut o: serde_json::Map<String, serde_json::Value>) {
        o.insert("nonce".into(), serde_json::Value::String(random_uuid_string()));

        // java: o.toString().getBytes() - the platform charset (UTF-8)
        let d = serde_json::Value::Object(o).to_string().into_bytes();
        let mut packet: Vec<u8> = Vec::with_capacity(d.len() + 8);
        // java: packet.putInt(Integer.reverseBytes(opcode.ordinal())) - a big-endian put of the reversed int is the
        // little-endian int
        packet.extend_from_slice(&opcode.ordinal().to_le_bytes());
        packet.extend_from_slice(&(d.len() as i32).to_le_bytes());
        packet.extend_from_slice(&d);

        self.write_buffer(&packet);
    }
}

/// java: public static Connection open(Consumer<Packet> callback)
pub fn open(callback: PacketCallback) -> Option<Box<dyn Connection>> {
    // Windows
    #[cfg(windows)]
    {
        use crate::meteordevelopment::discordipc::connection::win_connection::WinConnection;
        for i in 0..10 {
            if let Ok(connection) = WinConnection::new(&format!("\\\\.\\pipe\\discord-ipc-{i}"), callback.clone()) {
                return Some(Box::new(connection));
            }
        }
    }
    // Unix
    #[cfg(not(windows))]
    {
        use crate::meteordevelopment::discordipc::connection::unix_connection::UnixConnection;
        let mut name: Option<String> = None;

        for temp_path in UNIX_TEMP_PATHS {
            name = std::env::var(temp_path).ok();
            if name.is_some() {
                break;
            }
        }

        let mut name = name.unwrap_or_else(|| "/tmp".to_string());
        name.push_str("/discord-ipc-");

        for i in 0..10 {
            if let Ok(connection) = UnixConnection::new(&format!("{name}{i}"), callback.clone()) {
                return Some(Box::new(connection));
            }
        }
    }

    None
}

/// `UUID.randomUUID().toString()` - a version-4 UUID from the OS-seeded hasher keys and the clock
fn random_uuid_string() -> String {
    use std::hash::{BuildHasher, Hasher};
    let mut bytes = [0u8; 16];
    for (half, chunk) in bytes.chunks_mut(8).enumerate() {
        let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
        hasher.write_u128(
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0),
        );
        hasher.write_usize(half);
        chunk.copy_from_slice(&hasher.finish().to_be_bytes());
    }
    // version 4, IETF variant
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!("{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32])
}
