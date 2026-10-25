//! Ported from io.github.moulberry.repo.constants.ResourcePack (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;

/// java: @Value public final class ResourcePack
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourcePack {
    /// java: @SerializedName("item_models") ItemModels itemModels;
    item_models: ItemModels,
}

/// java: @Value public static final class ItemModels
///
/// `constants/resource_pack.json` carries both members; one absent there reads as empty (Java: `null`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemModels {
    /// java: List<String> removed;
    removed: Vec<String>,
    /// java: Map<String, String> renamed;
    renamed: HashMap<String, String>,
}

impl ItemModels {
    /// java: public ItemModels(final List<String> removed, final Map<String, String> renamed)
    pub fn new(removed: Vec<String>, renamed: HashMap<String, String>) -> Self {
        ItemModels { removed, renamed }
    }

    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        ItemModels {
            removed: gson::string_list(json, "removed").unwrap_or_default(),
            renamed: gson::member(json, "renamed").and_then(|v| gson::map(v, gson::string_key, gson::as_string)).unwrap_or_default(),
        }
    }

    /// java: public List<String> getRemoved()
    pub fn get_removed(&self) -> &[String] {
        &self.removed
    }

    /// java: public Map<String, String> getRenamed()
    pub fn get_renamed(&self) -> &HashMap<String, String> {
        &self.renamed
    }
}

impl ResourcePack {
    /// java: public ResourcePack(final ItemModels itemModels)
    pub fn new(item_models: ItemModels) -> Self {
        ResourcePack { item_models }
    }

    /// Gson's reflective binding of the class.
    pub fn from_json(json: &Value) -> Self {
        ResourcePack { item_models: gson::member(json, "item_models").map(ItemModels::from_json).unwrap_or_default() }
    }

    /// java: public ItemModels getItemModels()
    pub fn get_item_models(&self) -> &ItemModels {
        &self.item_models
    }
}
