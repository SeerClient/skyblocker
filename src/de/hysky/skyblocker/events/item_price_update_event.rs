//! Ported from de.hysky.skyblocker.events.ItemPriceUpdateEvent by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::events::Event;

/// java: Event<ItemPriceUpdateEvent> ON_PRICE_UPDATE = EventFactory.createArrayBacked(ItemPriceUpdateEvent.class, listeners -> () -> { for (ItemPriceUpdateEvent listener : listeners) { listener.onPriceUpdate()
pub static ON_PRICE_UPDATE: compat::MainThread<Event<dyn Fn()>> = compat::MainThread::new(|| Event::new());
/// the invoker of `ON_PRICE_UPDATE` (`ON_PRICE_UPDATE.invoker()`): the factory's lambda over the listeners
pub fn on_price_update_invoker() -> ItemPriceUpdateEvent {
    let callbacks: Vec<ItemPriceUpdateEvent> = ON_PRICE_UPDATE.listeners();
    Arc::new(move || {
        for listener in callbacks.iter() {
            (listener)();
        }
    })
}

/// java: @FunctionalInterface @Environment(EnvType.CLIENT) public interface ItemPriceUpdateEvent
/// a functional interface: `onPriceUpdate()` - a lambda / method reference is this closure type
pub type ItemPriceUpdateEvent = Arc<dyn Fn()>;
