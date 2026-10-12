//! Ported from de.hysky.skyblocker.skyblock.tabhud.widget.PlaceholderWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidgetBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::Information;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;

/// java: public class PlaceholderWidget extends HudWidget
#[derive(Clone)]
pub struct PlaceholderWidget {
    /// java: extends HudWidget
    pub base: HudWidgetBase,
}

impl std::ops::Deref for PlaceholderWidget {
    type Target = HudWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PlaceholderWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PlaceholderWidget {
    /// java: public PlaceholderWidget(String id)
    pub fn new(mut id: String) -> compat::Shared<Self> {
        let (_, __this) = HudWidgetBase::new_in(Information::new_canonical(Some(id.clone()), Some(component::literal(&id)), Arc::new(move |_| false)), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn HudWidget>), __h)
        });
        __this
    }

}

impl HudWidget for PlaceholderWidget {
    fn hud_widget_base(&self) -> &HudWidgetBase {
        &self.base
    }
    fn hud_widget_base_mut(&mut self) -> &mut HudWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        PlaceholderWidget::extract_widget_render_state_dyn
    }

    /// java: @Override public void extractWidgetRenderStateForConfig(GuiGraphicsExtractor context, float delta) (dispatch: the body for this class)
    fn extract_widget_render_state_for_config(&self) -> fn(&compat::Shared<dyn HudWidget>, &mut GuiGraphicsExtractor, f32) {
        PlaceholderWidget::extract_widget_render_state_for_config_dyn
    }

}

impl PlaceholderWidget {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor context, float delta)
    pub fn extract_widget_render_state_impl(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut delta: f32) {
        // java: w = h = 0;
        compat::write(__this, |__t| { __t.h = 0; __t.w = __t.h; });
    }

    /// PlaceholderWidget.extractWidgetRenderState as the dispatch of HudWidget.extractWidgetRenderState
    pub fn extract_widget_render_state_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut GuiGraphicsExtractor, __p1: f32) {
        Self::extract_widget_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void extractWidgetRenderStateForConfig(GuiGraphicsExtractor context, float delta)
    pub fn extract_widget_render_state_for_config_impl(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut delta: f32) {
        let mut text_renderer: Font = compat::font();
        { let __v = 15; compat::write(__this, |__t| __t.h = __v); }
        { let __v = (text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((({ let __f = compat::dispatch(__this, |__t| __t.get_information()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)) })).display_name().unwrap()))).wrapping_add(10); compat::write(__this, |__t| __t.w = __v); }
        context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), 0, 0, ({ let __f = compat::dispatch(__this, |__t| __t.get_width()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)) }), ({ let __f = compat::dispatch(__this, |__t| __t.get_height()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)) }), 0xAA000000u32 as i32);
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((({ let __f = compat::dispatch(__this, |__t| __t.get_information()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)) })).display_name().unwrap()), ({ let __f = compat::dispatch(__this, |__t| __t.get_width()); __f(&compat::view(__this, |a| a as compat::Shared<dyn HudWidget>)) }).wrapping_div(2), 3, -1);
    }

    /// PlaceholderWidget.extractWidgetRenderStateForConfig as the dispatch of HudWidget.extractWidgetRenderStateForConfig
    pub fn extract_widget_render_state_for_config_dyn(__this: &compat::Shared<dyn HudWidget>, __p0: &mut GuiGraphicsExtractor, __p1: f32) {
        Self::extract_widget_render_state_for_config_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
