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
    /// (the class is final, so `getClass() != o.getClass()` is "not a PositionedWidget": every caller hands one)
    pub fn equals(&self, o: &compat::Shared<PositionedWidget>) -> bool {
        let mut that_widget: compat::Shared<dyn HudWidget> = compat::read(o, |__t| __t.widget.clone());
        return ({ let __f = compat::dispatch(&self.widget, |__t| __t.equals()); __f(&self.widget, &that_widget) });
    }

    /// java: `a.equals(b)` on two references (`List.equals`, a set's membership test)
    pub fn equals_shared(a: &compat::Shared<PositionedWidget>, b: &compat::Shared<PositionedWidget>) -> bool {
        let widget = compat::read(a, |__t| __t.widget.clone());
        let that_widget = compat::read(b, |__t| __t.widget.clone());
        let __f = compat::dispatch(&widget, |__t| __t.equals());
        __f(&widget, &that_widget)
    }

    /// java: `List<PositionedWidget>.equals(List)` - the same size and each pair `equals`
    pub fn list_equals(a: &[compat::Shared<PositionedWidget>], b: &[compat::Shared<PositionedWidget>]) -> bool {
        a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| PositionedWidget::equals_shared(x, y))
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
