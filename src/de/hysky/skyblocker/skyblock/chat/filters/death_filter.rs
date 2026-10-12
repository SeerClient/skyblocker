//! Ported from de.hysky.skyblocker.skyblock.chat.filters.DeathFilter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class DeathFilter extends ChatPatternListener
#[derive(Clone)]
pub struct DeathFilter {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for DeathFilter {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DeathFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DeathFilter {
    /// java: public DeathFilter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in(" \\u2620 .*".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

}

impl ChatPatternListener for DeathFilter {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override protected ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        DeathFilter::state_dyn
    }

    /// java: @Override protected boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        DeathFilter::on_match_dyn
    }

}

impl ChatMessageListener for DeathFilter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl DeathFilter {
    /// java: @Override protected ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> ChatFilterResult {
        return SkyblockerConfigManager::get().chat.hide_death;
    }

    /// DeathFilter.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Some(Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a)))
    }

    /// java: @Override protected boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        return true;
    }

    /// DeathFilter.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
