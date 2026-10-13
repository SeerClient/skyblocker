//! Ported from de.hysky.skyblocker.utils.networth.NetworthDataSuppliers by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;
use crate::net::azureaaron::networth::data::skyblock_item_data::{self, SkyblockItemData};
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: private static Object2ObjectMap<String, SkyblockItemData> itemData = Object2ObjectMaps.emptyMap();
/// - written by `updateSkyblockItemData` on the async pool and read on the render thread, as the Java field is: a
/// reference swap (`Arc`) behind a lock, the map itself plain data.
pub static ITEM_DATA: std::sync::LazyLock<std::sync::RwLock<std::sync::Arc<std::collections::HashMap<String, SkyblockItemData>>>> = std::sync::LazyLock::new(|| std::sync::RwLock::new(std::sync::Arc::new(std::collections::HashMap::new())));

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
            // java: itemData = SkyblockItemData.MAP_CODEC.parse(JsonOps.INSTANCE, items).getOrThrow();
            let parsed = skyblock_item_data::MAP_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&serde_json::Value::Array(items.clone()))).get_or_throw().unwrap_or_else(|e| panic!("IllegalStateException: {e}"));
            *ITEM_DATA.write().unwrap_or_else(|e| e.into_inner()) = std::sync::Arc::new(parsed);
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Networth Data Supplier] Failed to parse items data! {}", compat::panic_message(&e)));
        }
    }

    /// java: static Object2ObjectMap<String, SkyblockItemData> getSkyblockItemData()
    pub fn get_skyblock_item_data() -> std::sync::Arc<std::collections::HashMap<String, SkyblockItemData>> {
        // java: return itemData; - the map itself (the reference the field holds now)
        return ITEM_DATA.read().unwrap_or_else(|e| e.into_inner()).clone();
    }

    /// java: static double getPrice(String id)
    pub fn get_price(id: &str) -> f64 {
        return ItemUtils::get_item_price_3(Some(id.to_string()), true, true).unwrap_or(((0) as f64) as f64);
    }

}
