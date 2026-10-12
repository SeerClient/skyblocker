//! Ported from de.hysky.skyblocker.injected.RecipeBookHolder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface RecipeBookHolder
pub trait RecipeBookHolder {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: default void registerRecipeBookToggleCallback(Runnable callback)
    fn register_recipe_book_toggle_callback(&self, callback: Arc<dyn Fn()>) {
        panic!("UnsupportedOperationException: {}", "Implemented via Mixin");
    }

}
