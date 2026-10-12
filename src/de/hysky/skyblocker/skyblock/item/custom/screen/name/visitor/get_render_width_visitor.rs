//! Ported from de.hysky.skyblocker.skyblock.item.custom.screen.name.visitor.GetRenderWidthVisitor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::base_visitor::BaseVisitor;
use crate::de::hysky::skyblocker::skyblock::item::custom::screen::name::visitor::base_visitor::BaseVisitorBase;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;

/// java: public class GetRenderWidthVisitor extends BaseVisitor
pub struct GetRenderWidthVisitor {
    /// java: extends BaseVisitor
    pub base: BaseVisitorBase,
    /// java: private final IntIntMutablePair widths = new IntIntMutablePair(0, 0);
    pub widths: (i32, i32),
    /// java: private final MutableComponent text = Component.empty();
    pub text: MutableComponent,
    /// java: private final Font textRenderer;
    pub text_renderer: Option<Font<'static>>,
}

impl std::ops::Deref for GetRenderWidthVisitor {
    type Target = BaseVisitorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GetRenderWidthVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GetRenderWidthVisitor {
    /// java: public GetRenderWidthVisitor(Font textRenderer, int selectionStart, int selectionEnd)
    pub fn new(mut text_renderer: Font, mut selection_start: i32, mut selection_end: i32) -> compat::Shared<Self> {
        let __init_text_renderer = Some(compat::font());
        let (_, __this) = BaseVisitorBase::new_in(selection_start, selection_end, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                widths: (0, 0),
                text: component::empty(),
                text_renderer: __init_text_renderer,
            });
            ((__h.clone() as compat::Shared<dyn BaseVisitor>), __h)
        });
        __this
    }

    /// java: public GetRenderWidthVisitor(int selectionStart, int selectionEnd)
    pub fn new_2(mut selection_start: i32, mut selection_end: i32) -> compat::Shared<Self> {
        let __this = Self::new(compat::font(), selection_start, selection_end);
        __this
    }

    /// java: public IntIntPair getWidths()
    pub fn get_widths(__this: &compat::Shared<Self>) -> (i32, i32) {
        return compat::read(__this, |__t| __t.widths.clone());
    }

}

impl BaseVisitor for GetRenderWidthVisitor {
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
        GetRenderWidthVisitor::visit_dyn
    }

}

impl GetRenderWidthVisitor {
    /// java: @Override protected void visit(Style style, String asString)
    pub fn visit_impl(__this: &compat::Shared<Self>, mut style: Style, mut as_string: String) {
        compat::write(__this, |__t| {
            let GetRenderWidthVisitor { base, widths, text, text_renderer } = __t;
            let text_renderer = text_renderer.as_ref().unwrap();
            // textRenderer.width(text)
            let width = |text: &MutableComponent| text_renderer.width_of(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()));
            if compat::str_len(&as_string) < base.sel_start {
                *text = std::mem::replace(text, component::empty()).append(component::literal(&as_string).set_style(style.clone()));
                base.sel_start = base.sel_start.wrapping_sub(compat::str_len(&as_string));
                return;
            }
            if base.sel_start >= 0 {
                *text = std::mem::replace(text, component::empty()).append((component::literal(&compat::substring2(&as_string, 0, base.sel_start))).set_style(style.clone()));
                // widths.first(textRenderer.width(text));
                widths.0 = width(text);
                as_string = compat::substring(&as_string, base.sel_start);
                base.sel_start = -1;
                if base.sel_size == 0 {
                    widths.1 = width(text);
                }
            }
            if base.sel_size == 0 {
                return;
            }
            if compat::str_len(&as_string) <= base.sel_size {
                *text = std::mem::replace(text, component::empty()).append(component::literal(&as_string).set_style(style.clone()));
                base.sel_size = base.sel_size.wrapping_sub(compat::str_len(&as_string));
                if base.sel_size == 0 {
                    widths.1 = width(text);
                }
            } else {
                *text = std::mem::replace(text, component::empty()).append((component::literal(&compat::substring2(&as_string, 0, base.sel_size))).set_style(style.clone()));
                // widths.second(textRenderer.width(text));
                widths.1 = width(text);
                base.sel_size = 0;
            }
        });
    }

    /// GetRenderWidthVisitor.visit as the dispatch of BaseVisitor.visit
    pub fn visit_dyn(__this: &compat::Shared<dyn BaseVisitor>, __p0: Style, __p1: String) {
        Self::visit_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
