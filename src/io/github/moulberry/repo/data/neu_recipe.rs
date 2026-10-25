//! Ported from io.github.moulberry.repo.data.NEURecipe (neurepoparser 1.12.0).

use std::any::Any;
use std::sync::Arc;

use crate::io::github::moulberry::repo::data::neu_ingredient::NEUIngredient;
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;

/// java: public interface NEURecipe
///
/// Recipes are shared as `Arc<dyn NEURecipe>` (Java's object references in the recipe cache's sets). `as_any` is
/// Java's `instanceof` / cast to the concrete recipe class.
pub trait NEURecipe: Any + Send + Sync {
    /// java: @ApiStatus.OverrideOnly default void fillItemInfo(NEUItem item) {}
    fn fill_item_info(&mut self, item: &Arc<NEUItem>) {
        let _ = item;
    }

    /// java: Collection<NEUIngredient> getAllInputs();
    fn get_all_inputs(&self) -> Vec<NEUIngredient>;

    /// java: Collection<NEUIngredient> getAllOutputs();
    fn get_all_outputs(&self) -> Vec<NEUIngredient>;

    /// `instanceof` / a cast to the implementing class.
    fn as_any(&self) -> &dyn Any;
}

impl dyn NEURecipe {
    /// `recipe instanceof T`.
    pub fn is<T: NEURecipe>(&self) -> bool {
        self.as_any().is::<T>()
    }

    /// `recipe instanceof T t` - the cast.
    pub fn downcast_ref<T: NEURecipe>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}
