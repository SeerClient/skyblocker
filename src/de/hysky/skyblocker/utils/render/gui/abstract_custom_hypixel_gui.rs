//! Ported from de.hysky.skyblocker.utils.render.gui.AbstractCustomHypixelGUI by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
use mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu;
use mcrust::net::minecraft::world::inventory::container_input::ContainerInput;
use mcrust::net::minecraft::world::item::item_stack::ItemStack;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public abstract class AbstractCustomHypixelGUI<T extends AbstractContainerMenu> extends AbstractContainerScreen<T> implements ContainerListener
///
/// `implements ContainerListener`: the menu holds a `compat::QueueSlotListener` (see `listen`), and the screen runs
/// `slotChanged` for each queued call in its next own call (`drain_slot_changes`) - the menu calls its listeners
/// inside the packet handler's hold of the player, where the screen object cannot be reached.
pub struct AbstractCustomHypixelGUIBase<T> {
    /// java: extends net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
    pub base: AbstractContainerScreen,
    /// java: public boolean isWaitingForServer = true;
    pub is_waiting_for_server: bool,
    /// java: AbstractContainerScreen.menu - the handler this screen was made for (`changeHandler` swaps it)
    pub menu: T,
    /// java: menu.containerId - the server's id of the menu the screen shows
    pub container_id: i32,
    /// the `slotChanged` calls the menu made since the screen's last own call
    pub pending_slot_changes: compat::SlotChangeQueue,
    /// the class's type parameter (`T extends AbstractContainerMenu`: the menu type)
    pub _menu: std::marker::PhantomData<fn() -> T>,
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
    ///
    /// (`handler.addSlotListener(this)` is `listen`, run by the screen factory on the menu it is handed)
    pub fn new(handler: T, _inventory: compat::PlayerInventory, title: MutableComponent, image_height: i32) -> Self {
        let mut this = Self {
            base: { let mut __s = AbstractContainerScreen::new((&title.get_string()).to_string(), String::new()); __s.image_width = 176; __s.image_height = image_height; __s },
            is_waiting_for_server: true,
            menu: handler,
            container_id: 0,
            pending_slot_changes: Default::default(),
            _menu: std::marker::PhantomData,
        };
        this.inventory_label_y = image_height.wrapping_sub(94);
        this
    }

    /// java: handler.addSlotListener(this); - on the server's menu the screen factory (`MenuScreens.fromPacket`) is
    /// handed. `addSlotListener` sends the menu's full state to the new listener at once (`broadcastFullState`).
    pub fn listen(&mut self, handler: &mut ChestMenu) {
        self.container_id = handler.menu.container_id;
        self.pending_slot_changes = compat::listen_slot_changes(handler, "AbstractCustomHypixelGUI");
    }

}

/// java: the virtual methods of AbstractCustomHypixelGUI (implemented by AbstractCustomHypixelGUIBase and every subclass)
pub trait AbstractCustomHypixelGUI<T: Clone + 'static> {
    fn abstract_custom_hypixel_gui_base(&self) -> &AbstractCustomHypixelGUIBase<T>;
    fn abstract_custom_hypixel_gui_base_mut(&mut self) -> &mut AbstractCustomHypixelGUIBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: protected void clickSlot(int slotID, int button)
    fn click_slot(&mut self, slot_id: i32, button: i32) {
        if self.abstract_custom_hypixel_gui_base().is_waiting_for_server {
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        // java: this.minecraft.gameMode.handleContainerInput(menu.containerId, slotID, ContainerUtils.getContainerClickButton(button), ContainerInput.PICKUP, minecraft.player);
        let container_id = self.abstract_custom_hypixel_gui_base().container_id;
        compat::handle_container_input(container_id, slot_id, crate::de::hysky::skyblocker::utils::container_utils::ContainerUtils::get_container_click_button(button), ContainerInput::Pickup);
        // java: menu.getCarried().setCount(0);
        mcrust::mcrust::modding::game::with_abstract_container_menu(|menu| {
            let mut carried = menu.get_carried().clone();
            carried.set_count(0);
            menu.set_carried(carried);
        });
        self.abstract_custom_hypixel_gui_base_mut().is_waiting_for_server = true;
    }

    /// java: protected void clickSlot(int slotID)
    fn click_slot_1(&mut self, slot_id: i32) {
        self.click_slot(slot_id, 0);
    }

    /// java: public void changeHandler(AuctionHouseScreenHandler newHandler)
    ///
    /// `menu.removeSlotListener(this); setHandler(newHandler); menu.addSlotListener(this);` - the old menu is gone
    /// with the server's new one (its listener with it); the screen listens to the new one (`chest`, the server's menu
    /// the factory is handed).
    fn change_handler(&mut self, new_handler: T, chest: &mut ChestMenu) {
        let base = self.abstract_custom_hypixel_gui_base_mut();
        base.menu = new_handler;
        base.listen(chest);
    }

    /// java: protected void refreshListener()
    ///
    /// (the port never removes the listener when the screen is removed - the queue keeps what the menu sent while
    /// a popup was up - so there is nothing to re-add)
    fn refresh_listener(&self) {
    }

    /// java: @Override public void removed()
    ///
    /// (`super.removed()` is the game's; the listener stays with the menu, see `refresh_listener`)
    fn removed(&self) {
    }

    /// java: @Override public void dataChanged(AbstractContainerMenu handler, int property, int value)
    fn data_changed(&self, _handler: &AbstractContainerMenu, _property: i32, _value: i32) {
    }

    /// java: protected abstract void onSlotChange(T handler, int slotID, ItemStack stack);
    fn on_slot_change(__this: &compat::Shared<Self>, handler: &T, slot_id: i32, stack: &ItemStack)
    where
        Self: Sized;
}

/// java: @Override public final void slotChanged(AbstractContainerMenu handler, int slotId, ItemStack stack) - for each
/// call the menu made since the screen's last own call: `onSlotChange(this.menu, slotId, stack); isWaitingForServer =
/// false;`
pub fn drain_slot_changes<T: Clone + 'static, S: AbstractCustomHypixelGUI<T> + 'static>(__this: &compat::Shared<S>) {
    let queue = compat::read(__this, |__t| __t.abstract_custom_hypixel_gui_base().pending_slot_changes.clone());
    for (slot_id, stack) in compat::take_slot_changes(&queue) {
        let menu: T = compat::read(__this, |__t| __t.abstract_custom_hypixel_gui_base().menu.clone());
        S::on_slot_change(__this, &menu, slot_id, &stack);
        compat::write(__this, |__t| __t.abstract_custom_hypixel_gui_base_mut().is_waiting_for_server = false);
    }
}

/// java: @Override protected void repositionElements() - `super.repositionElements(); refreshListener();` and then
/// `for (int i = 0; i < menu.getItems().size(); i++) onSlotChange(menu, i, menu.getItems().get(i));` - the screen
/// re-initialised (a popup over it closed, the window resized) is brought up to the menu's items.
pub fn reposition_elements<T: Clone + 'static, S: AbstractCustomHypixelGUI<T> + 'static>(__this: &compat::Shared<S>) {
    compat::read(__this, |__t| __t.refresh_listener());
    let container_id = compat::read(__this, |__t| __t.abstract_custom_hypixel_gui_base().container_id);
    let items: Vec<ItemStack> = compat::with_open_menu(|menu, containers| {
        if menu.container_id != container_id {
            return Vec::new();
        }
        menu.slots.iter().map(|slot| slot.get_item(containers).duplicate()).collect()
    })
    .unwrap_or_default();
    let menu: T = compat::read(__this, |__t| __t.abstract_custom_hypixel_gui_base().menu.clone());
    for (i, stack) in items.iter().enumerate() {
        S::on_slot_change(__this, &menu, i as i32, stack);
    }
}
