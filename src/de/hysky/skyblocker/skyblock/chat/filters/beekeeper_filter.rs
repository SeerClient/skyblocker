//! Ported from de.hysky.skyblocker.skyblock.chat.filters.BeekeeperFilter by fabric2seer.

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

/// java: private static final String BEEKEEPER_BUFF_CHANGED = "^New day! Your Beekeeper buff changed!$";
pub const BEEKEEPER_BUFF_CHANGED: &str = "^New day! Your Beekeeper buff changed!$";
/// java: private static final String BEEKEEPER_BUFF = "^New buff: .*(Honeyhives?|Honeycomb|Critter|Trees lathered).*\\.$";
pub const BEEKEEPER_BUFF: &str = "^New buff: .*(Honeyhives?|Honeycomb|Critter|Trees lathered).*\\.$";
/// java: private static final String TOGGLE_BEEKEEPER = "^You can disable this messaging by toggling BeeKeeper in your /hotf!$";
pub const TOGGLE_BEEKEEPER: &str = "^You can disable this messaging by toggling BeeKeeper in your /hotf!$";

/// java: public class BeekeeperFilter extends SimpleChatFilter
#[derive(Clone)]
pub struct BeekeeperFilter {
    /// java: extends SimpleChatFilter
    pub base: SimpleChatFilterBase,
}

impl std::ops::Deref for BeekeeperFilter {
    type Target = SimpleChatFilterBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BeekeeperFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BeekeeperFilter {
    /// java: public BeekeeperFilter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = SimpleChatFilterBase::new_in(format!("{}|{}|{}", BEEKEEPER_BUFF_CHANGED, BEEKEEPER_BUFF, TOGGLE_BEEKEEPER), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn SimpleChatFilter>), __h)
        });
        __this
    }

}

impl SimpleChatFilter for BeekeeperFilter {
    fn simple_chat_filter_base(&self) -> &SimpleChatFilterBase {
        &self.base
    }
    fn simple_chat_filter_base_mut(&mut self) -> &mut SimpleChatFilterBase {
        &mut self.base
    }

}

impl ChatPatternListener for BeekeeperFilter {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base.base
    }

    /// java: @Override protected ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        BeekeeperFilter::state_dyn
    }

    /// java: @Override protected final boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        <dyn SimpleChatFilter>::on_match_from::<Self>
    }

}

impl ChatMessageListener for BeekeeperFilter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl BeekeeperFilter {
    /// java: @Override protected ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> Option<ChatFilterResult> {
        return Some((if !(Utils::is_in_galatea()) && !(Utils::is_in_torrhus_canyon()) { SkyblockerConfigManager::get().chat.hide_toggle_beekeeper } else { ChatFilterResult::Pass }));
    }

    /// BeekeeperFilter.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
