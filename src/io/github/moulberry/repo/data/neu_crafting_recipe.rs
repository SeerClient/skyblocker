//! Ported from io.github.moulberry.repo.data.NEUCraftingRecipe (neurepoparser 1.12.0).

use std::any::Any;
use std::sync::Arc;

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_ingredient::{NEUIngredient, SENTINEL_EMPTY};
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter public class NEUCraftingRecipe implements NEURecipe
#[derive(Clone, Debug)]
pub struct NEUCraftingRecipe {
    /// java: NEUIngredient[] inputs; (9 slots, A1..C3; `null` for a slot the recipe does not name)
    pub(crate) inputs: [Option<NEUIngredient>; 9],
    /// java: @Nullable String extraText;
    pub(crate) extra_text: Option<String>,
    /// java: NEUIngredient output;
    pub(crate) output: Option<NEUIngredient>,
    /// java: int outputCount;
    pub(crate) output_count: i32,
}

/// java: public static class Serializer implements JsonDeserializer<NEUCraftingRecipe>
pub struct Serializer;

impl Serializer {
    /// java: public NEUCraftingRecipe deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(json: &Value) -> NEUCraftingRecipe {
        // java: JsonObject recipe = json.getAsJsonObject();
        let Some(recipe) = json.as_object() else {
            panic!("java.lang.IllegalStateException: Not a JSON Object: {}", json);
        };
        let x = ["1", "2", "3"];
        let y = ["A", "B", "C"];
        let mut cr = NEUCraftingRecipe { inputs: Default::default(), extra_text: None, output: None, output_count: 0 };
        for i in 0..9 {
            let name = format!("{}{}", y[i / 3], x[i % 3]);
            // java: if (!recipe.has(name)) continue;
            let Some(value) = recipe.get(&name) else {
                continue;
            };
            // java: String item = recipe.get(name).getAsString();
            match gson::as_string(value) {
                // java: if (item == null || item.isEmpty()) cr.inputs[i] = NEUIngredient.SENTINEL_EMPTY;
                None => cr.inputs[i] = Some(SENTINEL_EMPTY.clone()),
                Some(item) if item.is_empty() => cr.inputs[i] = Some(SENTINEL_EMPTY.clone()),
                // java: else cr.inputs[i] = NEUIngredient.fromString(item);
                Some(item) => cr.inputs[i] = Some(NEUIngredient::from_string(&item)),
            }
        }
        // java: cr.outputCount = -1;
        cr.output_count = -1;
        // java: if (recipe.has("count")) cr.outputCount = recipe.get("count").getAsInt();
        if let Some(count) = recipe.get("count") {
            cr.output_count = gson::as_i32(count).unwrap_or_else(|| panic!("NumberFormatException: {}", count));
        }
        // java: if (recipe.has("crafttext")) cr.extraText = recipe.get("crafttext").getAsString();
        if let Some(crafttext) = recipe.get("crafttext") {
            cr.extra_text = gson::as_string(crafttext);
        }
        // java: if (recipe.has("overrideOutputId")) cr.output = NEUIngredient.fromString(recipe.get("overrideOutputId").getAsString());
        if let Some(override_output_id) = recipe.get("overrideOutputId").and_then(gson::as_string) {
            cr.output = Some(NEUIngredient::from_string(&override_output_id));
        }
        cr
    }
}

impl NEURecipe for NEUCraftingRecipe {
    /// java: public void fillItemInfo(NEUItem item)
    fn fill_item_info(&mut self, item: &Arc<NEUItem>) {
        // java: if (this.extraText == null) this.extraText = item.crafttext;
        if self.extra_text.is_none() {
            self.extra_text = item.crafttext.clone();
        }
        // java: if (this.output == null) this.output = NEUIngredient.fromString(item.skyblockItemId);
        if self.output.is_none() {
            self.output = Some(NEUIngredient::from_string(&item.skyblock_item_id));
        }
        // java: if (outputCount > 0) this.output.amount = outputCount;
        if self.output_count > 0 {
            if let Some(output) = &mut self.output {
                output.amount = self.output_count as f64;
            }
        }
    }

    /// java: public Collection<NEUIngredient> getAllOutputs() { return Collections.singletonList(output); }
    fn get_all_outputs(&self) -> Vec<NEUIngredient> {
        self.output.iter().cloned().collect()
    }

    /// java: public Collection<NEUIngredient> getAllInputs()
    fn get_all_inputs(&self) -> Vec<NEUIngredient> {
        // java: for (NEUIngredient ingredient : inputs) if (ingredient != null) ingredientList.add(ingredient);
        self.inputs.iter().flatten().cloned().collect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NEUCraftingRecipe {
    /// java: public NEUIngredient[] getInputs()
    pub fn get_inputs(&self) -> &[Option<NEUIngredient>; 9] {
        &self.inputs
    }

    /// java: public @Nullable String getExtraText()
    pub fn get_extra_text(&self) -> Option<&str> {
        self.extra_text.as_deref()
    }

    /// java: public NEUIngredient getOutput()
    pub fn get_output(&self) -> Option<&NEUIngredient> {
        self.output.as_ref()
    }

    /// java: public int getOutputCount()
    pub fn get_output_count(&self) -> i32 {
        self.output_count
    }
}
