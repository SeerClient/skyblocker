//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.SimpleTooltipAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::tooltip_adder::TooltipAdder;

/// java: public abstract class SimpleTooltipAdder extends RegexContainerMatcher implements TooltipAdder
#[derive(Clone)]
pub struct SimpleTooltipAdderBase {
    /// java: extends RegexContainerMatcher
    pub base: RegexContainerMatcherBase,
    /// java: private final int priority;
    pub priority: i32,
}

impl std::ops::Deref for SimpleTooltipAdderBase {
    type Target = RegexContainerMatcherBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SimpleTooltipAdderBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SimpleTooltipAdderBase {
    /// java: protected SimpleTooltipAdder(@Language("RegExp") String titlePattern, int priority) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title_pattern: String, mut priority: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleTooltipAdder>, __R)) -> (compat::Shared<dyn SimpleTooltipAdder>, __R) {
        let __init_priority = priority;
        let (_, (__this, __r)) = RegexContainerMatcherBase::new2_in(title_pattern.clone(), move |__base| {
            let this = Self {
                base: __base,
                priority: __init_priority,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RegexContainerMatcher>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: protected SimpleTooltipAdder(Pattern titlePattern, int priority) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut title_pattern: compat::regex::Pattern, mut priority: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleTooltipAdder>, __R)) -> (compat::Shared<dyn SimpleTooltipAdder>, __R) {
        let __init_priority = priority;
        let (_, (__this, __r)) = RegexContainerMatcherBase::new3_in(title_pattern.clone(), move |__base| {
            let this = Self {
                base: __base,
                priority: __init_priority,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RegexContainerMatcher>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: protected SimpleTooltipAdder(int priority) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_1_in<__R>(mut priority: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleTooltipAdder>, __R)) -> (compat::Shared<dyn SimpleTooltipAdder>, __R) {
        let __init_priority = priority;
        let (_, (__this, __r)) = RegexContainerMatcherBase::new_in(move |__base| {
            let this = Self {
                base: __base,
                priority: __init_priority,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RegexContainerMatcher>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of SimpleTooltipAdder (implemented by SimpleTooltipAdderBase and every subclass)
pub trait SimpleTooltipAdder: RegexContainerMatcher + TooltipAdder {
    fn simple_tooltip_adder_base(&self) -> &SimpleTooltipAdderBase;
    fn simple_tooltip_adder_base_mut(&mut self) -> &mut SimpleTooltipAdderBase;

}

impl dyn SimpleTooltipAdder {
    /// java: @Override public int getPriority()
    pub fn get_priority_body(__this: &compat::Shared<dyn SimpleTooltipAdder>) -> i32 {
        return compat::read(__this, |__t| __t.simple_tooltip_adder_base().priority);
    }

    /// SimpleTooltipAdder.getPriority as the dispatch of TooltipAdder.getPriority for the concrete class `__C`
    pub fn get_priority_from<__C: SimpleTooltipAdder + 'static>(__this: &compat::Shared<dyn TooltipAdder>) -> i32 {
        <dyn SimpleTooltipAdder>::get_priority_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SimpleTooltipAdder>))
    }

}

// SimpleTooltipAdder.getPriority implements TooltipAdder.getPriority: it is emitted in each concrete subclass's `impl TooltipAdder` (Rust has no inherited impls)
