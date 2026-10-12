//! Ported from de.hysky.skyblocker.skyblock.radialMenu.menus.Storage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenuBase;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenuBase;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class Storage extends BasicMenu
#[derive(Clone)]
pub struct Storage {
    /// java: extends BasicMenu
    pub base: BasicMenuBase,
}

impl std::ops::Deref for Storage {
    type Target = BasicMenuBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Storage {
    /// java: public Storage()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = BasicMenuBase::new_in("Storage".to_string(), "storage".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn BasicMenu>), __h)
        });
        __this
    }

}

impl BasicMenu for Storage {
    fn basic_menu_base(&self) -> &BasicMenuBase {
        &self.base
    }
    fn basic_menu_base_mut(&mut self) -> &mut BasicMenuBase {
        &mut self.base
    }

}

impl RadialMenu for Storage {
    fn radial_menu_base(&self) -> &RadialMenuBase {
        &self.base.base
    }
    fn radial_menu_base_mut(&mut self) -> &mut RadialMenuBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean itemMatches(int slotId, ItemStack stack) (dispatch: the body for this class)
    fn item_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        Storage::item_matches_dyn
    }

    /// java: @Override public String getConfigId() (dispatch: the body for this class)
    fn get_config_id(&self) -> fn(&compat::Shared<dyn RadialMenu>) -> String {
        Storage::get_config_id_dyn
    }

    /// java: @Override public Component getTitle(Component title) (dispatch: the body for this class)
    fn get_title(&self) -> fn(&compat::Shared<dyn RadialMenu>, &MutableComponent) -> MutableComponent {
        <dyn BasicMenu>::get_title_from::<Self>
    }

    /// java: @Override public boolean titleMatches(String title) (dispatch: the body for this class)
    fn title_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, &str) -> bool {
        <dyn BasicMenu>::title_matches_from::<Self>
    }

    /// java: @Override public int remapClickSlotButton(int originalButton, int slotId) (dispatch: the body for this class)
    fn remap_click_slot_button(&self) -> fn(&compat::Shared<dyn RadialMenu>, i32, i32) -> i32 {
        <dyn BasicMenu>::remap_click_slot_button_from::<Self>
    }

    /// java: @Override public int clickSlotOffset(int slotId) (dispatch: the body for this class)
    fn click_slot_offset(&self) -> fn(&compat::Shared<dyn RadialMenu>, i32) -> i32 {
        <dyn BasicMenu>::click_slot_offset_from::<Self>
    }

    /// java: @Override public String[] getNavigationItemNames() (dispatch: the body for this class)
    fn get_navigation_item_names(&self) -> fn(&compat::Shared<dyn RadialMenu>) -> Vec<String> {
        <dyn BasicMenu>::get_navigation_item_names_from::<Self>
    }

}

impl Storage {
    /// java: @Override public boolean itemMatches(int slotId, ItemStack stack)
    pub fn item_matches_impl(__this: &compat::Shared<Self>, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        let mut item: Arc<Item> = stack.get_item();
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(&item), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().black())) || std::ptr::addr_eq(std::sync::Arc::as_ptr(&item), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().red())) || std::ptr::addr_eq(std::sync::Arc::as_ptr(&item), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().brown())) {
            return false;
        }
        return (slot_id != 4 && slot_id != 22);
    }

    /// Storage.itemMatches as the dispatch of RadialMenu.itemMatches
    pub fn item_matches_dyn(__this: &compat::Shared<dyn RadialMenu>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        Self::item_matches_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public String getConfigId()
    pub fn get_config_id_impl(__this: &compat::Shared<Self>) -> String {
        return "storage".to_string();
    }

    /// Storage.getConfigId as the dispatch of RadialMenu.getConfigId
    pub fn get_config_id_dyn(__this: &compat::Shared<dyn RadialMenu>) -> String {
        Self::get_config_id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
