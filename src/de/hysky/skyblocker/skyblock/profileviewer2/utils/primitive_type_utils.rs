//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.PrimitiveTypeUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::lang::number::Number;

/// java: public class PrimitiveTypeUtils
#[derive(Clone)]
pub struct PrimitiveTypeUtils {}

impl PrimitiveTypeUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int coerceInt(@Nullable Object object)
    pub fn coerce_int(mut object: Option<compat::Object>) -> i32 {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return object instanceof Number number ? number.intValue() : 0;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static double coerceDouble(@Nullable Object object)
    pub fn coerce_double(mut object: Option<compat::Object>) -> f64 {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return object instanceof Number number ? number.doubleValue() : 0d;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static boolean coerceBoolean(@Nullable Object object)
    pub fn coerce_boolean(mut object: Option<compat::Object>) -> bool {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return object instanceof Boolean bool ? bool : false;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
