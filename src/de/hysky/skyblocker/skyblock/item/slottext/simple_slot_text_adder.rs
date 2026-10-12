//! Ported from de.hysky.skyblocker.skyblock.item.slottext.SimpleSlotTextAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::ConfigInformation;
use crate::de::hysky::skyblocker::utils::container::slot_text_adder::SlotTextAdder;

/// java: public abstract class SimpleSlotTextAdder extends RegexContainerMatcher implements SlotTextAdder
#[derive(Clone)]
pub struct SimpleSlotTextAdderBase {
    /// java: extends RegexContainerMatcher
    pub base: RegexContainerMatcherBase,
    /// java: protected final @Nullable ConfigInformation configInformation;
    pub config_information: Option<ConfigInformation>,
}

impl std::ops::Deref for SimpleSlotTextAdderBase {
    type Target = RegexContainerMatcherBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SimpleSlotTextAdderBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SimpleSlotTextAdderBase {
    /// java: protected SimpleSlotTextAdder(@Language("RegExp") String titlePattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title_pattern: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R)) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R) {
        let (__this, __r) = Self::new4_in(title_pattern.clone(), None, __wrap);
        (__this, __r)
    }

    /// java: protected SimpleSlotTextAdder(Pattern titlePattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut title_pattern: compat::regex::Pattern, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R)) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R) {
        let (__this, __r) = Self::new5_in(title_pattern.clone(), None, __wrap);
        (__this, __r)
    }

    /// java: protected SimpleSlotTextAdder() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_0_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R)) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R) {
        let (__this, __r) = Self::new6_in(Some(None), __wrap);
        (__this, __r)
    }

    /// java: protected SimpleSlotTextAdder(@Language("RegExp") String titlePattern, @Nullable ConfigInformation configInformation) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new4_in<__R>(mut title_pattern: String, mut config_information: Option<ConfigInformation>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R)) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R) {
        let __init_config_information = config_information.clone();
        let (_, (__this, __r)) = RegexContainerMatcherBase::new2_in(title_pattern.clone(), move |__base| {
            let this = Self {
                base: __base,
                config_information: __init_config_information,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RegexContainerMatcher>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: protected SimpleSlotTextAdder(Pattern titlePattern, @Nullable ConfigInformation configInformation) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new5_in<__R>(mut title_pattern: compat::regex::Pattern, mut config_information: Option<ConfigInformation>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R)) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R) {
        let __init_config_information = config_information.clone();
        let (_, (__this, __r)) = RegexContainerMatcherBase::new3_in(title_pattern.clone(), move |__base| {
            let this = Self {
                base: __base,
                config_information: __init_config_information,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RegexContainerMatcher>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: protected SimpleSlotTextAdder(@Nullable ConfigInformation configInformation) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new6_in<__R>(mut config_information: Option<ConfigInformation>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R)) -> (compat::Shared<dyn SimpleSlotTextAdder>, __R) {
        let __init_config_information = config_information.clone();
        let (_, (__this, __r)) = RegexContainerMatcherBase::new_in(move |__base| {
            let this = Self {
                base: __base,
                config_information: __init_config_information,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RegexContainerMatcher>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of SimpleSlotTextAdder (implemented by SimpleSlotTextAdderBase and every subclass)
pub trait SimpleSlotTextAdder: RegexContainerMatcher + SlotTextAdder {
    fn simple_slot_text_adder_base(&self) -> &SimpleSlotTextAdderBase;
    fn simple_slot_text_adder_base_mut(&mut self) -> &mut SimpleSlotTextAdderBase;

}

impl dyn SimpleSlotTextAdder {
    /// java: @Override public @Nullable ConfigInformation getConfigInformation()
    pub fn get_config_information_body(__this: &compat::Shared<dyn SimpleSlotTextAdder>) -> Option<ConfigInformation> {
        return compat::read(__this, |__t| __t.simple_slot_text_adder_base().config_information.clone());
    }

    /// SimpleSlotTextAdder.getConfigInformation as the dispatch of SlotTextAdder.getConfigInformation for the concrete class `__C`
    pub fn get_config_information_from<__C: SimpleSlotTextAdder + 'static>(__this: &compat::Shared<dyn SlotTextAdder>) -> Option<ConfigInformation> {
        <dyn SimpleSlotTextAdder>::get_config_information_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SimpleSlotTextAdder>))
    }

}

// SimpleSlotTextAdder.getConfigInformation implements SlotTextAdder.getConfigInformation: it is emitted in each concrete subclass's `impl SlotTextAdder` (Rust has no inherited impls)
