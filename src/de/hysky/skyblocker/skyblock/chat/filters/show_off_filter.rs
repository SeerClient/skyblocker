//! Ported from de.hysky.skyblocker.skyblock.chat.filters.ShowOffFilter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::chat::filters::simple_chat_filter::SimpleChatFilter;
use crate::de::hysky::skyblocker::skyblock::chat::filters::simple_chat_filter::SimpleChatFilterBase;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::constants;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final String[] SHOW_TYPES = { "is holding", "is wearing", "is friends with a", "has" };
pub static SHOW_TYPES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["is holding".to_string(), "is wearing".to_string(), "is friends with a".to_string(), "has".to_string()]);

/// java: public class ShowOffFilter extends SimpleChatFilter
#[derive(Clone)]
pub struct ShowOffFilter {
    /// java: extends SimpleChatFilter
    pub base: SimpleChatFilterBase,
}

impl std::ops::Deref for ShowOffFilter {
    type Target = SimpleChatFilterBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ShowOffFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ShowOffFilter {
    /// java: public ShowOffFilter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleChatFilterBase::new_in(format!("{} (?:{}) \\[(.+)\\]", constants::PLAYER_NAME.pattern().to_string(), SHOW_TYPES.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("|")), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleChatFilter>), __h)
        });
        __this
    }

}

impl SimpleChatFilter for ShowOffFilter {
    fn simple_chat_filter_base(&self) -> &SimpleChatFilterBase {
        &self.base
    }
    fn simple_chat_filter_base_mut(&mut self) -> &mut SimpleChatFilterBase {
        &mut self.base
    }

}

impl ChatPatternListener for ShowOffFilter {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base.base
    }

    /// java: @Override protected ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        ShowOffFilter::state_dyn
    }

    /// java: @Override protected final boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        <dyn SimpleChatFilter>::on_match_from::<Self>
    }

}

impl ChatMessageListener for ShowOffFilter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl ShowOffFilter {
    /// java: @Override protected ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> Option<ChatFilterResult> {
        return Some(SkyblockerConfigManager::get().chat.hide_show_off);
    }

    /// ShowOffFilter.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
