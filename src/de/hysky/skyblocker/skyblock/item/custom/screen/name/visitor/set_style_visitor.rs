//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.name.visitor.SetStyleVisitor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::base_visitor::BaseVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::base_visitor::BaseVisitorBase;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::contents::plain_text_contents::PlainTextContents;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;

/// java: public class SetStyleVisitor extends BaseVisitor
#[derive(Clone)]
pub struct SetStyleVisitor {
    /// java: extends BaseVisitor
    pub base: BaseVisitorBase,
    /// java: private final MutableComponent newText = MutableComponent.create(new PlainTextContents.LiteralContents(""));
    pub new_text: MutableComponent,
    /// java: private final Style newStyle;
    pub new_style: Style,
}

impl std::ops::Deref for SetStyleVisitor {
    type Target = BaseVisitorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SetStyleVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SetStyleVisitor {
    /// java: public SetStyleVisitor(Style style, int selectionStart, int selectionEnd)
    pub fn new(mut style: Style, mut selection_start: i32, mut selection_end: i32) -> compat::Shared<Self> {
        let __init_new_style = style.clone();
        let (_, __this) = BaseVisitorBase::new_in(selection_start, selection_end, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                new_text: MutableComponent::create(PlainTextContents::new_literal_contents("".to_string())),
                new_style: __init_new_style,
            });
            ((__h.clone() as compat::Shared<dyn BaseVisitor>), __h)
        });
        __this
    }

    /// java: private void addNewText(MutableComponent newText)
    pub fn add_new_text(__this: &compat::Shared<Self>, new_text: &MutableComponent) {
        { let (__q0,) = (new_text.clone(),); __this.lock().borrow_mut().new_text.append(__q0) };
    }

    /// java: public MutableComponent getNewText()
    pub fn get_new_text(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.new_text.clone());
    }

}

impl BaseVisitor for SetStyleVisitor {
    fn base_visitor_base(&self) -> &BaseVisitorBase {
        &self.base
    }
    fn base_visitor_base_mut(&mut self) -> &mut BaseVisitorBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void visit(Style style, String asString) (dispatch: the body for this class)
    fn visit(&self) -> fn(&compat::Shared<dyn BaseVisitor>, Style, String) {
        SetStyleVisitor::visit_dyn
    }

}

impl SetStyleVisitor {
    /// java: @Override protected void visit(Style style, String asString)
    pub fn visit_impl(__this: &compat::Shared<Self>, mut style: Style, mut as_string: String) {
        if compat::str_len(&as_string) < compat::read(__this, |__t| __t.sel_start) {
            if as_string.is_empty() {
                return;
            }
            SetStyleVisitor::add_new_text(__this, &component::literal(&as_string).set_style(style.clone()));
            { let __v = (compat::read(__this, |__t| __t.sel_start)).wrapping_sub(compat::str_len(&as_string)); compat::write(__this, |__t| __t.sel_start = __v); }
            return;
        }
        if compat::read(__this, |__t| __t.sel_start) >= 0 {
            let mut substring: String = compat::substring2(&as_string, 0, compat::read(__this, |__t| __t.sel_start));
            SetStyleVisitor::add_new_text(__this, &component::literal(&substring).set_style(style.clone()));
            as_string = compat::substring(&as_string, compat::read(__this, |__t| __t.sel_start));
            { let __v = -1; compat::write(__this, |__t| __t.sel_start = __v); }
        }
        if compat::read(__this, |__t| __t.sel_size) <= 0 {
            if !as_string.is_empty() {
                SetStyleVisitor::add_new_text(__this, &component::literal(&as_string).set_style(style.clone()));
            }
            return;
        }
        if compat::str_len(&as_string) <= compat::read(__this, |__t| __t.sel_size) {
            SetStyleVisitor::add_new_text(__this, &component::literal(&as_string).set_style(__this.lock().borrow().new_style.apply_to(&style)));
        } else {
            SetStyleVisitor::add_new_text(__this, &(component::literal(&compat::substring2(&as_string, 0, compat::read(__this, |__t| __t.sel_size)))).set_style(__this.lock().borrow().new_style.apply_to(&style)));
            SetStyleVisitor::add_new_text(__this, &(component::literal(&compat::substring(&as_string, compat::read(__this, |__t| __t.sel_size)))).set_style(style.clone()));
        }
        { let __v = (compat::read(__this, |__t| __t.sel_size)).wrapping_sub(compat::str_len(&as_string)); compat::write(__this, |__t| __t.sel_size = __v); }
    }

    /// SetStyleVisitor.visit as the dispatch of BaseVisitor.visit
    pub fn visit_dyn(__this: &compat::Shared<dyn BaseVisitor>, __p0: Style, __p1: String) {
        Self::visit_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
