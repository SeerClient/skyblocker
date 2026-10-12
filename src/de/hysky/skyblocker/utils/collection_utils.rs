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
    pub fn enum_set_collector<T: 'static>(element_type: &crate::unresolved::Class) -> crate::unresolved::Collector {
        // TODO(fabric2seer): Collectors.toCollection outside `.collect(..)`
        //   java: return Collectors.toCollection(() -> EnumSet.noneOf(elementType));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <E> Pair<List<E>, List<E>> halve(List<E> list)
    pub fn halve<E: 'static>(list: &Vec<E>) -> (Vec<E>, Vec<E>) {
        // TODO(fabric2seer): Math.ceilDiv
        //   java: int mid = Math.ceilDiv(list.size(), 2);
        let mut mid: i32 = todo!("fabric2seer: the initialiser of `mid` (TODO above)");
        let mut first_half: Vec<E> = list[(0) as usize..(mid) as usize].to_vec();
        let mut second_half: Vec<E> = list[(mid) as usize..((list.len() as i32)) as usize].to_vec();
        return (first_half.clone(), second_half.clone());
    }

}
