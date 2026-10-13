//! Ported from de.hysky.skyblocker.utils.networth.NetworthCalculator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::net::minecraft::nbt::tag::Tag;
use crate::de::hysky::skyblocker::injected::skyblocker_stack::SkyblockerStack as _;

/// java: public class NetworthCalculator
#[derive(Clone)]
pub struct NetworthCalculator {}

impl NetworthCalculator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static NetworthResult getItemNetworth(ItemStack stack)
    pub fn get_item_networth(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) -> crate::unresolved::NetworthResult {
        return NetworthCalculator::get_item_networth_2(stack, stack.get_count());
    }

    /// java: public static NetworthResult getItemNetworth(ItemStack stack, int count)
    pub fn get_item_networth_2(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, mut count: i32) -> crate::unresolved::NetworthResult {
        // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut item_id: String = stack.get_skyblock_id();
            //   java: CompoundTag customData = stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            use mcrust::net::minecraft::core::component::data_component_getter::DataComponentGetter as _;
            let custom_data: Tag = stack.get(mcrust::net::minecraft::core::component::data_components::custom_data()).and_then(|o| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::item::component::custom_data::CustomData>(&o).map(|data| data.copy_tag())).unwrap_or_else(|| mcrust::net::minecraft::world::item::component::custom_data::empty().copy_tag());
            //   java: Dynamic<?> customDataDynamic = new Dynamic<>(NbtOps.INSTANCE, customData);
            let custom_data_dynamic: Dynamic = Dynamic::new_with(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance(), Some(mcrust::com::mojang::serialization::obj(custom_data.clone())));
            //   java: SkyblockItemStack skyblockItemStack = SkyblockItemStack.of(itemId, count, customDataDynamic, SkyblockItemMetadataRetriever.of(customData, itemId));
            //   java: return ItemCalculator.calculate(skyblockItemStack, NetworthDataSuppliers::getPrice, NetworthDataSuppliers.getSkyblockItemData());
            // rust: port gap - the networth library (`net.azureaaron.networth`: `SkyblockItemStack`, `ItemCalculator`,
            // `NetworthResult`) is `crate::unresolved` placeholders with no calculator behind them (GAPS_M #18): the
            // answer is the placeholder result.
            let _ = (item_id, count, custom_data_dynamic);
            crate::unresolved::NetworthResult
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                //   java: return new NetworthResult(0, 0, List.of());
                return crate::unresolved::NetworthResult;
            }
        }
    }

}
