//! Ported from io.github.moulberry.repo.data.NEUKatUpgradeRecipe (neurepoparser 1.12.0).

use std::any::Any;

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_ingredient::{self, NEUIngredient};
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class NEUKatUpgradeRecipe implements NEURecipe
#[derive(Clone, Debug)]
pub struct NEUKatUpgradeRecipe {
    /// java: NEUIngredient input;
    pub(crate) input: Option<NEUIngredient>,
    /// java: NEUIngredient output;
    pub(crate) output: Option<NEUIngredient>,
    /// java: List<NEUIngredient> items;
    pub(crate) items: Vec<NEUIngredient>,
    /// java: double coins;
    pub(crate) coins: f64,
    /// java: @SerializedName("time") long seconds;
    pub(crate) seconds: i64,
}

impl NEUKatUpgradeRecipe {
    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        NEUKatUpgradeRecipe {
            input: gson::member(json, "input").and_then(neu_ingredient::Serializer::read),
            output: gson::member(json, "output").and_then(neu_ingredient::Serializer::read),
            items: gson::member(json, "items").and_then(|v| gson::list(v, neu_ingredient::Serializer::read)).unwrap_or_default(),
            coins: gson::double(json, "coins"),
            seconds: gson::long(json, "time"),
        }
    }

    /// java: public NEUIngredient getInput()
    pub fn get_input(&self) -> Option<&NEUIngredient> {
        self.input.as_ref()
    }

    /// java: public NEUIngredient getOutput()
    pub fn get_output(&self) -> Option<&NEUIngredient> {
        self.output.as_ref()
    }

    /// java: public List<NEUIngredient> getItems()
    pub fn get_items(&self) -> &[NEUIngredient] {
        &self.items
    }

    /// java: public double getCoins()
    pub fn get_coins(&self) -> f64 {
        self.coins
    }

    /// java: public long getSeconds()
    pub fn get_seconds(&self) -> i64 {
        self.seconds
    }
}

impl NEURecipe for NEUKatUpgradeRecipe {
    /// java: public Collection<NEUIngredient> getAllInputs()
    fn get_all_inputs(&self) -> Vec<NEUIngredient> {
        let mut inputs: Vec<NEUIngredient> = Vec::new();
        // java: inputs.add(input); inputs.addAll(items); inputs.add(NEUIngredient.ofCoins(coins));
        inputs.extend(self.input.iter().cloned());
        inputs.extend(self.items.iter().cloned());
        inputs.push(NEUIngredient::of_coins(self.coins));
        inputs
    }

    /// java: public Collection<NEUIngredient> getAllOutputs() { return Collections.singleton(output); }
    fn get_all_outputs(&self) -> Vec<NEUIngredient> {
        self.output.iter().cloned().collect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
