//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.itemLoaders.InventoryItemLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoaderBase;

/// java: private static final String[] INVENTORIES = {"inv_contents", "inv_armor", "equipment_contents"};
pub static INVENTORIES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["inv_contents".to_string(), "inv_armor".to_string(), "equipment_contents".to_string()]);

/// java: public class InventoryItemLoader extends ItemLoader
#[derive(Clone)]
pub struct InventoryItemLoader {
    /// java: extends ItemLoader
    pub base: ItemLoaderBase,
}

impl std::ops::Deref for InventoryItemLoader {
    type Target = ItemLoaderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for InventoryItemLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl InventoryItemLoader {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ItemLoaderBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ItemLoader>), __h)
        });
        __this
    }

}

impl ItemLoader for InventoryItemLoader {
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
        InventoryItemLoader::load_items_dyn
    }

}

impl InventoryItemLoader {
    /// java: @Override public List<ItemStack> loadItems(JsonObject data)
    pub fn load_items_impl(__this: &compat::Shared<Self>, data: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut inventory_items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        for inventory in INVENTORIES.iter() {
            match inventory.as_str() {
                "inv_armor" => {
                    // java: List<ItemStack> inv = super.loadItems(data.getAsJsonObject(inventory)); inventoryItems.addAll(inv.reversed());
                    let mut inv: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = <dyn ItemLoader>::load_items_body(&compat::view(__this, |a| a as compat::Shared<dyn ItemLoader>), &compat::json::get_as_json_object_of(&data, inventory));
                    inv.reverse();
                    inventory_items.extend(inv);
                }
                "inv_contents" => {
                    inventory_items.extend((<dyn ItemLoader>::load_items_body(&compat::view(__this, |a| a as compat::Shared<dyn ItemLoader>), &compat::json::get_as_json_object_of(&data, inventory)))[(9) as usize..(((<dyn ItemLoader>::load_items_body(&compat::view(__this, |a| a as compat::Shared<dyn ItemLoader>), &compat::json::get_as_json_object_of(&data, inventory))).len() as i32)) as usize].to_vec().into_iter());
                    inventory_items.extend((<dyn ItemLoader>::load_items_body(&compat::view(__this, |a| a as compat::Shared<dyn ItemLoader>), &compat::json::get_as_json_object_of(&data, inventory)))[(0) as usize..(9) as usize].to_vec().into_iter());
                }
                _ => {
                    inventory_items.extend((<dyn ItemLoader>::load_items_body(&compat::view(__this, |a| a as compat::Shared<dyn ItemLoader>), &compat::json::get_as_json_object_of(&data, inventory))).iter().cloned());
                }
            }
        }
        return inventory_items;
    }

    /// InventoryItemLoader.loadItems as the dispatch of ItemLoader.loadItems
    pub fn load_items_dyn(__this: &compat::Shared<dyn ItemLoader>, __p0: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::load_items_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
