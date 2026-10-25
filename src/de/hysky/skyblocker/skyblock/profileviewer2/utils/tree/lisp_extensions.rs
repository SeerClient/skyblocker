//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.tree.LispExtensions by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: public static final Codec<LispAst.Program> PROGRAM_CODEC = Codec.STRING.xmap(source -> LispParser.Companion.parse("<codec>", source), LispAst.Program::toSource);
///
/// (the parser's exception is the decode's error, as Java's `xmap` function throwing out of `parse` is)
pub static PROGRAM_CODEC: compat::MainThread<CodecRef<crate::compat::nea_lisp::Program>> = compat::MainThread::new(|| {
    mcrust::com::mojang::serialization::codec::comap_flat_map(
        mcrust::com::mojang::serialization::codec::string_codec(),
        |source: String| match crate::compat::nea_lisp::LispParser::parse("<codec>", &source) {
            Ok(program) => mcrust::com::mojang::serialization::data_result::DataResult::success(program),
            Err(message) => mcrust::com::mojang::serialization::data_result::DataResult::error(std::sync::Arc::new(move || message.clone())),
        },
        |program: &crate::compat::nea_lisp::Program| program.to_source(),
    )
});

/// java: public class LispExtensions
#[derive(Clone)]
pub struct LispExtensions {}

impl LispExtensions {
    /// java: protected LispExtensions()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @LispBinding(name = "pow") public LispData.LispNumber pow(double base, double exponent)
    pub fn pow(&self, mut base: f64, mut exponent: f64) -> crate::compat::nea_lisp::LispData {
        return crate::compat::nea_lisp::LispData::LispNumber(base.powf(exponent));
    }

    /// java: @LispBinding(name = "round") public LispData.LispNumber round(double number)
    pub fn round(&self, mut number: f64) -> crate::compat::nea_lisp::LispData {
        // (`Math.round(double)` - `floor(x + 0.5)` as a long)
        return crate::compat::nea_lisp::LispData::LispNumber(compat::java_round_f64(number) as f64);
    }

    /// java: @LispBinding(name = "ceil") public LispData.LispNumber ceil(double number)
    pub fn ceil(&self, mut number: f64) -> crate::compat::nea_lisp::LispData {
        return crate::compat::nea_lisp::LispData::LispNumber(number.ceil());
    }

    /// java: @LispBinding(name = "floor") public LispData.LispNumber floor(double number)
    pub fn floor(&self, mut number: f64) -> crate::compat::nea_lisp::LispData {
        return crate::compat::nea_lisp::LispData::LispNumber(number.floor());
    }

    /// java: @LispBinding(name = "format-int") public LispData.LispString formatInt(int number)
    pub fn format_int(&self, mut number: i32) -> crate::compat::nea_lisp::LispData {
        return crate::compat::nea_lisp::LispData::LispString(crate::de::hysky::skyblocker::utils::formatters::INTEGER_NUMBERS.get_mut().format(number as f64));
    }

}

impl LispExtensions {
    /// `new AutoBinder().bindTo(new LispExtensions(), frame)` - each `@LispBinding(name = ..)` method bound under its
    /// name, its parameters converted from the Lisp values it is called with (`double` / `int` from a number)
    pub fn bind_to(&self, frame: &crate::compat::nea_lisp::StackFrame) {
        use crate::compat::nea_lisp::{LispData, LispExecutionContext};
        fn number(context: &LispExecutionContext, callsite: &crate::compat::nea_lisp::LispNode, value: Option<&LispData>) -> Option<f64> {
            match value.and_then(LispData::as_number) {
                Some(n) => Some(n),
                None => {
                    context.report_error("Expected a number argument", callsite);
                    None
                }
            }
        }
        let this = self.clone();
        LispExecutionContext::bind_function_in(frame, "pow", move |context, callsite, values| match (number(context, callsite, values.first()), number(context, callsite, values.get(1))) {
            (Some(base), Some(exponent)) if values.len() == 2 => this.pow(base, exponent),
            _ => LispData::LispNil,
        });
        let this = self.clone();
        LispExecutionContext::bind_function_in(frame, "round", move |context, callsite, values| match number(context, callsite, values.first()) {
            Some(n) if values.len() == 1 => this.round(n),
            _ => LispData::LispNil,
        });
        let this = self.clone();
        LispExecutionContext::bind_function_in(frame, "ceil", move |context, callsite, values| match number(context, callsite, values.first()) {
            Some(n) if values.len() == 1 => this.ceil(n),
            _ => LispData::LispNil,
        });
        let this = self.clone();
        LispExecutionContext::bind_function_in(frame, "floor", move |context, callsite, values| match number(context, callsite, values.first()) {
            Some(n) if values.len() == 1 => this.floor(n),
            _ => LispData::LispNil,
        });
        let this = self.clone();
        LispExecutionContext::bind_function_in(frame, "format-int", move |context, callsite, values| match number(context, callsite, values.first()) {
            Some(n) if values.len() == 1 => this.format_int(n as i32),
            _ => LispData::LispNil,
        });
    }
}
