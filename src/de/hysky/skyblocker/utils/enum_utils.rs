//! Ported from de.hysky.skyblocker.utils.EnumUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// `java.lang.Enum<E>`'s surface that generic code reaches (`T extends Enum<T>`): a converted enum answers it from its
/// own `name()` / `ordinal()` / `values()`.
pub trait JavaEnum: Sized + 'static {
    /// java: `Enum.name()`
    fn java_name(&self) -> &'static str;
    /// java: `Enum.ordinal()`
    fn java_ordinal(&self) -> i32;
    /// java: `getDeclaringClass().getEnumConstants()`
    fn java_values() -> Vec<Self>;
}

/// java: public class EnumUtils
#[derive(Clone)]
pub struct EnumUtils {}

impl EnumUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <T extends Enum<T>> T cycle(T current)
    pub fn cycle<T: 'static>(current: &T) -> T {
        // TODO(fabric2seer): `.getDeclaringClass(..)` on a T
        //   java: T[] values = current.getDeclaringClass().getEnumConstants();
        let mut values: Vec<T> = todo!("fabric2seer: the initialiser of `values` (TODO above)");
        // TODO(fabric2seer): `.ordinal(..)` on a T
        //   java: return values[(current.ordinal() + 1) % values.length];
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <T> T cycle(List<T> list, int currentIndex)
    pub fn cycle_2<T: Clone + 'static>(list: &Vec<T>, mut current_index: i32) -> T {
        return list[((current_index.wrapping_add(1)).wrapping_rem(list.len() as i32)) as usize].clone();
    }

    /// java: public static <T extends Enum<T>> T cycleBackwards(T current)
    pub fn cycle_backwards<T: 'static>(current: &T) -> T {
        // TODO(fabric2seer): `.getDeclaringClass(..)` on a T
        //   java: T[] values = current.getDeclaringClass().getEnumConstants();
        let mut values: Vec<T> = todo!("fabric2seer: the initialiser of `values` (TODO above)");
        // TODO(fabric2seer): `.ordinal(..)` on a T
        //   java: return values[(current.ordinal() - 1 + values.length) % values.length];
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
