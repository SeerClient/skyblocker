//! Ported from de.hysky.skyblocker.utils.container.ContainerMatcher by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface ContainerMatcher extends Predicate<Screen>
pub trait ContainerMatcher: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @Override boolean test(Screen screen); (dispatch: the body for this class)
    fn test(&self) -> fn(&compat::Shared<dyn ContainerMatcher>, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool;

    /// java: boolean isEnabled(); (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool;

    /// java: default boolean skyblockOnly() (dispatch: the body for this class)
    fn skyblock_only(&self) -> fn(&compat::Shared<dyn ContainerMatcher>) -> bool {
        <dyn ContainerMatcher>::skyblock_only_body
    }

}

impl dyn ContainerMatcher {
    /// java: default boolean skyblockOnly()
    pub fn skyblock_only_body(__this: &compat::Shared<dyn ContainerMatcher>) -> bool {
        return true;
    }

}
