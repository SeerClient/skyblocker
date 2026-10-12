//! Ported from de.hysky.skyblocker.utils.render.gui.AbstractCustomHypixelGUI by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::auction::auction_house_screen_handler::AuctionHouseScreenHandler;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::player::inventory::Inventory;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public abstract class AbstractCustomHypixelGUI<T extends AbstractContainerMenu> extends AbstractContainerScreen<T> implements ContainerListener
pub struct AbstractCustomHypixelGUIBase<T> {
    /// java: extends net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
    pub base: AbstractContainerScreen,
    /// java: public boolean isWaitingForServer = true;
    pub is_waiting_for_server: bool,
}

impl<T: 'static> std::ops::Deref for AbstractCustomHypixelGUIBase<T> {
    type Target = AbstractContainerScreen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for AbstractCustomHypixelGUIBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> AbstractCustomHypixelGUIBase<T> {
    /// java: public AbstractCustomHypixelGUI(T handler, Inventory inventory, Component title, int imageHeight)
    pub fn new(mut handler: T, inventory: compat::PlayerInventory, mut title: MutableComponent, mut image_height: i32) -> Self {
        let mut this = Self {
            base: { let mut __s = AbstractContainerScreen::new((&title.get_string()).to_string(), String::new()); __s.image_width = 176; __s.image_height = image_height; __s },
            is_waiting_for_server: true,
        };
        this.inventory_label_y = image_height.wrapping_sub(94);
        // TODO(fabric2seer): `.addSlotListener(..)` on a T
        //   java: handler.addSlotListener(this);
        this
    }

}

/// java: the virtual methods of AbstractCustomHypixelGUI (implemented by AbstractCustomHypixelGUIBase and every subclass)
pub trait AbstractCustomHypixelGUI<T> {
    fn abstract_custom_hypixel_gui_base(&self) -> &AbstractCustomHypixelGUIBase<T>;
    fn abstract_custom_hypixel_gui_base_mut(&mut self) -> &mut AbstractCustomHypixelGUIBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected void clickSlot(int slotID, int button)
    fn click_slot(&mut self, mut slot_id: i32, mut button: i32) {
        if self.abstract_custom_hypixel_gui_base().is_waiting_for_server {
            return;
        }
        if false /* java: compat::minecraft().game_mode == null: never null here */ || mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        // TODO(fabric2seer): field `containerId` of a T
        //   java: this.minecraft.gameMode.handleContainerInput(menu.containerId, slotID, ContainerUtils.getContainerClickButton(button), ContainerInput.PICKUP, minecraft.player);
        // TODO(fabric2seer): `.getCarried(..)` on a T
        //   java: menu.getCarried().setCount(0);
        self.abstract_custom_hypixel_gui_base_mut().is_waiting_for_server = true;
    }

    /// java: protected void clickSlot(int slotID)
    fn click_slot_1(&mut self, mut slot_id: i32) {
        self.click_slot(slot_id, 0);
    }

    /// java: public void changeHandler(AuctionHouseScreenHandler newHandler)
    fn change_handler(&self, new_handler: &compat::Shared<AuctionHouseScreenHandler>) {
        // TODO(fabric2seer): `.removeSlotListener(..)` on a T
        //   java: menu.removeSlotListener(this);
        self.menu = new_handler;
        // TODO(fabric2seer): `.addSlotListener(..)` on a T
        //   java: menu.addSlotListener(this);
    }

    /// java: protected void refreshListener()
    fn refresh_listener(&self) {
        // TODO(fabric2seer): `.removeSlotListener(..)` on a T
        //   java: menu.removeSlotListener(this);
        // TODO(fabric2seer): `.addSlotListener(..)` on a T
        //   java: menu.addSlotListener(this);
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&self) {
        // TODO(fabric2seer): `reposition_elements` takes 1 argument(s) in the port (`build: impl FnOnce(&mut Screen)`), Java passes 0
        // rust: self.abstract_custom_hypixel_gui_base().screen.reposition_elements();
        self.refresh_listener();
        // TODO(fabric2seer): `.getItems(..)` on a T
        //   java: for (int i = 0; i < menu.getItems().size(); i++) {
        //   java: 			onSlotChange(menu, i, menu.getItems().get(i));
        //   java: 		}
    }

    /// java: @Override public void removed()
    fn removed(&self) {
        ();
        // TODO(fabric2seer): `.removeSlotListener(..)` on a T
        //   java: menu.removeSlotListener(this);
    }

    /// java: @Override public final void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack)
    fn slot_changed(&mut self, handler: &AbstractContainerMenu, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
        // TODO(fabric2seer): `AbstractContainerScreen.menu` has no Rust field in the port (searched net.minecraft.client.gui.screens.inventory.AbstractContainerScreen and its supertypes)
        // rust: { let __s0 = (self.menu).clone(); self.on_slot_change(&__s0, slot_id, stack) };
        self.abstract_custom_hypixel_gui_base_mut().is_waiting_for_server = false;
    }

    /// java: protected abstract void onSlotChange(T handler, int slotID, ItemStack stack);
    fn on_slot_change(&mut self, handler: &T, slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack);

    /// java: @Override public void dataChanged(AbstractContainerMenu handler, int property, int value)
    fn data_changed(&self, handler: &AbstractContainerMenu, mut property: i32, mut value: i32) {
    }

}
