//! Ported from de.hysky.skyblocker.events.PlaySoundEvents by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::events::Event;
use mcrust::net::minecraft::network::protocol::game::clientbound_sound_packet::ClientboundSoundPacket;
use mcrust::net::minecraft::sounds::sound_event::SoundEvent;

/// java: public static final Event<FromServer> FROM_SERVER = EventFactory.createArrayBacked(FromServer.class, callbacks -> packet -> { for (FromServer callback : callbacks) { callback.onPlaySoundFromServer(pac
pub static FROM_SERVER: compat::MainThread<Event<dyn Fn(ClientboundSoundPacket)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `FROM_SERVER` (`FROM_SERVER.invoker()`): the factory's lambda over the listeners
pub fn from_server_invoker() -> FromServer {
    let callbacks: Vec<FromServer> = FROM_SERVER.listeners();
    Arc::new(move |packet| {
        for callback in callbacks.iter() {
            (callback)(packet.clone());
        }
    })
}
/// java: public static final Event<AllowSound> ALLOW_SOUND = EventFactory.createArrayBacked(AllowSound.class, callbacks -> sound -> { boolean allowSound = true; for (AllowSound callback : callbacks) { allowSou
pub static ALLOW_SOUND: compat::MainThread<Event<dyn Fn(SoundEvent) -> bool>> = compat::MainThread::new(|| Event::new());
/// the invoker of `ALLOW_SOUND` (`ALLOW_SOUND.invoker()`): the factory's lambda over the listeners
pub fn allow_sound_invoker() -> AllowSound {
    let callbacks: Vec<AllowSound> = ALLOW_SOUND.listeners();
    Arc::new(move |sound| {
        let mut allow_sound: bool = true;
        for callback in callbacks.iter() {
            allow_sound = ((allow_sound & (callback)(sound.clone()))) as bool;
        }
        return allow_sound;
    })
}

/// java: public class PlaySoundEvents
#[derive(Clone)]
pub struct PlaySoundEvents {}

impl PlaySoundEvents {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface FromServer
/// a functional interface: `onPlaySoundFromServer(ClientboundSoundPacket packet)` - a lambda / method reference is this closure type
pub type FromServer = Arc<dyn Fn(ClientboundSoundPacket)>;

/// java: @Environment(EnvType.CLIENT) @FunctionalInterface public interface AllowSound
/// a functional interface: `allowSound(SoundEvent sound)` - a lambda / method reference is this closure type
pub type AllowSound = Arc<dyn Fn(SoundEvent) -> bool>;
