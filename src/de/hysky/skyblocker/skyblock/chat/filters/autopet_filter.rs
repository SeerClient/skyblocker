//! Ported from de.hysky.skyblocker.skyblock.chat.filters.AutopetFilter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::head_textures;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::de::hysky::skyblocker::utils::render::gui::basic_toast::BasicToast;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final FlexibleItemStack ICON = ItemUtils.createSkull(HeadTextures.AUTOPET_RULES_2);
pub static ICON: compat::MainThread<compat::Shared<FlexibleItemStack>> = compat::MainThread::new(|| ItemUtils::create_skull(head_textures::AUTOPET_RULES_2));

/// java: public class AutopetFilter extends ChatPatternListener
#[derive(Clone)]
pub struct AutopetFilter {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for AutopetFilter {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AutopetFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AutopetFilter {
    /// java: public AutopetFilter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in("^Autopet equipped your .*! VIEW RULE$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

}

impl ChatPatternListener for AutopetFilter {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        AutopetFilter::on_match_dyn
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        AutopetFilter::state_dyn
    }

}

impl ChatMessageListener for AutopetFilter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl AutopetFilter {
    /// java: @Override public boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        if SkyblockerConfigManager::get().chat.hide_autopet == ChatFilterResult::ActionBar {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&message.get_string().replace("VIEW RULE", "")); });
        } else if SkyblockerConfigManager::get().chat.hide_autopet == ChatFilterResult::Toast {
            compat::minecraft().gui.toast_manager().borrow_mut().add_toast(Box::new(BasicToast::new_3(component::literal(&message.get_string().replace("VIEW RULE", "")), (((SkyblockerConfigManager::get().chat.toast_display_duration * (1000 as f32))) as i64), FlexibleItemStack::get_stack(ICON.get_mut()))));
        }
        return true;
    }

    /// AutopetFilter.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> ChatFilterResult {
        if SkyblockerConfigManager::get().chat.hide_autopet == ChatFilterResult::ActionBar || SkyblockerConfigManager::get().chat.hide_autopet == ChatFilterResult::Toast {
            return ChatFilterResult::Filter;
        } else {
            return SkyblockerConfigManager::get().chat.hide_autopet;
        }
    }

    /// AutopetFilter.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Some(Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a)))
    }

}
