//! Ported from de.hysky.skyblocker.skyblock.item.slottext.SlotTextMode by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public enum SlotTextMode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SlotTextMode {
    Enabled,
    HoldToShow,
    PressToToggle,
    HoldToHide,
    Disabled,
}

impl SlotTextMode {
    pub const VALUES: [SlotTextMode; 5] = [SlotTextMode::Enabled, SlotTextMode::HoldToShow, SlotTextMode::PressToToggle, SlotTextMode::HoldToHide, SlotTextMode::Disabled];

    /// java: values()
    pub fn values() -> Vec<SlotTextMode> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            SlotTextMode::Enabled => "ENABLED",
            SlotTextMode::HoldToShow => "HOLD_TO_SHOW",
            SlotTextMode::PressToToggle => "PRESS_TO_TOGGLE",
            SlotTextMode::HoldToHide => "HOLD_TO_HIDE",
            SlotTextMode::Disabled => "DISABLED",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> SlotTextMode {
        match name {
            "ENABLED" => SlotTextMode::Enabled,
            "HOLD_TO_SHOW" => SlotTextMode::HoldToShow,
            "PRESS_TO_TOGGLE" => SlotTextMode::PressToToggle,
            "HOLD_TO_HIDE" => SlotTextMode::HoldToHide,
            "DISABLED" => SlotTextMode::Disabled,
            _ => panic!("IllegalArgumentException: No enum constant SlotTextMode.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.uiAndVisuals.slotText.mode." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for SlotTextMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", SlotTextMode::to_string(*self))
    }
}
