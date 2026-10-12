//! Ported from de.hysky.skyblocker.utils.render.gui.ItemButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::button::CreateNarration;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ItemButtonWidget extends Button
pub struct ItemButtonWidget {
    /// java: extends net.minecraft.client.gui.components.Button
    pub base: AbstractWidget,
    /// java: private final ItemStack item;
    pub item: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack>,
}

impl std::ops::Deref for ItemButtonWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemButtonWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemButtonWidget {
    /// java: public ItemButtonWidget(int x, int y, int width, int height, ItemStack item, Component message, OnPress onPress, CreateNarration narrationSupplier)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut message: MutableComponent, mut on_press: OnPress, mut narration_supplier: CreateNarration) -> Self {
        let __init_item = Some(item.copy());
        // TODO(fabric2seer): `new Button(..)`: the port has no `Button::new`
        //   java: super(x, y, width, height, message, onPress, narrationSupplier);
        let mut this: Self = todo!("constructor");
        this.set_tooltip(Tooltip::create(message.get_string()));
        // TODO(fabric2seer): `Button.setTooltipDelay` has no Rust method in the port (looked for `set_tooltip_delay` in Button and its supertypes)
        // rust: this.set_tooltip_delay(&std::time::Duration::from_millis((250) as u64));
        this
    }

    /// java: public ItemButtonWidget(int x, int y, int width, int height, ItemStack item, Component message, OnPress onPress)
    pub fn new_7(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut message: MutableComponent, mut on_press: OnPress) -> Self {
        let mut this = Self::new(x, y, width, height, item.clone(), message.clone(), on_press, CreateNarration::Default);
        this
    }

    /// java: public ItemButtonWidget(int x, int y, ItemStack item, Component message, OnPress onPress)
    pub fn new_5(mut x: i32, mut y: i32, mut item: mcrust::net::minecraft::world::item::item_stack::ItemStack, mut message: MutableComponent, mut on_press: OnPress) -> Self {
        let mut this = Self::new(x, y, 20, 20, item.clone(), message.clone(), on_press, CreateNarration::Default);
        this
    }

    /// java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float delta)
    pub fn extract_contents(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut delta: f32) {
        // TODO(fabric2seer): `Button.extractDefaultSprite` has no Rust method in the port (looked for `extract_default_sprite` in Button and its supertypes)
        // rust: self.extract_default_sprite(graphics);
        graphics.item(self.item.as_ref().unwrap(), (self.get_x().wrapping_add(self.get_width().wrapping_div(2))).wrapping_sub(8), (self.get_y().wrapping_add(self.get_height().wrapping_div(2))).wrapping_sub(8));
    }

}

// NOTE(fabric2seer): extractContents override(s) of Button - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
