//! Ported from de.hysky.skyblocker.utils.container.SimpleContainerSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::container::container_solver::ContainerSolver;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcher;
use crate::de::hysky::skyblocker::utils::container::regex_container_matcher::RegexContainerMatcherBase;

/// java: public abstract class SimpleContainerSolver extends RegexContainerMatcher implements ContainerSolver
#[derive(Clone)]
pub struct SimpleContainerSolverBase {
    /// java: extends RegexContainerMatcher
    pub base: RegexContainerMatcherBase,
}

impl std::ops::Deref for SimpleContainerSolverBase {
    type Target = RegexContainerMatcherBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SimpleContainerSolverBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SimpleContainerSolverBase {
    /// java: protected SimpleContainerSolver(@Language("RegExp") String titlePattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title_pattern: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleContainerSolver>, __R)) -> (compat::Shared<dyn SimpleContainerSolver>, __R) {
        let (_, (__this, __r)) = RegexContainerMatcherBase::new2_in(title_pattern.clone(), move |__base| {
            let this = Self {
                base: __base,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RegexContainerMatcher>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: protected SimpleContainerSolver(Pattern titlePattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut title_pattern: compat::regex::Pattern, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleContainerSolver>, __R)) -> (compat::Shared<dyn SimpleContainerSolver>, __R) {
        let (_, (__this, __r)) = RegexContainerMatcherBase::new3_in(title_pattern.clone(), move |__base| {
            let this = Self {
                base: __base,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RegexContainerMatcher>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of SimpleContainerSolver (implemented by SimpleContainerSolverBase and every subclass)
pub trait SimpleContainerSolver: RegexContainerMatcher + ContainerSolver {
    fn simple_container_solver_base(&self) -> &SimpleContainerSolverBase;
    fn simple_container_solver_base_mut(&mut self) -> &mut SimpleContainerSolverBase;

}

impl dyn SimpleContainerSolver {
    /// java: @Override public Pattern getTitlePattern()
    pub fn get_title_pattern_body(__this: &compat::Shared<dyn SimpleContainerSolver>) -> Option<compat::regex::Pattern> {
        debug_assert!(<dyn RegexContainerMatcher>::get_title_pattern_body(&compat::view(__this, |a| a as compat::Shared<dyn RegexContainerMatcher>)).is_some());
        return <dyn RegexContainerMatcher>::get_title_pattern_body(&compat::view(__this, |a| a as compat::Shared<dyn RegexContainerMatcher>));
    }

    /// SimpleContainerSolver.getTitlePattern as the dispatch of RegexContainerMatcher.getTitlePattern for the concrete class `__C`
    pub fn get_title_pattern_from<__C: SimpleContainerSolver + 'static>(__this: &compat::Shared<dyn RegexContainerMatcher>) -> Option<compat::regex::Pattern> {
        <dyn SimpleContainerSolver>::get_title_pattern_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SimpleContainerSolver>))
    }

}

// SimpleContainerSolver.getTitlePattern implements RegexContainerMatcher.getTitlePattern: it is emitted in each concrete subclass's `impl RegexContainerMatcher` (Rust has no inherited impls)
