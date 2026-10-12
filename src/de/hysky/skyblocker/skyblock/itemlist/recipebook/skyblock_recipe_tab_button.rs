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
        let __init_icon = icon.clone();
        // TODO(fabric2seer): static `RecipeBookTabButton.SPRITES` has no Rust counterpart in the port's RecipeBookTabButton file
        //   java: super(0, 0, 35, 27, RecipeBookTabButton.SPRITES, _ -> {});
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: @Override public void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_contents(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        // TODO(fabric2seer): `ImageButton.sprites` has no Rust field in the port (searched net.minecraft.client.gui.components.ImageButton and its supertypes)
        if true /* java: compat::read(__this, |__t| __t.sprites.clone()) != null: never null here */ {
            let mut x: i32 = __this.lock().borrow().get_x();
            if compat::read(__this, |__t| __t.selected) {
                x = x.wrapping_sub(2);
            }
            // TODO(fabric2seer): `ImageButton.sprites` has no Rust field in the port (searched net.minecraft.client.gui.components.ImageButton and its supertypes)
            // rust: gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), { let (__q0, __q1,) = (true, compat::read(__this, |__t| __t.selected),); __this.lock().borrow().sprites.get(__q0, __q1) }, x, __this.lock().borrow().get_y(), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
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

// NOTE(fabric2seer): extractContents, handleCursor override(s) of ImageButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
