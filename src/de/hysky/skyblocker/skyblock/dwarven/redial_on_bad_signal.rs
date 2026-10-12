//! Ported from de.hysky.skyblocker.skyblock.dwarven.RedialOnBadSignal by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::constants;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class RedialOnBadSignal extends ChatPatternListener
#[derive(Clone)]
pub struct RedialOnBadSignal {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for RedialOnBadSignal {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RedialOnBadSignal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RedialOnBadSignal {
    /// java: public RedialOnBadSignal()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in("^\\[NPC] (Fred): ✆ .*$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

}

impl ChatPatternListener for RedialOnBadSignal {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override protected ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        RedialOnBadSignal::state_dyn
    }

    /// java: @Override protected boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        RedialOnBadSignal::on_match_dyn
    }

}

impl ChatMessageListener for RedialOnBadSignal {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl RedialOnBadSignal {
    /// java: @Override protected ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> Option<ChatFilterResult> {
        return Some((if SkyblockerConfigManager::get().mining.redial_on_bad_signal { ChatFilterResult::Filter } else { ChatFilterResult::Pass }));
    }

    /// RedialOnBadSignal.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        if !(message.get_string().contains("§k")) {
            return false;
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        let mut name: String = matcher.group(1).unwrap();
        let mut call_message: MutableComponent = (constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.config.mining.redialOnBadSignal.message", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(name.clone())]));
        call_message.with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command(format!("/call {}", name))))).with_color_format(Some(ChatFormatting::Aqua)));
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&call_message.to_string()); });
        return false;
    }

    /// RedialOnBadSignal.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
