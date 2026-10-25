//! Ported from io.github.moulberry.repo.data.NEUForgeRecipe (neurepoparser 1.12.0).

use std::any::Any;
use std::sync::Arc;

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_ingredient::{self, NEUIngredient};
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class NEUForgeRecipe implements NEURecipe
#[derive(Clone, Debug)]
pub struct NEUForgeRecipe {
    /// java: List<NEUIngredient> inputs;
    pub(crate) inputs: Vec<NEUIngredient>,
    /// java: int count;
    pub(crate) count: i32,
    /// java: String overrideOutputId;
    pub(crate) override_output_id: Option<String>,
    /// java: int duration;
    pub(crate) duration: i32,
    /// java: transient NEUIngredient outputStack;
    pub(crate) output_stack: Option<NEUIngredient>,
    /// java: @Nullable String extraText;
    pub(crate) extra_text: Option<String>,
}

impl NEUForgeRecipe {
    /// Gson's reflective binding of the class (the `NEUIngredient` elements through its `Serializer`).
    pub fn from_json(json: &Value) -> Self {
        NEUForgeRecipe {
            inputs: gson::member(json, "inputs").and_then(|v| gson::list(v, neu_ingredient::Serializer::read)).unwrap_or_default(),
            count: gson::int(json, "count"),
            override_output_id: gson::string(json, "overrideOutputId"),
            duration: gson::int(json, "duration"),
            output_stack: None,
            extra_text: gson::string(json, "extraText"),
        }
    }

    /// java: public List<NEUIngredient> getInputs()
    pub fn get_inputs(&self) -> &[NEUIngredient] {
        &self.inputs
    }

    /// java: public int getDuration()
    pub fn get_duration(&self) -> i32 {
        self.duration
    }

    /// java: public NEUIngredient getOutputStack()
    pub fn get_output_stack(&self) -> Option<&NEUIngredient> {
        self.output_stack.as_ref()
    }

    /// java: public @Nullable String getExtraText()
    pub fn get_extra_text(&self) -> Option<&str> {
        self.extra_text.as_deref()
    }
}

impl NEURecipe for NEUForgeRecipe {
    /// java: public void fillItemInfo(NEUItem item)
    fn fill_item_info(&mut self, item: &Arc<NEUItem>) {
        // java: if (extraText == null) extraText = item.crafttext;
        if self.extra_text.is_none() {
            self.extra_text = item.crafttext.clone();
        }
        // java: outputStack = NEUIngredient.fromString(overrideOutputId == null ? item.skyblockItemId : overrideOutputId);
        let mut output_stack = match &self.override_output_id {
            None => NEUIngredient::from_string(&item.skyblock_item_id),
            Some(id) => NEUIngredient::from_string(id),
        };
        // java: if (count > 0) outputStack.amount = count;
        if self.count > 0 {
            output_stack.amount = self.count as f64;
        }
        self.output_stack = Some(output_stack);
    }

    /// java: public Collection<NEUIngredient> getAllOutputs() { return Collections.singletonList(outputStack); }
    fn get_all_outputs(&self) -> Vec<NEUIngredient> {
        self.output_stack.iter().cloned().collect()
    }

    /// java: public Collection<NEUIngredient> getAllInputs() { return inputs; }
    fn get_all_inputs(&self) -> Vec<NEUIngredient> {
        self.inputs.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
