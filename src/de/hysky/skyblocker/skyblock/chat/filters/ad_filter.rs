//! Ported from de.hysky.skyblocker.skyblock.chat.filters.AdFilter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::constants;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final Pattern[] AD_FILTERS = new Pattern[] { Pattern.compile("^(?:i(?:m|'m| am)? |(?:is )?any(?: ?one|1) )?(?:buy|sell|lowball|trade?)(?:ing)?(?:\\W|$)", Pattern.CASE_INSENSITIVE), Patt
pub static AD_FILTERS: LazyLock<Vec<compat::regex::Pattern>> = LazyLock::new(|| vec![compat::regex::Pattern::compile_with_flags("^(?:i(?:m|'m| am)? |(?:is )?any(?: ?one|1) )?(?:buy|sell|lowball|trade?)(?:ing)?(?:\\W|$)", mcrust::java::util::regex::pattern::CASE_INSENSITIVE), compat::regex::Pattern::compile("(.)\\1{7,}"), compat::regex::Pattern::compile_with_flags("\\W(?:on|in|check|at) my (?:ah|bin)(?:\\W|$)", mcrust::java::util::regex::pattern::CASE_INSENSITIVE)]);

/// java: public class AdFilter extends ChatPatternListener
#[derive(Clone)]
pub struct AdFilter {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for AdFilter {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AdFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AdFilter {
    /// java: public AdFilter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in(format!("{}: (.+)", constants::PLAYER_NAME.pattern().to_string()), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

}

impl ChatPatternListener for AdFilter {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public boolean onMatch(Component _message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        AdFilter::on_match_dyn
    }

    /// java: @Override protected ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        AdFilter::state_dyn
    }

}

impl ChatMessageListener for AdFilter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl AdFilter {
    /// java: @Override public boolean onMatch(Component _message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, _message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        let mut message: String = matcher.group(2).unwrap();
        for ad_filter in AD_FILTERS.iter() {
            if ad_filter.matcher(&message).find() {
                return true;
            }
        }
        return false;
    }

    /// AdFilter.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override protected ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> ChatFilterResult {
        return SkyblockerConfigManager::get().chat.hide_ads;
    }

    /// AdFilter.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Some(Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a)))
    }

}
