//! Ported from de.hysky.skyblocker.skyblock.barn.HungryHiker by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Map<String, String> foods;
pub static FOODS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| {
    let mut value;
    value = std::collections::HashMap::new();
    value.insert("from a cow.".to_string(), component::translatable("item.minecraft.cooked_beef").get_string());
    value.insert("meat from a fowl.".to_string(), component::translatable("item.minecraft.cooked_chicken").get_string());
    value.insert("red on the inside, green on the outside.".to_string(), component::translatable("item.minecraft.melon_slice").get_string());
    value.insert("a cooked potato.".to_string(), component::translatable("item.minecraft.baked_potato").get_string());
    value.insert("a stew.".to_string(), component::translatable("item.minecraft.rabbit_stew").get_string());
    value.insert("a grilled meat.".to_string(), component::translatable("item.minecraft.cooked_porkchop").get_string());
    value.insert("red and crunchy.".to_string(), component::translatable("item.minecraft.apple").get_string());
    value.insert("made of wheat.".to_string(), component::translatable("item.minecraft.bread").get_string());
    value
});

/// java: public class HungryHiker extends ChatPatternListener
#[derive(Clone)]
pub struct HungryHiker {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for HungryHiker {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for HungryHiker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HungryHiker {
    /// java: public HungryHiker()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in("^\\[NPC] Hungry Hiker: (The food I want is|(I asked for) food that is) ([a-zA-Z, '\\-]*\\.)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

}

impl ChatPatternListener for HungryHiker {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        HungryHiker::state_dyn
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        HungryHiker::on_match_dyn
    }

}

impl ChatMessageListener for HungryHiker {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl HungryHiker {
    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> ChatFilterResult {
        return (if SkyblockerConfigManager::get().other_locations.barn.solve_hungry_hiker { ChatFilterResult::Filter } else { ChatFilterResult::Pass });
    }

    /// HungryHiker.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Some(Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a)))
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        let mut food_description: String = matcher.group(3).unwrap();
        let Some(mut food) = FOODS.get(&food_description).cloned() else {
            return false;
        };
        let mut middle_part_of_the_message_to_send: String = (if matcher.group(2).is_some() { matcher.group(2).unwrap() } else { matcher.group(1).unwrap() });
        Utils::send_message_to_bypass_events(&component::empty().append(component::literal("[NPC] Hungry Hiker").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW)).append(component::literal(&format!(": {} {}.", middle_part_of_the_message_to_send, food))));
        return true;
    }

    /// HungryHiker.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
