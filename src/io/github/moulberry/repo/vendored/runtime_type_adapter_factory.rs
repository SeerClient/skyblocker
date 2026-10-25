//! Ported from io.github.moulberry.repo.vendored.RuntimeTypeAdapterFactory (neurepoparser 1.12.0) - Gson's
//! `RuntimeTypeAdapterFactory` extra, vendored by the library with NEU's modifications (a fallback type and a default
//! type tag).
//!
//! Only the READ half is ported: the library never serializes a recipe (`write` is reached only by `Gson.toJson`,
//! which nothing calls), and Gson's `TypeAdapterFactory.create` dispatch is the `read` below, called by the
//! repository wherever Java's Gson meets a `NEURecipe`.

use serde_json::Value;

/// A subtype's delegate adapter: Java's `gson.getDelegateAdapter(this, TypeToken.get(subtype)).fromJsonTree(..)`.
pub type Delegate<R> = fn(&Value) -> Box<R>;

/// java: public final class RuntimeTypeAdapterFactory<T> implements TypeAdapterFactory
pub struct RuntimeTypeAdapterFactory<R: ?Sized> {
    /// java: private final Class<?> baseType;
    base_type: &'static str,
    /// java: private final String typeFieldName;
    type_field_name: String,
    /// java: private final Map<String, Class<?>> labelToSubtype (+ the delegates `create` looks up for them)
    label_to_delegate: Vec<(String, Delegate<R>)>,
    /// java: private final boolean maintainType;
    maintain_type: bool,
    /// java: private Class<? extends T> fallbackType; (NEU modification)
    fallback: Option<Delegate<R>>,
    /// java: private String defaultType; (NEU modification)
    default_type: Option<String>,
}

impl<R: ?Sized> RuntimeTypeAdapterFactory<R> {
    /// Creates a new runtime type adapter for `baseType` using `typeFieldName` as the type field name.
    /// `maintainType` flag decides if the type will be stored in pojo or not.
    ///
    /// java: public static <T> RuntimeTypeAdapterFactory<T> of(Class<T> baseType, String typeFieldName, boolean maintainType)
    pub fn of(base_type: &'static str, type_field_name: &str, maintain_type: bool) -> Self {
        RuntimeTypeAdapterFactory {
            base_type,
            type_field_name: type_field_name.to_string(),
            label_to_delegate: Vec::new(),
            maintain_type,
            fallback: None,
            default_type: None,
        }
    }

    /// Registers `type` identified by `label`. Labels are case sensitive.
    ///
    /// java: public RuntimeTypeAdapterFactory<T> registerSubtype(Class<? extends T> type, String label)
    pub fn register_subtype(mut self, r#type: Delegate<R>, label: &str) -> Self {
        // java: if (subtypeToLabel.containsKey(type) || labelToSubtype.containsKey(label)) throw new IllegalArgumentException("types and labels must be unique");
        if self.label_to_delegate.iter().any(|(l, _)| l == label) {
            panic!("IllegalArgumentException: types and labels must be unique");
        }
        self.label_to_delegate.push((label.to_string(), r#type));
        self
    }

    /// java: public RuntimeTypeAdapterFactory<T> setFallbackType(Class<? extends T> type) (NEU modification)
    pub fn set_fallback_type(mut self, r#type: Delegate<R>) -> Self {
        self.fallback = Some(r#type);
        self
    }

    /// java: public RuntimeTypeAdapterFactory<T> setDefaultTypeTag(String type) (NEU modification)
    pub fn set_default_type_tag(mut self, r#type: &str) -> Self {
        self.default_type = Some(r#type.to_string());
        self
    }

    /// The created adapter's `read(JsonReader in)`.
    ///
    /// `Err` is the `JsonParseException` (or the `IllegalStateException` of `getAsJsonObject` on a non-object).
    pub fn read(&self, json_element: &Value) -> Result<Box<R>, String> {
        let Some(object) = json_element.as_object() else {
            return Err(format!("java.lang.IllegalStateException: Not a JSON Object: {}", json_element));
        };
        // java: if (maintainType) labelJsonElement = jsonElement.getAsJsonObject().get(typeFieldName);
        //       else labelJsonElement = jsonElement.getAsJsonObject().remove(typeFieldName);
        let mut label_json_element: Option<Value> = object.get(&self.type_field_name).cloned();
        let stripped: Value;
        let delegate_input: &Value = if self.maintain_type {
            json_element
        } else {
            let mut copy = object.clone();
            copy.remove(&self.type_field_name);
            stripped = Value::Object(copy);
            &stripped
        };

        // NEU modification
        // java: if (defaultType != null && labelJsonElement == null) labelJsonElement = new JsonPrimitive(defaultType);
        if label_json_element.is_none() {
            if let Some(default_type) = &self.default_type {
                label_json_element = Some(Value::String(default_type.clone()));
            }
        }

        let Some(label_json_element) = label_json_element else {
            return Err(format!(
                "JsonParseException: cannot deserialize {} because it does not define a field named {}",
                self.base_type, self.type_field_name
            ));
        };
        // java: String label = labelJsonElement.getAsString();
        let label: String = match &label_json_element {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            other => return Err(format!("java.lang.UnsupportedOperationException: JsonNull/JsonObject/JsonArray getAsString: {}", other)),
        };
        // java: TypeAdapter<R> delegate = labelToDelegate.getOrDefault(label, fallbackAdapter);
        let delegate: Option<Delegate<R>> =
            self.label_to_delegate.iter().find(|(l, _)| *l == label).map(|(_, d)| *d).or(self.fallback);
        let Some(delegate) = delegate else {
            return Err(format!(
                "JsonParseException: cannot deserialize {} subtype named {}; did you forget to register a subtype?",
                self.base_type, label
            ));
        };
        // java: return delegate.fromJsonTree(jsonElement);
        Ok(delegate(delegate_input))
    }
}
