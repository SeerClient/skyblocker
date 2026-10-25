//! Ported from io.github.moulberry.repo.data.NEUIngredient (neurepoparser 1.12.0).

use std::sync::LazyLock;

use serde_json::Value;

use crate::io::github::moulberry::repo::data::neu_item::NEUItem;

/// java: public static final String NEU_SENTINEL_EMPTY = "NEU_SENTINEL_EMPTY";
pub const NEU_SENTINEL_EMPTY: &str = "NEU_SENTINEL_EMPTY";
/// java: public static final String NEU_SENTINEL_COINS = "SKYBLOCK_COIN";
pub const NEU_SENTINEL_COINS: &str = "SKYBLOCK_COIN";
/// java: public static final NEUIngredient SENTINEL_EMPTY = new NEUIngredient();
///       static { SENTINEL_EMPTY.itemId = NEU_SENTINEL_EMPTY; SENTINEL_EMPTY.amount = 0; }
pub static SENTINEL_EMPTY: LazyLock<NEUIngredient> =
    LazyLock::new(|| NEUIngredient { item_id: NEU_SENTINEL_EMPTY.to_string(), amount: 0.0 });

/// java: @Getter public class NEUIngredient
///
/// A value here: Java shares one object between a recipe and its caches, and the only write after construction is
/// a recipe's `fillItemInfo` setting its own output's `amount` before anything reads it.
#[derive(Clone, Debug, PartialEq)]
pub struct NEUIngredient {
    /// java: @NEUId String itemId;
    pub(crate) item_id: String,
    /// java: double amount;
    pub(crate) amount: f64,
}

impl NEUIngredient {
    /// java: public static NEUIngredient ofCoins(double coins)
    pub fn of_coins(coins: f64) -> NEUIngredient {
        NEUIngredient { amount: coins, item_id: NEU_SENTINEL_COINS.to_string() }
    }

    /// java: public static NEUIngredient fromItem(NEUItem item, int count)
    pub fn from_item(item: &NEUItem, count: i32) -> NEUIngredient {
        NEUIngredient { amount: count as f64, item_id: item.get_skyblock_item_id().to_string() }
    }

    /// java: public static NEUIngredient fromString(String string)
    ///
    /// Java's `IllegalArgumentException` / `NumberFormatException` is a panic.
    pub fn from_string(string: &str) -> NEUIngredient {
        // java: String[] parts = string.split(":"); - Java drops trailing empty strings
        let mut parts: Vec<&str> = string.split(':').collect();
        while parts.len() > 1 && parts.last() == Some(&"") {
            parts.pop();
        }
        if parts.len() == 1 && parts[0].is_empty() && !string.is_empty() {
            // "::" splits to nothing in Java
            parts.clear();
        }
        let amount: f64 = if parts.len() == 2 {
            // java: Double.parseDouble(parts[1])
            parse_double(parts[1]).unwrap_or_else(|| panic!("NumberFormatException: For input string: \"{}\"", parts[1]))
        } else if parts.len() == 1 {
            1.0
        } else {
            panic!("IllegalArgumentException: Could not parse ingredient {}", string);
        };
        let item_id: String = parts[0].to_string();
        // java: if (NEU_SENTINEL_EMPTY.equals(ingredient.itemId)) return SENTINEL_EMPTY;
        if item_id == NEU_SENTINEL_EMPTY {
            return SENTINEL_EMPTY.clone();
        }
        NEUIngredient { item_id, amount }
    }

    /// java: public String getItemId()
    pub fn get_item_id(&self) -> &str {
        &self.item_id
    }

    /// java: public double getAmount()
    pub fn get_amount(&self) -> f64 {
        self.amount
    }
}

/// `Double.parseDouble`: surrounding whitespace is trimmed and a trailing `d`/`D`/`f`/`F` type suffix is accepted.
fn parse_double(s: &str) -> Option<f64> {
    let t = s.trim();
    t.parse::<f64>().ok().or_else(|| {
        let stripped = t.strip_suffix(['d', 'D', 'f', 'F'])?;
        stripped.parse::<f64>().ok()
    })
}

/// java: public static class Serializer implements JsonDeserializer<NEUIngredient>
pub struct Serializer;

impl Serializer {
    /// java: public NEUIngredient deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(json: &Value) -> NEUIngredient {
        // java: if (!json.isJsonPrimitive()) / if (!p.isString()) throw new JsonParseException("Expected string for ingredient, found " + json);
        let Value::String(as_string) = json else {
            panic!("JsonParseException: Expected string for ingredient, found {}", json);
        };
        NEUIngredient::from_string(as_string)
    }

    /// Gson reading a `NEUIngredient` field / list element (`null` stays `null`).
    pub(crate) fn read(json: &Value) -> Option<NEUIngredient> {
        if json.is_null() { None } else { Some(Self::deserialize(json)) }
    }
}

impl std::fmt::Display for NEUIngredient {
    /// java: public String toString() { return String.format("NEUIngredient{%s:%f}", itemId, amount); }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NEUIngredient{{{}:{:.6}}}", self.item_id, self.amount)
    }
}
