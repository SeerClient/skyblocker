//! Ported from de.hysky.skyblocker.skyblock.tabhud.config.StyledButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
        // TODO(fabric2seer): `new Button(..)`: the port has no `Button::new`
        //   java: super(0, 0, width, height, message, onPress, DEFAULT_NARRATION);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor context, int mouseX, int mouseY, float deltaTicks)
    pub fn extract_contents(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta_ticks: f32) {
        let mut y: i32 = (__this.lock().borrow().get_y()).wrapping_sub(1);
        let mut y2: i32 = y.wrapping_add(__this.lock().borrow().get_height());
        TopBarWidget::draw_button_border(context, __this.lock().borrow().get_x(), y, y2);
        TopBarWidget::draw_button_border(context, __this.lock().borrow().get_right(), y, y2);
        if __this.lock().borrow().is_hovered() {
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), y, (__this.lock().borrow().get_right()).wrapping_add(1), y2, argb::color_alpha_rgb(100, 0));
        } else {
            context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().get_x(), y, (__this.lock().borrow().get_right()).wrapping_add(1), y2, argb::color_alpha_rgb(50, 0));
        }
        // TODO(fabric2seer): field `NONE` of a value of unknown type
        //   java: this.extractDefaultLabel(context.textRendererForWidget(this, GuiGraphicsExtractor.HoveredTextEffects.NONE));
        if __this.lock().borrow().is_hovered() {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): static `CursorTypes.NOT_ALLOWED` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: context.request_cursor(&(if __this.lock().borrow().is_active() { CursorTypes::POINTING_HAND.clone() } else { CursorTypes::NOT_ALLOWED.clone() }));
        }
    }

}

// NOTE(fabric2seer): extractContents override(s) of Button - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
