//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.ToggleButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::top_bar_widget::TopBarWidget;
use crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::argb;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: class ToggleButtonWidget extends AbstractButton
pub struct ToggleButtonWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractButton
    pub base: AbstractWidget,
    /// java: private final BooleanConsumer onPress;
    pub on_press: Arc<dyn Fn(bool)>,
    /// java: private boolean state;
    pub state: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ToggleButtonWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ToggleButtonWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ToggleButtonWidget {
    /// java: ToggleButtonWidget(int width, int height, Component message, BooleanConsumer onPress)
    pub fn new(mut width: i32, mut height: i32, mut message: MutableComponent, on_press: Arc<dyn Fn(bool)>) -> compat::Shared<Self> {
        let __init_on_press = on_press.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, width, height, (&message.get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            on_press: __init_on_press,
            state: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float deltaTicks)
    /// (`graphics` is the frame's widget render context, which the label's scrolling draw takes)
    pub fn extract_contents(__this: &compat::Shared<Self>, graphics: &mut WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        let (x, right, top, bottom, width, height, hovered, active, alpha, state) = compat::read(__this, |__t| (__t.get_x(), __t.get_right(), __t.get_y(), __t.get_bottom(), __t.get_width(), __t.get_height(), __t.is_hovered(), __t.active, __t.alpha, __t.state));
        let __message = compat::read(__this, |__t| __t.get_message_component());
        let mut y: i32 = top.wrapping_sub(1);
        let mut y2: i32 = y.wrapping_add(height);
        TopBarWidget::draw_button_border(&graphics.graphics, x, y, y2);
        TopBarWidget::draw_button_border(&graphics.graphics, right, y, y2);
        if hovered {
            graphics.graphics.fill(graphics.gl, x, y, right.wrapping_add(1), y2, argb::color_alpha_rgb(100, 0));
        } else {
            graphics.graphics.fill(graphics.gl, x, y, right.wrapping_add(1), y2, argb::color_alpha_rgb(50, 0));
        }
        let mut color: i32 = ((if active { 0xFFFFFFFFu32 as i32 } else { 0xFFA0A0A0u32 as i32 }) | (mth::ceil_f32(alpha * 255.0f32)).wrapping_shl(24));
        let mut text_width: i32 = compat::font().width_of_cached(&__message);
        let mut start_x: i32;
        let mut end_x: i32;
        let mut square_x: i32;
        let mut padded_text_width: i32 = (text_width.wrapping_add(9)).wrapping_add(2);
        if padded_text_width > width.wrapping_sub(6) {
            square_x = (right.wrapping_sub(3)).wrapping_sub(9);
            end_x = square_x.wrapping_sub(2);
            start_x = x.wrapping_add(3);
        } else {
            let mut center_x: i32 = x.wrapping_add(width.wrapping_div(2));
            square_x = (center_x.wrapping_add(padded_text_width.wrapping_div(2))).wrapping_sub(9);
            start_x = center_x.wrapping_sub(padded_text_width.wrapping_div(2));
            end_x = square_x.wrapping_sub(2);
        }
        // java: graphics.textRendererForWidget(this, GuiGraphicsExtractor.HoveredTextEffects.NONE).acceptScrollingWithDefaultCenter(getMessage(), startX, endX, getY(), getBottom());
        compat::accept_scrolling_with_default_center(graphics, &__message, start_x, end_x, top, bottom);
        let mut square_y: i32 = top.wrapping_add((height.wrapping_sub(9)).wrapping_div(2));
        GuiHelper::border(&mut graphics.graphics, square_x, square_y, 9, 9, color);
        if state {
            graphics.graphics.fill(graphics.gl, square_x.wrapping_add(2), square_y.wrapping_add(2), square_x.wrapping_add(7), square_y.wrapping_add(7), color);
        }
        let _ = (mouse_x, mouse_y, delta_ticks);
    }

    /// java: public void setState(boolean state)
    pub fn set_state(__this: &compat::Shared<Self>, mut state: bool) {
        { let __v = state; compat::write(__this, |__t| __t.state = __v); }
    }

    /// java: public boolean getState()
    pub fn get_state(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.state);
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    pub fn on_press(__this: &compat::Shared<Self>, input: &dyn InputWithModifiers) {
        { let __v = !(compat::read(__this, |__t| __t.state)); compat::write(__this, |__t| __t.state = __v); }
        ((compat::read(__this, |__t| __t.on_press.clone())))(compat::read(__this, |__t| __t.state));
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// ToggleButtonWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for ToggleButtonWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ToggleButtonWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<ToggleButtonWidget> {
    /// java: AbstractButton.extractWidgetRenderState
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        // `AbstractButton.extractWidgetRenderState` - `this.extractContents(..)`, this class's override
        self.lend_ref(widget, |__h| ToggleButtonWidget::extract_contents(__h, context, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| ToggleButtonWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public void onPress(InputWithModifiers input)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| ToggleButtonWidget::on_press(__h, event))
    }

}

