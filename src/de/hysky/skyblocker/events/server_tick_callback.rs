//! Ported from de.hysky.skyblocker.events.ServerTickCallback by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::events::Event;

/// java: Event<ServerTickCallback> EVENT = EventFactory.createArrayBacked(ServerTickCallback.class, listeners -> () -> { for (ServerTickCallback listener : listeners) { listener.onTick(); } });
pub static EVENT: compat::MainThread<Event<dyn Fn()>> = compat::MainThread::new(|| Event::new());
/// the invoker of `EVENT` (`EVENT.invoker()`): the factory's lambda over the listeners
pub fn event_invoker() -> ServerTickCallback {
    let callbacks: Vec<ServerTickCallback> = EVENT.listeners();
    Arc::new(move || {
        for listener in callbacks.iter() {
            (listener)();
        }
    })
}

/// java: public interface ServerTickCallback
/// a functional interface: `onTick()` - a lambda / method reference is this closure type
pub type ServerTickCallback = Arc<dyn Fn()>;
