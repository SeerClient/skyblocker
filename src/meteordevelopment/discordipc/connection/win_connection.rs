//! Ported from meteordevelopment.discordipc.connection.WinConnection (discord-ipc 1.1).
//!
//! `new RandomAccessFile(name, "rw")` over the Discord client's named pipe: a synchronous read and write handle
//! (`CreateFileW` with `GENERIC_READ | GENERIC_WRITE`, what `OpenOptions::read(true).write(true)` opens). A blocking
//! `ReadFile` on a synchronous pipe handle holds up every other I/O on it, so Java's read thread never reads before
//! the bytes are there: `readFully` polls `raf.length()` (the bytes waiting in the pipe) every 100 ms. The bytes
//! waiting are `PeekNamedPipe`'s `lpTotalBytesAvail`.
//!
//! `raf.close()` makes the read thread's next call throw, which ends it (`catch (Exception ignored)`); here the handle
//! is shared with the thread and `close` takes it away.

#![cfg(windows)]

use crate::meteordevelopment::discordipc::connection::connection::{Connection, PacketCallback};
use crate::meteordevelopment::discordipc::opcode::Opcode;
use crate::meteordevelopment::discordipc::packet::Packet;
use std::fs::File;
use std::io::{Read, Write};
use std::os::windows::io::AsRawHandle;
use std::sync::{Arc, Mutex};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn PeekNamedPipe(
        h_named_pipe: *mut std::ffi::c_void,
        lp_buffer: *mut std::ffi::c_void,
        n_buffer_size: u32,
        lp_bytes_read: *mut u32,
        lp_total_bytes_avail: *mut u32,
        lp_bytes_left_this_message: *mut u32,
    ) -> i32;
}

/// java: public class WinConnection extends Connection
pub struct WinConnection {
    /// java: private final RandomAccessFile raf;
    raf: Arc<Mutex<Option<File>>>,
    /// java: private final Consumer<Packet> callback;
    callback: PacketCallback,
}

impl WinConnection {
    /// java: WinConnection(String name, Consumer<Packet> callback) throws IOException
    pub fn new(name: &str, callback: PacketCallback) -> std::io::Result<WinConnection> {
        let raf = std::fs::OpenOptions::new().read(true).write(true).open(name)?;
        let connection = WinConnection { raf: Arc::new(Mutex::new(Some(raf))), callback };

        let raf = connection.raf.clone();
        let callback = connection.callback.clone();
        std::thread::Builder::new().name("Discord IPC - Read thread".to_string()).spawn(move || run(raf, callback))?;

        Ok(connection)
    }
}

/// java: private void run()
fn run(raf: Arc<Mutex<Option<File>>>, callback: PacketCallback) {
    let mut int_b = [0u8; 4];

    let result: std::io::Result<()> = (|| -> std::io::Result<()> { loop {
        // Opcode
        read_fully(&raf, &mut int_b)?;
        let opcode = Opcode::value_of(i32::from_le_bytes(int_b));

        // Length
        read_fully(&raf, &mut int_b)?;
        let length = i32::from_le_bytes(int_b);

        // Data
        let mut data_b = vec![0u8; usize::try_from(length).map_err(|_| std::io::Error::other("NegativeArraySizeException"))?];
        read_fully(&raf, &mut data_b)?;
        let data = String::from_utf8_lossy(&data_b).into_owned();

        // Call callback - java: JsonParser.parseString(data).getAsJsonObject()
        let json: serde_json::Value = serde_json::from_str(&data).map_err(std::io::Error::other)?;
        let serde_json::Value::Object(object) = json else {
            return Err(std::io::Error::other("IllegalStateException: Not a JSON Object"));
        };
        callback(Packet::new(opcode, object));
    } })();
    // java: } catch (Exception ignored) {}
    let _ = result;
}

/// The bytes waiting in the pipe (`raf.length()`)
fn available(file: &File) -> std::io::Result<u32> {
    let mut total: u32 = 0;
    // SAFETY: a valid pipe handle owned by `file`; only the out-count is written.
    let ok = unsafe {
        PeekNamedPipe(file.as_raw_handle(), std::ptr::null_mut(), 0, std::ptr::null_mut(), &mut total, std::ptr::null_mut())
    };
    if ok == 0 { Err(std::io::Error::last_os_error()) } else { Ok(total) }
}

/// java: private void readFully(ByteBuffer buffer) throws IOException
fn read_fully(raf: &Mutex<Option<File>>, buffer: &mut [u8]) -> std::io::Result<()> {
    let closed = || std::io::Error::other("Stream Closed");
    // java: while (raf.length() < buffer.remaining()) { Thread.onSpinWait(); Thread.sleep(100); }
    loop {
        let waiting = {
            let guard = raf.lock().unwrap_or_else(|p| p.into_inner());
            available(guard.as_ref().ok_or_else(closed)?)?
        };
        if waiting as usize >= buffer.len() {
            break;
        }
        std::hint::spin_loop();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // java: while (buffer.hasRemaining()) raf.getChannel().read(buffer);
    let mut guard = raf.lock().unwrap_or_else(|p| p.into_inner());
    let file = guard.as_mut().ok_or_else(closed)?;
    file.read_exact(buffer)
}

impl Connection for WinConnection {
    /// java: @Override protected void write(ByteBuffer buffer) - `raf.write(buffer.array())`
    fn write_buffer(&self, buffer: &[u8]) {
        let mut guard = self.raf.lock().unwrap_or_else(|p| p.into_inner());
        let result = match guard.as_mut() {
            Some(file) => file.write_all(buffer),
            None => Err(std::io::Error::other("Stream Closed")),
        };
        // The file's lock is released before the catch runs (`ConnectionMixin`'s redirect closes this connection).
        drop(guard);
        if let Err(e) = result {
            // java: e.printStackTrace(); - @Redirect-ed by Skyblocker's `ConnectionMixin` (a mixin into this library,
            // which is part of the mod, so the redirect is applied at its call site).
            crate::mixins::connection_mixin::write(&e);
        }
    }

    /// java: @Override public void close()
    fn close(&self) {
        // java: raf.close(); - dropping the file closes the handle
        self.raf.lock().unwrap_or_else(|p| p.into_inner()).take();
    }
}
