//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.StyledButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::component::Component as _;
use crate::de::hysky::skyblocker::skyblock::tabhud::config::top_bar_widget::TopBarWidget;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::argb;

/// java: class StyledButtonWidget extends Button
pub struct StyledButtonWidget {
    /// java: extends net.minecraft.client.gui.components.Button
    pub base: AbstractWidget,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for StyledButtonWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StyledButtonWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StyledButtonWidget {
    /// java: StyledButtonWidget(int width, int height, net.minecraft.network.chat.Component message, OnPress onPress)
    pub fn new(mut width: i32, mut height: i32, mut message: MutableComponent, mut on_press: OnPress) -> compat::Shared<Self> {
        // java: super(0, 0, width, height, message, onPress, DEFAULT_NARRATION); - `Button`'s constructor is its builder's
        // `build()` (`DEFAULT_NARRATION` is the builder's default)
        let __base: AbstractWidget = mcrust::net::minecraft::client::gui::components::button::builder(message.get_string(), on_press).pos(0, 0).size(width, height).build();
        let __this: compat::Shared<Self> = compat::shared(Self { base: __base, self_ref: std::sync::Weak::new() });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    /// (`context` is the frame's widget render context, which the label's scrolling draw takes)
    pub fn extract_contents(__this: &compat::Shared<Self>, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        let (x, right, top, height, hovered, active) = compat::read(__this, |__t| (__t.get_x(), __t.get_right(), __t.get_y(), __t.get_height(), __t.is_hovered(), __t.is_active()));
        let mut y: i32 = top.wrapping_sub(1);
        let mut y2: i32 = y.wrapping_add(height);
        TopBarWidget::draw_button_border(&context.graphics, x, y, y2);
        TopBarWidget::draw_button_border(&context.graphics, right, y, y2);
        if hovered {
            context.graphics.fill(context.gl, x, y, right.wrapping_add(1), y2, argb::color_alpha_rgb(100, 0));
        } else {
            context.graphics.fill(context.gl, x, y, right.wrapping_add(1), y2, argb::color_alpha_rgb(50, 0));
        }
        // java: this.extractDefaultLabel(context.textRendererForWidget(this, GuiGraphicsExtractor.HoveredTextEffects.NONE));
        // (`AbstractButton.extractDefaultLabel` - `extractScrollingStringOverContents(output, getMessage(), 2)`: the message
        // scrolled between `getX() + 2` and `getRight() - 2`, centred, over the full height)
        let __message = compat::read(__this, |__t| __t.get_message_component());
        compat::accept_scrolling_with_default_center(context, &__message, x.wrapping_add(2), right.wrapping_sub(2), top, top.wrapping_add(height));
        if hovered {
            // java: context.requestCursor(this.isActive() ? CursorTypes.POINTING_HAND : CursorTypes.NOT_ALLOWED);
            compat::batch_w_request_cursor(if active { compat::BatchWCursor::PointingHand } else { compat::BatchWCursor::NotAllowed });
        }
        let _ = (mouse_x, mouse_y, delta_ticks);
    }

}

/// StyledButtonWidget's widget half (`super`'s fields: x, y, size, message, the press), lent by the game for each call
impl compat::WidgetHalf for StyledButtonWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// StyledButtonWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on. The press
/// is `Button`'s own (`OnPress` on the widget half), which the game runs.
impl mcrust::mcrust::modding::widgets::ModWidget for compat::WidgetAdapter<StyledButtonWidget> {
    /// java: `AbstractButton.extractWidgetRenderState` - `this.extractContents(..)`, this class's override
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| StyledButtonWidget::extract_contents(__h, context, mouse_x, mouse_y, a))
    }

    /// java: `Button.updateWidgetNarration` - `defaultButtonNarrationText(output)`
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput) {
        widget.default_button_narration_text(output);
    }
}
