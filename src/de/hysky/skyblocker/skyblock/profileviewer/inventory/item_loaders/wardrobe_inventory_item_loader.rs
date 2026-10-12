//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.itemLoaders.WardrobeInventoryItemLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoaderBase;

/// java: public class WardrobeInventoryItemLoader extends ItemLoader
#[derive(Clone)]
pub struct WardrobeInventoryItemLoader {
    /// java: extends ItemLoader
    pub base: ItemLoaderBase,
    /// java: private final int activeSlot;
    pub active_slot: i32,
    /// java: private final JsonObject activeArmorSet;
    pub active_armor_set: serde_json::Map<String, serde_json::Value>,
}

impl std::ops::Deref for WardrobeInventoryItemLoader {
    type Target = ItemLoaderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for WardrobeInventoryItemLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl WardrobeInventoryItemLoader {
    /// java: public WardrobeInventoryItemLoader(JsonObject inventory)
    pub fn new(mut inventory: serde_json::Map<String, serde_json::Value>) -> compat::Shared<Self> {
        let __init_active_slot = compat::json::get_as_int(&inventory.get("wardrobe_equipped_slot").cloned().unwrap());
        let __init_active_armor_set = compat::json::get_as_json_object(&inventory.get("inv_armor").cloned().unwrap());
        let (_, __this) = ItemLoaderBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                active_slot: __init_active_slot,
                active_armor_set: __init_active_armor_set,
            });
            ((__h.clone() as compat::Shared<dyn ItemLoader>), __h)
        });
        __this
    }

}

impl ItemLoader for WardrobeInventoryItemLoader {
    fn item_loader_base(&self) -> &ItemLoaderBase {
        &self.base
    }
    fn item_loader_base_mut(&mut self) -> &mut ItemLoaderBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public List<ItemStack> loadItems(JsonObject data) (dispatch: the body for this class)
    fn load_items(&self) -> fn(&compat::Shared<dyn ItemLoader>, &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        WardrobeInventoryItemLoader::load_items_dyn
    }

}

impl WardrobeInventoryItemLoader {
    /// java: @Override public List<ItemStack> loadItems(JsonObject data)
    pub fn load_items_impl(__this: &compat::Shared<Self>, data: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut item_list: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            item_list.extend(<dyn ItemLoader>::load_items_body(&compat::view(__this, |a| a as compat::Shared<dyn ItemLoader>), data).into_iter());
            if compat::read(__this, |__t| __t.active_slot) != -1 {
                let mut active_armour: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = (<dyn ItemLoader>::load_items_body(&compat::view(__this, |a| a as compat::Shared<dyn ItemLoader>), &compat::read(__this, |__t| __t.active_armor_set.clone()))).iter().rev().cloned().collect::<Vec<_>>();
                for i in 0..4 {
                    let mut base_index: i32 = ((compat::read(__this, |__t| __t.active_slot)).wrapping_sub(1)).wrapping_rem(9);
                    let mut page: i32 = ((compat::read(__this, |__t| __t.active_slot)).wrapping_sub(1)).wrapping_div(9);
                    let mut slot_index: i32 = ((page.wrapping_mul(36)).wrapping_add(i.wrapping_mul(9))).wrapping_add(base_index);
                    item_list[(slot_index) as usize] = active_armour[(i) as usize].clone();
                }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Profile Viewer] Failed to load wardrobe items"));
        }
        return item_list;
    }

    /// WardrobeInventoryItemLoader.loadItems as the dispatch of ItemLoader.loadItems
    pub fn load_items_dyn(__this: &compat::Shared<dyn ItemLoader>, __p0: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::load_items_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
