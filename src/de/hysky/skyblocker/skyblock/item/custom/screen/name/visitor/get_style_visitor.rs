//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.name.visitor.GetStyleVisitor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::base_visitor::BaseVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::base_visitor::BaseVisitorBase;
use mcrust::net::minecraft::network::chat::style::Style;

/// java: public class GetStyleVisitor extends BaseVisitor
#[derive(Clone)]
pub struct GetStyleVisitor {
    /// java: extends BaseVisitor
    pub base: BaseVisitorBase,
    /// java: private Style style = Style.EMPTY;
    pub style: Style,
    /// java: private final boolean noSize = selSize == 0;
    pub no_size: bool,
    /// java: private boolean done = false;
    pub done: bool,
}

impl std::ops::Deref for GetStyleVisitor {
    type Target = BaseVisitorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GetStyleVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GetStyleVisitor {
    /// java: public GetStyleVisitor(int selectionStart, int selectionEnd)
    pub fn new(mut selection_start: i32, mut selection_end: i32) -> compat::Shared<Self> {
        // TODO(fabric2seer): the initialiser of `noSize`: instance field `selSize` read in a static context
        //   java: private final boolean noSize = selSize == 0;
        let (_, __this) = BaseVisitorBase::new_in(selection_start, selection_end, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                style: mcrust::net::minecraft::network::chat::style::Style::EMPTY,
                no_size: Default::default(),
                done: false,
            });
            ((__h.clone() as compat::Shared<dyn BaseVisitor>), __h)
        });
        __this
    }

    /// java: public Style getStyle()
    pub fn get_style(__this: &compat::Shared<Self>) -> Style {
        return compat::read(__this, |__t| __t.style.clone());
    }

}

impl BaseVisitor for GetStyleVisitor {
    fn base_visitor_base(&self) -> &BaseVisitorBase {
        &self.base
    }
    fn base_visitor_base_mut(&mut self) -> &mut BaseVisitorBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void visit(Style st, String asString) (dispatch: the body for this class)
    fn visit(&self) -> fn(&compat::Shared<dyn BaseVisitor>, Style, String) {
        GetStyleVisitor::visit_dyn
    }

}

impl GetStyleVisitor {
    /// java: @Override protected void visit(Style st, String asString)
    pub fn visit_impl(__this: &compat::Shared<Self>, mut st: Style, mut as_string: String) {
        if compat::read(__this, |__t| __t.done) {
            return;
        }
        if compat::str_len(&as_string) < compat::read(__this, |__t| __t.sel_start) {
            { let __v = (compat::read(__this, |__t| __t.sel_start)).wrapping_sub(compat::str_len(&as_string)); compat::write(__this, |__t| __t.sel_start = __v); }
            return;
        }
        if compat::read(__this, |__t| __t.sel_start) != 0 {
            as_string = compat::substring(&as_string, compat::read(__this, |__t| __t.sel_start));
        }
        { let __v = 0; compat::write(__this, |__t| __t.sel_start = __v); }
        if compat::read(__this, |__t| __t.no_size) {
            { let __v = st.clone(); compat::write(__this, |__t| __t.style = __v); }
            { let __v = true; compat::write(__this, |__t| __t.done = __v); }
            return;
        }
        { let __v = (compat::read(__this, |__t| __t.sel_size)).wrapping_sub(compat::str_len(&as_string)); compat::write(__this, |__t| __t.sel_size = __v); }
        { let __v = (compat::read(__this, |__t| __t.sel_size) <= 0); compat::write(__this, |__t| __t.done = __v); }
        if as_string.is_empty() {
            return;
        }
        { let __v = { let (__q0,) = (Some((__this.lock().borrow().style.is_obfuscated() || st.is_obfuscated())),); ({ let (__q0,) = (Some((__this.lock().borrow().style.is_strikethrough() || st.is_strikethrough())),); ({ let (__q0,) = (Some((__this.lock().borrow().style.is_underlined() || st.is_underlined())),); ({ let (__q0,) = (Some((__this.lock().borrow().style.is_italic() || st.is_italic())),); ({ let (__q0,) = (Some((__this.lock().borrow().style.is_bold() || st.is_bold())),); __this.lock().borrow().style.with_bold(__q0) }).with_italic(__q0) }).with_underlined(__q0) }).with_strikethrough(__q0) }).with_obfuscated(__q0) }; compat::write(__this, |__t| __t.style = __v); }
    }

    /// GetStyleVisitor.visit as the dispatch of BaseVisitor.visit
    pub fn visit_dyn(__this: &compat::Shared<dyn BaseVisitor>, __p0: Style, __p1: String) {
        Self::visit_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
