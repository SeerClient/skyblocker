//! Ported from de.hysky.skyblocker.skyblock.dwarven.fossil.FossilMuncher by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Map<String, String> ANSWERS = Map.of( "lived underground and dug tunnels", "Claw Fossil", "had a really fancy tail", "Clubbed Fossil", "was the king of his kind", "Footprint Fossi
pub static ANSWERS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("lived underground and dug tunnels".to_string(), "Claw Fossil".to_string()), ("had a really fancy tail".to_string(), "Clubbed Fossil".to_string()), ("was the king of his kind".to_string(), "Footprint Fossil".to_string()), ("lived underwater".to_string(), "Helix Fossil".to_string()), ("was kinda spiny u know".to_string(), "Spine Fossil".to_string()), ("lived in herds and was quite woolly".to_string(), "Tusk Fossil".to_string()), ("is pretty rough to look at".to_string(), "Ugly Fossil".to_string()), ("had a really pointy beak".to_string(), "Webbed Fossil".to_string())]));

/// java: public class FossilMuncher extends ChatPatternListener
#[derive(Clone)]
pub struct FossilMuncher {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for FossilMuncher {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FossilMuncher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FossilMuncher {
    /// java: public FossilMuncher()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in("^\\[NPC] Fossil Muncher: the fossil i want ([a-zA-Z, \\-]*)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

}

impl ChatPatternListener for FossilMuncher {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        FossilMuncher::state_dyn
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        FossilMuncher::on_match_dyn
    }

}

impl ChatMessageListener for FossilMuncher {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl FossilMuncher {
    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> Option<ChatFilterResult> {
        return Some((if SkyblockerConfigManager::get().mining.glacite.solve_fossil_muncher { ChatFilterResult::Filter } else { ChatFilterResult::Pass }));
    }

    /// FossilMuncher.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        logger().info(&format!("Original Fossil Muncher message: {}", message.get_string()));
        let mut riddle: String = matcher.group(1).unwrap();
        let mut answer: String = ANSWERS.get(&riddle).cloned().unwrap_or_else(|| riddle.clone());
        mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&component::empty().append(component::literal("[NPC] ").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW)).append(component::literal("Fossil Muncher").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::GOLD)).append_str(&format!(": {}", answer))); });
        return true;
    }

    /// FossilMuncher.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
