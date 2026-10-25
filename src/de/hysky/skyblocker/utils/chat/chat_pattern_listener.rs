//! Ported from de.hysky.skyblocker.utils.chat.ChatPatternListener by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::chat::chat_filter_result::ChatFilterResult;
use crate::de::hysky::skyblocker::utils::chat::chat_message_listener::ChatMessageListener;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: protected static final String NUMBER = "-?[0-9]{1,3}(?>,[0-9]{3})*(?:\\.[1-9])?";
pub const NUMBER: &str = "-?[0-9]{1,3}(?>,[0-9]{3})*(?:\\.[1-9])?";

/// java: public abstract class ChatPatternListener implements ChatMessageListener
#[derive(Clone)]
pub struct ChatPatternListenerBase {
    /// java: public final Pattern pattern;
    pub pattern: compat::regex::Pattern,
}

impl ChatPatternListenerBase {
    /// java: protected ChatPatternListener(@Language("RegExp") String pattern) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut pattern: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ChatPatternListener>, __R)) -> (compat::Shared<dyn ChatPatternListener>, __R) {
        let __init_pattern = compat::regex::Pattern::compile(&pattern);
        let this = Self {
            pattern: __init_pattern,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of ChatPatternListener (implemented by ChatPatternListenerBase and every subclass)
pub trait ChatPatternListener: ChatMessageListener {
    fn chat_pattern_listener_base(&self) -> &ChatPatternListenerBase;
    fn chat_pattern_listener_base_mut(&mut self) -> &mut ChatPatternListenerBase;

    /// java: protected abstract ChatFilterResult state(); (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn ChatPatternListener>) -> Option<ChatFilterResult>;

    /// java: protected abstract boolean onMatch(Component message, Matcher matcher); (dispatch: the body for this class)
    fn on_match(&self) -> fn(&compat::Shared<dyn ChatPatternListener>, &MutableComponent, &compat::regex::Matcher) -> bool;

}

impl dyn ChatPatternListener {
    /// java: @Override public final ChatFilterResult onMessage(Component message, String asString)
    pub fn on_message_body(__this: &compat::Shared<dyn ChatPatternListener>, message: &MutableComponent, as_string: &str) -> ChatFilterResult {
        let mut state: ChatFilterResult = ({ let __f = compat::dispatch(__this, |__t| __t.state()); __f(__this) }).unwrap_or(ChatFilterResult::Pass);
        if state == ChatFilterResult::Pass {
            return ChatFilterResult::Pass;
        }
        let mut m: compat::regex::Matcher = __this.lock().borrow().chat_pattern_listener_base().pattern.matcher(as_string);
        if m.matches() && { let __f = compat::dispatch(__this, |__t| __t.on_match()); __f(__this, message, &m) } {
            return state;
        }
        return ChatFilterResult::Pass;
    }

    /// ChatPatternListener.onMessage as the dispatch of ChatMessageListener.onMessage for the concrete class `__C`
    pub fn on_message_from<__C: ChatPatternListener + 'static>(__this: &compat::Shared<dyn ChatMessageListener>, __p0: &MutableComponent, __p1: &str) -> ChatFilterResult {
        <dyn ChatPatternListener>::on_message_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ChatPatternListener>), __p0, __p1)
    }

}

// ChatPatternListener.onMessage implements ChatMessageListener.onMessage: it is emitted in each concrete subclass's `impl ChatMessageListener` (Rust has no inherited impls)
