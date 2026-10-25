//! Ported from de.hysky.skyblocker.utils.render.LevelRenderExtractionCallback by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use mcrust::mcrust::modding::events::Event;

/// java: Event<LevelRenderExtractionCallback> EVENT = EventFactory.createArrayBacked(LevelRenderExtractionCallback.class, callbacks -> collector -> { for (LevelRenderExtractionCallback callback : callbacks) { 
pub static EVENT: compat::MainThread<Event<dyn Fn(compat::Shared<dyn PrimitiveCollector>)>> = compat::MainThread::new(|| Event::new());
/// the invoker of `EVENT` (`EVENT.invoker()`): the factory's lambda over the listeners
pub fn event_invoker() -> LevelRenderExtractionCallback {
    Arc::new(move |collector| invoke(collector))
}

/// `EVENT.invoker().onExtract(collector)` run directly - what `RenderHelper.startExtraction` calls every frame.
///
/// PERF (port-only, same result): the event's own cached, ordered listener slice (`listeners_shared`, rebuilt only
/// when a listener registers) is iterated in place - no `Vec` copy of ~50 `Arc`s and no invoker closure built per
/// frame. Same snapshot semantics as `listeners()`: a listener registered during the walk runs from the next frame.
pub fn invoke(collector: compat::Shared<dyn PrimitiveCollector>) {
    if !EVENT.has_listeners() {
        return;
    }
    let callbacks = EVENT.listeners_shared();
    for callback in callbacks.iter() {
        // One listener's panic must not stop the rest every frame (Fabric's
        // invoker would let the exception escape once; here it would repeat).
        let c = collector.clone();
        if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (callback)(c))) {
            static WARNED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
            if !WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                let msg = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
                eprintln!("[Skyblocker] a LevelRenderExtractionCallback listener panicked: {msg} (further panics are silent)");
            }
        }
    }
}

/// java: public interface LevelRenderExtractionCallback
/// a functional interface: `onExtract(PrimitiveCollector collector)` - a lambda / method reference is this closure type
pub type LevelRenderExtractionCallback = Arc<dyn Fn(compat::Shared<dyn PrimitiveCollector>)>;
