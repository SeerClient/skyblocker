//! Ported from net.azureaaron.hmapi.events.HypixelPacketEvents.

use crate::compat;
use crate::net::azureaaron::hmapi::network::packet::s2c::hypixel_s2c_packet::HypixelS2CPacket;
use std::sync::Arc;

/// java: public interface PacketCallback { void onPacket(HypixelS2CPacket packet); }
pub type PacketCallback = Arc<dyn Fn(&HypixelS2CPacket)>;

/// java: Fabric's `Event<PacketCallback>` from `EventFactory.createArrayBacked(..)`: the invoker runs every registered
/// callback in registration order. The callbacks are the mod's (render-thread objects), so the event lives on that
/// thread, where `HypixelNetworkingImpl.handlePayload` hands it the packets (`CLIENT.execute`).
pub struct Event {
    callbacks: Vec<PacketCallback>,
}

impl Event {
    /// java: Event.register(listener)
    pub fn register(&mut self, callback: PacketCallback) {
        self.callbacks.push(callback);
    }

    /// java: Event.invoker().onPacket(packet)
    pub fn invoke(&self, packet: &HypixelS2CPacket) {
        for callback in self.callbacks.clone() {
            callback(packet);
        }
    }
}

/// java: private static Event<PacketCallback> createEvent()
fn create_event() -> Event {
    Event { callbacks: Vec::new() }
}

/// java: public static final Event<PacketCallback> PARTY_INFO = createEvent();
pub static PARTY_INFO: compat::MainThread<Event> = compat::MainThread::new(create_event);
/// java: public static final Event<PacketCallback> PLAYER_INFO = createEvent();
pub static PLAYER_INFO: compat::MainThread<Event> = compat::MainThread::new(create_event);
/// java: public static final Event<PacketCallback> HELLO = createEvent();
pub static HELLO: compat::MainThread<Event> = compat::MainThread::new(create_event);
/// java: public static final Event<PacketCallback> LOCATION_UPDATE = createEvent();
pub static LOCATION_UPDATE: compat::MainThread<Event> = compat::MainThread::new(create_event);
