//! Ported from meteordevelopment.discordipc.DiscordIPC (discord-ipc 1.1).
//!
//! The class's static fields are one `State` behind a lock: the read thread (`onPacket`) and the caller's thread both
//! reach them, as they do in Java (which does not synchronise). The lock is never held while a callback (`onError`,
//! `onReady`) runs, so a callback may call back into this class.

use crate::meteordevelopment::discordipc::connection::connection::{self, Connection};
use crate::meteordevelopment::discordipc::ipc_user::IPCUser;
use crate::meteordevelopment::discordipc::opcode::Opcode;
use crate::meteordevelopment::discordipc::packet::Packet;
use crate::meteordevelopment::discordipc::rich_presence::RichPresence;
use std::sync::{Arc, LazyLock, Mutex, MutexGuard};

/// java: `BiConsumer<Integer, String> onError`
pub type ErrorCallback = Arc<dyn Fn(i32, String) + Send + Sync>;
/// java: `Runnable onReady`
pub type ReadyCallback = Arc<dyn Fn() + Send + Sync>;

/// The static fields
struct State {
    /// java: private static BiConsumer<Integer, String> onError = DiscordIPC::defaultErrorCallback;
    on_error: Option<ErrorCallback>,
    /// java: private static Connection c;
    c: Option<Arc<dyn Connection>>,
    /// java: private static Runnable onReady;
    on_ready: Option<ReadyCallback>,
    /// java: private static boolean receivedDispatch;
    received_dispatch: bool,
    /// java: private static JsonObject queuedActivity;
    queued_activity: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: private static IPCUser user;
    user: Option<IPCUser>,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| {
    Mutex::new(State {
        on_error: Some(Arc::new(DiscordIPC::default_error_callback)),
        c: None,
        on_ready: None,
        received_dispatch: false,
        queued_activity: None,
        user: None,
    })
});

fn state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap_or_else(|p| p.into_inner())
}

/// java: public class DiscordIPC
pub struct DiscordIPC;

impl DiscordIPC {
    /// java: public static void setOnError(BiConsumer<Integer, String> onError) - sets the error callback
    pub fn set_on_error(on_error: Option<ErrorCallback>) {
        state().on_error = on_error;
    }

    /// java: public static boolean start(long appId, Runnable onReady)
    ///
    /// Tries to open a connection to a locally running Discord instance. `on_ready` is called when a successful
    /// connection happens, from that point `get_user()` will return a non-null object up until `stop()` is called or
    /// an error happens. Answers true if a connection was opened successfully.
    pub fn start(app_id: i64, on_ready: Option<ReadyCallback>) -> bool {
        // Open connection
        let Some(c) = connection::open(Arc::new(DiscordIPC::on_packet)) else {
            state().c = None;
            return false;
        };
        let c: Arc<dyn Connection> = Arc::from(c);
        {
            let mut state = state();
            state.c = Some(c.clone());
            state.on_ready = on_ready;
        }

        // Handshake
        let mut o = serde_json::Map::new();
        o.insert("v".into(), serde_json::Value::from(1));
        o.insert("client_id".into(), serde_json::Value::String(app_id.to_string()));
        c.write(Opcode::Handshake, o);

        true
    }

    /// java: public static boolean isConnected() - true if it is currently connected to a local Discord instance
    pub fn is_connected() -> bool {
        state().c.is_some()
    }

    /// java: public static IPCUser getUser() - the user that is logged in in the connected Discord instance
    pub fn get_user() -> Option<IPCUser> {
        state().user.clone()
    }

    /// java: public static void setActivity(RichPresence presence) - sets account's activity
    pub fn set_activity(presence: &RichPresence) {
        let send = {
            let mut state = state();
            if state.c.is_none() {
                return;
            }
            state.queued_activity = Some(presence.to_json());
            state.received_dispatch
        };
        if send {
            DiscordIPC::send_activity();
        }
    }

    /// java: public static void stop() - closes the connection to the locally running Discord instance if it is open
    pub fn stop() {
        let mut state = state();
        if let Some(c) = state.c.take() {
            c.close();

            state.on_ready = None;
            state.received_dispatch = false;
            state.queued_activity = None;
            state.user = None;
        }
    }

    /// java: private static void sendActivity()
    fn send_activity() {
        let (c, queued_activity) = {
            let mut state = state();
            // java: queuedActivity = null; (after the write) - taken here, written below
            (state.c.clone(), state.queued_activity.take())
        };
        let Some(c) = c else {
            // java: c.write(..) on a null `c` - a NullPointerException
            panic!("NullPointerException: DiscordIPC.c is null");
        };
        let mut args = serde_json::Map::new();
        args.insert("pid".into(), serde_json::Value::from(DiscordIPC::get_pid()));
        // java: args.add("activity", queuedActivity) - `add(.., null)` adds a JSON null
        args.insert("activity".into(), queued_activity.map(serde_json::Value::Object).unwrap_or(serde_json::Value::Null));

        let mut o = serde_json::Map::new();
        o.insert("cmd".into(), serde_json::Value::String("SET_ACTIVITY".into()));
        o.insert("args".into(), serde_json::Value::Object(args));

        c.write(Opcode::Frame, o);
    }

    /// java: private static void onPacket(Packet packet)
    fn on_packet(packet: Packet) {
        let get_int = |o: &serde_json::Map<String, serde_json::Value>, k: &str| -> i32 {
            // java: get(k).getAsInt() - a missing member is a NullPointerException
            match o.get(k) {
                Some(serde_json::Value::Number(n)) => n.as_i64().map(|v| v as i32).unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i32),
                Some(serde_json::Value::String(s)) => s.parse::<i32>().unwrap_or_else(|e| panic!("NumberFormatException: {e}")),
                _ => panic!("NullPointerException: no int member {k}"),
            }
        };
        let get_string = |o: &serde_json::Map<String, serde_json::Value>, k: &str| -> String {
            match o.get(k) {
                Some(serde_json::Value::String(s)) => s.clone(),
                Some(serde_json::Value::Null) | None => panic!("NullPointerException: no string member {k}"),
                Some(other) => other.to_string(),
            }
        };

        // Close
        if packet.opcode() == Opcode::Close {
            let on_error = state().on_error.clone();
            if let Some(on_error) = on_error {
                on_error(get_int(packet.data(), "code"), get_string(packet.data(), "message"));
            }
            DiscordIPC::stop();
        }
        // Frame
        else if packet.opcode() == Opcode::Frame {
            // Error
            if packet.data().contains_key("evt") && get_string(packet.data(), "evt") == "ERROR" {
                let d = match packet.data().get("data") {
                    Some(serde_json::Value::Object(d)) => d.clone(),
                    _ => panic!("ClassCastException: data is not a JsonObject"),
                };
                let on_error = state().on_error.clone();
                if let Some(on_error) = on_error {
                    on_error(get_int(&d, "code"), get_string(&d, "message"));
                }
            }
            // Dispatch
            else if packet.data().contains_key("cmd") && get_string(packet.data(), "cmd") == "DISPATCH" {
                let user_json = match packet.data().get("data").and_then(|d| d.as_object()).and_then(|d| d.get("user")) {
                    Some(serde_json::Value::Object(user)) => Some(user.clone()),
                    // java: GSON.fromJson((JsonElement) null, IPCUser.class) is null
                    Some(serde_json::Value::Null) | None => None,
                    Some(_) => panic!("ClassCastException: user is not a JsonObject"),
                };
                let (on_ready, send) = {
                    let mut state = state();
                    state.received_dispatch = true;
                    state.user = user_json.as_ref().map(IPCUser::from_json);
                    (state.on_ready.clone(), state.queued_activity.is_some())
                };

                if let Some(on_ready) = on_ready {
                    on_ready();
                }
                if send {
                    DiscordIPC::send_activity();
                }
            }
        }
    }

    /// java: private static int getPID() - `ManagementFactory.getRuntimeMXBean().getName()`'s `pid@host`
    fn get_pid() -> i32 {
        std::process::id() as i32
    }

    /// java: private static void defaultErrorCallback(int code, String message)
    fn default_error_callback(code: i32, message: String) {
        eprintln!("Discord IPC error {code} with message: {message}");
    }
}
