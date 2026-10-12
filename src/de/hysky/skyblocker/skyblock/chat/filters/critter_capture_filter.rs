//! Ported from de.hysky.skyblocker.skyblock.chat.filters.CritterCaptureFilter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::chat::filters::simple_chat_filter::SimpleChatFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::simple_chat_filter::SimpleChatFilterBase;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class CritterCaptureFilter extends SimpleChatFilter
#[derive(Clone)]
pub struct CritterCaptureFilter {
    /// java: extends SimpleChatFilter
    pub base: SimpleChatFilterBase,
}

impl std::ops::Deref for CritterCaptureFilter {
    type Target = SimpleChatFilterBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CritterCaptureFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CritterCaptureFilter {
    /// java: public CritterCaptureFilter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleChatFilterBase::new_in("^(CAPTURE|LOOT SHARE)! .*$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleChatFilter>), __h)
        });
        __this
    }

}

impl SimpleChatFilter for CritterCaptureFilter {
    fn simple_chat_filter_base(&self) -> &SimpleChatFilterBase {
        &self.base
    }
    fn simple_chat_filter_base_mut(&mut self) -> &mut SimpleChatFilterBase {
        &mut self.base
    }

}

impl ChatPatternListener for CritterCaptureFilter {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base.base
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        CritterCaptureFilter::state_dyn
    }

    /// java: @Override protected final boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        <dyn SimpleChatFilter>::on_match_from::<Self>
    }

}

impl ChatMessageListener for CritterCaptureFilter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl CritterCaptureFilter {
    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> Option<ChatFilterResult> {
        return Some((if Utils::is_in_safari() { SkyblockerConfigManager::get().chat.hide_critter_capture } else { ChatFilterResult::Pass }));
    }

    /// CritterCaptureFilter.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
