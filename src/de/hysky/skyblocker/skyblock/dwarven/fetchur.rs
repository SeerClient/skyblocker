//! Ported from de.hysky.skyblocker.skyblock.dwarven.Fetchur by fabric2seer.

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

/// java: private static final Map<String, String> answers;
pub static ANSWERS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| {
    let mut value;
    value = std::collections::HashMap::new();
    value.insert("yellow and see through".to_string(), component::translatable("block.minecraft.yellow_stained_glass").get_string());
    value.insert("circular and sometimes moves".to_string(), component::translatable("item.minecraft.compass").get_string());
    value.insert("expensive minerals".to_string(), "Mithril".to_string());
    value.insert("useful during celebrations".to_string(), component::translatable("item.minecraft.firework_rocket").get_string());
    value.insert("hot and gives energy".to_string(), "Cheap / Decent / Black Coffee".to_string());
    value.insert("tall and can be opened".to_string(), format!("{} / {}", component::literal("Any Wooden Door").get_string(), component::translatable("block.minecraft.iron_door").get_string()));
    value.insert("brown and fluffy".to_string(), component::translatable("item.minecraft.rabbit_foot").get_string());
    value.insert("explosive but more than usual".to_string(), "Superboom TNT".to_string());
    value.insert("wearable and grows".to_string(), component::translatable("block.minecraft.pumpkin").get_string());
    value.insert("shiny and makes sparks".to_string(), component::translatable("item.minecraft.flint_and_steel").get_string());
    value.insert("green and some dudes trade stuff for it".to_string(), component::translatable("item.minecraft.emerald").get_string());
    value.insert("red and soft".to_string(), component::translatable("block.minecraft.red_wool").get_string());
    value
});

/// java: public class Fetchur extends ChatPatternListener
#[derive(Clone)]
pub struct Fetchur {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for Fetchur {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Fetchur {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Fetchur {
    /// java: public Fetchur()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in("^\\[NPC] Fetchur: (?:its|theyre) ([a-zA-Z, \\-]*)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

}

impl ChatPatternListener for Fetchur {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Fetchur::state_dyn
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        Fetchur::on_match_dyn
    }

}

impl ChatMessageListener for Fetchur {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl Fetchur {
    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> ChatFilterResult {
        return (if SkyblockerConfigManager::get().mining.dwarven_mines.solve_fetchur { ChatFilterResult::Filter } else { ChatFilterResult::Pass });
    }

    /// Fetchur.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Some(Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a)))
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        logger().info(&format!("Original Fetchur message: {}", message.get_string()));
        let mut riddle: String = matcher.group(1).unwrap();
        let mut answer: String = ANSWERS.get(&riddle).cloned().unwrap_or_else(|| riddle.clone());
        mcrust::mcrust::modding::game::with_player(|player| { compat::batch_w_send_feedback(&component::empty().append(component::literal("[NPC] Fetchur").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW)).append_str(&format!(": {}", answer))); });
        return true;
    }

    /// Fetchur.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
