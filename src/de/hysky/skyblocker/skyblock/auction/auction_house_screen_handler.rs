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
    /// java: ChestMenu.containerRows (the superclass's field, answered by getRowCount())
    pub container_rows: i32,
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
            container_rows: rows,
        });
        // java: the constructor's body - run on the server's menu when the game attaches this object
        compat::defer_menu_init(&__this, move |__this_ref| {
            let __this = __this_ref.clone();
            // Shift player inventory by 2 pixels and also remove the yOffset
            let slot_count: i32 = compat::read(&__this, |__t| __t.slots.len() as i32);
            for i in rows.wrapping_mul(9)..slot_count {
                // java: SlotAccessor slotAccessor = (SlotAccessor) slot; slotAccessor.setY(slot.y + 2 - yOffset);
                compat::write(&__this, |__t| {
                    let slot = &mut __t.slots[i as usize];
                    slot.y = slot.y.wrapping_add(2).wrapping_sub(y_offset);
                });
            }
            // disable ALL THE OTHER SLOTS MWAHAHAHA and also move the good ones around and stuff
            for i in 0..rows.wrapping_mul(9) {
                let mut line_i: i32 = i.wrapping_rem(9);
                if !is_view && i > 9 && i < (rows.wrapping_sub(1)).wrapping_mul(9) && line_i > 1 && line_i < 8 {
                    let mut mini_inventory_slot: i32 = (line_i.wrapping_sub(2)).wrapping_add(((i.wrapping_div(9)).wrapping_sub(1)).wrapping_mul(6));
                    // java: slotAccessor.setX(8 + miniInventorySlot % 8 * 18); slotAccessor.setY(18 + miniInventorySlot / 8 * 18);
                    compat::write(&__this, |__t| {
                        let slot = &mut __t.slots[i as usize];
                        slot.x = 8i32.wrapping_add(mini_inventory_slot.wrapping_rem(8).wrapping_mul(18));
                        slot.y = 18i32.wrapping_add(mini_inventory_slot.wrapping_div(8).wrapping_mul(18));
                    });
                } else {
                    // java: slots.set(i, new Slot(slot.container, slot.getContainerSlot(), slot.x, slot.y) { isActive() -> false });
                    compat::write(&__this, |__t| {
                        let slot = &__t.slots[i as usize];
                        let replacement = Slot::new(slot.container.clone(), slot.get_container_slot(), slot.x, slot.y).with_mod_slot(std::sync::Arc::new(compat::SlotFns {
                            is_active: Some(Box::new(move |__slot: &Slot, __super: bool| {
                                return false;
                            })),
                            ..Default::default()
                        }));
                        __t.slots[i as usize] = replacement;
                    });
                }
            }
        });
        __this
    }

    /// java: ChestMenu.getRowCount() (inherited)
    pub fn get_row_count(&self) -> i32 {
        self.container_rows
    }

    /// java: public static AuctionHouseScreenHandler of(ChestMenu original, boolean isView)
    pub fn of(original: &ChestMenu, mut is_view: bool) -> compat::Shared<AuctionHouseScreenHandler> {
        debug_assert!(mcrust::mcrust::modding::game::with_player(|_| ()).is_some());
        return AuctionHouseScreenHandler::new(&original.get_type(), original.menu.container_id, compat::PlayerInventory, original.get_container(), original.get_row_count(), is_view);
    }

    /// java: @Override public void setItem(int slot, int revision, ItemStack stack)
    pub fn set_item(__this: &compat::Shared<Self>, mut slot: i32, mut revision: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        //   java: super.setItem(slot, revision, stack);
        // The override's whole body is the super call: `MenuOverrides` keeps the menu's own `setItem` for this class, so
        // the game's body runs (this function, which nothing reaches, is the override's place).
        let _ = (__this, slot, revision, stack);
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

