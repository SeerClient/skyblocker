//! Ported from de.hysky.skyblocker.skyblock.item.SkyblockCraftingTableScreenHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::area::TheRift;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::mcrust::modding::container_menus::ModSlot;
use mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType;
use mcrust::net::minecraft::world::container::Container;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: private static final int[] normalSlots = new int[]{ 10, 11, 12, 16, 19, 20, 21, 23, 25, 28, 29, 30, 34 };
pub static NORMAL_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![10, 11, 12, 16, 19, 20, 21, 23, 25, 28, 29, 30, 34]);
/// java: private static final int[] riftNormalSlots = new int[]{ 11, 12, 13, 20, 21, 22, 24, 29, 30, 31 };
pub static RIFT_NORMAL_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![11, 12, 13, 20, 21, 22, 24, 29, 30, 31]);

/// java: public class SkyblockCraftingTableScreenHandler extends ChestMenu
pub struct SkyblockCraftingTableScreenHandler {
    /// java: extends net.minecraft.world.inventory.ChestMenu
    pub base: AbstractContainerMenu,
    /// java: public final boolean mirrorverse;
    pub mirrorverse: bool,
}

impl std::ops::Deref for SkyblockCraftingTableScreenHandler {
    type Target = AbstractContainerMenu;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockCraftingTableScreenHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockCraftingTableScreenHandler {
    /// java: public SkyblockCraftingTableScreenHandler(MenuType<?> type, int syncId, Inventory playerInventory, Container inventory, int rows)
    pub fn new(r#type: &MenuType, mut sync_id: i32, player_inventory: compat::PlayerInventory, inventory: &dyn Container, mut rows: i32) -> compat::Shared<Self> {
        let __init_mirrorverse = compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<TheRift>().is_some_and(|__e| *__e == TheRift::Mirrorverse));
        let __this = compat::shared(Self {
            base: AbstractContainerMenu::new(0),
            mirrorverse: __init_mirrorverse,
        });
        // java: the constructor's body - run on the server's menu when the game attaches this object
        compat::defer_menu_init(&__this, move |__this_ref| {
            let __this = __this_ref.clone();
            let mut active_slots: Vec<i32> = (if compat::read(&__this, |__t| __t.mirrorverse) { RIFT_NORMAL_SLOTS.clone() } else { NORMAL_SLOTS.clone() });
            for i in 0..rows.wrapping_mul(9) {
                let original_slot: Slot = compat::read(&__this, |__t| __t.slots[(i) as usize].clone());
                // Arrays.binarySearch(activeSlots, i) >= 0 (the slot lists are sorted)
                if active_slots.binary_search(&i).is_ok() {
                    let mut coords: Vec<i32> = SkyblockCraftingTableScreenHandler::get_coords(&__this, i);
                    let mut slot: Slot = Slot::new(original_slot.container.clone(), original_slot.get_container_slot(), coords[(0) as usize], coords[(1) as usize]);
                    slot.index = i;
                    compat::write(&__this, |__t| __t.slots[(i) as usize] = slot.clone());
                } else {
                    let mut slot: compat::Shared<DisabledSlot> = DisabledSlot::new(original_slot.container.clone(), original_slot.get_container_slot(), -20, -20);
                    { let __v = i; compat::write(&slot, |__t| __t.index = __v); }
                    let __s = compat::slot_with_class(slot.clone());
                    compat::write(&__this, |__t| __t.slots[(i) as usize] = __s);
                }
            }
            let mut y_offset: i32 = ((rows.wrapping_sub(4)).wrapping_mul(18)).wrapping_add(19);
            for i in rows.wrapping_mul(9)..(compat::read(&__this, |__t| __t.slots.len()) as i32) {
                let original_slot: Slot = compat::read(&__this, |__t| __t.slots[(i) as usize].clone());
                let mut slot: Slot = Slot::new(original_slot.container.clone(), original_slot.get_container_slot(), original_slot.x, original_slot.y.wrapping_sub(y_offset));
                slot.index = i;
                compat::write(&__this, |__t| __t.slots[(i) as usize] = slot.clone());
            }
        });
        __this
    }

    /// java: public SkyblockCraftingTableScreenHandler(ChestMenu handler, Inventory playerInventory)
    pub fn new_2(handler: &ChestMenu, player_inventory: compat::PlayerInventory) -> compat::Shared<Self> {
        let __this = Self::new(&handler.get_type(), handler.menu.container_id, compat::PlayerInventory, handler.get_container(), handler.get_row_count());
        __this
    }

    /// java: private int[] getCoords(int slot)
    pub fn get_coords(__this: &compat::Shared<Self>, mut slot: i32) -> Vec<i32> {
        if compat::read(__this, |__t| __t.mirrorverse) {
            if slot == 24 {
                return vec![124, 35];
            }
            let mut grid_x: i32 = (slot.wrapping_rem(9)).wrapping_sub(2);
            let mut grid_y: i32 = (slot.wrapping_div(9)).wrapping_sub(1);
            return vec![30i32.wrapping_add(grid_x.wrapping_mul(18)), 17i32.wrapping_add(grid_y.wrapping_mul(18))];
        } else {
            if slot == 23 {
                return vec![124, 35];
            }
            if slot == 16 || slot == 25 || slot == 34 {
                let mut y: i32 = (((slot.wrapping_div(9)).wrapping_sub(1)).wrapping_mul(18)).wrapping_add(8);
                return vec![152, y];
            }
            let mut grid_x: i32 = (slot.wrapping_rem(9)).wrapping_sub(1);
            let mut grid_y: i32 = (slot.wrapping_div(9)).wrapping_sub(1);
            return vec![30i32.wrapping_add(grid_x.wrapping_mul(18)), 17i32.wrapping_add(grid_y.wrapping_mul(18))];
        }
    }

}

/// SkyblockCraftingTableScreenHandler's menu half: the server's menu, lent while its constructor's body runs
impl compat::MenuHalf for SkyblockCraftingTableScreenHandler {
    fn menu_half(&mut self) -> &mut AbstractContainerMenu {
        &mut self.base
    }
}

/// SkyblockCraftingTableScreenHandler's overrides of the menu's own answers (`ModMenu`)
impl compat::MenuOverrides for SkyblockCraftingTableScreenHandler {
}

/// java: public static class DisabledSlot extends Slot
#[derive(Clone)]
pub struct DisabledSlot {
    /// java: extends net.minecraft.world.inventory.Slot
    pub base: Slot,
}

impl std::ops::Deref for DisabledSlot {
    type Target = Slot;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DisabledSlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DisabledSlot {
    /// java: public DisabledSlot(Container inventory, int index, int x, int y)
    /// `inventory` is the slot's container tag (the port's `Slot` names its container).
    pub fn new(mut inventory: mcrust::net::minecraft::world::inventory::slot::SlotContainer, mut index: i32, mut x: i32, mut y: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: Slot::new(inventory, index, x, y),
        });
        __this
    }

    /// java: @Override public boolean isActive()
    pub fn is_active(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

}

/// DisabledSlot's slot half (`super`'s fields), the slot's state when the port asks the class
impl compat::SlotHalf for DisabledSlot {
    fn slot_half(&mut self) -> &mut Slot {
        &mut self.base
    }
}

/// DisabledSlot's overrides, asked by the port's slot (`compat::slot_with_class` attaches them)
impl ModSlot for compat::SlotClass<DisabledSlot> {
    /// java: @Override public boolean isActive()
    fn is_active(&self, slot: &Slot, super_answer: bool) -> bool {
        self.ask(slot, super_answer, |__h| DisabledSlot::is_active(__h))
    }

}

// NOTE(fabric2seer): isActive override(s) of Slot - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
