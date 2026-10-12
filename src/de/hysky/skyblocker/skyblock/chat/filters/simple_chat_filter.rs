//! Ported from de.hysky.skyblocker.skyblock.chat.filters.SimpleChatFilter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public abstract class SimpleChatFilter extends ChatPatternListener
#[derive(Clone)]
pub struct SimpleChatFilterBase {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for SimpleChatFilterBase {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SimpleChatFilterBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SimpleChatFilterBase {
    /// java: protected SimpleChatFilter(@Language("RegExp") String pattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut pattern: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleChatFilter>, __R)) -> (compat::Shared<dyn SimpleChatFilter>, __R) {
        let (_, (__this, __r)) = ChatPatternListenerBase::new_in(pattern.clone(), move |__base| {
            let this = Self {
                base: __base,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of SimpleChatFilter (implemented by SimpleChatFilterBase and every subclass)
pub trait SimpleChatFilter: ChatPatternListener {
    fn simple_chat_filter_base(&self) -> &SimpleChatFilterBase;
    fn simple_chat_filter_base_mut(&mut self) -> &mut SimpleChatFilterBase;

}

impl dyn SimpleChatFilter {
    /// java: @Override protected final boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_body(__this: &compat::Shared<dyn SimpleChatFilter>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        return true;
    }

    /// SimpleChatFilter.onMatch as the dispatch of ChatPatternListener.onMatch for the concrete class `__C`
    pub fn on_match_from<__C: SimpleChatFilter + 'static>(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        <dyn SimpleChatFilter>::on_match_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SimpleChatFilter>), __p0, __p1)
    }

}

// SimpleChatFilter.onMatch implements ChatPatternListener.onMatch: it is emitted in each concrete subclass's `impl ChatPatternListener` (Rust has no inherited impls)
