//! Ported from io.github.moulberry.repo.data.NEUTradeRecipe (neurepoparser 1.12.0).

use std::any::Any;

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_ingredient::{self, NEUIngredient};
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class NEUTradeRecipe implements NEURecipe
#[derive(Clone, Debug)]
pub struct NEUTradeRecipe {
    /// java: NEUIngredient cost;
    pub(crate) cost: Option<NEUIngredient>,
    /// java: int min;
    pub(crate) min: i32,
    /// java: int max;
    pub(crate) max: i32,
    /// java: NEUIngredient result;
    pub(crate) result: Option<NEUIngredient>,
}

impl NEUTradeRecipe {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        NEUTradeRecipe {
            cost: gson::member(json, "cost").and_then(neu_ingredient::Serializer::read),
            min: gson::int(json, "min"),
            max: gson::int(json, "max"),
            result: gson::member(json, "result").and_then(neu_ingredient::Serializer::read),
        }
    }

    /// java: public NEUIngredient getCost()
    pub fn get_cost(&self) -> Option<&NEUIngredient> {
        self.cost.as_ref()
    }

    /// java: public int getMin()
    pub fn get_min(&self) -> i32 {
        self.min
    }

    /// java: public int getMax()
    pub fn get_max(&self) -> i32 {
        self.max
    }

    /// java: public NEUIngredient getResult()
    pub fn get_result(&self) -> Option<&NEUIngredient> {
        self.result.as_ref()
    }
}

impl NEURecipe for NEUTradeRecipe {
    /// java: public Collection<NEUIngredient> getAllInputs() { return Collections.singletonList(cost); }
    fn get_all_inputs(&self) -> Vec<NEUIngredient> {
        self.cost.iter().cloned().collect()
    }

    /// java: public Collection<NEUIngredient> getAllOutputs() { return Collections.singletonList(result); }
    fn get_all_outputs(&self) -> Vec<NEUIngredient> {
        self.result.iter().cloned().collect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
