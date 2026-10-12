//! Ported from de.hysky.skyblocker.utils.container.RegexContainerMatcher by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;

/// java: public abstract class RegexContainerMatcher implements ContainerMatcher
#[derive(Clone)]
pub struct RegexContainerMatcherBase {
    /// java: public final @Nullable Pattern titlePattern;
    pub title_pattern: Option<compat::regex::Pattern>,
    /// java: protected @Nullable String[] groups = null;
    pub groups: Option<Vec<String>>,
}

impl RegexContainerMatcherBase {
    /// java: protected RegexContainerMatcher() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn RegexContainerMatcher>, __R)) -> (compat::Shared<dyn RegexContainerMatcher>, __R) {
        let (__this, __r) = Self::new3_in(None, __wrap);
        (__this, __r)
    }

    /// java: protected RegexContainerMatcher(@Language("RegExp") String titlePattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut title_pattern: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn RegexContainerMatcher>, __R)) -> (compat::Shared<dyn RegexContainerMatcher>, __R) {
        let (__this, __r) = Self::new3_in(compat::regex::Pattern::compile(&title_pattern), __wrap);
        (__this, __r)
    }

    /// java: protected RegexContainerMatcher(@Nullable Pattern titlePattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new3_in<__R>(mut title_pattern: compat::regex::Pattern, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn RegexContainerMatcher>, __R)) -> (compat::Shared<dyn RegexContainerMatcher>, __R) {
        let __init_title_pattern = Some(title_pattern.clone());
        let this = Self {
            title_pattern: __init_title_pattern,
            groups: None,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of RegexContainerMatcher (implemented by RegexContainerMatcherBase and every subclass)
pub trait RegexContainerMatcher: ContainerMatcher {
    fn regex_container_matcher_base(&self) -> &RegexContainerMatcherBase;
    fn regex_container_matcher_base_mut(&mut self) -> &mut RegexContainerMatcherBase;

    /// java: public boolean test(String title) (dispatch: the body for this class)
    fn test_string(&self) -> fn(&compat::Shared<dyn RegexContainerMatcher>, &str) -> bool {
        <dyn RegexContainerMatcher>::test_string_body
    }

    /// java: public @Nullable Pattern getTitlePattern() (dispatch: the body for this class)
    fn get_title_pattern(&self) -> fn(&compat::Shared<dyn RegexContainerMatcher>) -> Option<compat::regex::Pattern> {
        <dyn RegexContainerMatcher>::get_title_pattern_body
    }

}

impl dyn RegexContainerMatcher {
    /// java: @Override public boolean test(Screen screen)
    pub fn test_body(__this: &compat::Shared<dyn RegexContainerMatcher>, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        return ({ let __f = compat::dispatch(__this, |__t| __t.test_string()); __f(__this, &screen.get_title().to_string()) });
    }

    /// RegexContainerMatcher.test as the dispatch of ContainerMatcher.test for the concrete class `__C`
    pub fn test_from<__C: RegexContainerMatcher + 'static>(__this: &compat::Shared<dyn ContainerMatcher>, __p0: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn RegexContainerMatcher>::test_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn RegexContainerMatcher>), __p0)
    }

    /// java: public boolean test(String title)
    pub fn test_string_body(__this: &compat::Shared<dyn RegexContainerMatcher>, title: &str) -> bool {
        if compat::read(__this, |__t| __t.regex_container_matcher_base().title_pattern.clone()).is_none() {
            return true;
        }
        let mut matcher: compat::regex::Matcher = __this.lock().borrow().regex_container_matcher_base().title_pattern.as_ref().unwrap().matcher(title);
        if matcher.matches() {
            let mut group_count: i32 = matcher.group_count();
            if group_count >= 1 {
                { let __v = Some((0..(group_count) as usize).map(|_| Default::default()).collect::<Vec<_>>()); compat::write(__this, |__t| __t.regex_container_matcher_base_mut().groups = __v); }
                for i in 0..group_count {
                    __this.lock().borrow_mut().regex_container_matcher_base_mut().groups.as_mut().unwrap()[(i) as usize] = matcher.group(i.wrapping_add(1)).unwrap();
                }
            }
            return true;
        }
        return false;
    }

    /// java: public @Nullable Pattern getTitlePattern()
    pub fn get_title_pattern_body(__this: &compat::Shared<dyn RegexContainerMatcher>) -> Option<compat::regex::Pattern> {
        return compat::read(__this, |__t| __t.regex_container_matcher_base().title_pattern.clone());
    }

}

// RegexContainerMatcher.test implements ContainerMatcher.test: it is emitted in each concrete subclass's `impl ContainerMatcher` (Rust has no inherited impls)
