//! Ported from de.hysky.skyblocker.skyblock.item.SkyblockItemData by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::player_head_hash_cache::PlayerHeadHashCache;
use crate::de::hysky::skyblocker::utils::networth::networth_data_suppliers::NetworthDataSuppliers;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: public class SkyblockItemData
#[derive(Clone)]
pub struct SkyblockItemData {}

impl SkyblockItemData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: updateItems().thenAcceptAsync(items -> { PlayerHeadHashCache.loadSkins(items); NetworthDataSuppliers.updateSkyblockItemData(items); });
        // (`thenAcceptAsync(action)` runs on the default async pool: a worker thread, as `compat::executor` hands out)
        let _ = SkyblockItemData::update_items().then_accept_async(
            |items: &Vec<serde_json::Value>| {
                PlayerHeadHashCache::load_skins(items);
                NetworthDataSuppliers::update_skyblock_item_data(items);
            },
            std::sync::Arc::new(|task: Box<dyn FnOnce() + Send>| compat::executor(task)),
        );
    }

    /// java: private static CompletableFuture<JsonArray> updateItems()
    pub fn update_items() -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Vec<serde_json::Value>> {
        return CompletableFuture::supply_async(move || Ok((|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // java: String response = Http.sendGetRequest("https://api.hypixel.net/v2/resources/skyblock/items");
                let mut response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://api.hypixel.net/v2/resources/skyblock/items");
                let mut items_data: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException")));
                return compat::json::get_as_json_array_of(&items_data, "items");
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    logger().error(&format!("[Skyblocker Item Data Loader] Failed to load items data from the Hypixel API!"));
                }
            }
            panic!("IllegalStateException");
        })()), compat::executor);
    }

}
