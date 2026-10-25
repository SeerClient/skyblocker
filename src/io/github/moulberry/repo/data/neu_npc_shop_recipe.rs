//! Ported from io.github.moulberry.repo.data.NEUNpcShopRecipe (neurepoparser 1.12.0).

use std::any::Any;
use std::sync::{Arc, Weak};

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_ingredient::{self, NEUIngredient};
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class NEUNpcShopRecipe implements NEURecipe
#[derive(Clone, Debug)]
pub struct NEUNpcShopRecipe {
    /// java: List<NEUIngredient> cost;
    pub(crate) cost: Vec<NEUIngredient>,
    /// java: NEUIngredient result;
    pub(crate) result: Option<NEUIngredient>,
    /// java: NEUItem isSoldBy; - the item the recipe belongs to (a `Weak`: the item owns its recipes).
    pub(crate) is_sold_by: Weak<NEUItem>,
}

impl NEUNpcShopRecipe {
    /// Gson's reflective binding of the class. (`isSoldBy` is not `transient`, but no repository recipe carries it and
    /// `fillItemInfo` overwrites it.)
    pub fn from_json(json: &Value) -> Self {
        NEUNpcShopRecipe {
            cost: gson::member(json, "cost").and_then(|v| gson::list(v, neu_ingredient::Serializer::read)).unwrap_or_default(),
            result: gson::member(json, "result").and_then(neu_ingredient::Serializer::read),
            is_sold_by: Weak::new(),
        }
    }

    /// java: public List<NEUIngredient> getCost()
    pub fn get_cost(&self) -> &[NEUIngredient] {
        &self.cost
    }

    /// java: public NEUIngredient getResult()
    pub fn get_result(&self) -> Option<&NEUIngredient> {
        self.result.as_ref()
    }

    /// java: public NEUItem getIsSoldBy()
    pub fn get_is_sold_by(&self) -> Option<Arc<NEUItem>> {
        self.is_sold_by.upgrade()
    }
}

impl NEURecipe for NEUNpcShopRecipe {
    /// java: public void fillItemInfo(NEUItem item) { isSoldBy = item; }
    fn fill_item_info(&mut self, item: &Arc<NEUItem>) {
        self.is_sold_by = Arc::downgrade(item);
    }

    /// java: public Collection<NEUIngredient> getAllInputs() { return cost; }
    fn get_all_inputs(&self) -> Vec<NEUIngredient> {
        self.cost.clone()
    }

    /// java: public Collection<NEUIngredient> getAllOutputs() { return Collections.singletonList(result); }
    fn get_all_outputs(&self) -> Vec<NEUIngredient> {
        self.result.iter().cloned().collect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
