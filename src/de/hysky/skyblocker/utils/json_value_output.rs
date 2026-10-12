//! Ported from de.hysky.skyblocker.utils.JsonValueOutput by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::java::lang::number::Number;
use mcrust::net::minecraft::util::problem_reporter::ProblemReporter;
use mcrust::com::mojang::serialization::encoder::Encoder as _;
use std::rc::Rc;

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
    pub fn write<T: std::fmt::Debug + 'static>(&mut self, mut name: String, codec: &CodecRef<T>, value: &T) {
        match codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), value) {
            DataResult::Success(success, _) => {
                // java: this.output.add(name, success.value());
                self.output.as_mut().unwrap().insert(name.clone(), compat::json_from_port(&success));
            }
            DataResult::Error(error) => {
                // java: this.reporter.report(() -> "Failed to encode '" + value + "' into field '" + name + "': " + error);
                // (`String.valueOf(value)`: the value's `Debug` form - `T` carries no `toString` here)
                // `+ error` is `DataResult.Error.toString()`: "DataResult.Error['" + message() + "'" + partial + "]"
                let error_string = format!("DataResult.Error['{}'{}]", error.message(), error.partial_value().map(|p| format!(": {p:?}")).unwrap_or_default());
                let description = format!("Failed to encode '{:?}' into field '{}': {}", value, name, error_string);
                self.reporter.as_ref().unwrap().clone().report(Rc::new(EncodeProblem(description)));
                if let Some(partial) = error.partial_value() {
                    self.output.as_mut().unwrap().insert(name.clone(), compat::json_from_port(partial));
                }
            }
        }
    }

    /// java: public void writeNumber(String field, Number value)
    pub fn write_number(&mut self, field: &str, value: &Number) {
        // java: output.addProperty(field, value) - a JsonPrimitive of the boxed number
        let json: serde_json::Value = match *value {
            Number::Byte(v) => serde_json::json!(v),
            Number::Short(v) => serde_json::json!(v),
            Number::Int(v) => serde_json::json!(v),
            Number::Long(v) => serde_json::json!(v),
            Number::Float(v) => serde_json::json!(v),
            Number::Double(v) => serde_json::json!(v),
        };
        self.output.as_mut().unwrap().insert(field.to_string(), json);
    }

    /// java: public void writeBool(String field, boolean value)
    pub fn write_bool(&mut self, field: &str, mut value: bool) {
        self.output.as_mut().unwrap().insert(field.to_string(), serde_json::json!(value));
    }

    /// java: public void writeString(String field, String value)
    pub fn write_string(&mut self, field: &str, value: &str) {
        self.output.as_mut().unwrap().insert(field.to_string(), serde_json::json!(value.to_string()));
    }

}

/// The `ProblemReporter.Problem` lambda of `write`: `() -> description`.
#[derive(Debug)]
struct EncodeProblem(String);

impl mcrust::net::minecraft::util::problem_reporter::Problem for EncodeProblem {
    fn description(&self) -> String {
        self.0.clone()
    }
}
