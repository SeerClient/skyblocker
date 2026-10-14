//! Ported from de.hysky.skyblocker.skyblock.storageoverlay.StorageOverlayScreenHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::container_menus::ModSlot;
use mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType;
use mcrust::net::minecraft::world::container::Container;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: public class StorageOverlayScreenHandler extends ChestMenu
pub struct StorageOverlayScreenHandler {
    /// java: extends net.minecraft.world.inventory.ChestMenu
    pub base: AbstractContainerMenu,
    /// java: private final int rows;
    pub rows: i32,
}

impl std::ops::Deref for StorageOverlayScreenHandler {
    type Target = AbstractContainerMenu;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StorageOverlayScreenHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StorageOverlayScreenHandler {
    /// java: public StorageOverlayScreenHandler(MenuType<?> type, int syncId, Inventory playerInventory, Container inventory, int rows, boolean isBackpack, int height)
    pub fn new(r#type: &MenuType, mut sync_id: i32, player_inventory: compat::PlayerInventory, inventory: &dyn Container, mut rows: i32, mut is_backpack: bool, mut height: i32) -> compat::Shared<Self> {
        let __init_rows = rows;
        let mut y_offset: i32 = rows.wrapping_mul(18);
        let __this = compat::shared(Self {
            base: AbstractContainerMenu::new(0),
            rows: __init_rows,
        });
        // java: the constructor's body - run on the server's menu when the game attaches this object
        compat::defer_menu_init(&__this, move |__this_ref| {
            let __this = __this_ref.clone();
            StorageOverlayScreenHandler::shift_inventory(&__this, (height.wrapping_sub(y_offset)).wrapping_sub(113));
            if !is_backpack {
                for i in 0..rows.wrapping_mul(9) {
                    let slot: Slot = compat::read(&__this, |__t| __t.slots[(i) as usize].clone());
                    __this.lock().borrow_mut().slots[(i) as usize] = compat::slot_with_class(InactiveSlot::new(slot.container, slot.get_container_slot(), slot.x, slot.y));
                }
            } else {
                for i in 0..9 {
                    let slot: Slot = compat::read(&__this, |__t| __t.slots[(i) as usize].clone());
                    __this.lock().borrow_mut().slots[(i) as usize] = compat::slot_with_class(InactiveSlot::new(slot.container, slot.get_container_slot(), slot.x, slot.y));
                }
                StorageOverlayScreenHandler::update_backpack_slots(&__this, height);
            }
        });
        __this
    }

    /// java: public StorageOverlayScreenHandler(ChestMenu handler, boolean isBackpack, int height, Inventory playerInventory)
    pub fn new_4(handler: &ChestMenu, mut is_backpack: bool, mut height: i32, player_inventory: compat::PlayerInventory) -> compat::Shared<Self> {
        let __this = Self::new(&handler.get_type(), handler.menu.container_id, compat::PlayerInventory, handler.get_container(), handler.get_row_count(), is_backpack, height);
        __this
    }

    /// java: public void shiftInventory(int shift)
    pub fn shift_inventory(__this: &compat::Shared<Self>, mut shift: i32) {
        for i in { (compat::read(__this, |__t| __t.rows)).wrapping_mul(9)..(__this.lock().borrow().slots.len() as i32) } {
            let original_slot: Slot = compat::read(__this, |__t| __t.slots[(i) as usize].clone());
            let mut slot: Slot = Slot::new(original_slot.container, original_slot.get_container_slot(), original_slot.x, original_slot.y.wrapping_add(shift));
            slot.index = i;
            __this.lock().borrow_mut().slots[(i) as usize] = slot.clone();
        }
    }

    /// java: public void updateBackpackSlots(int height)
    pub fn update_backpack_slots(__this: &compat::Shared<Self>, mut height: i32) {
        for i in 9..(compat::read(__this, |__t| __t.rows)).wrapping_mul(9) {
            let original_slot: Slot = compat::read(__this, |__t| __t.slots[(i) as usize].clone());
            let mut slot: Slot = Slot::new(original_slot.container, original_slot.get_container_slot(), 0, -20).with_mod_slot(std::sync::Arc::new(compat::SlotFns { is_active: Some(Box::new(move |__slot: &Slot, __super: bool| {
                return (__slot.y > 8 && __slot.y < height.wrapping_sub(94));
            })), is_highlightable: Some(Box::new(move |__slot: &Slot, __super: bool| {
                return (__slot.y > 8 && __slot.y < height.wrapping_sub(94));
            })), ..Default::default() }));
            slot.index = i;
            __this.lock().borrow_mut().slots[(i) as usize] = slot.clone();
        }
    }

    /// java: public void moveBackpackSlots(int x, int y, int columns)
    pub fn move_backpack_slots(__this: &compat::Shared<Self>, mut x: i32, mut y: i32, mut columns: i32) {
        for i in 9..(compat::read(__this, |__t| __t.rows)).wrapping_mul(9) {
            let mut item_x: i32 = x.wrapping_add(((i.wrapping_sub(9)).wrapping_rem(columns)).wrapping_mul(18));
            let mut item_y: i32 = y.wrapping_add(((i.wrapping_sub(9)).wrapping_div(columns)).wrapping_mul(18));
            // java: Slot slot = this.slots.get(i); SlotAccessor slotAccessor = (SlotAccessor) slot; slotAccessor.setX(itemX); slotAccessor.setY(itemY);
            // (`this.slots` is the menu this object rides on: the player's open one, where the screen draws and
            // hit-tests the slots)
            let _ = mcrust::mcrust::modding::game::with_abstract_container_menu(|__menu| {
                if let Some(slot) = __menu.slots.get_mut((i) as usize) {
                    slot.x = item_x;
                    slot.y = item_y;
                }
            });
        }
    }

}

/// StorageOverlayScreenHandler's menu half: the server's menu, lent while its constructor's body runs
impl compat::MenuHalf for StorageOverlayScreenHandler {
    fn menu_half(&mut self) -> &mut AbstractContainerMenu {
        &mut self.base
    }
}

/// StorageOverlayScreenHandler's overrides of the menu's own answers (`ModMenu`)
impl compat::MenuOverrides for StorageOverlayScreenHandler {
}

/// java: private static class InactiveSlot extends Slot
#[derive(Clone)]
pub struct InactiveSlot {
    /// java: extends net.minecraft.world.inventory.Slot
    pub base: Slot,
}

impl std::ops::Deref for InactiveSlot {
    type Target = Slot;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for InactiveSlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl InactiveSlot {
    /// java: InactiveSlot(Container container, int slot, int x, int y)
    /// (`container` is the port's tag of the container the slot reads)
    pub fn new(mut container: mcrust::net::minecraft::world::inventory::slot::SlotContainer, mut slot: i32, mut x: i32, mut y: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: Slot::new(container, slot, x, y),
        });
        __this
    }

    /// java: @Override public boolean isActive()
    pub fn is_active(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

}

/// InactiveSlot's slot half (`super`'s fields), the slot's state when the port asks the class
impl compat::SlotHalf for InactiveSlot {
    fn slot_half(&mut self) -> &mut Slot {
        &mut self.base
    }
}

/// InactiveSlot's overrides, asked by the port's slot (`compat::slot_with_class` attaches them)
impl ModSlot for compat::SlotClass<InactiveSlot> {
    /// java: @Override public boolean isActive()
    fn is_active(&self, slot: &Slot, super_answer: bool) -> bool {
        self.ask(slot, super_answer, |__h| InactiveSlot::is_active(__h))
    }

}

// NOTE(fabric2seer): isActive override(s) of Slot - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
