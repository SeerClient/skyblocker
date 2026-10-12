//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.PlainTextElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementBase;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class PlainTextElement extends Element
#[derive(Clone)]
pub struct PlainTextElement {
    /// java: extends Element
    pub base: ElementBase,
    /// java: private final List<Component> lines = new ArrayList<>();
    pub lines: Vec<MutableComponent>,
}

impl std::ops::Deref for PlainTextElement {
    type Target = ElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PlainTextElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PlainTextElement {
    /// java: public PlainTextElement(@Nullable Component txt)
    pub fn new(mut txt: Option<MutableComponent>) -> compat::Shared<Self> {
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                lines: Vec::new(),
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        let __h0 = (if txt.is_none() { Some(component::literal("No data").with_style_format(ChatFormatting::Gray)) } else { txt.clone() }).unwrap();
        __this.lock().borrow_mut().lines.push(__h0);
        { let __v = (element::PAD_L).wrapping_add(element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().lines[0].clone()))); compat::write(&__this, |__t| __t.width = __v); }
        { let __v = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT; compat::write(&__this, |__t| __t.height = __v); }
        __this
    }

    /// java: public PlainTextElement(@Nullable Component line1, @Nullable Component line2)
    pub fn new_2(mut line1: Option<MutableComponent>, mut line2: Option<MutableComponent>) -> compat::Shared<Self> {
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                lines: Vec::new(),
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        let __h0 = (if line1.is_none() { Some(component::literal("No data").with_style_format(ChatFormatting::Gray)) } else { line1.clone() }).unwrap();
        __this.lock().borrow_mut().lines.push(__h0);
        let __h0 = (if line2.is_none() { Some(component::literal("No data").with_style_format(ChatFormatting::Gray)) } else { line2.clone() }).unwrap();
        __this.lock().borrow_mut().lines.push(__h0);
        { let __v = (element::PAD_L).wrapping_add((element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().lines[(0) as usize].clone()))).max(element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().lines[(1) as usize].clone())))); compat::write(&__this, |__t| __t.width = __v); }
        { let __v = ((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_mul(2)).wrapping_add(element::PAD_S); compat::write(&__this, |__t| __t.height = __v); }
        __this
    }

}

impl Element for PlainTextElement {
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
        PlainTextElement::extract_render_state_dyn
    }

}

impl PlainTextElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut y_offset: i32 = 0;
        for line in compat::read(__this, |__t| __t.lines.clone()) {
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(line.clone()), x.wrapping_add(element::PAD_L), y.wrapping_add(y_offset), mcrust::net::minecraft::util::common_colors::WHITE, false);
            y_offset = y_offset.wrapping_add((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(element::PAD_S));
        }
    }

    /// PlainTextElement.extractRenderState as the dispatch of Element.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn Element>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
