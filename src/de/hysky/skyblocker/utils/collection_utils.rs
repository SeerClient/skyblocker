//! Ported from de.hysky.skyblocker.utils.CollectionUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public final class CollectionUtils
#[derive(Clone)]
pub struct CollectionUtils {}

impl CollectionUtils {
    /// java: private CollectionUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <T extends Enum<T>> Collector<T, ?, EnumSet<T>> enumSetCollector(Class<T> elementType)
    ///
    /// The mod's `EnumSet<T>` is a `HashSet<T>` (`ChatRule.validLocations`); the collector is the function a stream
    /// is collected with: `enum_set_collector::<Location>(TypeId::of::<Location>())(&mut iter)`.
    pub fn enum_set_collector<T: Eq + std::hash::Hash + 'static>(element_type: std::any::TypeId) -> impl Fn(&mut dyn Iterator<Item = T>) -> std::collections::HashSet<T> {
        // java: return Collectors.toCollection(() -> EnumSet.noneOf(elementType));
        let _ = element_type;
        move |stream: &mut dyn Iterator<Item = T>| {
            let mut set: std::collections::HashSet<T> = std::collections::HashSet::new();
            for e in stream {
                set.insert(e);
            }
            set
        }
    }

    /// java: public static <E> Pair<List<E>, List<E>> halve(List<E> list)
    pub fn halve<E: Clone + 'static>(list: &Vec<E>) -> (Vec<E>, Vec<E>) {
        // java: int mid = Math.ceilDiv(list.size(), 2);
        let mut mid: i32 = list.len().div_ceil(2) as i32;
        let mut first_half: Vec<E> = list[(0) as usize..(mid) as usize].to_vec();
        let mut second_half: Vec<E> = list[(mid) as usize..((list.len() as i32)) as usize].to_vec();
        return (first_half.clone(), second_half.clone());
    }

}
