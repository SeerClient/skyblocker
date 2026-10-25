//! Ported from net.azureaaron.legacyitemdfu.fixers.fixes.TextFixes (legacy-item-dfu 1.0.4+26.2).
//!
//! `JsonParser.parseString` is `compat::legacyitemdfu_json_parse_string`; a `JsonObject` built with `addProperty` is a
//! `Value::Object` (Gson's `JsonElement` is the game's `lwjgl::json::Value`).

use crate::compat;
use crate::net::azureaaron::legacyitemdfu::utils::json_helper::JsonHelper;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use mcrust::com::mojang::serialization::dynamic_ops::Ops;
use mcrust::lwjgl::json::Value;
use std::sync::LazyLock;

/// java: private static final String EMPTY_TEXT = text("");
static EMPTY_TEXT: LazyLock<String> = LazyLock::new(|| TextFixes::text_string(""));

/// java: public class TextFixes
pub struct TextFixes;

impl TextFixes {
    /// java: public static <T> Dynamic<T> text(DynamicOps<T> ops, String string)
    pub fn text(ops: &Ops, string: &str) -> Dynamic {
        let string2 = TextFixes::text_string(string);
        Dynamic::new_with(ops.clone(), Some(ops.create_string(&string2)))
    }

    /// java: public static <T> Dynamic<T> empty(DynamicOps<T> ops)
    pub fn empty(ops: &Ops) -> Dynamic {
        Dynamic::new_with(ops.clone(), Some(ops.create_string(&EMPTY_TEXT)))
    }

    /// java: private static String text(String string) - the `(String)` overload.
    fn text_string(string: &str) -> String {
        // java: JsonObject jsonObject = new JsonObject(); jsonObject.addProperty("text", string);
        let json_object = Value::Object(vec![("text".to_string(), Value::String(string.to_string()))]);
        JsonHelper::to_sorted_string(&json_object)
    }

    /// java: public static <T> Dynamic<T> translate(DynamicOps<T> ops, String key)
    pub fn translate(ops: &Ops, key: &str) -> Dynamic {
        let json_object = Value::Object(vec![("translate".to_string(), Value::String(key.to_string()))]);
        Dynamic::new_with(ops.clone(), Some(ops.create_string(&JsonHelper::to_sorted_string(&json_object))))
    }

    /// java: public static <T> Dynamic<T> fixText(Dynamic<T> dynamic)
    pub fn fix_text(dynamic: Dynamic) -> Dynamic {
        // java: DataFixUtils.orElse(dynamic.asString().map(string -> text(dynamic.getOps(), string)).result(), dynamic)
        let fixed: Option<Dynamic> = dynamic.as_string().map(|string| TextFixes::text(dynamic.get_ops(), &string)).result();
        fixed.unwrap_or(dynamic)
    }

    /// java: public static Dynamic<?> text(Dynamic<?> dynamic) - the `(Dynamic)` overload.
    pub fn text_dynamic(dynamic: &Dynamic) -> Dynamic {
        let optional = dynamic.as_string().result();
        let Some(string) = optional else {
            return dynamic.clone();
        };
        if !string.is_empty() && string != "null" {
            // java: string.charAt(0) / string.charAt(string.length() - 1) - compared with ASCII brackets only
            let c = string.chars().next().unwrap_or('\0');
            let d = string.chars().last().unwrap_or('\0');
            if c == '"' && d == '"' || c == '{' && d == '}' || c == '[' && d == ']' {
                // java: try { .. } catch (JsonParseException var6) {}
                if let Ok(json_element) = compat::legacyitemdfu_json_parse_string(&string) {
                    // java: jsonElement.isJsonPrimitive() -> text(dynamic.getOps(), jsonElement.getAsString())
                    match &json_element {
                        Value::String(text) => return TextFixes::text(dynamic.get_ops(), text),
                        Value::Bool(flag) => return TextFixes::text(dynamic.get_ops(), if *flag { "true" } else { "false" }),
                        Value::Number(_) => return TextFixes::text(dynamic.get_ops(), &JsonHelper::to_sorted_string(&json_element)),
                        _ => {}
                    }
                    return dynamic.create_string(&JsonHelper::to_sorted_string(&json_element));
                }
            }
            TextFixes::text(dynamic.get_ops(), &string)
        } else {
            TextFixes::empty(dynamic.get_ops())
        }
    }

    /// java: public static Optional<String> getTranslate(String json)
    pub fn get_translate(json: &str) -> Option<String> {
        // java: try { .. } catch (JsonParseException var4) {}
        if let Ok(Value::Object(json_object)) = compat::legacyitemdfu_json_parse_string(json) {
            // java: JsonElement jsonElement2 = jsonObject.get("translate"); - Gson's `JsonObject.get` is the LAST put
            let json_element2 = json_object.iter().rev().find(|(key, _)| key == "translate").map(|(_, value)| value);
            match json_element2 {
                Some(Value::String(text)) => return Some(text.clone()),
                Some(Value::Bool(flag)) => return Some(if *flag { "true" } else { "false" }.to_string()),
                Some(number @ Value::Number(_)) => return Some(JsonHelper::to_sorted_string(number)),
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::TextFixes;
    use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
    use mcrust::com::mojang::serialization::json_ops::JsonOps;

    #[test]
    fn a_plain_string_becomes_a_sorted_text_component() {
        let ops = JsonOps::instance();
        assert_eq!(TextFixes::text(&ops, "Hi").as_string().result().as_deref(), Some(r#"{"text":"Hi"}"#));
        assert_eq!(TextFixes::empty(&ops).as_string().result().as_deref(), Some(r#"{"text":""}"#));
        assert_eq!(TextFixes::get_translate(r#"{"translate":"a.b"}"#).as_deref(), Some("a.b"));
    }
}
