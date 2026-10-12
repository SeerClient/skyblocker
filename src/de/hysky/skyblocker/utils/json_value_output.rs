//! Ported from de.hysky.skyblocker.utils.JsonValueOutput by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::java::lang::number::Number;
use mcrust::net::minecraft::util::problem_reporter::ProblemReporter;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: public class JsonValueOutput
pub struct JsonValueOutput {
    /// java: private final JsonObject output;
    pub output: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: private final ProblemReporter reporter;
    pub reporter: Option<Rc<dyn ProblemReporter>>,
}

impl JsonValueOutput {
    /// java: public JsonValueOutput(ProblemReporter reporter, JsonObject output)
    pub fn new(mut reporter: Rc<dyn ProblemReporter>, mut output: serde_json::Map<String, serde_json::Value>) -> Self {
        let __init_output = Some(output.clone());
        let __init_reporter = Some(reporter.clone());
        let mut this = Self {
            output: __init_output,
            reporter: __init_reporter,
        };
        this
    }

    /// java: public JsonValueOutput(JsonObject output)
    pub fn new_1(mut output: serde_json::Map<String, serde_json::Value>) -> Self {
        let mut this = Self::new(mcrust::net::minecraft::util::problem_reporter::discarding(), output.clone());
        this
    }

    /// java: public <T> void write(final String name, final Codec<T> codec, final T value)
    pub fn write<T: 'static>(&mut self, mut name: String, codec: &CodecRef<T>, value: &T) {
        if let Some(success) = (codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), value)).as_any().downcast_ref::<DataResult<serde_json::Value>>() {
            // TODO(fabric2seer): `Success.value` has no Rust method in the port (looked for `value` in DataResult and JavaSrc)
            // rust: self.output.as_mut().unwrap().insert(name.clone(), compat::json::to_value(success.value()));
        } else if let Some(error) = (codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), value)).as_any().downcast_ref::<mcrust::com::mojang::serialization::data_result::Error<serde_json::Value>>() {
            // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.util.ProblemReporter.Problem`)
            // rust: self.reporter.as_ref().unwrap().report(|| format!("Failed to encode '{}' into field '{}': {}", compat::jstring(&value), name, compat::jstring(&error)));
            if let Some(partial) = error.partial_value() {
                self.output.as_mut().unwrap().insert(name.clone(), compat::json::to_value(partial.clone()));
            }
        } else {
            panic!("MatchException: {}", compat::jstring(&None));
        }
    }

    /// java: public void writeNumber(String field, Number value)
    pub fn write_number(&self, field: &str, value: &Number) {
        self.output.as_mut().unwrap().insert(field.to_string(), serde_json::json!(value.clone()));
    }

    /// java: public void writeBool(String field, boolean value)
    pub fn write_bool(&self, field: &str, mut value: bool) {
        self.output.as_mut().unwrap().insert(field.to_string(), serde_json::json!(value));
    }

    /// java: public void writeString(String field, String value)
    pub fn write_string(&self, field: &str, value: &str) {
        self.output.as_mut().unwrap().insert(field.to_string(), serde_json::json!(value.to_string()));
    }

}
