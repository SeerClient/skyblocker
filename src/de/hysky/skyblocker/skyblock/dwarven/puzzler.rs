//! Ported from de.hysky.skyblocker.skyblock.dwarven.Puzzler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListener;
use crate::de::hysky::skyblocker::utils::chat::chat_pattern_listener::ChatPatternListenerBase;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::level::block::blocks;

/// java: public class Puzzler extends ChatPatternListener
#[derive(Clone)]
pub struct Puzzler {
    /// java: extends ChatPatternListener
    pub base: ChatPatternListenerBase,
}

impl std::ops::Deref for Puzzler {
    type Target = ChatPatternListenerBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Puzzler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Puzzler {
    /// java: public Puzzler()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ChatPatternListenerBase::new_in("^\\[NPC] Puzzler: ((?:▲|▶|◀|▼){10})$".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ChatPatternListener>), __h)
        });
        __this
    }

}

impl ChatPatternListener for Puzzler {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase {
        &self.base
    }
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase {
        &mut self.base
    }

    /// java: @Override public ChatFilterResult state() (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Puzzler::state_dyn
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher) (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool {
        Puzzler::on_match_dyn
    }

}

impl ChatMessageListener for Puzzler {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString) (dispatch: the body for this class)
    fn on_message(&self) -> fn(&compat::Shared<dyn ChatMessageListener>, &MutableComponent, &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_from::<Self>
    }

}

impl Puzzler {
    /// java: @Override public ChatFilterResult state()
    pub fn state_impl(__this: &compat::Shared<Self>) -> Option<ChatFilterResult> {
        return (if SkyblockerConfigManager::get().mining.dwarven_mines.solve_puzzler { None } else { Some(ChatFilterResult::Pass) });
    }

    /// Puzzler.state as the dispatch of ChatPatternListener.state
    pub fn state_dyn(__this: &compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult> {
        Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean onMatch(Component message, Matcher matcher)
    pub fn on_match_impl(__this: &compat::Shared<Self>, message: &MutableComponent, matcher: &compat::regex::Matcher) -> bool {
        let mut x: i32 = 181;
        let mut z: i32 = 135;
        for c in matcher.group(1).unwrap().chars().collect::<Vec<char>>() {
            if c == '▲' {
                z += 1;
            } else if c == '▼' {
                z -= 1;
            } else if c == '◀' {
                x += 1;
            } else if c == '▶' {
                x -= 1;
            }
        }
        let mut world: Option<ClientLevel> = mcrust::mcrust::modding::game::level();
        if let Some(world) = &world {
            world.set_block_and_update(&BlockPos::new(x, 195, z), &blocks::blocks().get("crimson_planks").default_block_state());
        }
        return false;
    }

    /// Puzzler.onMatch as the dispatch of ChatPatternListener.onMatch
    pub fn on_match_dyn(__this: &compat::Shared<dyn ChatPatternListener>, __p0: &MutableComponent, __p1: &compat::regex::Matcher) -> bool {
        Self::on_match_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
