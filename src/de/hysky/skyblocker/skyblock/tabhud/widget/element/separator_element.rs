//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.SeparatorElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element_based_widget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class SeparatorElement extends Element
#[derive(Clone)]
pub struct SeparatorElement {
    /// java: extends Element
    pub base: ElementBase,
    /// java: private final @Nullable Component text;
    pub text: Option<MutableComponent>,
    /// java: private final int textWidth;
    pub text_width: i32,
}

impl std::ops::Deref for SeparatorElement {
    type Target = ElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SeparatorElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SeparatorElement {
    /// java: public SeparatorElement(@Nullable Component text)
    pub fn new(mut text: Option<MutableComponent>) -> compat::Shared<Self> {
        let __init_text = text.clone();
        let __init_text_width = (if text.is_some() { element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone().unwrap())) } else { 0 });
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                text: __init_text,
                text_width: __init_text_width,
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        { let __v = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT; compat::write(&__this, |__t| __t.height = __v); }
        { let __v = (compat::read(&__this, |__t| __t.text_width)).wrapping_add(4); compat::write(&__this, |__t| __t.width = __v); }
        __this
    }

}

impl Element for SeparatorElement {
    fn element_base(&self) -> &ElementBase {
        &self.base
    }
    fn element_base_mut(&mut self) -> &mut ElementBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn Element>, &mut GuiGraphicsExtractor, i32, i32) {
        SeparatorElement::extract_render_state_dyn
    }

}

impl SeparatorElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut parent_width: i32 = (({ let __f = compat::dispatch(__this, |__t| __t.get_parent()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Element>)) })).get_width();
        if compat::read(__this, |__t| __t.text.clone()).is_some() && !(compat::read(__this, |__t| __t.text.clone()).unwrap() == component::null_to_empty(Some(""))) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x.wrapping_sub((element_based_widget::BORDER_SZE_E).wrapping_div(2)), y.wrapping_add((compat::read(__this, |__t| __t.height)).wrapping_div(2)), x.wrapping_add(2), (y.wrapping_add((compat::read(__this, |__t| __t.height)).wrapping_div(2))).wrapping_add(1), 0xFF55FFFFu32 as i32);
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.text.clone()).unwrap()), x.wrapping_add(4), y, 0xFF55FFFFu32 as i32, false);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), ((x.wrapping_add(compat::read(__this, |__t| __t.text_width))).wrapping_add(2)).wrapping_add(4), y.wrapping_add((compat::read(__this, |__t| __t.height)).wrapping_div(2)), (((x.wrapping_add(parent_width)).wrapping_sub(element_based_widget::BORDER_SZE_E)).wrapping_sub(element_based_widget::BORDER_SZE_W)).wrapping_add(2), (y.wrapping_add((compat::read(__this, |__t| __t.height)).wrapping_div(2))).wrapping_add(1), 0xFF55FFFFu32 as i32);
        } else {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), x.wrapping_sub(2), y.wrapping_add((compat::read(__this, |__t| __t.height)).wrapping_div(2)), (((x.wrapping_add(parent_width)).wrapping_sub(element_based_widget::BORDER_SZE_E)).wrapping_sub(element_based_widget::BORDER_SZE_W)).wrapping_add(2), (y.wrapping_add((compat::read(__this, |__t| __t.height)).wrapping_div(2))).wrapping_add(1), 0xFF55FFFFu32 as i32);
        }
    }

    /// SeparatorElement.extractRenderState as the dispatch of Element.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn Element>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
