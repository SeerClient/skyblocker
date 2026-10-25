//! Ported from meteordevelopment.discordipc.connection.UnixConnection (discord-ipc 1.1).
//!
//! Java opens a non-blocking `SocketChannel` on the Unix domain socket and its read thread `select()`s and reads the
//! packet in three states (opcode, length, data). A blocking read of each part on the read thread's own handle of the
//! socket (`try_clone`) reads the same bytes in the same order; `close()` shuts the socket down, which ends the
//! thread's read with an error as closing the selector and channel ends Java's (`catch (Exception ignored)`).

#![cfg(unix)]

use crate::meteordevelopment::discordipc::connection::connection::{Connection, PacketCallback};
use crate::meteordevelopment::discordipc::opcode::Opcode;
use crate::meteordevelopment::discordipc::packet::Packet;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::Mutex;

/// java: private enum State
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Opcode,
    Length,
    Data,
}

/// java: public class UnixConnection extends Connection
pub struct UnixConnection {
    /// java: private final SocketChannel sc;
    sc: Mutex<UnixStream>,
    /// java: private final Consumer<Packet> callback;
    callback: PacketCallback,
}

impl UnixConnection {
    /// java: public UnixConnection(String name, Consumer<Packet> callback) throws IOException
    pub fn new(name: &str, callback: PacketCallback) -> std::io::Result<UnixConnection> {
        let sc = UnixStream::connect(name)?;
        let reader = sc.try_clone()?;
        let connection = UnixConnection { sc: Mutex::new(sc), callback };

        let callback = connection.callback.clone();
        std::thread::Builder::new().name("Discord IPC - Read thread".to_string()).spawn(move || run(reader, callback))?;

        Ok(connection)
    }
}

/// java: private void run()
fn run(mut sc: UnixStream, callback: PacketCallback) {
    let mut state = State::Opcode;

    let mut int_b = [0u8; 4];
    let mut data_b: Vec<u8> = Vec::new();

    let mut opcode: Option<Opcode> = None;

    let result: std::io::Result<()> = (|| -> std::io::Result<()> { loop {
        match state {
            State::Opcode => {
                sc.read_exact(&mut int_b)?;
                opcode = Some(Opcode::value_of(i32::from_le_bytes(int_b)));
                state = State::Length;
            }
            State::Length => {
                sc.read_exact(&mut int_b)?;
                let length = i32::from_le_bytes(int_b);
                data_b = vec![0u8; usize::try_from(length).map_err(|_| std::io::Error::other("NegativeArraySizeException"))?];
                state = State::Data;
            }
            State::Data => {
                sc.read_exact(&mut data_b)?;

                // java: Charset.defaultCharset().decode(dataB.rewind()).toString()
                let data = String::from_utf8_lossy(&data_b).into_owned();
                let json: serde_json::Value = serde_json::from_str(&data).map_err(std::io::Error::other)?;
                let serde_json::Value::Object(object) = json else {
                    return Err(std::io::Error::other("IllegalStateException: Not a JSON Object"));
                };
                callback(Packet::new(opcode.expect("opcode read first"), object));

                data_b = Vec::new();
                state = State::Opcode;
            }
        }
    } })();
    // java: } catch (Exception ignored) {}
    let _ = result;
}

impl Connection for UnixConnection {
    /// java: @Override protected void write(ByteBuffer buffer)
    fn write_buffer(&self, buffer: &[u8]) {
        // The stream's lock is released before the catch runs (`ConnectionMixin`'s redirect closes this connection).
        let result = self.sc.lock().unwrap_or_else(|p| p.into_inner()).write_all(buffer);
        if let Err(e) = result {
            // java: e.printStackTrace(); - @Redirect-ed by Skyblocker's `ConnectionMixin` (a mixin into this library,
            // which is part of the mod, so the redirect is applied at its call site).
            crate::mixins::connection_mixin::write(&e);
        }
    }

    /// java: @Override public void close()
    fn close(&self) {
        if let Err(e) = self.sc.lock().unwrap_or_else(|p| p.into_inner()).shutdown(std::net::Shutdown::Both) {
            eprintln!("java.io.IOException: {e}");
        }
    }
}
