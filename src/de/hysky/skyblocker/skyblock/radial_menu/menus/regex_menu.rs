//! Ported from de.hysky.skyblocker.skyblock.radialMenu.menus.RegexMenu by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenuBase;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu;

/// java: public abstract class RegexMenu extends BasicMenu
#[derive(Clone)]
pub struct RegexMenuBase {
    /// java: extends BasicMenu
    pub base: BasicMenuBase,
    /// java: private final Pattern pattern;
    pub pattern: compat::regex::Pattern,
}

impl std::ops::Deref for RegexMenuBase {
    type Target = BasicMenuBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RegexMenuBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RegexMenuBase {
    /// java: public RegexMenu(@Language("RegExp") String regex, String id) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut regex: String, mut id: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn RegexMenu>, __R)) -> (compat::Shared<dyn RegexMenu>, __R) {
        let __init_pattern = compat::regex::Pattern::compile_with_flags(&regex, mcrust::java::util::regex::pattern::CASE_INSENSITIVE);
        let (_, (__this, __r)) = BasicMenuBase::new_in(regex.clone(), id.clone(), move |__base| {
            let this = Self {
                base: __base,
                pattern: __init_pattern,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn BasicMenu>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of RegexMenu (implemented by RegexMenuBase and every subclass)
pub trait RegexMenu: BasicMenu {
    fn regex_menu_base(&self) -> &RegexMenuBase;
    fn regex_menu_base_mut(&mut self) -> &mut RegexMenuBase;

}

impl dyn RegexMenu {
    /// java: @Override public boolean titleMatches(String title)
    pub fn title_matches_body(__this: &compat::Shared<dyn RegexMenu>, title: &str) -> bool {
        return ({ let __f = compat::dispatch(__this, |__t| __t.get_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn RadialMenu>)) } && __this.lock().borrow_mut().regex_menu_base_mut().pattern.matcher(title).matches());
    }

    /// RegexMenu.titleMatches as the dispatch of RadialMenu.titleMatches for the concrete class `__C`
    pub fn title_matches_from<__C: RegexMenu + 'static>(__this: &compat::Shared<dyn RadialMenu>, __p0: &str) -> bool {
        <dyn RegexMenu>::title_matches_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn RegexMenu>), __p0)
    }

}

// RegexMenu.titleMatches implements RadialMenu.titleMatches: it is emitted in each concrete subclass's `impl RadialMenu` (Rust has no inherited impls)
