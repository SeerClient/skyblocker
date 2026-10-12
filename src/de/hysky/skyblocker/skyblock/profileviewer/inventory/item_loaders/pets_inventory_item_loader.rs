//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.itemLoaders.PetsInventoryItemLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::pet_info;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::item_loaders::item_loader::ItemLoaderBase;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::pet::Pet;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: public class PetsInventoryItemLoader extends ItemLoader
#[derive(Clone)]
pub struct PetsInventoryItemLoader {
    /// java: extends ItemLoader
    pub base: ItemLoaderBase,
}

impl std::ops::Deref for PetsInventoryItemLoader {
    type Target = ItemLoaderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PetsInventoryItemLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PetsInventoryItemLoader {
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

impl ItemLoader for PetsInventoryItemLoader {
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
        PetsInventoryItemLoader::load_items_dyn
    }

}

impl PetsInventoryItemLoader {
    /// java: @Override public List<ItemStack> loadItems(JsonObject data)
    pub fn load_items_impl(__this: &compat::Shared<Self>, data: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut pet_list: Vec<Pet> = Vec::new();
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut pets_data: Option<serde_json::Map<String, serde_json::Value>> = Some(compat::json::get_as_json_object_of(&data, "pets_data"));
            if pets_data.is_some() && compat::json::has(&pets_data.as_ref().unwrap(), "pets") {
                for pet_element in compat::json::get_as_json_array(&pets_data.as_ref().unwrap().get("pets").cloned().unwrap()).iter() {
                    let mut pet_info: PetInfo = (pet_info::CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&compat::json::to_string(&pet_element)).expect("JsonSyntaxException")))).get_or_throw().unwrap();
                    pet_list.push(Pet::new(pet_info.clone()));
                }
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Profile Viewer] Failed to load pets"));
        }
        // java: petList.sort(Comparator.comparingInt(Pet::getTier).thenComparingInt(Pet::getLevel).reversed());
        pet_list.sort_by(|a: &Pet, b: &Pet| b.get_tier().cmp(&a.get_tier()).then_with(|| b.get_level().cmp(&a.get_level())));
        let mut item_list: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        for pet in pet_list.iter() {
            item_list.push(FlexibleItemStack::get_stack_or_throw(&pet.get_icon()));
        }
        return item_list;
    }

    /// PetsInventoryItemLoader.loadItems as the dispatch of ItemLoader.loadItems
    pub fn load_items_dyn(__this: &compat::Shared<dyn ItemLoader>, __p0: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        Self::load_items_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
