//! Ported from de.hysky.skyblocker.utils.render.LevelRenderExtractionCallback by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::mcrust::modding::events::Event;

/// java: Event<LevelRenderExtractionCallback> EVENT = EventFactory.createArrayBacked(LevelRenderExtractionCallback.class, callbacks -> collector -> { for (LevelRenderExtractionCallback callback : callbacks) { 
pub static EVENT: compat::MainThread<Event<dyn Fn(compat::Shared<dyn PrimitiveCollector>)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `EVENT` (`EVENT.invoker()`): the factory's lambda over the listeners
pub fn event_invoker() -> LevelRenderExtractionCallback {
    let callbacks: Vec<LevelRenderExtractionCallback> = EVENT.listeners();
    Arc::new(move |collector| {
        for callback in callbacks.iter() {
            (callback)(collector.clone());
        }
    })
}

/// java: public interface LevelRenderExtractionCallback
/// a functional interface: `onExtract(PrimitiveCollector collector)` - a lambda / method reference is this closure type
pub type LevelRenderExtractionCallback = Arc<dyn Fn(compat::Shared<dyn PrimitiveCollector>)>;
