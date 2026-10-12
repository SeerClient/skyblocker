//! Ported from de.hysky.skyblocker.skyblock.barn.TreasureHunter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final Map<String, String> locations;
pub static LOCATIONS: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| {
    let mut value;
    value = std::collections::HashMap::new();
    value.insert("There's a treasure chest somewhere in a small cave in the gorge.".to_string(), "258 70 -492".to_string());
    value.insert("I was in the desert earlier, and I saw something near a red sand rock.".to_string(), "357 82 -319".to_string());
    value.insert("There's this guy who collects animals to experiment on, I think I saw something near his house.".to_string(), "259 184 -564".to_string());
    value.insert("There's a small house in the gorge, I saw some treasure near there.".to_string(), "297 87 -562".to_string());
    value.insert("There's this guy who says he has the best sheep in the world. I think I saw something around his hut.".to_string(), "392 85 -372".to_string());
    value.insert("I spotted something by an odd looking mushroom on one of the ledges in the Mushroom Gorge, you should check it out.".to_string(), "305 73 -557".to_string());
    value.insert("There are some small ruins out in the desert, might want to check them out.".to_string(), "320 102 -471".to_string());
    value.insert("Some dirt was kicked up by the water pool in the overgrown Mushroom Cave. Have a look over there.".to_string(), "234 56 -410".to_string());
    value.insert("There are some old stone structures in the Mushroom Gorge, give them a look.".to_string(), "223 54 -503".to_string());
    value.insert("In the Mushroom Gorge where blue meets the ceiling and floor, you will find what you are looking for.".to_string(), "205 42 -527".to_string());
    value.insert("There was a haystack with a crop greener than usual around it, I think there is something near there.".to_string(), "334 82 -389".to_string());
    value.insert("There's a single piece of tall grass growing in the desert, I saw something there.".to_string(), "283 76 -363".to_string());
    value.insert("I saw some treasure by a cow skull near the village.".to_string(), "141 77 -397".to_string());
    value.insert("Near a melon patch inside a tunnel in the mountain I spotted something.".to_string(), "257 100 -569".to_string());
    value.insert("I saw something near a farmer's cart, you should check it out.".to_string(), "155 90 -591".to_string());
    value.insert("I remember there was a stone pillar made only of cobblestone in the oasis, could be something there.".to_string(), "122 66 -409".to_string());
    value.insert("I thought I saw something near the smallest stone pillar in the oasis.".to_string(), "94 65 -455".to_string());
    value.insert("I found something by a mossy stone pillar in the oasis, you should take a look.".to_string(), "179 93 -537".to_string());
    value.insert("Down in the glowing Mushroom Cave, there was a weird looking mushroom, check it out.".to_string(), "182 44 -451".to_string());
    value.insert("Something caught my eye by the red sand near the bridge over the gorge.".to_string(), "306 105 -489".to_string());
    value.insert("I seem to recall seeing something near the well in the village.".to_string(), "170 77 -375".to_string());
    value.insert("I was down near the lower oasis yesterday, I think I saw something under the bridge.".to_string(), "142 69 -448".to_string());
    value.insert("I was at the upper oasis today, I recall seeing something on the cobblestone stepping stones.".to_string(), "188 77 -459".to_string());
    value
});

/// java: public class TreasureHunter extends ChatPatternListener
#[derive(Clone)]
pub struct TreasureHunter {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for TreasureHunter {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TreasureHunter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TreasureHunter {
    /// java: public TreasureHunter()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in("^\\[NPC] Treasure Hunter: ([a-zA-Z, '\\-\\.]*)$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

    /// java: private static void requestWaypoint(String location)
    pub fn request_waypoint(location: &str) {
        let mut command: String = format!("/skyblocker waypoints individual {} Treasure", location);
        let mut request_message: MutableComponent = (constants::PREFIX.get_mut())().append((component::translatable_args("skyblocker.waypoints.chat.display", (compat::regex::split(&location, " ")).iter().map(|__a0| mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(__a0.parse::<i32>().expect("NumberFormatException").to_string())).collect::<Vec<_>>())).with_style_format(ChatFormatting::Aqua).with_style_fn(|style| style.with_click_event(Some(ClickEvent::new_run_command(compat::trim(&command))))));
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&request_message.to_string()); });
    }

}

impl ChatPatternListener for TreasureHunter {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        TreasureHunter::state_dyn
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        TreasureHunter::on_match_dyn
    }

}

impl ChatMessageListener for TreasureHunter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl TreasureHunter {
    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> ChatFilterResult {
        return (if SkyblockerConfigManager::get().other_locations.barn.solve_treasure_hunter { ChatFilterResult::Filter } else { ChatFilterResult::Pass });
    }

    /// TreasureHunter.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Some(Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a)))
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        let client = compat::minecraft();
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return false;
        }
        let mut hint: String = matcher.group(1).unwrap();
        let Some(mut location) = LOCATIONS.get(&hint).cloned() else {
            return false;
        };
        Utils::send_message_to_bypass_events(&component::empty().append(component::literal("[NPC] Treasure Hunter").with_color_text(mcrust::net::minecraft::network::chat::text_color::TextColor::YELLOW)).append(component::literal(&format!(": Go mine around {}", location))));
        TreasureHunter::request_waypoint(&location);
        return true;
    }

    /// TreasureHunter.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
