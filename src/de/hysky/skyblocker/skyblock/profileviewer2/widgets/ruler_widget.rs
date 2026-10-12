//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.RulerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::abstract_profile_viewer_screen;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::profile_viewer_widget;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public final class RulerWidget extends AbstractWidget
#[derive(Clone)]
pub struct RulerWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
}

impl std::ops::Deref for RulerWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RulerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RulerWidget {
    /// java: public RulerWidget()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 0, 0, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
        });
        { let __v = false; compat::write(&__this, |__t| __t.active = __v); }
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut border_width: i32 = (abstract_profile_viewer_screen::BACKGROUND_WIDTH).wrapping_sub((*profile_viewer_widget::CONTENT_PADDING).wrapping_mul(2));
        let mut border_height: i32 = (abstract_profile_viewer_screen::BACKGROUND_HEIGHT).wrapping_sub((*profile_viewer_widget::CONTENT_PADDING).wrapping_mul(2));
        GuiHelper::border(graphics, __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), border_width, border_height, mcrust::net::minecraft::util::common_colors::HIGH_CONTRAST_DIAMOND);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), (__this.lock().borrow().get_x()).wrapping_add(1), (__this.lock().borrow().get_y()).wrapping_add(1), mcrust::net::minecraft::util::common_colors::RED);
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, output: &dyn NarrationElementOutput) {
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

}

/// RulerWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for RulerWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// RulerWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<RulerWidget> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| RulerWidget::extract_widget_render_state(__h, &mut context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput output)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| RulerWidget::update_widget_narration(__h, output))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, shouldTakeFocusAfterInteraction, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
