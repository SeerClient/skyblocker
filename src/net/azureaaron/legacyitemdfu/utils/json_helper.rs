//! Ported from net.azureaaron.legacyitemdfu.utils.JsonHelper (legacy-item-dfu 1.0.4+26.2).
//!
//! Gson's `JsonElement` is the game's `lwjgl::json::Value` (json_ops.rs), and `com.google.gson.stream.JsonWriter` is
//! the game's `com::google::gson::stream::json_writer::JsonWriter` (a `StringWriter` behind it: `close` answers the text).

use mcrust::com::google::gson::stream::json_writer::JsonWriter;
use mcrust::lwjgl::json::Value;

/// java: public final class JsonHelper
pub struct JsonHelper;

impl JsonHelper {
    /// java: public static String toSortedString(JsonElement json)
    pub fn to_sorted_string(json: &Value) -> String {
        let mut json_writer = JsonWriter::new();
        // java: writeSorted(jsonWriter, json, Comparator.naturalOrder()) - an IOException is `throw new AssertionError(var4)`
        if let Err(var4) = JsonHelper::write_sorted(&mut json_writer, Some(json), Some(&|a: &str, b: &str| mcrust::javacompat::java_string_compare(a, b))) {
            panic!("java.lang.AssertionError: {var4}");
        }
        json_writer.close().unwrap_or_else(|var4| panic!("java.lang.AssertionError: {var4}"))
    }

    /// java: private static void writeSorted(JsonWriter writer, @Nullable JsonElement json, @Nullable Comparator<String> comparator) throws IOException
    fn write_sorted(writer: &mut JsonWriter, json: Option<&Value>, comparator: Option<&dyn Fn(&str, &str) -> i32>) -> Result<(), String> {
        match json {
            None | Some(Value::Null) => writer.null_value(),
            // java: jsonPrimitive.isNumber() -> writer.value(jsonPrimitive.getAsNumber())
            Some(Value::Number(number)) => writer.value_number(*number),
            Some(Value::Bool(flag)) => writer.value_bool(*flag),
            Some(Value::String(text)) => writer.value_string(text),
            Some(Value::Array(elements)) => {
                writer.begin_array()?;
                for json_element in elements {
                    JsonHelper::write_sorted(writer, Some(json_element), comparator)?;
                }
                writer.end_array()
            }
            Some(Value::Object(entries)) => {
                writer.begin_object()?;
                for (key, value) in JsonHelper::sort(entries, comparator) {
                    writer.name(key)?;
                    JsonHelper::write_sorted(writer, Some(value), comparator)?;
                }
                writer.end_object()
            }
        }
    }

    /// java: private static Collection<Entry<String, JsonElement>> sort(Collection<Entry<String, JsonElement>> entries, @Nullable Comparator<String> comparator)
    fn sort<'a>(entries: &'a [(String, Value)], comparator: Option<&dyn Fn(&str, &str) -> i32>) -> Vec<&'a (String, Value)> {
        let mut list: Vec<&(String, Value)> = entries.iter().collect();
        if let Some(comparator) = comparator {
            // java: list.sort(Entry.comparingByKey(comparator)) - a stable sort, as Rust's `sort_by` is
            list.sort_by(|a, b| comparator(&a.0, &b.0).cmp(&0));
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::JsonHelper;
    use mcrust::lwjgl::json::Value;

    #[test]
    fn object_keys_are_written_in_natural_order() {
        let json = Value::Object(vec![("text".to_string(), Value::String("a".to_string())), ("bold".to_string(), Value::Bool(true))]);
        assert_eq!(JsonHelper::to_sorted_string(&json), r#"{"bold":true,"text":"a"}"#);
    }
}
