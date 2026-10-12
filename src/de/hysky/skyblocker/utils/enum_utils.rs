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
    pub fn cycle<T: JavaEnum>(current: &T) -> T {
        // java: T[] values = current.getDeclaringClass().getEnumConstants();
        let mut values: Vec<T> = T::java_values();
        // java: return values[(current.ordinal() + 1) % values.length];
        let index = (current.java_ordinal().wrapping_add(1)).wrapping_rem(values.len() as i32);
        return values.swap_remove(index as usize);
    }

    /// java: public static <T> T cycle(List<T> list, int currentIndex)
    pub fn cycle_2<T: Clone + 'static>(list: &Vec<T>, mut current_index: i32) -> T {
        return list[((current_index.wrapping_add(1)).wrapping_rem(list.len() as i32)) as usize].clone();
    }

    /// java: public static <T extends Enum<T>> T cycleBackwards(T current)
    pub fn cycle_backwards<T: JavaEnum>(current: &T) -> T {
        // java: T[] values = current.getDeclaringClass().getEnumConstants();
        let mut values: Vec<T> = T::java_values();
        // java: return values[(current.ordinal() - 1 + values.length) % values.length];
        let index = ((current.java_ordinal().wrapping_sub(1)).wrapping_add(values.len() as i32)).wrapping_rem(values.len() as i32);
        return values.swap_remove(index as usize);
    }

}

/// `Enum<SkyblockItemRarity>` for generic code (`SkyblockItemRarity.next()` is `EnumUtils.cycle(this)`).
impl JavaEnum for crate::de::hysky::skyblocker::skyblock::item::skyblock_item_rarity::SkyblockItemRarity {
    fn java_name(&self) -> &'static str {
        self.name()
    }
    fn java_ordinal(&self) -> i32 {
        self.ordinal()
    }
    fn java_values() -> Vec<Self> {
        Self::values()
    }
}
