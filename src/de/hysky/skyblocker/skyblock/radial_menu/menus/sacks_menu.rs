//! Ported from de.hysky.skyblocker.skyblock.radialMenu.menus.SacksMenu by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenuBase;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::regex_menu::RegexMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::regex_menu::RegexMenuBase;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenuBase;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class SacksMenu extends RegexMenu
#[derive(Clone)]
pub struct SacksMenu {
    /// java: extends RegexMenu
    pub base: RegexMenuBase,
}

impl std::ops::Deref for SacksMenu {
    type Target = RegexMenuBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SacksMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SacksMenu {
    /// java: public SacksMenu()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = RegexMenuBase::new_in(".* sack".to_string(), "sacks".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn RegexMenu>), __h)
        });
        __this
    }

}

impl RegexMenu for SacksMenu {
    fn regex_menu_base(&self) -> &RegexMenuBase {
        &self.base
    }
    fn regex_menu_base_mut(&mut self) -> &mut RegexMenuBase {
        &mut self.base
    }

}

impl BasicMenu for SacksMenu {
    fn basic_menu_base(&self) -> &BasicMenuBase {
        &self.base.base
    }
    fn basic_menu_base_mut(&mut self) -> &mut BasicMenuBase {
        &mut self.base.base
    }

}

impl RadialMenu for SacksMenu {
    fn radial_menu_base(&self) -> &RadialMenuBase {
        &self.base.base.base
    }
    fn radial_menu_base_mut(&mut self) -> &mut RadialMenuBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public String[] getNavigationItemNames() (dispatch: the body for this class)
    fn get_navigation_item_names(&self) -> fn(&compat::Shared<dyn RadialMenu>) -> Vec<String> {
        SacksMenu::get_navigation_item_names_dyn
    }

    /// java: @Override public Component getTitle(Component title) (dispatch: the body for this class)
    fn get_title(&self) -> fn(&compat::Shared<dyn RadialMenu>, &MutableComponent) -> MutableComponent {
        <dyn BasicMenu>::get_title_from::<Self>
    }

    /// java: @Override public boolean titleMatches(String title) (dispatch: the body for this class)
    fn title_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, &str) -> bool {
        <dyn RegexMenu>::title_matches_from::<Self>
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

}

impl SacksMenu {
    /// java: @Override public String[] getNavigationItemNames()
    pub fn get_navigation_item_names_impl(__this: &compat::Shared<Self>) -> Vec<String> {
        return vec!["Rune Type Filter".to_string(), "Close".to_string(), "Go Back".to_string(), "Item Tier".to_string()];
    }

    /// SacksMenu.getNavigationItemNames as the dispatch of RadialMenu.getNavigationItemNames
    pub fn get_navigation_item_names_dyn(__this: &compat::Shared<dyn RadialMenu>) -> Vec<String> {
        Self::get_navigation_item_names_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
