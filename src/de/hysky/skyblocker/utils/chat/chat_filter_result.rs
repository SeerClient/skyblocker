//! Ported from de.hysky.skyblocker.utils.chat.ChatFilterResult by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public enum ChatFilterResult
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChatFilterResult {
    Pass,
    Filter,
    ActionBar,
    Toast,
}

impl ChatFilterResult {
    pub const VALUES: [ChatFilterResult; 4] = [ChatFilterResult::Pass, ChatFilterResult::Filter, ChatFilterResult::ActionBar, ChatFilterResult::Toast];

    /// java: values()
    pub fn values() -> Vec<ChatFilterResult> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ChatFilterResult::Pass => "PASS",
            ChatFilterResult::Filter => "FILTER",
            ChatFilterResult::ActionBar => "ACTION_BAR",
            ChatFilterResult::Toast => "TOAST",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ChatFilterResult {
        match name {
            "PASS" => ChatFilterResult::Pass,
            "FILTER" => ChatFilterResult::Filter,
            "ACTION_BAR" => ChatFilterResult::ActionBar,
            "TOAST" => ChatFilterResult::Toast,
            _ => panic!("IllegalArgumentException: No enum constant ChatFilterResult.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.chat.filter.chatFilterResult." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for ChatFilterResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ChatFilterResult::to_string(*self))
    }
}
