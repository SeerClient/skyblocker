//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.IcoTextElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
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

/// java: class IcoTextElement extends Element
#[derive(Clone)]
pub struct IcoTextElement {
    /// java: extends Element
    pub base: ElementBase,
    /// java: private FlexibleItemStack ico;
    pub ico: Option<compat::Shared<FlexibleItemStack>>,
    /// java: private final Component text;
    pub text: Option<MutableComponent>,
}

impl std::ops::Deref for IcoTextElement {
    type Target = ElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IcoTextElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IcoTextElement {
    /// java: IcoTextElement(@Nullable FlexibleItemStack ico, @Nullable Component txt)
    pub fn new(mut ico: Option<compat::Shared<FlexibleItemStack>>, mut txt: Option<MutableComponent>) -> compat::Shared<Self> {
        let __init_ico = (if ico.is_none() { Some(ico::BARRIER.get_mut().clone()) } else { ico.clone() });
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                ico: __init_ico,
                text: None,
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        if let Some(txt) = &txt {
            { let __v = Some(txt.clone()); compat::write(&__this, |__t| __t.text = __v); }
        } else {
            { let __v = Some(ico::BARRIER.get_mut().clone()); compat::write(&__this, |__t| __t.ico = __v); }
            { let __v = Some(component::literal("No data").with_style_format(ChatFormatting::Gray)); compat::write(&__this, |__t| __t.text = __v); }
        }
        let mut icon_dim: i32 = ((compat::read(&__this, |__t| __t.ico_dim.clone())))();
        { let __v = (icon_dim.wrapping_add(element::PAD_L)).wrapping_add(element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(&__this, |__t| __t.text.clone()).unwrap()))); compat::write(&__this, |__t| __t.width = __v); }
        { let __v = icon_dim; compat::write(&__this, |__t| __t.height = __v); }
        __this
    }

}

impl Element for IcoTextElement {
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
        IcoTextElement::extract_render_state_dyn
    }

}

impl IcoTextElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int x, int y)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut offset: i32 = (if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.compact_widgets { 2 } else { 4 });
        ({ let __f = compat::dispatch(__this, |__t| __t.extract_icon()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Element>), graphics, &compat::read(__this, |__t| __t.ico.clone()).unwrap(), x, y) });
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.text.clone()).unwrap()), (x.wrapping_add(((compat::read(__this, |__t| __t.ico_dim.clone())))())).wrapping_add(element::PAD_L), y.wrapping_add(offset), mcrust::net::minecraft::util::common_colors::WHITE, false);
    }

    /// IcoTextElement.extractRenderState as the dispatch of Element.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn Element>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
