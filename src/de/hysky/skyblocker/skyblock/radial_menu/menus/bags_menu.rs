//! Ported from de.hysky.skyblocker.skyblock.radialMenu.menus.BagsMenu by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenuBase;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenuBase;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class BagsMenu extends BasicMenu
#[derive(Clone)]
pub struct BagsMenu {
    /// java: extends BasicMenu
    pub base: BasicMenuBase,
}

impl std::ops::Deref for BagsMenu {
    type Target = BasicMenuBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BagsMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BagsMenu {
    /// java: public BagsMenu()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = BasicMenuBase::new_in("your bags".to_string(), "general".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn BasicMenu>), __h)
        });
        __this
    }

}

impl BasicMenu for BagsMenu {
    fn basic_menu_base(&self) -> &BasicMenuBase {
        &self.base
    }
    fn basic_menu_base_mut(&mut self) -> &mut BasicMenuBase {
        &mut self.base
    }

}

impl RadialMenu for BagsMenu {
    fn radial_menu_base(&self) -> &RadialMenuBase {
        &self.base.base
    }
    fn radial_menu_base_mut(&mut self) -> &mut RadialMenuBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Component getTitle(Component title) (dispatch: the body for this class)
    fn get_title(&self) -> fn(&compat::Shared<dyn RadialMenu>, &MutableComponent) -> MutableComponent {
        <dyn BasicMenu>::get_title_from::<Self>
    }

    /// java: @Override public boolean titleMatches(String title) (dispatch: the body for this class)
    fn title_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, &str) -> bool {
        <dyn BasicMenu>::title_matches_from::<Self>
    }

    /// java: @Override public boolean itemMatches(int slotId, ItemStack stack) (dispatch: the body for this class)
    fn item_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        <dyn BasicMenu>::item_matches_from::<Self>
    }

    /// java: @Override public String getConfigId() (dispatch: the body for this class)
    fn get_config_id(&self) -> fn(&compat::Shared<dyn RadialMenu>) -> String {
        <dyn BasicMenu>::get_config_id_from::<Self>
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
