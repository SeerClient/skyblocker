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

    /// java: `object instanceof Number number` - the boxed numeric types an `Object` can hold
    fn as_number(object: &Option<compat::Object>) -> Option<Number> {
        let object = object.as_ref()?;
        if let Some(v) = object.downcast_ref::<Number>() { return Some(*v); }
        if let Some(v) = object.downcast_ref::<i8>() { return Some(Number::Byte(*v)); }
        if let Some(v) = object.downcast_ref::<i16>() { return Some(Number::Short(*v)); }
        if let Some(v) = object.downcast_ref::<i32>() { return Some(Number::Int(*v)); }
        if let Some(v) = object.downcast_ref::<i64>() { return Some(Number::Long(*v)); }
        if let Some(v) = object.downcast_ref::<f32>() { return Some(Number::Float(*v)); }
        if let Some(v) = object.downcast_ref::<f64>() { return Some(Number::Double(*v)); }
        None
    }

    /// java: public static int coerceInt(@Nullable Object object)
    pub fn coerce_int(object: Option<compat::Object>) -> i32 {
        // java: return object instanceof Number number ? number.intValue() : 0;
        match PrimitiveTypeUtils::as_number(&object) { Some(number) => number.int_value(), None => 0 }
    }

    /// java: public static double coerceDouble(@Nullable Object object)
    pub fn coerce_double(object: Option<compat::Object>) -> f64 {
        // java: return object instanceof Number number ? number.doubleValue() : 0d;
        match PrimitiveTypeUtils::as_number(&object) { Some(number) => number.double_value(), None => 0.0 }
    }

    /// java: public static boolean coerceBoolean(@Nullable Object object)
    pub fn coerce_boolean(object: Option<compat::Object>) -> bool {
        // java: return object instanceof Boolean bool ? bool : false;
        match object.as_ref().and_then(|o| o.downcast_ref::<bool>()) { Some(b) => *b, None => false }
    }

}
