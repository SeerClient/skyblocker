//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.widgets.PaginationWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;

/// java: public final class PaginationWidget extends ImageButton
pub struct PaginationWidget {
    /// java: extends net.minecraft.client.gui.components.ImageButton
    pub base: AbstractWidget,
}

impl std::ops::Deref for PaginationWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PaginationWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PaginationWidget {
    /// java: public PaginationWidget(boolean forwards, OnPress onPress)
    ///
    /// (`ImageButton`'s fields: the half's kind carries the sprites, its `on_press` the press, which the game runs
    /// with the widget list the button is in)
    pub fn new(mut forwards: bool, mut on_press: OnPress) -> compat::Shared<Self> {
        use mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_page;
        // java: super(0, 0, RecipeBookPage.TURN_PAGE_SPRITE_WIDTH, RecipeBookPage.TURN_PAGE_SPRITE_HEIGHT, forwards ? RecipeBookPage.PAGE_FORWARD_SPRITES : RecipeBookPage.PAGE_BACKWARD_SPRITES, onPress);
        let sprites = if forwards {
            mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites::with_focused(recipe_book_page::page_forward_sprite(), recipe_book_page::page_forward_highlighted_sprite())
        } else {
            mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites::with_focused(recipe_book_page::page_backward_sprite(), recipe_book_page::page_backward_highlighted_sprite())
        };
        let base = AbstractWidget::new(0, 0, recipe_book_page::TURN_PAGE_SPRITE_WIDTH, recipe_book_page::TURN_PAGE_SPRITE_HEIGHT, String::new(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::ImageButton(sprites), on_press);
        compat::shared(Self { base })
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction(&self) -> bool {
        return false;
    }

}

/// PaginationWidget's widget half (`super`'s fields: x, y, size, sprites, `onPress`, ...), lent by the game for each call
impl compat::WidgetHalf for PaginationWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// PaginationWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl mcrust::mcrust::modding::widgets::ModWidget for compat::WidgetAdapter<PaginationWidget> {
    /// java: ImageButton.extractContents - `graphics.blitSprite(RenderPipelines.GUI_TEXTURED, this.sprites.get(this.isActive(),
    /// this.isHoveredOrFocused()), this.getX(), this.getY(), this.width, this.height);` (AbstractButton's
    /// extractWidgetRenderState, then `handleCursor`)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = (mouse_x, mouse_y, a);
        let sprites = self.lend_ref(widget, |__h| compat::read(__h, |__t| match &__t.base.kind {
            mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::ImageButton(sprites) => Some(sprites.clone()),
            _ => None,
        }));
        if let Some(sprites) = sprites {
            let sprite = sprites.get(widget.is_active(), widget.is_hovered_or_focused()).clone();
            mcrust::net::minecraft::client::gui::gui_graphics_extractor::blit_sprite_identifier(context.gl, context.minecraft, mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &sprite, widget.get_x(), widget.get_y(), widget.get_width(), widget.get_height());
        }
    }

    /// java: AbstractButton.updateWidgetNarration - `this.defaultButtonNarrationText(output)`
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput) {
        widget.default_button_narration_text(output);
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    fn should_take_focus_after_interaction(&self) -> bool {
        self.call(|__h| compat::read(__h, |__t| __t.should_take_focus_after_interaction()))
    }

    /// java: AbstractButton.handleCursor
    fn handle_cursor(&self, widget: &AbstractWidget, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        widget.handle_cursor(cursors)
    }
}
