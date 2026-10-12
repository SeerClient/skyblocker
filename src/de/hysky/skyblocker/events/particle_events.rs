//! Ported from de.hysky.skyblocker.events.ParticleEvents by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::events::Event;
use mcrust::net::minecraft::network::protocol::game::clientbound_level_particles_packet::ClientboundLevelParticlesPacket;

/// java: public static final Event<FromServer> FROM_SERVER = EventFactory.createArrayBacked(FromServer.class, callbacks -> packet -> { for (FromServer callback : callbacks) { callback.onParticleFromServer(pack
pub static FROM_SERVER: compat::MainThread<Event<dyn Fn(ClientboundLevelParticlesPacket)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `FROM_SERVER` (`FROM_SERVER.invoker()`): the factory's lambda over the listeners
pub fn from_server_invoker() -> FromServer {
    let callbacks: Vec<FromServer> = FROM_SERVER.listeners();
    Arc::new(move |packet| {
        for callback in callbacks.iter() {
            (callback)(packet.clone());
        }
    })
}

/// java: public class ParticleEvents
#[derive(Clone)]
pub struct ParticleEvents {}

impl ParticleEvents {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface FromServer
/// a functional interface: `onParticleFromServer(ClientboundLevelParticlesPacket packet)` - a lambda / method reference is this closure type
pub type FromServer = Arc<dyn Fn(ClientboundLevelParticlesPacket)>;
