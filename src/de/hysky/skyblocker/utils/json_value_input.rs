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
        // TODO(fabric2seer): JsonElement.getClass
        //   java: return () -> "Expected json primitive for " + name + ", got " + element.getClass().getSimpleName() + " instead.";
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public <T> Optional<T> read(String name, Codec<T> codec)
    pub fn read<T: 'static>(&self, name: &str, codec: &CodecRef<T>) -> Option<T> {
        let Some(mut element) = self.input.as_ref().unwrap().get(name).cloned() else {
            return None;
        };
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (codec.parse(JsonOps.INSTANCE, element)) {
        //   java: 			case DataResult.Success<T> success -> Optional.of(success.value());
        //   java: 			case DataResult.Error<T> error -> {
        //   java: 				reporter.report(() -> "Failed to decode value '" + element + "' from field '" + name + "': " + error.message());
        //   java: 				yield error.partialValue();
        //   java: 			}
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public <T> Optional<T> read(MapCodec<T> codec)
    pub fn read_1<T: 'static>(&self, codec: &MapCodecRef<T>) -> Option<T> {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (JsonOps.INSTANCE.getMap(this.input).flatMap(/* lambda$read$0 */ map -> codec.decode(JsonOps.INSTANCE, map))) {
        //   java: 			case DataResult.Success<T> success -> Optional.of(success.value());
        //   java: 			case DataResult.Error<T> error -> {
        //   java: 				this.reporter.report(() ->  "Failed to decode from map: " + error.message());
        //   java: 				yield error.partialValue();
        //   java: 			}
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public OptionalInt readInt(String name)
    pub fn read_int(&self, name: &str) -> Option<i32> {
        let Some(mut element) = self.read_number(name) else {
            return None;
        };
        // TODO(fabric2seer): `.intValue(..)` on a Number
        //   java: return OptionalInt.of(element.intValue());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public int readIntOr(String name, int defaultValue)
    pub fn read_int_or(&self, name: &str, mut default_value: i32) -> i32 {
        let Some(mut element) = self.read_number(name) else {
            return default_value;
        };
        // TODO(fabric2seer): `.intValue(..)` on a Number
        //   java: return element.intValue();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public OptionalLong readLong(String name)
    pub fn read_long(&self, name: &str) -> Option<i64> {
        let Some(mut element) = self.read_number(name) else {
            // TODO(fabric2seer): static call OptionalLong.empty(..)
            //   java: return OptionalLong.empty();
        };
        // TODO(fabric2seer): static call OptionalLong.of(..)
        //   java: return OptionalLong.of(element.longValue());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public long readLongOr(String name, long defaultValue)
    pub fn read_long_or(&self, name: &str, mut default_value: i64) -> i64 {
        let Some(mut element) = self.read_number(name) else {
            return default_value;
        };
        // TODO(fabric2seer): `.longValue(..)` on a Number
        //   java: return element.longValue();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public OptionalDouble readDouble(String name)
    pub fn read_double(&self, name: &str) -> Option<f64> {
        let Some(mut element) = self.read_number(name) else {
            return None;
        };
        // TODO(fabric2seer): `.longValue(..)` on a Number
        //   java: return OptionalDouble.of(element.longValue());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public double readDoubleOr(String name, double defaultValue)
    pub fn read_double_or(&self, name: &str, mut default_value: f64) -> f64 {
        let Some(mut element) = self.read_number(name) else {
            return default_value;
        };
        // TODO(fabric2seer): `.doubleValue(..)` on a Number
        //   java: return element.doubleValue();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public Optional<Float> readFloat(String name)
    pub fn read_float(&self, name: &str) -> Option<f32> {
        let Some(mut element) = self.read_number(name) else {
            return None;
        };
        // TODO(fabric2seer): `.floatValue(..)` on a Number
        //   java: return Optional.of(element.floatValue());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public float readFloatOr(String name, float defaultValue)
    pub fn read_float_or(&self, name: &str, mut default_value: f32) -> f32 {
        let Some(mut element) = self.read_number(name) else {
            return default_value;
        };
        // TODO(fabric2seer): `.floatValue(..)` on a Number
        //   java: return element.floatValue();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public Optional<Boolean> readBoolean(String name)
    pub fn read_boolean(&self, name: &str) -> Option<bool> {
        let Some(mut element) = self.input.as_ref().unwrap().get(name).cloned() else {
            return None;
        };
        if !(compat::json::is_primitive(&element)) {
            self.reporter.as_ref().unwrap().report(JsonValueInput::expected_primtive_problem(name, &element));
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
            self.reporter.as_ref().unwrap().report(JsonValueInput::expected_primtive_problem(name, &element));
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
            self.reporter.as_ref().unwrap().report(JsonValueInput::expected_primtive_problem(name, &element));
            return None;
        }
        // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(compat::json::get_as_number(&compat::json::get_as_json_primitive(&element)));
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.util.ProblemReporter.Problem`)
                // rust: self.reporter.as_ref().unwrap().report(|| format!("Expected valid number for {}, got {} instead.", name, compat::json::get_as_string(&element)));
                return None;
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
