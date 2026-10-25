//! Ported from io.github.moulberry.repo.data.NEUUnknownRecipe (neurepoparser 1.12.0).

use std::any::Any;
use std::sync::{Arc, Weak};

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_ingredient::NEUIngredient;
use crate::io::github::moulberry::repo::data::neu_item::NEUItem;
use crate::io::github::moulberry::repo::data::neu_recipe::NEURecipe;
use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Getter @AllArgsConstructor public class NEUUnknownRecipe implements NEURecipe
#[derive(Clone, Debug)]
pub struct NEUUnknownRecipe {
    /// java: @Nullable final String type;
    pub(crate) r#type: Option<String>,
    /// java: final JsonObject tree;
    pub(crate) tree: serde_json::Map<String, Value>,
    /// java: NEUItem item; (a `Weak`: the item owns its recipes)
    pub(crate) item: Weak<NEUItem>,
}

/// java: public static class Serializer implements JsonDeserializer<NEUUnknownRecipe>
pub struct Serializer;

impl Serializer {
    /// java: public NEUUnknownRecipe deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(json: &Value) -> NEUUnknownRecipe {
        // java: JsonElement type = json.getAsJsonObject().get("type");
        let Some(object) = json.as_object() else {
            panic!("java.lang.IllegalStateException: Not a JSON Object: {}", json);
        };
        let r#type: Option<&Value> = object.get("type");
        // java: return new NEUUnknownRecipe(type != null && type.isJsonPrimitive() ? type.getAsString() : null, json.getAsJsonObject());
        let r#type: Option<String> = match r#type {
            Some(t) if !t.is_null() && !t.is_object() && !t.is_array() => gson::as_string(t),
            _ => None,
        };
        NEUUnknownRecipe::new(r#type, object.clone())
    }
}

impl NEUUnknownRecipe {
    /// java: public NEUUnknownRecipe(@Nullable final String type, final JsonObject tree)
    pub fn new(r#type: Option<String>, tree: serde_json::Map<String, Value>) -> Self {
        NEUUnknownRecipe { r#type, tree, item: Weak::new() }
    }

    /// java: public @Nullable String getType()
    pub fn get_type(&self) -> Option<&str> {
        self.r#type.as_deref()
    }

    /// java: public JsonObject getTree()
    pub fn get_tree(&self) -> &serde_json::Map<String, Value> {
        &self.tree
    }

    /// java: public NEUItem getItem()
    pub fn get_item(&self) -> Option<Arc<NEUItem>> {
        self.item.upgrade()
    }
}

impl NEURecipe for NEUUnknownRecipe {
    /// java: public Collection<NEUIngredient> getAllInputs() { return Collections.singleton(NEUIngredient.fromItem(item, 1)); }
    fn get_all_inputs(&self) -> Vec<NEUIngredient> {
        match self.item.upgrade() {
            Some(item) => vec![NEUIngredient::from_item(&item, 1)],
            None => panic!("java.lang.NullPointerException: NEUUnknownRecipe.item"),
        }
    }

    /// java: public Collection<NEUIngredient> getAllOutputs() { return Collections.emptyList(); }
    fn get_all_outputs(&self) -> Vec<NEUIngredient> {
        Vec::new()
    }

    /// java: public void fillItemInfo(NEUItem item) { this.item = item; }
    fn fill_item_info(&mut self, item: &Arc<NEUItem>) {
        self.item = Arc::downgrade(item);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
