//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.tree.LispExtensions by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

// TODO(fabric2seer): static field LispParser.Companion
//   java: public static final Codec<LispAst.Program> PROGRAM_CODEC = Codec.STRING.xmap(source -> LispParser.Companion.parse("<codec>", source), LispAst.Program::toSource);
/// java: public static final Codec<LispAst.Program> PROGRAM_CODEC = Codec.STRING.xmap(source -> LispParser.Companion.parse("<codec>", source), LispAst.Program::toSource);
pub static PROGRAM_CODEC: compat::MainThread<CodecRef<crate::unresolved::Program>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `PROGRAM_CODEC` - see the TODO above"));

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
    pub fn pow(&self, mut base: f64, mut exponent: f64) -> crate::unresolved::LispNumber {
        // TODO(fabric2seer): `new LispNumber(..)`
        //   java: return new LispData.LispNumber(Math.pow(base, exponent));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @LispBinding(name = "round") public LispData.LispNumber round(double number)
    pub fn round(&self, mut number: f64) -> crate::unresolved::LispNumber {
        // TODO(fabric2seer): `new LispNumber(..)`
        //   java: return new LispData.LispNumber(Math.round(number));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @LispBinding(name = "ceil") public LispData.LispNumber ceil(double number)
    pub fn ceil(&self, mut number: f64) -> crate::unresolved::LispNumber {
        // TODO(fabric2seer): `new LispNumber(..)`
        //   java: return new LispData.LispNumber(Math.ceil(number));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @LispBinding(name = "floor") public LispData.LispNumber floor(double number)
    pub fn floor(&self, mut number: f64) -> crate::unresolved::LispNumber {
        // TODO(fabric2seer): `new LispNumber(..)`
        //   java: return new LispData.LispNumber(Math.floor(number));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @LispBinding(name = "format-int") public LispData.LispString formatInt(int number)
    pub fn format_int(&self, mut number: i32) -> crate::unresolved::LispString {
        // TODO(fabric2seer): `new LispString(..)`
        //   java: return new LispData.LispString(Formatters.INTEGER_NUMBERS.format(number));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
