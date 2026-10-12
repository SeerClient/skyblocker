//! Ported from de.hysky.skyblocker.utils.datafixer.ItemStackComponentizationFixer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::registry_utils::RegistryUtils;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::net::minecraft::commands::arguments::item::item_input::ItemInput;
use mcrust::net::minecraft::commands::arguments::item::item_parser::ItemParser;
use mcrust::net::minecraft::core::component::data_component_patch::DataComponentPatch;
use mcrust::net::minecraft::core::component::data_component_type::DataComponentType;
use mcrust::net::minecraft::core::component::typed_data_component::TypedDataComponent;
use mcrust::net::minecraft::core::holder_lookup;
use mcrust::net::minecraft::nbt::tag::Tag;
use mcrust::net::minecraft::resources::registry_ops::RegistryOps;
use mcrust::net::minecraft::util::datafix::data_fixers;
use mcrust::com::mojang::datafixers::data_fixer::DataFixer as _;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::net::minecraft::core::registry::Registry as _;

/// java: private static final int ITEM_NBT_DATA_VERSION = 3817;
pub const ITEM_NBT_DATA_VERSION: i32 = 3817;
/// java: private static final int ITEM_COMPONENTS_DATA_VERSION = 4325;
pub const ITEM_COMPONENTS_DATA_VERSION: i32 = 4325;

/// java: public class ItemStackComponentizationFixer
#[derive(Clone)]
pub struct ItemStackComponentizationFixer {}

impl ItemStackComponentizationFixer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static ItemStack fixUpItem(CompoundTag nbt)
    pub fn fix_up_item(nbt: &Tag) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        // TODO(fabric2seer): `create_serialization_context` takes 2 argument(s) in the port (`provider: Arc<dyn Provider>, parent: crate::com::mojang::serialization::dynamic_ops::Ops`), Java passes 1
        // TODO(fabric2seer): `new` takes 1 argument(s) in the port (`ops: Ops`), Java passes 2
        // rust: let mut dynamic: Dynamic = data_fixers::get_data_fixer().update(&mcrust::net::minecraft::util::datafix::fixes::references::ITEM_STACK, Dynamic::new(holder_lookup::create_serialization_context(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance()), nbt.clone()), ITEM_NBT_DATA_VERSION, ITEM_COMPONENTS_DATA_VERSION).unwrap();
        let mut dynamic: Dynamic = todo!("fabric2seer: the initialiser of `dynamic` (TODO above)");
        return mcrust::net::minecraft::world::item::item_stack::codec().parse_dynamic(&dynamic).get_or_throw().unwrap();
    }

    /// java: public static FlexibleItemStack fixUpFlexibleItem(CompoundTag nbt)
    pub fn fix_up_flexible_item(nbt: &Tag) -> compat::Shared<FlexibleItemStack> {
        // TODO(fabric2seer): `create_serialization_context` takes 2 argument(s) in the port (`provider: Arc<dyn Provider>, parent: crate::com::mojang::serialization::dynamic_ops::Ops`), Java passes 1
        // TODO(fabric2seer): `new` takes 1 argument(s) in the port (`ops: Ops`), Java passes 2
        // rust: let mut dynamic: Dynamic = data_fixers::get_data_fixer().update(&mcrust::net::minecraft::util::datafix::fixes::references::ITEM_STACK, Dynamic::new(holder_lookup::create_serialization_context(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance()), nbt.clone()), ITEM_NBT_DATA_VERSION, ITEM_COMPONENTS_DATA_VERSION).unwrap();
        let mut dynamic: Dynamic = todo!("fabric2seer: the initialiser of `dynamic` (TODO above)");
        return flexible_item_stack::CODEC.get_mut().parse_dynamic(&dynamic).get_or_throw().unwrap();
    }

    /// java: public static String componentsAsString(ItemStack stack)
    pub fn components_as_string(stack: &mut mcrust::net::minecraft::world::item::item_stack::ItemStack) -> String {
        return ItemStackComponentizationFixer::components_as_string_data_component_patch(&stack.get_components_patch()).unwrap();
    }

    /// java: public static String componentsAsString(DataComponentPatch components)
    pub fn components_as_string_data_component_patch(components: &DataComponentPatch) -> Option<String> {
        // TODO(fabric2seer): `create_serialization_context` takes 2 argument(s) in the port (`provider: Arc<dyn Provider>, parent: crate::com::mojang::serialization::dynamic_ops::Ops`), Java passes 1
        // rust: let mut nbt_registry_ops: RegistryOps = holder_lookup::create_serialization_context(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance());
        let mut nbt_registry_ops: RegistryOps = todo!("fabric2seer: the initialiser of `nbtRegistryOps` (TODO above)");
        return Some(format!("{:?}", components.entry_set().iter().cloned().map(|entry| {
            let mut component_type: Arc<DataComponentType> = entry.0.clone();
            let Some(mut component_id) = mcrust::net::minecraft::core::registries::built_in_registries::data_component_type().get_key(&component_type) else {
                return Default::default(); // java: return null
            };
            let component = &entry.1;
            if component.is_none() {
                return format!("!{}", compat::jstring(&component_id));
            }
            let mut encoded_component: Option<Tag> = (TypedDataComponent::create_unchecked(component_type.clone(), component.clone().unwrap())).encode_value(&nbt_registry_ops).result().unwrap();
            if encoded_component.is_none() {
                return Default::default(); // java: return null
            }
            return format!("{}={}", compat::jstring(&component_id), compat::jstring(&encoded_component.clone().unwrap()));
        }).filter(|__a0| true).collect::<Vec<_>>()));
    }

    /// java: public static ItemStack fromItemString(String itemString, int count)
    pub fn from_item_string(item_string: &str, mut count: i32) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        let mut reader: ItemParser = ItemParser::new(RegistryUtils::get_registry_wrapper_lookup());
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut result: ItemInput = reader.parse(&mut StringReader::new(item_string)).unwrap();
            let mut stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = mcrust::net::minecraft::world::item::item_stack::ItemStack::of(*result.item(), count);
            stack.apply_components_patch(result.components());
            return stack;
        })) {
            Ok(__v) => return __v,
            Err(_) => {
            }
        }
        return mcrust::net::minecraft::world::item::item_stack::empty();
    }

    /// java: public static ItemStack fromComponentsString(String itemId, int count, String componentsString)
    pub fn from_components_string(item_id: &str, mut count: i32, components_string: &str) -> mcrust::net::minecraft::world::item::item_stack::ItemStack {
        return ItemStackComponentizationFixer::from_item_string(&format!("{}{}", item_id, components_string), count);
    }

}
