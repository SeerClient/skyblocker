//! Ported from de.hysky.skyblocker.utils.container.ContainerSolver by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::container::container_matcher::ContainerMatcher;
use crate::de::hysky::skyblocker::utils::render::gui::color_highlight::ColorHighlight;
use crate::de::hysky::skyblocker::utils::resettable::Resettable;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::client::gui::screens::inventory::container_screen::ContainerScreen;
use mcrust::net::minecraft::world::inventory::slot::Slot;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::ContainerScreenHost as _;

/// java: public interface ContainerSolver extends ContainerMatcher, Resettable
pub trait ContainerSolver: ContainerMatcher + Resettable {
    /// java: List<ColorHighlight> getColors(Int2ObjectMap<ItemStack> slots); (dispatch: the body for this class)
    fn get_colors(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> Vec<ColorHighlight>;

    /// java: default void start(AbstractContainerScreen<?> screen) (dispatch: the body for this class)
    fn start(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &AbstractContainerScreen) {
        <dyn ContainerSolver>::start_body
    }

    /// java: default void markDirty() (dispatch: the body for this class)
    fn mark_dirty(&self) -> fn(&compat::Shared<dyn ContainerSolver>) {
        <dyn ContainerSolver>::mark_dirty_body
    }

    /// java: default boolean isSolverSlot(Slot slot, Screen screen) (dispatch: the body for this class)
    fn is_solver_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, &Slot, &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        <dyn ContainerSolver>::is_solver_slot_body
    }

    /// java: default boolean onClickSlot(int slot, ItemStack stack, int screenId, int button) (dispatch: the body for this class)
    fn on_click_slot(&self) -> fn(&compat::Shared<dyn ContainerSolver>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack, i32, i32) -> bool {
        <dyn ContainerSolver>::on_click_slot_body
    }

    /// java: default boolean chestScreensOnly() (dispatch: the body for this class)
    fn chest_screens_only(&self) -> fn(&compat::Shared<dyn ContainerSolver>) -> bool {
        <dyn ContainerSolver>::chest_screens_only_body
    }

    /// java: default boolean chestInventoryOnly() (dispatch: the body for this class)
    fn chest_inventory_only(&self) -> fn(&compat::Shared<dyn ContainerSolver>) -> bool {
        <dyn ContainerSolver>::chest_inventory_only_body
    }

}

impl dyn ContainerSolver {
    /// java: default void start(AbstractContainerScreen<?> screen)
    pub fn start_body(__this: &compat::Shared<dyn ContainerSolver>, screen: &AbstractContainerScreen) {
    }

    /// java: @Override default void reset()
    pub fn reset_body(__this: &compat::Shared<dyn ContainerSolver>) {
    }

    /// ContainerSolver.reset as the dispatch of Resettable.reset for the concrete class `__C`
    pub fn reset_from<__C: ContainerSolver + 'static>(__this: &compat::Shared<dyn Resettable>) {
        <dyn ContainerSolver>::reset_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ContainerSolver>))
    }

    /// java: default void markDirty()
    pub fn mark_dirty_body(__this: &compat::Shared<dyn ContainerSolver>) {
    }

    /// java: default boolean isSolverSlot(Slot slot, Screen screen)
    pub fn is_solver_slot_body(__this: &compat::Shared<dyn ContainerSolver>, slot: &Slot, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> bool {
        if !({ let __f = compat::dispatch(__this, |__t| __t.chest_inventory_only()); __f(__this) }) {
            return true;
        }
        // java: if (screen instanceof ContainerScreen generic) return slot.index < generic.getMenu().getRowCount() * 9;
        // The port's `Screen` is the husk; a `ContainerScreen`'s menu is the player's open `ChestMenu` (MenuScreens
        // opens a ContainerScreen for exactly the chest menus), which `with_chest_menu` reaches.
        if let Some(row_count) = mcrust::mcrust::modding::game::with_chest_menu(|chest_menu| chest_menu.get_row_count()) {
            return slot.index < row_count.wrapping_mul(9);
        }
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        // TODO(fabric2seer): the mod keeps the game's `Inventory` (Java holds a reference to the live object); the port's `Inventory` is not a value to copy and the port lends the mod no handle to it
        // rust: return !std::ptr::eq(&slot.container, &mcrust::mcrust::modding::game::with_player(|player| player.base.base.get_inventory().clone()).unwrap());
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: default boolean onClickSlot(int slot, ItemStack stack, int screenId, int button)
    pub fn on_click_slot_body(__this: &compat::Shared<dyn ContainerSolver>, mut slot: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut screen_id: i32, mut button: i32) -> bool {
        return false;
    }

    /// java: default boolean chestScreensOnly()
    pub fn chest_screens_only_body(__this: &compat::Shared<dyn ContainerSolver>) -> bool {
        return true;
    }

    /// java: default boolean chestInventoryOnly()
    pub fn chest_inventory_only_body(__this: &compat::Shared<dyn ContainerSolver>) -> bool {
        return true;
    }

}

/// java: static void trimEdges(Int2ObjectMap<ItemStack> slots, int rows)
pub fn trim_edges(slots: &mut std::collections::HashMap<i32, mcrust::net::minecraft::world::item::item_stack::ItemStack>, mut rows: i32) {
    for i in 0..rows {
        slots.remove(&9i32.wrapping_mul(i));
        slots.remove(&(9i32.wrapping_mul(i)).wrapping_add(8));
    }
    for i in 1..8 {
        slots.remove(&i);
        slots.remove(&((rows.wrapping_sub(1)).wrapping_mul(9)).wrapping_add(i));
    }
}
