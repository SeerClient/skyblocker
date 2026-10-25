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
    /// (the `ListTag`: its elements)
    pub fn decompress(data: &serde_json::Map<String, serde_json::Value>) -> Option<Vec<Tag>> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // java: return NbtIo.readCompressed(new ByteArrayInputStream(Base64.getDecoder().decode(data.get("data").getAsString())), NbtAccounter.unlimitedHeap()).getListOrEmpty("i");
            let bytes: Vec<u8> = mcrust::java::util::base64::get_decoder().decode(&compat::json::get_as_string(&data.get("data").cloned().expect("NullPointerException"))).unwrap_or_else(|e| panic!("IllegalArgumentException: {e}"));
            let compound: Tag = mcrust::net::minecraft::nbt::nbt_io::read_compressed(&bytes, mcrust::net::minecraft::nbt::nbt_accounter::NbtAccounter::unlimited_heap()).unwrap_or_else(|e| panic!("IOException: {e}"));
            // (a ListTag: its elements)
            return Some(compound.get_list_or_empty("i"));
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
        // java: ListTag containerContent = decompress(data); (null when the data could not be read: `size()` throws)
        let mut container_content: Vec<Tag> = ItemLoaderBase::decompress(data).expect("NullPointerException");
        let mut item_list: Vec<mcrust::net::minecraft::world::item::item_stack::ItemStack> = Vec::new();
        for i in 0..(container_content.len() as i32) {
            let mut nbt: Tag = list_tag::get_compound_or_empty(&container_content, i);
            if nbt.get_int_or("id", 0) == 0 {
                item_list.push(mcrust::net::minecraft::world::item::item_stack::empty());
                continue;
            }
            // java: ItemStack stack = LegacyItemStackFixer.fixLegacyStack(nbt, ItemStack.CODEC, ItemStack.EMPTY, ItemStack::set);
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = crate::de::hysky::skyblocker::utils::datafixer::legacy_item_stack_fixer::LegacyItemStackFixer::fix_legacy_stack(
                &nbt,
                &mcrust::net::minecraft::world::item::item_stack::codec(),
                &mcrust::net::minecraft::world::item::item_stack::empty(),
                &|s: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack, r#type, value| {
                    s.set(r#type, Some(value));
                },
            );
            if stack.is_empty() {
                let mut fallback: mcrust::net::minecraft::world::item::item_stack::ItemStack = FlexibleItemStack::get_stack_or_throw(ico::BARRIER.get_mut()).copy();
                fallback.set(mcrust::net::minecraft::core::component::data_components::custom_name().clone(), Some(mcrust::com::mojang::serialization::obj(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::literal(&format!("Error: {}", compat::optional_string(&nbt.get_compound_or_empty("tag").get_compound_or_empty("ExtraAttributes").get_string_opt("id"))))))));
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
                // java: customData.put("aaron-mod", Util.make(new CompoundTag(), comp -> comp.putBoolean("alwaysDisplaySkyblockInfo", true)));
                // (`ItemUtils.getCustomData` is the stack's own `CustomData` tag in Java, so the put lands in the stack; the port's is
                // a copy, which is written back)
                let mut aaron_mod: Tag = Tag::compound();
                aaron_mod.put_boolean("alwaysDisplaySkyblockInfo", true);
                custom_data.put("aaron-mod", aaron_mod);
                stack.set(mcrust::net::minecraft::core::component::data_components::custom_data().clone(), Some(mcrust::com::mojang::serialization::obj(mcrust::net::minecraft::world::item::component::custom_data::CustomData::of(&custom_data))));
            }
            item_list.push(stack.clone());
        }
        return item_list;
    }

}

impl ItemLoader for ItemLoaderBase {
    fn item_loader_base(&self) -> &ItemLoaderBase {
        self
    }
    fn item_loader_base_mut(&mut self) -> &mut ItemLoaderBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
