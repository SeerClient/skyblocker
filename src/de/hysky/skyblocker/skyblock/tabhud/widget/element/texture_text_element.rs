//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.element.TextureTextElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::Element;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::element::element::ElementBase;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class TextureTextElement extends Element
#[derive(Clone)]
pub struct TextureTextElement {
    /// java: extends Element
    pub base: ElementBase,
    /// java: private final Identifier texture;
    pub texture: Identifier,
    /// java: private final Component text;
    pub text: MutableComponent,
    /// java: private final int textureWidth;
    pub texture_width: i32,
    /// java: private final int textureHeight;
    pub texture_height: i32,
}

impl std::ops::Deref for TextureTextElement {
    type Target = ElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TextureTextElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TextureTextElement {
    /// java: public TextureTextElement(Component text, Identifier texture, int textureWidth, int textureHeight)
    pub fn new(mut text: MutableComponent, mut texture: Identifier, mut texture_width: i32, mut texture_height: i32) -> compat::Shared<Self> {
        let __init_text = text.clone();
        let __init_texture = texture.clone();
        let __init_texture_width = texture_width;
        let __init_texture_height = texture_height;
        let (_, __this) = ElementBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                texture: __init_texture,
                text: __init_text,
                texture_width: __init_texture_width,
                texture_height: __init_texture_height,
            });
            ((__h.clone() as compat::Shared<dyn Element>), __h)
        });
        { let __v = (texture_width.wrapping_add(element::PAD_L)).wrapping_add(element::TXT_REND.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(&__this, |__t| __t.text.clone())))); compat::write(&__this, |__t| __t.width = __v); }
        { let __v = texture_height.max(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT); compat::write(&__this, |__t| __t.height = __v); }
        __this
    }

}

impl Element for TextureTextElement {
    fn element_base(&self) -> &ElementBase {
        &self.base
    }
    fn element_base_mut(&mut self) -> &mut ElementBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int x, int y) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn Element>, &mut GuiGraphicsExtractor, i32, i32) {
        TextureTextElement::extract_render_state_dyn
    }

}

impl TextureTextElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int x, int y)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        let mut offset: i32 = (if SkyblockerConfigManager::get().ui_and_visuals.tab_hud.compact_widgets { 2 } else { 4 });
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &compat::read(__this, |__t| __t.texture.clone()), x, y, ((0) as f32) as f32, ((0) as f32) as f32, compat::read(__this, |__t| __t.texture_width), compat::read(__this, |__t| __t.texture_height), compat::read(__this, |__t| __t.texture_width), compat::read(__this, |__t| __t.texture_height));
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.text.clone())), (x.wrapping_add(compat::read(__this, |__t| __t.texture_width))).wrapping_add(element::PAD_L), y.wrapping_add(offset), mcrust::net::minecraft::util::common_colors::WHITE, false);
    }

    /// TextureTextElement.extractRenderState as the dispatch of Element.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn Element>, __p0: &mut GuiGraphicsExtractor, __p1: i32, __p2: i32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

}
