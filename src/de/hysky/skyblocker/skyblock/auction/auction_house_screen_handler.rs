//! Ported from de.hysky.skyblocker.skyblock.auction.AuctionHouseScreenHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::screens::menu_screens::MenuType;
use mcrust::net::minecraft::world::container::Container;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: public class AuctionHouseScreenHandler extends ChestMenu
pub struct AuctionHouseScreenHandler {
    /// java: extends net.minecraft.world.inventory.ChestMenu
    pub base: AbstractContainerMenu,
}

impl std::ops::Deref for AuctionHouseScreenHandler {
    type Target = AbstractContainerMenu;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AuctionHouseScreenHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AuctionHouseScreenHandler {
    /// java: public AuctionHouseScreenHandler(MenuType<?> type, int syncId, Inventory playerInventory, Container inventory, int rows, boolean isView)
    pub fn new(r#type: &MenuType, mut sync_id: i32, player_inventory: compat::PlayerInventory, inventory: &dyn Container, mut rows: i32, mut is_view: bool) -> compat::Shared<Self> {
        let mut y_offset: i32 = (rows.wrapping_sub(4)).wrapping_mul(18);
        let __this = compat::shared(Self {
            base: AbstractContainerMenu::new(0),
        });
        // java: the constructor's body - run on the server's menu when the game attaches this object
        compat::defer_menu_init(&__this, move |__this_ref| {
            let __this = __this_ref.clone();
            for i in rows.wrapping_mul(9)..(__this.lock().borrow().slots.len() as i32) {
                let slot = &__this.lock().borrow().slots[(i) as usize];
                let mut slot_accessor: crate::unresolved::SlotAccessor = slot;
                // TODO(fabric2seer): `.setY(..)` on a SlotAccessor
                //   java: slotAccessor.setY(slot.y + 2 - yOffset);
            }
            for i in 0..rows.wrapping_mul(9) {
                let mut line_i: i32 = i.wrapping_rem(9);
                let slot = &__this.lock().borrow().slots[(i) as usize];
                if !is_view && i > 9 && i < (rows.wrapping_sub(1)).wrapping_mul(9) && line_i > 1 && line_i < 8 {
                    let mut mini_inventory_slot: i32 = (line_i.wrapping_sub(2)).wrapping_add(((i.wrapping_div(9)).wrapping_sub(1)).wrapping_mul(6));
                    let mut slot_accessor: crate::unresolved::SlotAccessor = slot;
                    // TODO(fabric2seer): `.setX(..)` on a SlotAccessor
                    //   java: slotAccessor.setX(8 + miniInventorySlot % 8 * 18);
                    // TODO(fabric2seer): `.setY(..)` on a SlotAccessor
                    //   java: slotAccessor.setY(18 + miniInventorySlot / 8 * 18);
                } else {
                    __this.lock().borrow_mut().slots[(i) as usize] = Slot::new(slot.container.clone(), slot.get_container_slot(), slot.x, slot.y).with_mod_slot(std::sync::Arc::new(compat::SlotFns { is_active: Some(Box::new(move |__slot: &Slot, __super: bool| {
                    return false;
                })), ..Default::default() }));
                }
            }
        });
        __this
    }

    /// java: public static AuctionHouseScreenHandler of(ChestMenu original, boolean isView)
    pub fn of(original: &ChestMenu, mut is_view: bool) -> compat::Shared<AuctionHouseScreenHandler> {
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        return AuctionHouseScreenHandler::new(&original.get_type(), original.menu.container_id, compat::PlayerInventory, original.get_container(), original.get_row_count(), is_view);
    }

    /// java: @Override public void setItem(int slot, int revision, ItemStack stack)
    pub fn set_item(__this: &compat::Shared<Self>, mut slot: i32, mut revision: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        // TODO(fabric2seer): `set_item` takes 4 argument(s) in the port (`inventory: &mut Inventory, slot: i32, state_id: i32, item_stack: ItemStack`), Java passes 3
        // rust: { let (__q0, __q1, __q2,) = (&mut slot, revision, stack.clone(),); __this.lock().borrow_mut().base.set_item(__q0, __q1, __q2) };
    }

}

/// AuctionHouseScreenHandler's menu half: the server's menu, lent while its constructor's body runs
impl compat::MenuHalf for AuctionHouseScreenHandler {
    fn menu_half(&mut self) -> &mut AbstractContainerMenu {
        &mut self.base
    }
}

/// AuctionHouseScreenHandler's overrides of the menu's own answers (`ModMenu`)
impl compat::MenuOverrides for AuctionHouseScreenHandler {
}

// NOTE(fabric2seer): setItem override(s) of ChestMenu - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
