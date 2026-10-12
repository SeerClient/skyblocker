//! Ported from de.hysky.skyblocker.skyblock.radialMenu.menus.FastTravelMenu by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::basic_menu::BasicMenuBase;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::regex_menu::RegexMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::regex_menu::RegexMenuBase;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenuBase;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::items;

/// java: public class FastTravelMenu extends RegexMenu
#[derive(Clone)]
pub struct FastTravelMenu {
    /// java: extends RegexMenu
    pub base: RegexMenuBase,
}

impl std::ops::Deref for FastTravelMenu {
    type Target = RegexMenuBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FastTravelMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FastTravelMenu {
    /// java: public FastTravelMenu()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = RegexMenuBase::new_in("(fast travel)|(.* warps)".to_string(), "fastTravel".to_string(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn RegexMenu>), __h)
        });
        __this
    }

}

impl RegexMenu for FastTravelMenu {
    fn regex_menu_base(&self) -> &RegexMenuBase {
        &self.base
    }
    fn regex_menu_base_mut(&mut self) -> &mut RegexMenuBase {
        &mut self.base
    }

}

impl BasicMenu for FastTravelMenu {
    fn basic_menu_base(&self) -> &BasicMenuBase {
        &self.base.base
    }
    fn basic_menu_base_mut(&mut self) -> &mut BasicMenuBase {
        &mut self.base.base
    }

}

impl RadialMenu for FastTravelMenu {
    fn radial_menu_base(&self) -> &RadialMenuBase {
        &self.base.base.base
    }
    fn radial_menu_base_mut(&mut self) -> &mut RadialMenuBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean itemMatches(int slotId, ItemStack stack) (dispatch: the body for this class)
    fn item_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, i32, &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        FastTravelMenu::item_matches_dyn
    }

    /// java: @Override public Component getTitle(Component title) (dispatch: the body for this class)
    fn get_title(&self) -> fn(&compat::Shared<dyn RadialMenu>, &MutableComponent) -> MutableComponent {
        <dyn BasicMenu>::get_title_from::<Self>
    }

    /// java: @Override public boolean titleMatches(String title) (dispatch: the body for this class)
    fn title_matches(&self) -> fn(&compat::Shared<dyn RadialMenu>, &str) -> bool {
        <dyn RegexMenu>::title_matches_from::<Self>
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

impl FastTravelMenu {
    /// java: @Override public boolean itemMatches(int slotId, ItemStack stack)
    pub fn item_matches_impl(__this: &compat::Shared<Self>, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        if slot_id == 50 || slot_id == 53 {
            return false;
        }
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(&stack.get_item()), std::sync::Arc::as_ptr(&items::items().get("bedrock"))) || std::ptr::addr_eq(std::sync::Arc::as_ptr(&stack.get_item()), std::sync::Arc::as_ptr(&items::items().get("player_head"))) && ItemUtils::get_head_texture(stack) == "eyJ0aW1lc3RhbXAiOjE1ODU3NjY0NDM5MTYsInByb2ZpbGVJZCI6ImIwZDRiMjhiYzFkNzQ4ODlhZjBlODY2MWNlZTk2YWFiIiwicHJvZmlsZU5hbWUiOiJNaW5lU2tpbl9vcmciLCJzaWduYXR1cmVSZXF1aXJlZCI6dHJ1ZSwidGV4dHVyZXMiOnsiU0tJTiI6eyJ1cmwiOiJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2Y3NjliYmI5ZmIyMzE2ODA4MTMxZTZjMmQwMmNlMTRlM2FhYjY3NGRlYjRiNTU4YmIyNjVjMzE0MWQxOTliMDgifX19" {
            return false;
        }
        return !std::ptr::addr_eq(std::sync::Arc::as_ptr(&stack.get_item()), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()));
    }

    /// FastTravelMenu.itemMatches as the dispatch of RadialMenu.itemMatches
    pub fn item_matches_dyn(__this: &compat::Shared<dyn RadialMenu>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        Self::item_matches_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
