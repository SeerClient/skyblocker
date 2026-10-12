//! Ported from de.hysky.skyblocker.utils.ExclusiveGroupingSet by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: public final class ExclusiveGroupingSet<T extends Enum<T>>
#[derive(Clone)]
pub struct ExclusiveGroupingSet<T> {
    /// java: @VisibleForTesting final List<Set<T>> sets;
    pub sets: Vec<std::collections::HashSet<T>>,
}

impl<T: Clone + 'static> ExclusiveGroupingSet<T> {
    /// java: public ExclusiveGroupingSet()
    pub fn new() -> compat::Shared<Self> {
        let __this = Self::new_1(Vec::new());
        __this
    }

    /// java: private ExclusiveGroupingSet(List<Set<T>> sets)
    pub fn new_1(mut sets: Vec<std::collections::HashSet<T>>) -> compat::Shared<Self> {
        let __init_sets = sets.clone();
        let __this = compat::shared(Self {
            sets: __init_sets,
        });
        __this
    }

    /// java: public static <T extends Enum<T>> Codec<ExclusiveGroupingSet<T>> getCodec(Codec<Set<T>> setCodec)
    pub fn get_codec<T: 'static>(set_codec: &CodecRef<std::collections::HashSet<T>>) -> CodecRef<compat::Shared<ExclusiveGroupingSet<T>>> {
        // TODO(fabric2seer): static call Function.identity(..)
        //   java: return setCodec.listOf()
        //   java: 				.<List<Set<T>>>xmap(ArrayList::new, Function.identity())
        //   java: 				.xmap(ExclusiveGroupingSet::new, s -> s.sets);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public Optional<Set<T>> getGroup(T location)
    pub fn get_group(__this: &compat::Shared<Self>, location: &T) -> Option<std::collections::HashSet<T>> {
        // TODO(fabric2seer): Set.copyOf(..)
        //   java: return sets.stream().filter(locations -> locations.contains(location)).findAny().map(EnumSet::copyOf);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public void group(Set<T> locations)
    pub fn group(__this: &compat::Shared<Self>, locations: &std::collections::HashSet<T>) {
        ExclusiveGroupingSet::remove_all(__this, locations);
        if (locations.len() as i32) > 1 {
            let __h0 = locations.iter().cloned().collect::<Vec<_>>();
            __this.lock().borrow_mut().sets.push(__h0);
        }
    }

    /// java: public void remove(T location)
    pub fn remove(__this: &compat::Shared<Self>, location: &T) {
        for set in compat::read(__this, |__t| __t.sets.clone()) {
            set.remove(location);
        }
        ExclusiveGroupingSet::clean_up(__this);
    }

    /// java: public void removeAll(Collection<T> locations)
    pub fn remove_all(__this: &compat::Shared<Self>, locations: &Vec<T>) {
        for set in compat::read(__this, |__t| __t.sets.clone()) {
            { for __x in locations.iter().cloned() { set.remove(&__x); } };
        }
        ExclusiveGroupingSet::clean_up(__this);
    }

    /// java: public void cleanUp()
    pub fn clean_up(__this: &compat::Shared<Self>) {
        let __h0 = |s| !(((s.len() as i32) <= 1));
        __this.lock().borrow_mut().sets.retain(__h0);
    }

}
