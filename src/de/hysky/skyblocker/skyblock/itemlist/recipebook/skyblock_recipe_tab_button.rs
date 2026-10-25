//! Ported from de.hysky.skyblocker.skyblock.itemlist.recipebook.SkyblockRecipeTabButton by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: public class SkyblockRecipeTabButton extends ImageButton
pub struct SkyblockRecipeTabButton {
    /// java: extends net.minecraft.client.gui.components.ImageButton
    pub base: AbstractWidget,
    /// java: protected final ItemStack icon;
    pub icon: mcrust::net::minecraft::world::item::item_stack::ItemStack,
    /// java: private boolean selected;
    pub selected: bool,
    /// java: `ImageButton.sprites` (protected): `RecipeBookTabButton.SPRITES`
    pub sprites: mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites,
}

impl std::ops::Deref for SkyblockRecipeTabButton {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockRecipeTabButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockRecipeTabButton {
    /// java: protected SkyblockRecipeTabButton(ItemStack icon)
    pub fn new(mut icon: mcrust::net::minecraft::world::item::item_stack::ItemStack) -> compat::Shared<Self> {
        // java: super(0, 0, 35, 27, RecipeBookTabButton.SPRITES, _ -> {});
        // (`RecipeBookTabButton.SPRITES = new WidgetSprites(TAB, TAB_SELECTED)`; the press is the empty lambda)
        let sprites = mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites::with_focused(mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_tab_button::tab_sprite(), mcrust::net::minecraft::client::gui::screens::recipebook::recipe_book_tab_button::tab_selected_sprite());
        let base = AbstractWidget::new(0, 0, 35, 27, String::new(), mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::ImageButton(sprites.clone()), mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress::Inert);
        // java: this.icon = icon;
        compat::shared(Self { base, icon, selected: false, sprites })
    }

    /// java: `AbstractWidget.extractRenderState(graphics, mouseX, mouseY, delta)` (inherited): when visible, the hover
    /// state, then `AbstractButton.extractWidgetRenderState` - `extractContents` and `handleCursor`
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mouse_x: i32, mouse_y: i32, delta: f32) {
        if !compat::read(__this, |__t| __t.base.visible) {
            return;
        }
        compat::write(__this, |__t| __t.base.set_hovered_from_extract(mouse_x, mouse_y));
        SkyblockRecipeTabButton::extract_contents(__this, graphics, mouse_x, mouse_y, delta);
        // java: handleCursor(graphics) - `if (!selected) super.handleCursor(context)`: a hovered button asks for the
        // pointing hand (`isActive()`; an inactive one, NOT_ALLOWED)
        let (selected, hovered, active) = compat::read(__this, |__t| (__t.selected, __t.base.is_hovered(), __t.base.is_active()));
        if !selected && hovered {
            compat::batch_w_request_cursor(if active { compat::BatchWCursor::PointingHand } else { compat::BatchWCursor::NotAllowed });
        }
    }

    /// java: `AbstractWidget.mouseClicked(MouseButtonEvent, boolean)` (inherited): a valid click on the active, visible
    /// button plays the down sound and runs `onPress` (the empty lambda)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, click: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent, _doubled: bool) -> bool {
        let clicked = compat::read(__this, |__t| __t.base.mouse_clicked(click.button(), click.x(), click.y()).is_some());
        if clicked {
            compat::batch_w_play_down_sound();
        }
        clicked
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_contents(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        // java: if (this.sprites != null) - the constructor's sprites, never null
        {
            let mut x: i32 = __this.lock().borrow().get_x();
            if compat::read(__this, |__t| __t.selected) {
                x = x.wrapping_sub(2);
            }
            // java: graphics.blitSprite(RenderPipelines.GUI_TEXTURED, this.sprites.get(true, this.selected), x, this.getY(), this.width, this.height);
            let (sprite, y, width, height) = compat::read(__this, |__t| (__t.sprites.get(true, __t.selected).clone(), __t.get_y(), __t.width, __t.height));
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &sprite, x, y, width, height);
            let mut offset: i32 = (if compat::read(__this, |__t| __t.selected) { -2 } else { 0 });
            graphics.fake_item(&compat::read(__this, |__t| __t.icon.clone()), ((__this.lock().borrow().get_x()).wrapping_add(9)).wrapping_add(offset), (__this.lock().borrow().get_y()).wrapping_add(5));
        }
    }

    /// java: @Override protected void handleCursor(GuiGraphicsExtractor context)
    /// (the port's `handleCursor` answers the cursor the widget asks for, from the screen's cursor set)
    pub fn handle_cursor(__this: &compat::Shared<Self>, cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        // java: if (!selected) super.handleCursor(context);
        if !(compat::read(__this, |__t| __t.selected)) {
            return __this.lock().borrow().base.handle_cursor(cursors);
        }
        None
    }

    /// java: public void select()
    pub fn select(__this: &compat::Shared<Self>) {
        { let __v = true; compat::write(__this, |__t| __t.selected = __v); }
    }

    /// java: public void unselect()
    pub fn unselect(__this: &compat::Shared<Self>) {
        { let __v = false; compat::write(__this, |__t| __t.selected = __v); }
    }

}

// (the button is drawn and clicked by `SkyblockRecipeBookComponent`, its only owner: `extract_render_state` / `mouse_clicked` above)
