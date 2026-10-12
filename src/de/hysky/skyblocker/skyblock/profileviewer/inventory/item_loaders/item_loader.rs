//! Ported from de.hysky.skyblocker.skyblock.profileviewer.inventory.itemLoaders.ItemLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::pet_info;
use crate::de::hysky::skyblocker::skyblock::item::pet_info::PetInfo;
use crate::de::hysky::skyblocker::skyblock::profileviewer::inventory::pet::Pet;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::ico;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use mcrust::net::minecraft::nbt::list_tag;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::network::chat::component;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: public class ItemLoader
#[derive(Clone)]
pub struct ItemLoaderBase {}

impl ItemLoaderBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn ItemLoader>, __R)) -> (compat::Shared<dyn ItemLoader>, __R) {
        let this = Self {
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        Self::new_in(|this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn ItemLoader>), __h)
        }).1
    }

    /// java: private static ListTag decompress(JsonObject data)
    pub fn decompress(data: &serde_json::Map<String, serde_json::Value>) -> Option<Tag> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `new ByteArrayInputStream(..)`
            //   java: return NbtIo.readCompressed(new ByteArrayInputStream(Base64.getDecoder().decode(data.get("data").getAsString())), NbtAccounter.unlimitedHeap()).getListOrEmpty("i");
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to decompress item data"));
            }
        }
        return None;
    }

}

/// java: the virtual methods of ItemLoader (implemented by ItemLoaderBase and every subclass)
pub trait ItemLoader: compat::DynAny {
    fn item_loader_base(&self) -> &ItemLoaderBase;
    fn item_loader_base_mut(&mut self) -> &mut ItemLoaderBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public List<ItemStack> loadItems(JsonObject data) (dispatch: the body for this class)
    fn load_items(&self) -> fn(&compat::Shared<dyn ItemLoader>, &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        <dyn ItemLoader>::load_items_body
    }

}

impl dyn ItemLoader {
    /// java: public List<ItemStack> loadItems(JsonObject data)
    pub fn load_items_body(__this: &compat::Shared<dyn ItemLoader>, data: &serde_json::Map<String, serde_json::Value>) -> Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> {
        let mut container_content: Tag = ItemLoaderBase::decompress(data).unwrap();
        let mut item_list: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        // TODO(fabric2seer): `size` takes 1 argument(s) in the port (`elements: &[Tag]`), Java passes 0
        for i in 0..list_tag::size() {
            // TODO(fabric2seer): `get_compound_or_empty` takes 2 argument(s) in the port (`elements: &[Tag], index: i32`), Java passes 1
            // rust: let mut nbt: Tag = list_tag::get_compound_or_empty(&i);
            let mut nbt: Tag = todo!("fabric2seer: the initialiser of `nbt` (TODO above)");
            if nbt.get_int_or("id", 0) == 0 {
                item_list.push(mcrust::net::minecraft::world::item::item_stack::empty());
                continue;
            }
            // TODO(fabric2seer): `.set(..)` on a value of unknown type
            //   java: ItemStack stack = LegacyItemStackFixer.fixLegacyStack(nbt, ItemStack.CODEC, ItemStack.EMPTY, ItemStack::set);
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = todo!("fabric2seer: the initialiser of `stack` (TODO above)");
            if stack.is_empty() {
                let mut fallback: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(ico::BARRIER.get_mut()).copy();
                fallback.set(mcrust::net::minecraft::core::component::data_components::custom_name().clone(), Some(component::literal(&format!("Error: {}", nbt.get_compound_or_empty("tag").get_compound_or_empty("ExtraAttributes").get_string("id")))));
                item_list.push(fallback.clone());
                continue;
            }
            let mut item_id: String = stack.get_skyblock_id();
            let mut custom_data: Tag = ItemUtils::get_custom_data(&stack);
            if item_id == "PET" {
                let mut pet_info: PetInfo = (pet_info::CODEC.get_mut().parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::from_str::<serde_json::Value>(&custom_data.get_string_or("petInfo", "")).expect("JsonSyntaxException")))).get_or_throw().unwrap();
                let mut pet: Pet = Pet::new(pet_info.clone());
                item_list.push(FlexibleItemStack::get_stack_or_throw(&pet.get_icon()));
                continue;
            }
            if stack.has(mcrust::net::minecraft::core::component::data_components::custom_data()) {
                // TODO(fabric2seer): `.putBoolean(..)` on a T
                //   java: customData.put("aaron-mod", Util.make(new CompoundTag(), comp -> comp.putBoolean("alwaysDisplaySkyblockInfo", true)));
            }
            item_list.push(stack.clone());
        }
        return item_list;
    }

}

impl ItemLoader for ItemLoaderBase {
    fn item_loader_base(&self) -> &ItemLoaderBase {
        &self
    }
    fn item_loader_base_mut(&mut self) -> &mut ItemLoaderBase {
        &mut self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
