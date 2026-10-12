//! Ported from de.hysky.skyblocker.utils.networth.NetworthDataSuppliers by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;

// TODO(fabric2seer): static call Object2ObjectMaps.emptyMap(..)
//   java: private static Object2ObjectMap<String, SkyblockItemData> itemData = Object2ObjectMaps.emptyMap();
/// java: private static Object2ObjectMap<String, SkyblockItemData> itemData = Object2ObjectMaps.emptyMap();
pub static ITEM_DATA: compat::MainThread<std::collections::HashMap<String, crate::unresolved::SkyblockItemData>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `itemData` - see the TODO above"));

/// java: public class NetworthDataSuppliers
#[derive(Clone)]
pub struct NetworthDataSuppliers {}

impl NetworthDataSuppliers {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void updateSkyblockItemData(JsonArray items)
    pub fn update_skyblock_item_data(items: &Vec<serde_json::Value>) {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static field SkyblockItemData.MAP_CODEC
            //   java: itemData = SkyblockItemData.MAP_CODEC.parse(JsonOps.INSTANCE, items).getOrThrow();
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Networth Data Supplier] Failed to parse items data!"));
        }
    }

    /// java: static Object2ObjectMap<String, SkyblockItemData> getSkyblockItemData()
    pub fn get_skyblock_item_data() -> std::collections::HashMap<String, crate::unresolved::SkyblockItemData> {
        return ITEM_DATA.get_mut().clone();
    }

    /// java: static double getPrice(String id)
    pub fn get_price(id: &str) -> f64 {
        return ItemUtils::get_item_price_3(Some(id.to_string()), true, true).unwrap_or(((0) as f64) as f64);
    }

}
