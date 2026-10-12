//! Ported from de.hysky.skyblocker.compatibility.CatharsisCompatibility by fabric2seer. Entrypoint: catharsis:imc/hidden_gui_elements.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static Predicate<String> hiddenModElementsProvider = _ -> false;
pub static HIDDEN_MOD_ELEMENTS_PROVIDER: compat::MainThread<Arc<dyn Fn(String) -> bool>> = compat::MainThread::new(|| Arc::new(move |_| false));

pub fn hidden_gui_elements(provider: Arc<dyn Fn(String) -> bool>) {
    *HIDDEN_MOD_ELEMENTS_PROVIDER.get_mut() = provider.clone();
}

pub fn is_gui_element_hidden(element: &str) -> bool {
    return (HIDDEN_MOD_ELEMENTS_PROVIDER.get_mut())(element.to_string());
}
