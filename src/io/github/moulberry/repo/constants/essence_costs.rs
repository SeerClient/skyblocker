//! Ported from io.github.moulberry.repo.constants.EssenceCosts (neurepoparser 1.12.0).

use std::collections::HashMap;

use serde_json::Value;

use crate::io::github::moulberry::repo::neu_repository::gson;
use crate::io::github::moulberry::repo::neu_repository_exception::NEURepositoryException;

/// java: @Getter public class EssenceCosts
#[derive(Clone, Debug, Default)]
pub struct EssenceCosts {
    /// java: Map<@NEUId String, EssenceCost> costs;
    costs: HashMap<String, EssenceCost>,
}

/// java: @Data public static class EssenceCost
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EssenceCost {
    /// java: String type;
    pub(crate) r#type: String,
    /// java: Map<Integer, Integer> essenceCosts;
    pub(crate) essence_costs: HashMap<i32, i32>,
    /// java: Map<Integer, List<String>> itemCosts;
    pub(crate) item_costs: HashMap<i32, Vec<String>>,
}

impl EssenceCost {
    /// java: public String getType()
    pub fn get_type(&self) -> &str {
        &self.r#type
    }

    /// java: public Map<Integer, Integer> getEssenceCosts()
    pub fn get_essence_costs(&self) -> &HashMap<i32, i32> {
        &self.essence_costs
    }

    /// java: public Map<Integer, List<String>> getItemCosts()
    pub fn get_item_costs(&self) -> &HashMap<i32, Vec<String>> {
        &self.item_costs
    }

    /// java: public void setType(final String type)
    pub fn set_type(&mut self, r#type: String) {
        self.r#type = r#type;
    }

    /// java: public void setEssenceCosts(final Map<Integer, Integer> essenceCosts)
    pub fn set_essence_costs(&mut self, essence_costs: HashMap<i32, i32>) {
        self.essence_costs = essence_costs;
    }

    /// java: public void setItemCosts(final Map<Integer, List<String>> itemCosts)
    pub fn set_item_costs(&mut self, item_costs: HashMap<i32, Vec<String>>) {
        self.item_costs = item_costs;
    }
}

impl EssenceCosts {
    /// java: public EssenceCosts(JsonObject json) throws NEURepositoryException
    pub fn new(json: &Value) -> Result<Self, NEURepositoryException> {
        // java: catch (ClassCastException | NumberFormatException e) { throw new NEURepositoryException("constants/essencecosts.json", "Invalid JSON Syntax", e); }
        let invalid = |cause: String| NEURepositoryException::new("constants/essencecosts.json", "Invalid JSON Syntax", Some(cause));
        let mut costs: HashMap<String, EssenceCost> = HashMap::new();
        let Some(json) = json.as_object() else {
            return Err(invalid(format!("Not a JSON Object: {}", json)));
        };
        for (key, value) in json {
            let mut essence_cost = EssenceCost::default();
            // java: JsonObject object = value.getAsJsonObject();
            let Some(object) = value.as_object() else {
                return Err(invalid(format!("Not a JSON Object: {}", value)));
            };
            // java: essenceCost.type = object.get("type").getAsString();
            essence_cost.r#type = object
                .get("type")
                .and_then(gson::as_string)
                .ok_or_else(|| invalid(format!("no type in {}", key)))?;
            // java: int i = 1; while (true) { JsonElement costThing = object.get(i + ""); if (costThing == null) break; ..; i++; }
            let mut i: i32 = 1;
            while let Some(cost_thing) = object.get(&i.to_string()) {
                let cost: i32 = gson::as_i32(cost_thing).ok_or_else(|| invalid(format!("NumberFormatException: {}", cost_thing)))?;
                essence_cost.essence_costs.insert(i, cost);
                i += 1;
            }
            // java: if (object.has("items")) { JsonObject items = object.getAsJsonObject("items"); ..
            if let Some(items) = object.get("items") {
                let Some(items) = items.as_object() else {
                    return Err(invalid(format!("Not a JSON Object: {}", items)));
                };
                for (item_key, item_cost) in items {
                    let Some(item_cost_list) = item_cost.as_array() else {
                        return Err(invalid(format!("Not a JSON Array: {}", item_cost)));
                    };
                    let mut item_cost_list_trans: Vec<String> = Vec::new();
                    for json_element in item_cost_list {
                        item_cost_list_trans.push(gson::as_string(json_element).ok_or_else(|| invalid(format!("{}", json_element)))?);
                    }
                    // java: essenceCost.itemCosts.put(Integer.parseInt(itemCost.getKey()), itemCostListTrans);
                    let tier: i32 = item_key.parse::<i32>().map_err(|_| invalid(format!("NumberFormatException: For input string: \"{}\"", item_key)))?;
                    essence_cost.item_costs.insert(tier, item_cost_list_trans);
                }
            }
            costs.insert(key.clone(), essence_cost);
        }
        Ok(EssenceCosts { costs })
    }

    /// java: public Map<@NEUId String, EssenceCost> getCosts()
    pub fn get_costs(&self) -> &HashMap<String, EssenceCost> {
        &self.costs
    }
}
