//! Ported from de.hysky.skyblocker.utils.JsonValueInput by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::java::lang::number::Number;
use mcrust::net::minecraft::util::problem_reporter::Problem;
use mcrust::net::minecraft::util::problem_reporter::ProblemReporter;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::dynamic_ops::DynamicOps as _;
use mcrust::com::mojang::serialization::map_decoder::MapDecoder as _;
use mcrust::com::mojang::serialization::data_result::DataResult;
use std::rc::Rc;

/// A `ProblemReporter.Problem` lambda (`() -> "..."`): its description.
#[derive(Debug)]
pub struct DescribedProblem(pub String);

impl Problem for DescribedProblem {
    fn description(&self) -> String {
        self.0.clone()
    }
}

/// `element.getClass().getSimpleName()` of a Gson tree node.
fn json_class_name(element: &serde_json::Value) -> &'static str {
    match element {
        serde_json::Value::Object(_) => "JsonObject",
        serde_json::Value::Array(_) => "JsonArray",
        serde_json::Value::Null => "JsonNull",
        _ => "JsonPrimitive",
    }
}

/// java: public class JsonValueInput
pub struct JsonValueInput {
    /// java: private final JsonObject input;
    pub input: Option<serde_json::Map<String, serde_json::Value>>,
    /// java: private final ProblemReporter reporter;
    pub reporter: Option<Rc<dyn ProblemReporter>>,
}

impl JsonValueInput {
    /// java: public JsonValueInput(ProblemReporter reporter, JsonObject input)
    pub fn new(mut reporter: Rc<dyn ProblemReporter>, mut input: serde_json::Map<String, serde_json::Value>) -> Self {
        let __init_input = Some(input.clone());
        let __init_reporter = Some(reporter.clone());
        let mut this = Self {
            input: __init_input,
            reporter: __init_reporter,
        };
        this
    }

    /// java: public JsonValueInput(JsonObject input)
    pub fn new_1(mut input: serde_json::Map<String, serde_json::Value>) -> Self {
        let mut this = Self::new(mcrust::net::minecraft::util::problem_reporter::discarding(), input.clone());
        this
    }

    /// java: private static ProblemReporter.Problem expectedPrimtiveProblem(String name, JsonElement element)
    pub fn expected_primtive_problem(name: &str, element: &serde_json::Value) -> Rc<dyn Problem> {
        // java: return () -> "Expected json primitive for " + name + ", got " + element.getClass().getSimpleName() + " instead.";
        return Rc::new(DescribedProblem(format!("Expected json primitive for {}, got {} instead.", name, json_class_name(element))));
    }

    /// java: public <T> Optional<T> read(String name, Codec<T> codec)
    pub fn read<T: Clone + 'static>(&self, name: &str, codec: &CodecRef<T>) -> Option<T> {
        let Some(mut element) = self.input.as_ref().unwrap().get(name).cloned() else {
            return None;
        };
        return match codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&element)) {
            DataResult::Success(success, _) => Some(success),
            DataResult::Error(error) => {
                self.reporter.clone().unwrap().report(Rc::new(DescribedProblem(format!("Failed to decode value '{}' from field '{}': {}", element, name, error.message()))));
                error.partial_value().cloned()
            }
        };
    }

    /// java: public <T> Optional<T> read(MapCodec<T> codec)
    pub fn read_1<T: Clone + 'static>(&self, codec: &MapCodecRef<T>) -> Option<T> {
        // java: JsonOps.INSTANCE.getMap(this.input).flatMap(map -> codec.decode(JsonOps.INSTANCE, map))
        let ops = mcrust::com::mojang::serialization::json_ops::JsonOps::instance();
        let input = compat::json_to_port(&serde_json::Value::Object(self.input.clone().unwrap()));
        return match ops.get_map(&ops, &input).flat_map(|map| codec.decode(&ops, &map)) {
            DataResult::Success(success, _) => Some(success),
            DataResult::Error(error) => {
                self.reporter.clone().unwrap().report(Rc::new(DescribedProblem(format!("Failed to decode from map: {}", error.message()))));
                error.partial_value().cloned()
            }
        };
    }

    /// java: public OptionalInt readInt(String name)
    pub fn read_int(&self, name: &str) -> Option<i32> {
        let Some(mut element) = self.read_number(name) else {
            return None;
        };
        return Some(element.int_value());
    }

    /// java: public int readIntOr(String name, int defaultValue)
    pub fn read_int_or(&self, name: &str, mut default_value: i32) -> i32 {
        let Some(mut element) = self.read_number(name) else {
            return default_value;
        };
        return element.int_value();
    }

    /// java: public OptionalLong readLong(String name)
    pub fn read_long(&self, name: &str) -> Option<i64> {
        let Some(mut element) = self.read_number(name) else {
            return None;
        };
        return Some(element.long_value());
    }

    /// java: public long readLongOr(String name, long defaultValue)
    pub fn read_long_or(&self, name: &str, mut default_value: i64) -> i64 {
        let Some(mut element) = self.read_number(name) else {
            return default_value;
        };
        return element.long_value();
    }

    /// java: public OptionalDouble readDouble(String name)
    pub fn read_double(&self, name: &str) -> Option<f64> {
        let Some(mut element) = self.read_number(name) else {
            return None;
        };
        // java: return OptionalDouble.of(element.longValue()); (sic: the long value, widened)
        return Some(element.long_value() as f64);
    }

    /// java: public double readDoubleOr(String name, double defaultValue)
    pub fn read_double_or(&self, name: &str, mut default_value: f64) -> f64 {
        let Some(mut element) = self.read_number(name) else {
            return default_value;
        };
        return element.double_value();
    }

    /// java: public Optional<Float> readFloat(String name)
    pub fn read_float(&self, name: &str) -> Option<f32> {
        let Some(mut element) = self.read_number(name) else {
            return None;
        };
        return Some(element.float_value());
    }

    /// java: public float readFloatOr(String name, float defaultValue)
    pub fn read_float_or(&self, name: &str, mut default_value: f32) -> f32 {
        let Some(mut element) = self.read_number(name) else {
            return default_value;
        };
        return element.float_value();
    }

    /// java: public Optional<Boolean> readBoolean(String name)
    pub fn read_boolean(&self, name: &str) -> Option<bool> {
        let Some(mut element) = self.input.as_ref().unwrap().get(name).cloned() else {
            return None;
        };
        if !(compat::json::is_primitive(&element)) {
            self.reporter.clone().unwrap().report(JsonValueInput::expected_primtive_problem(name, &element));
            return None;
        }
        return Some(compat::json::get_as_boolean(&element));
    }

    /// java: public boolean readBooleanOr(String name, boolean defaultValue)
    pub fn read_boolean_or(&self, name: &str, mut default_value: bool) -> bool {
        let Some(mut element) = self.input.as_ref().unwrap().get(name).cloned() else {
            return default_value;
        };
        if !(compat::json::is_primitive(&element)) {
            self.reporter.clone().unwrap().report(JsonValueInput::expected_primtive_problem(name, &element));
            return default_value;
        }
        return compat::json::get_as_boolean(&element);
    }

    /// java: private @Nullable Number readNumber(String name)
    pub fn read_number(&self, name: &str) -> Option<Number> {
        let Some(mut element) = self.input.as_ref().unwrap().get(name).cloned() else {
            return None;
        };
        if !(compat::json::is_primitive(&element)) {
            self.reporter.clone().unwrap().report(JsonValueInput::expected_primtive_problem(name, &element));
            return None;
        }
        // java: try { return element.getAsJsonPrimitive().getAsNumber(); } catch (NumberFormatException _) {..}
        // (a JSON number, or a string read as one; a boolean is Gson's UnsupportedOperationException, not caught)
        let number: Option<Number> = match &element {
            serde_json::Value::Number(n) => Some(match n.as_i64() {
                Some(v) => Number::Long(v),
                None => Number::Double(n.as_f64().unwrap_or(0.0)),
            }),
            serde_json::Value::String(s) => match s.trim().parse::<i64>() {
                Ok(v) => Some(Number::Long(v)),
                Err(_) => s.trim().parse::<f64>().ok().map(Number::Double),
            },
            other => panic!("UnsupportedOperationException: Primitive is neither a number nor a string: {other}"),
        };
        if number.is_none() {
            self.reporter.clone().unwrap().report(Rc::new(DescribedProblem(format!("Expected valid number for {}, got {} instead.", name, compat::json::get_as_string(&element)))));
        }
        return number;
    }

}
