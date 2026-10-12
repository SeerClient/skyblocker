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
    pub fn new(mut icon: compat::Shared<FlexibleItemStack>, mut on_press: OnPress) -> Self {
        let __init_icon = icon.clone();
        // TODO(fabric2seer): `new Button(..)`: the port has no `Button::new`
        //   java: super(0, 0, SIZE, SIZE, CommonComponents.EMPTY, onPress, DEFAULT_NARRATION);
        let mut this: Self = todo!("constructor");
        this
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

// NOTE(fabric2seer): extractContents override(s) of Button - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
