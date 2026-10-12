//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.PositionedWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::pipeline::position_rule::PositionRule;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;

/// java: public final class PositionedWidget
#[derive(Clone)]
pub struct PositionedWidget {
    /// java: public final HudWidget widget;
    pub widget: compat::Shared<dyn HudWidget>,
    /// java: public PositionRule rule;
    pub rule: PositionRule,
    /// java: public boolean fromTab = false;
    pub from_tab: bool,
    /// java: boolean positioned = false;
    pub positioned: bool,
    /// java: boolean visible = false;
    pub visible: bool,
}

impl PositionedWidget {
    /// java: public PositionedWidget(HudWidget widget, PositionRule rule)
    pub fn new(mut widget: compat::Shared<dyn HudWidget>, mut rule: PositionRule) -> compat::Shared<Self> {
        let __init_widget = widget.clone();
        let __init_rule = rule.clone();
        let __this = compat::shared(Self {
            widget: __init_widget,
            rule: __init_rule,
            from_tab: false,
            positioned: false,
            visible: false,
        });
        __this
    }

    /// java: @Override public boolean equals(@Nullable Object o)
    pub fn equals(&self, mut o: Option<compat::Object>) -> bool {
        // TODO(fabric2seer): the condition: call to `getClass(..)` (not a method of this class)
        //   java: o == null || getClass() != o.getClass()
        if todo!("fabric2seer: the condition (TODO above)") {
            return false;
        }
        let mut that: compat::Shared<PositionedWidget> = o.clone().unwrap();
        return ({ let __f = compat::dispatch(&self.widget, |__t| __t.equals()); __f(&self.widget, &compat::read(&that, |__t| __t.widget.clone())) });
    }

    /// java: @Override public int hashCode()
    pub fn hash_code(&self) -> i32 {
        return ({ let __f = compat::dispatch(&self.widget, |__t| __t.hash_code()); __f(&self.widget) });
    }

    /// java: @GenToString @Override public native String toString();
    pub fn to_string(&self) -> String {
        todo!("native method: its body is generated at build time by an annotation processor (@GenToString)")
    }

}

impl std::fmt::Display for PositionedWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PositionedWidget::to_string(self))
    }
}
