//! package de.hysky.skyblocker.compatibility.rei
pub mod info;
pub mod recipe;
pub mod skyblock_item_comparator;
pub mod skyblock_transfer_handler;
pub mod skyblocker_focused_stack_provider;

// ---- REI's side of the boundary ----
//
// REI (`me.shedaniel.rei`, Roughly Enough Items) is a separate mod that SeerClient never loads: its plugin entrypoint
// (`SkyblockerREIClientPlugin`) is never called, so the categories, displays and generators of this package are only
// reached when REI hands them its objects - which does not happen. REI's API types are the `crate::unresolved` names;
// the functions below are REI's static factories and accessors as that boundary answers them: an `EntryStack` /
// `Rectangle` that REI never built carries no value and no geometry, and every factory answers its type's stand-in.

/// `EntryStacks.of(itemStack)`.
pub(crate) fn rei_entry_stacks_of(item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> crate::unresolved::EntryStack {
    let _ = item_stack;
    crate::unresolved::EntryStack
}

/// `EntryIngredient.of(entryStack)`.
pub(crate) fn rei_entry_ingredient_of(entry_stack: crate::unresolved::EntryStack) -> crate::unresolved::EntryIngredient {
    let _ = entry_stack;
    crate::unresolved::EntryIngredient
}

/// `entryIngredient.getFirst()`.
pub(crate) fn rei_entry_ingredient_first(entry_ingredient: &crate::unresolved::EntryIngredient) -> crate::unresolved::EntryStack {
    let _ = entry_ingredient;
    crate::unresolved::EntryStack
}

/// `entryStack.getValue() instanceof ItemStack itemStack` - the stack, or `None` when the value is not an `ItemStack`.
pub(crate) fn rei_entry_stack_item(entry_stack: &crate::unresolved::EntryStack) -> Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
    let _ = entry_stack;
    None
}

/// `CategoryIdentifier.of(identifier)`.
pub(crate) fn rei_category_identifier_of(identifier: &mcrust::net::minecraft::resources::identifier::Identifier) -> crate::unresolved::CategoryIdentifier {
    let _ = identifier;
    crate::unresolved::CategoryIdentifier
}

/// An `EntryStack` as REI's `Renderer` (`EntryStack extends Renderer`).
pub(crate) fn rei_entry_stack_renderer(entry_stack: crate::unresolved::EntryStack) -> crate::unresolved::Renderer {
    let _ = entry_stack;
    crate::unresolved::Renderer
}

/// `bounds.x` / `getX()`, `bounds.y` / `getY()`, `getWidth()`, `getHeight()`, `getCenterX()`, `getCenterY()`.
pub(crate) fn rei_rectangle(bounds: &crate::unresolved::Rectangle) -> ReiRectangle {
    let _ = bounds;
    ReiRectangle::default()
}

/// The geometry of a REI `Rectangle` (see `rei_rectangle`).
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ReiRectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl ReiRectangle {
    /// `Rectangle.getCenterX()`: `x + width / 2`.
    pub fn get_center_x(&self) -> i32 {
        self.x + self.width / 2
    }

    /// `Rectangle.getCenterY()`: `y + height / 2`.
    pub fn get_center_y(&self) -> i32 {
        self.y + self.height / 2
    }
}

/// `Widgets.createRecipeBase(bounds)`.
pub(crate) fn rei_widgets_create_recipe_base(bounds: &crate::unresolved::Rectangle) -> crate::unresolved::Widget {
    let _ = bounds;
    crate::unresolved::Widget
}

/// `Widgets.createSlot(new Point(x, y))`.
pub(crate) fn rei_widgets_create_slot(x: i32, y: i32) -> crate::unresolved::Slot {
    let _ = (x, y);
    crate::unresolved::Slot
}

/// `slot.markInput()` / `.markOutput()` (`input`), `.backgroundEnabled(background)`, `.entry(entryStack)`: the slot
/// widget, as a `Widget` (`Slot extends WidgetWithBounds`).
pub(crate) fn rei_slot_widget(slot: crate::unresolved::Slot, input: Option<bool>, background: Option<bool>, entry_stack: crate::unresolved::EntryStack) -> crate::unresolved::Widget {
    let _ = (slot, input, background, entry_stack);
    crate::unresolved::Widget
}

/// `Widgets.createDrawableWidget((context, mouseX, mouseY, delta) -> ..)`.
pub(crate) fn rei_widgets_create_drawable_widget(drawable: impl Fn(&mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor, i32, i32, f32) + 'static) -> crate::unresolved::Widget {
    let _ = drawable;
    crate::unresolved::Widget
}

/// `Widgets.createArrow(new Point(x, y))`.
pub(crate) fn rei_widgets_create_arrow(x: i32, y: i32) -> crate::unresolved::Widget {
    let _ = (x, y);
    crate::unresolved::Widget
}

/// `Widgets.createLabel(new Point(x, y), text)`.
pub(crate) fn rei_widgets_create_label(x: i32, y: i32, text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent) -> crate::unresolved::Widget {
    let _ = (x, y, text);
    crate::unresolved::Widget
}

/// `Widgets.wrapVanillaWidget(child)`.
pub(crate) fn rei_widgets_wrap_vanilla_widget(child: mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> crate::unresolved::Widget {
    let _ = child;
    crate::unresolved::Widget
}
