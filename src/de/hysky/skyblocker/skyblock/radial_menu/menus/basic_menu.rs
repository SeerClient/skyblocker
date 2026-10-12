//! Ported from de.hysky.skyblocker.skyblock.radialMenu.menus.BasicMenu by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenuBase;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public abstract class BasicMenu extends RadialMenu
#[derive(Clone)]
pub struct BasicMenuBase {
    /// java: extends RadialMenu
    pub base: RadialMenuBase,
    /// java: final String title;
    pub title: String,
    /// java: final String id;
    pub id: String,
}

impl std::ops::Deref for BasicMenuBase {
    type Target = RadialMenuBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BasicMenuBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BasicMenuBase {
    /// java: public BasicMenu(String title, String id) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut title: String, mut id: String, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn BasicMenu>, __R)) -> (compat::Shared<dyn BasicMenu>, __R) {
        let __init_title = title.clone();
        let __init_id = id.clone();
        let (_, (__this, __r)) = RadialMenuBase::new_in(move |__base| {
            let this = Self {
                base: __base,
                title: __init_title,
                id: __init_id,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn RadialMenu>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of BasicMenu (implemented by BasicMenuBase and every subclass)
pub trait BasicMenu: RadialMenu {
    fn basic_menu_base(&self) -> &BasicMenuBase;
    fn basic_menu_base_mut(&mut self) -> &mut BasicMenuBase;

}

impl dyn BasicMenu {
    /// java: @Override public Component getTitle(Component title)
    pub fn get_title_body(__this: &compat::Shared<dyn BasicMenu>, title: &MutableComponent) -> MutableComponent {
        return title.clone();
    }

    /// BasicMenu.getTitle as the dispatch of RadialMenu.getTitle for the concrete class `__C`
    pub fn get_title_from<__C: BasicMenu + 'static>(__this: &compat::Shared<dyn RadialMenu>, __p0: &MutableComponent) -> MutableComponent {
        <dyn BasicMenu>::get_title_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn BasicMenu>), __p0)
    }

    /// java: @Override public boolean titleMatches(String title)
    pub fn title_matches_body(__this: &compat::Shared<dyn BasicMenu>, title: &str) -> bool {
        return ({ let __f = compat::dispatch(__this, |__t| __t.get_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn RadialMenu>)) } && compat::equals_ignore_case(&title, &compat::read(__this, |__t| __t.basic_menu_base().title.clone())));
    }

    /// BasicMenu.titleMatches as the dispatch of RadialMenu.titleMatches for the concrete class `__C`
    pub fn title_matches_from<__C: BasicMenu + 'static>(__this: &compat::Shared<dyn RadialMenu>, __p0: &str) -> bool {
        <dyn BasicMenu>::title_matches_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn BasicMenu>), __p0)
    }

    /// java: @Override public boolean itemMatches(int slotId, ItemStack stack)
    pub fn item_matches_body(__this: &compat::Shared<dyn BasicMenu>, mut slot_id: i32, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        return !std::ptr::addr_eq(std::sync::Arc::as_ptr(&stack.get_item()), std::sync::Arc::as_ptr(mcrust::net::minecraft::world::item::items::stained_glass_pane().black()));
    }

    /// BasicMenu.itemMatches as the dispatch of RadialMenu.itemMatches for the concrete class `__C`
    pub fn item_matches_from<__C: BasicMenu + 'static>(__this: &compat::Shared<dyn RadialMenu>, __p0: i32, __p1: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> bool {
        <dyn BasicMenu>::item_matches_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn BasicMenu>), __p0, __p1)
    }

    /// java: @Override public String getConfigId()
    pub fn get_config_id_body(__this: &compat::Shared<dyn BasicMenu>) -> String {
        return compat::read(__this, |__t| __t.basic_menu_base().id.clone());
    }

    /// BasicMenu.getConfigId as the dispatch of RadialMenu.getConfigId for the concrete class `__C`
    pub fn get_config_id_from<__C: BasicMenu + 'static>(__this: &compat::Shared<dyn RadialMenu>) -> String {
        <dyn BasicMenu>::get_config_id_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn BasicMenu>))
    }

    /// java: @Override public int remapClickSlotButton(int originalButton, int slotId)
    pub fn remap_click_slot_button_body(__this: &compat::Shared<dyn BasicMenu>, mut original_button: i32, mut slot_id: i32) -> i32 {
        return original_button;
    }

    /// BasicMenu.remapClickSlotButton as the dispatch of RadialMenu.remapClickSlotButton for the concrete class `__C`
    pub fn remap_click_slot_button_from<__C: BasicMenu + 'static>(__this: &compat::Shared<dyn RadialMenu>, __p0: i32, __p1: i32) -> i32 {
        <dyn BasicMenu>::remap_click_slot_button_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn BasicMenu>), __p0, __p1)
    }

    /// java: @Override public int clickSlotOffset(int slotId)
    pub fn click_slot_offset_body(__this: &compat::Shared<dyn BasicMenu>, mut slot_id: i32) -> i32 {
        return 0;
    }

    /// BasicMenu.clickSlotOffset as the dispatch of RadialMenu.clickSlotOffset for the concrete class `__C`
    pub fn click_slot_offset_from<__C: BasicMenu + 'static>(__this: &compat::Shared<dyn RadialMenu>, __p0: i32) -> i32 {
        <dyn BasicMenu>::click_slot_offset_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn BasicMenu>), __p0)
    }

    /// java: @Override public String[] getNavigationItemNames()
    pub fn get_navigation_item_names_body(__this: &compat::Shared<dyn BasicMenu>) -> Vec<String> {
        return vec!["Go Back".to_string(), "Close".to_string()];
    }

    /// BasicMenu.getNavigationItemNames as the dispatch of RadialMenu.getNavigationItemNames for the concrete class `__C`
    pub fn get_navigation_item_names_from<__C: BasicMenu + 'static>(__this: &compat::Shared<dyn RadialMenu>) -> Vec<String> {
        <dyn BasicMenu>::get_navigation_item_names_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn BasicMenu>))
    }

}

// BasicMenu.getTitle implements RadialMenu.getTitle: it is emitted in each concrete subclass's `impl RadialMenu` (Rust has no inherited impls)
// BasicMenu.titleMatches implements RadialMenu.titleMatches: it is emitted in each concrete subclass's `impl RadialMenu` (Rust has no inherited impls)
// BasicMenu.itemMatches implements RadialMenu.itemMatches: it is emitted in each concrete subclass's `impl RadialMenu` (Rust has no inherited impls)
// BasicMenu.getConfigId implements RadialMenu.getConfigId: it is emitted in each concrete subclass's `impl RadialMenu` (Rust has no inherited impls)
// BasicMenu.remapClickSlotButton implements RadialMenu.remapClickSlotButton: it is emitted in each concrete subclass's `impl RadialMenu` (Rust has no inherited impls)
// BasicMenu.clickSlotOffset implements RadialMenu.clickSlotOffset: it is emitted in each concrete subclass's `impl RadialMenu` (Rust has no inherited impls)
// BasicMenu.getNavigationItemNames implements RadialMenu.getNavigationItemNames: it is emitted in each concrete subclass's `impl RadialMenu` (Rust has no inherited impls)
