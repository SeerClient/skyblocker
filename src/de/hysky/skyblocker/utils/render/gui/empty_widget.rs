//! Ported from de.hysky.skyblocker.utils.render.gui.EmptyWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::gui::basic_widget::BasicWidget;
use crate::de::hysky::skyblocker::utils::render::gui::basic_widget::BasicWidgetBase;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: public class EmptyWidget extends BasicWidget
#[derive(Clone)]
pub struct EmptyWidget {
    /// java: extends BasicWidget
    pub base: BasicWidgetBase,
}

impl std::ops::Deref for EmptyWidget {
    type Target = BasicWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EmptyWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EmptyWidget {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = BasicWidgetBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn BasicWidget>), __h)
        });
        __this
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor context, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
    }

}

impl BasicWidget for EmptyWidget {
    fn basic_widget_base(&self) -> &BasicWidgetBase {
        &self.base
    }
    fn basic_widget_base_mut(&mut self) -> &mut BasicWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
