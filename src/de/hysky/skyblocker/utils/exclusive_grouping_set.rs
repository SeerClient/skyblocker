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

impl<T: Clone + Eq + std::hash::Hash + 'static> ExclusiveGroupingSet<T> {
    /// java: public ExclusiveGroupingSet()
    pub fn new() -> compat::Shared<Self> {
        let __this = Self::new_1(Vec::new());
        __this
    }

    /// java: private ExclusiveGroupingSet(List<Set<T>> sets)
    pub fn new_1(mut sets: Vec<std::collections::HashSet<T>>) -> compat::Shared<Self> {
        let __init_sets = sets;
        let __this = compat::shared(Self {
            sets: __init_sets,
        });
        __this
    }

    /// java: public static <T extends Enum<T>> Codec<ExclusiveGroupingSet<T>> getCodec(Codec<Set<T>> setCodec)
    pub fn get_codec(set_codec: &CodecRef<std::collections::HashSet<T>>) -> CodecRef<compat::Shared<ExclusiveGroupingSet<T>>> {
        // java: return setCodec.listOf().<List<Set<T>>>xmap(ArrayList::new, Function.identity())
        //           .xmap(ExclusiveGroupingSet::new, s -> s.sets);
        // (`ArrayList::new` copies the decoded list into a mutable one: a `Vec` already is one)
        let list_codec: CodecRef<Vec<std::collections::HashSet<T>>> = codec::list(set_codec.clone());
        let mutable_list_codec: CodecRef<Vec<std::collections::HashSet<T>>> = codec::xmap(list_codec, |sets: Vec<std::collections::HashSet<T>>| sets.into_iter().collect::<Vec<_>>(), |sets: &Vec<std::collections::HashSet<T>>| sets.clone());
        return codec::xmap(mutable_list_codec, |sets: Vec<std::collections::HashSet<T>>| ExclusiveGroupingSet::new_1(sets), |s: &compat::Shared<ExclusiveGroupingSet<T>>| compat::read(s, |__t| __t.sets.clone()));
    }

    /// java: public Optional<Set<T>> getGroup(T location)
    pub fn get_group(__this: &compat::Shared<Self>, location: &T) -> Option<std::collections::HashSet<T>> {
        // java: return sets.stream().filter(locations -> locations.contains(location)).findAny().map(EnumSet::copyOf);
        return compat::read(__this, |__t| __t.sets.iter().find(|locations| locations.contains(location)).cloned());
    }

    /// java: public void group(Set<T> locations)
    pub fn group(__this: &compat::Shared<Self>, locations: &std::collections::HashSet<T>) {
        ExclusiveGroupingSet::remove_all(__this, &locations.iter().cloned().collect::<Vec<_>>());
        if (locations.len() as i32) > 1 {
            // java: sets.add(EnumSet.copyOf(locations));
            let __h0: std::collections::HashSet<T> = locations.clone();
            compat::write(__this, |__t| __t.sets.push(__h0));
        }
    }

    /// java: public void remove(T location)
    pub fn remove(__this: &compat::Shared<Self>, location: &T) {
        compat::write(__this, |__t| {
            for set in __t.sets.iter_mut() {
                set.remove(location);
            }
        });
        ExclusiveGroupingSet::clean_up(__this);
    }

    /// java: public void removeAll(Collection<T> locations)
    pub fn remove_all(__this: &compat::Shared<Self>, locations: &Vec<T>) {
        compat::write(__this, |__t| {
            for set in __t.sets.iter_mut() {
                for __x in locations.iter() {
                    set.remove(__x);
                }
            }
        });
        ExclusiveGroupingSet::clean_up(__this);
    }

    /// java: public void cleanUp()
    pub fn clean_up(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| __t.sets.retain(|s| !((s.len() as i32) <= 1)));
    }

}
