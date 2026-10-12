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
    pub fn new(mut forwards: bool, mut on_press: OnPress) -> Self {
        // TODO(fabric2seer): static `RecipeBookPage.PAGE_FORWARD_SPRITES` has no Rust counterpart in the port's RecipeBookPage file
        //   java: super(0, 0, RecipeBookPage.TURN_PAGE_SPRITE_WIDTH, RecipeBookPage.TURN_PAGE_SPRITE_HEIGHT, forwards ? RecipeBookPage.PAGE_FORWARD_SPRITES : RecipeBookPage.PAGE_BACKWARD_SPRITES, onPress);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public boolean shouldTakeFocusAfterInteraction()
    pub fn should_take_focus_after_interaction(&self) -> bool {
        return false;
    }

}

// NOTE(fabric2seer): shouldTakeFocusAfterInteraction override(s) of ImageButton - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
