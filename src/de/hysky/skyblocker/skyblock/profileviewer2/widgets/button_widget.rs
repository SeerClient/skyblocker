//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.ButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final Identifier NORMAL = SkyblockerMod.id("profile_viewer2/button");
pub static NORMAL: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/button"));
/// java: private static final Identifier NORMAL_TOGGLED = SkyblockerMod.id("profile_viewer2/button_toggled");
pub static NORMAL_TOGGLED: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/button_toggled"));
/// java: private static final Identifier HIGHLIGHTED = SkyblockerMod.id("profile_viewer2/button_highlighted");
pub static HIGHLIGHTED: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/button_highlighted"));
/// java: private static final Identifier HIGHLIGHTED_TOGGLED = SkyblockerMod.id("profile_viewer2/button_toggled_highlighted");
pub static HIGHLIGHTED_TOGGLED: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("profile_viewer2/button_toggled_highlighted"));
/// java: private static final int SIZE = 20;
pub const SIZE: i32 = 20;

/// java: public class ButtonWidget extends Button
pub struct ButtonWidget {
    /// java: extends net.minecraft.client.gui.components.Button
    pub base: AbstractWidget,
    /// java: private final FlexibleItemStack icon;
    pub icon: compat::Shared<FlexibleItemStack>,
}

impl std::ops::Deref for ButtonWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ButtonWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ButtonWidget {
    /// java: public ButtonWidget(FlexibleItemStack icon, OnPress onPress)
    ///
    /// (the press is the widget half's `on_press`: the game runs it with the widget list the button is in, as
    /// `Button.onPress` runs `onPress.onPress(this)`)
    pub fn new(mut icon: compat::Shared<FlexibleItemStack>, mut on_press: OnPress) -> compat::Shared<Self> {
        // java: super(0, 0, SIZE, SIZE, CommonComponents.EMPTY, onPress, DEFAULT_NARRATION);
        let base = AbstractWidget::new(0, 0, SIZE, SIZE, String::new(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::Button, on_press);
        compat::shared(Self { base, icon })
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(&mut self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if self.is_focused() && self.is_hovered() {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), HIGHLIGHTED_TOGGLED.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        } else if self.is_focused() {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), NORMAL_TOGGLED.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        } else if self.is_hovered() {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), HIGHLIGHTED.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        } else {
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), NORMAL.get_mut(), self.get_x(), self.get_y(), self.get_width(), self.get_height());
        }
        graphics.fake_item(&FlexibleItemStack::get_stack_or_throw(&self.icon), self.get_x().wrapping_add((self.get_width().wrapping_sub(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_div(2)), self.get_y().wrapping_add((self.get_height().wrapping_sub(mcrust::net::minecraft::client::gui::render::gui_renderer::DEFAULT_ITEM_SIZE)).wrapping_div(2)));
    }

}

/// ButtonWidget's widget half (`super`'s fields: x, y, size, message, `onPress`, ...), lent by the game for each call
impl compat::WidgetHalf for ButtonWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// ButtonWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl mcrust::mcrust::modding::widgets::ModWidget for compat::WidgetAdapter<ButtonWidget> {
    /// java: AbstractButton.extractWidgetRenderState - `this.extractContents(graphics, mouseX, mouseY, a);
    /// this.handleCursor(graphics);` (the cursor is `handle_cursor`)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| compat::write(__h, |__t| __t.extract_contents(&context.graphics, mouse_x, mouse_y, a)))
    }

    /// java: AbstractButton.updateWidgetNarration - `this.defaultButtonNarrationText(output)`
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput) {
        widget.default_button_narration_text(output);
    }

    /// java: AbstractButton.handleCursor
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }
}
