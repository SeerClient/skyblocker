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
            // TODO(fabric2seer): `.copyTag(..)` on a value of unknown type
            //   java: CompoundTag customData = stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            let mut custom_data: Tag = todo!("fabric2seer: the initialiser of `customData` (TODO above)");
            // TODO(fabric2seer): `new` takes 1 argument(s) in the port (`ops: Ops`), Java passes 2
            // rust: let mut custom_data_dynamic: Dynamic = Dynamic::new(mcrust::net::minecraft::nbt::nbt_ops::NbtOps::instance(), custom_data.clone());
            let mut custom_data_dynamic: Dynamic = todo!("fabric2seer: the initialiser of `customDataDynamic` (TODO above)");
            // TODO(fabric2seer): static call SkyblockItemStack.of(..)
            //   java: SkyblockItemStack skyblockItemStack = SkyblockItemStack.of(itemId, count, customDataDynamic, SkyblockItemMetadataRetriever.of(customData, itemId));
            let mut skyblock_item_stack: crate::unresolved::SkyblockItemStack = todo!("fabric2seer: the initialiser of `skyblockItemStack` (TODO above)");
            // TODO(fabric2seer): static call ItemCalculator.calculate(..)
            //   java: return ItemCalculator.calculate(skyblockItemStack, NetworthDataSuppliers::getPrice, NetworthDataSuppliers.getSkyblockItemData());
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                // TODO(fabric2seer): `new NetworthResult(..)`
                //   java: return new NetworthResult(0, 0, List.of());
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
