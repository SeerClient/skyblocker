//! Ported from de.hysky.skyblocker.utils.ServerTickCounter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::events::server_tick_callback;
use mcrust::net::minecraft::network::protocol::common::clientbound_ping_packet::ClientboundPingPacket;

/// java: private static int lastId = -1;
pub static LAST_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(-1));

/// java: public class ServerTickCounter
#[derive(Clone)]
pub struct ServerTickCounter {}

impl ServerTickCounter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            ServerTickCounter::reset();
        }));
    }

    /// java: public static void onServerTick(ClientboundPingPacket packet)
    pub fn on_server_tick(packet: &ClientboundPingPacket) {
        if packet.get_id() != compat::get_state(&LAST_ID) {
            compat::set_state(&LAST_ID, packet.get_id());
            (server_tick_callback::event_invoker())();
        }
    }

    /// java: private static void reset()
    pub fn reset() {
        compat::set_state(&LAST_ID, -1);
    }

}
