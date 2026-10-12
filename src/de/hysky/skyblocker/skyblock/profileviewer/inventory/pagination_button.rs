//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.PaginationButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_page::ProfileViewerPage;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class PaginationButton extends AbstractWidget
#[derive(Clone)]
pub struct PaginationButton {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private final ProfileViewerPage screen;
    pub screen: compat::Shared<dyn ProfileViewerPage>,
    /// java: private final boolean isNextButton;
    pub is_next_button: bool,
    /// java: private final Identifier TEXTURE;
    pub texture: Option<Identifier>,
    /// java: private final Identifier HIGHLIGHT;
    pub highlight: Option<Identifier>,
}

impl std::ops::Deref for PaginationButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PaginationButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PaginationButton {
    /// java: public PaginationButton(ProfileViewerPage screen, int x, int y, boolean isNextButton)
    pub fn new(mut screen: compat::Shared<dyn ProfileViewerPage>, mut x: i32, mut y: i32, mut is_next_button: bool) -> compat::Shared<Self> {
        let __init_screen = screen.clone();
        let __init_is_next_button = is_next_button;
        let __this = compat::shared(Self {
            base: AbstractWidget::new(x, y, 12, 17, (&component::empty().get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            screen: __init_screen,
            is_next_button: __init_is_next_button,
            texture: None,
            highlight: None,
        });
        if is_next_button {
            { let __v = Some(Identifier::with_default_namespace("textures/gui/sprites/recipe_book/page_forward.png")); compat::write(&__this, |__t| __t.texture = __v); }
            { let __v = Some(Identifier::with_default_namespace("textures/gui/sprites/recipe_book/page_forward_highlighted.png")); compat::write(&__this, |__t| __t.highlight = __v); }
        } else {
            { let __v = Some(Identifier::with_default_namespace("textures/gui/sprites/recipe_book/page_backward.png")); compat::write(&__this, |__t| __t.texture = __v); }
            { let __v = Some(Identifier::with_default_namespace("textures/gui/sprites/recipe_book/page_backward_highlighted.png")); compat::write(&__this, |__t| __t.highlight = __v); }
        }
        __this
    }

    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &compat::read(__this, |__t| __t.texture.clone()).unwrap(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), ((0) as f32) as f32, ((0) as f32) as f32, 12, 17, 12, 17);
        if { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) } {
            gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &compat::read(__this, |__t| __t.highlight.clone()).unwrap(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), ((0) as f32) as f32, ((0) as f32) as f32, 12, 17, 12, 17);
        }
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    pub fn on_click(__this: &compat::Shared<Self>, click: &MouseButtonEvent, mut doubled: bool) {
        if compat::read(__this, |__t| __t.is_next_button) {
            ({ let __r = &compat::read(__this, |__t| __t.screen.clone()); let __f = compat::dispatch(__r, |__t| __t.next_page()); __f(__r) });
        } else {
            ({ let __r = &compat::read(__this, |__t| __t.screen.clone()); let __f = compat::dispatch(__r, |__t| __t.previous_page()); __f(__r) });
        }
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &dyn NarrationElementOutput) {
    }

}

/// PaginationButton's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for PaginationButton {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// PaginationButton as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<PaginationButton> {
    /// java: @Override protected void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| PaginationButton::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| PaginationButton::update_widget_narration(__h, output))
    }

    /// java: @Override public void onClick(MouseButtonEvent click, boolean doubled)
    fn on_click(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) {
        self.lend(widget, |__h| PaginationButton::on_click(__h, event, double_click))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, onClick, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
