//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.itemLoaders.BackpackItemLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoaderBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::world::item::component::item_lore::ItemLore;

/// java: public class BackpackItemLoader extends ItemLoader
#[derive(Clone)]
pub struct BackpackItemLoader {
    /// java: extends ItemLoader
    pub base: ItemLoaderBase,
}

impl std::ops::Deref for BackpackItemLoader {
    type Target = ItemLoaderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BackpackItemLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BackpackItemLoader {
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

impl ItemLoader for BackpackItemLoader {
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
        BackpackItemLoader::load_items_dyn
    }

}

impl BackpackItemLoader {
    /// java: @Override public List<ItemStack> loadItems(JsonObject data)
    pub fn load_items_impl(__this: &compat::Shared<Self>, data: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut backpack_items: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        let mut sorted_entries: Vec<(String, serde_json::Value)> = compat::sorted_by(data.iter().map(|(k, v)| (k.clone(), v.clone())), |e1, e2| {
            let mut key1: i32 = e1.0.trim().parse::<i32>().expect("NumberFormatException");
            let mut key2: i32 = e2.0.trim().parse::<i32>().expect("NumberFormatException");
            return compat::ord_int(key1.cmp(&key2));
        }).collect::<Vec<_>>();
        for i in 0..(sorted_entries.len() as i32) {
            backpack_items.extend(<dyn ItemLoader>::load_items_body(&compat::view(__this, |a| a as compat::Shared<dyn ItemLoader>), &compat::json::get_as_json_object(&(sorted_entries[(i) as usize].1))).into_iter());
            let mut padding_needed: i32 = (45i32.wrapping_sub((backpack_items.len() as i32).wrapping_rem(45))).wrapping_rem(45);
            for j in 0..padding_needed {
                let mut padding_item: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(&FlexibleItemStack::copy(ico::GRAY_DYE.get_mut()));
                padding_item.set(mcrust::net::minecraft::core::component::data_components::custom_name().clone(), Some(component::translatable("skyblocker.profileviewer.inventory.inactive")));
                padding_item.set(mcrust::net::minecraft::core::component::data_components::lore().clone(), Some(ItemLore::new(vec![component::translatable("skyblocker.profileviewer.inventory.inactive.description.backpack"), component::translatable("skyblocker.profileviewer.inventory.inactive.description.general")]).unwrap()));
                backpack_items.push(padding_item.clone());
            }
        }
        return backpack_items;
    }

    /// BackpackItemLoader.loadItems as the dispatch of ItemLoader.loadItems
    pub fn load_items_dyn(__this: &compat::Shared<dyn ItemLoader>, __p0: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::load_items_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
