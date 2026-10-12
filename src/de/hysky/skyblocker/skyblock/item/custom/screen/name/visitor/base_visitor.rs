//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.name.visitor.BaseVisitor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::style::Style;

/// java: abstract class BaseVisitor implements FormattedText.StyledContentConsumer<Void>
#[derive(Clone)]
pub struct BaseVisitorBase {
    /// java: protected int selStart;
    pub sel_start: i32,
    /// java: protected int selSize;
    pub sel_size: i32,
}

impl BaseVisitorBase {
    /// java: BaseVisitor(int selectionStart, int selectionEnd) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut selection_start: i32, mut selection_end: i32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn BaseVisitor>, __R)) -> (compat::Shared<dyn BaseVisitor>, __R) {
        let __init_sel_start = selection_start.min(selection_end);
        let __init_sel_size = (selection_start.wrapping_sub(selection_end)).wrapping_abs();
        let this = Self {
            sel_start: __init_sel_start,
            sel_size: __init_sel_size,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of BaseVisitor (implemented by BaseVisitorBase and every subclass)
pub trait BaseVisitor: compat::DynAny {
    fn base_visitor_base(&self) -> &BaseVisitorBase;
    fn base_visitor_base_mut(&mut self) -> &mut BaseVisitorBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public final Optional<Void> accept(Style style, String asString) (dispatch: the body for this class)
    fn accept(&self) -> fn(&compat::Shared<dyn BaseVisitor>, &Style, &str) -> Option<()> {
        <dyn BaseVisitor>::accept_body
    }

    /// java: protected abstract void visit(Style style, String asString); (dispatch: the body for this class)
    fn visit(&self) -> fn(&compat::Shared<dyn BaseVisitor>, Style, String);

}

impl dyn BaseVisitor {
    /// java: @Override public final Optional<Void> accept(Style style, String asString)
    pub fn accept_body(__this: &compat::Shared<dyn BaseVisitor>, style: &Style, as_string: &str) -> Option<()> {
        ({ let __f = compat::dispatch(__this, |__t| __t.visit()); __f(__this, style.clone(), as_string.to_string()) });
        return None;
    }

}
