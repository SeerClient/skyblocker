//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.IcoFatTextElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementBase;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int ICO_OFFS = 4;
pub const ICO_OFFS: i32 = 4;

/// java: class IcoFatTextElement extends Element
#[derive(Clone)]
pub struct IcoFatTextElement {
    /// java: extends Element
    pub base: ElementBase,
    /// java: private FlexibleItemStack ico;
    pub ico: Option<compat::Shared<FlexibleItemStack>>,
    /// java: private final Component line1;
    pub line1: Option<MutableComponent>,
    /// java: private final Component line2;
    pub line2: Option<MutableComponent>,
}

impl std::ops::Deref for IcoFatTextElement {
    type Target = ElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IcoFatTextElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IcoFatTextElement {
    /// java: IcoFatTextElement(@Nullable FlexibleItemStack ico, @Nullable Component l1, @Nullable Component l2)
    pub fn new(mut ico: Option<compat::Shared<FlexibleItemStack>>, mut l1: Option<MutableComponent>, mut l2: Option<MutableComponent>) -> compat::Shared<Self> {
        let __init_ico = (if ico.is_none() { Some(ico::BARRIER.get_mut().clone()) } else { ico.clone() });
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                ico: __init_ico,
                line1: None,
                line2: None,
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        if l1.is_none() || l2.is_none() {
            { let __v = Some(ico::BARRIER.get_mut().clone()); compat::write(&__this, |__t| __t.ico = __v); }
            { let __v = Some(component::literal("No data").with_style_format(ChatFormatting::Gray)); compat::write(&__this, |__t| __t.line1 = __v); }
            { let __v = Some(component::literal("No data").with_style_format(ChatFormatting::Gray)); compat::write(&__this, |__t| __t.line2 = __v); }
        } else {
            { let __v = l1.clone(); compat::write(&__this, |__t| __t.line1 = __v); }
            { let __v = l2.clone(); compat::write(&__this, |__t| __t.line2 = __v); }
        }
        { let __v = ((((compat::read(&__this, |__t| __t.ico_dim.clone())))()).wrapping_add(element::PAD_L)).wrapping_add((element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(&__this, |__t| __t.line1.clone()).unwrap()))).max(element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(&__this, |__t| __t.line2.clone()).unwrap())))); compat::write(&__this, |__t| __t.width = __v); }
        { let __v = ((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(element::PAD_S)).wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT); compat::write(&__this, |__t| __t.height = __v); }
        __this
    }

}

impl Element for IcoFatTextElement {
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
        IcoFatTextElement::extract_render_state_dyn
    }

}

impl IcoFatTextElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut text_x: i32 = (x.wrapping_add(((compat::read(__this, |__t| __t.ico_dim.clone())))())).wrapping_add(element::PAD_L);
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_icon()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Element>), graphics, &compat::read(__this, |__t| __t.ico.clone()).unwrap(), x, y.wrapping_add(ICO_OFFS)) });
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.line1.clone()).unwrap()), text_x, y, mcrust::net::minecraft::util::common_colors::WHITE, false);
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.line2.clone()).unwrap()), text_x, (y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_add(element::PAD_S), mcrust::net::minecraft::util::common_colors::WHITE, false);
    }

    /// IcoFatTextElement.extractRenderState as the dispatch of Element.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn Element>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
