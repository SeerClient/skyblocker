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
        // TODO(fabric2seer): `then_accept_async` takes 2 argument(s) in the port (`action: F, executor: Executor`), Java passes 1
        // rust: SkyblockItemData::update_items().then_accept_async(|items| {
        // rust: PlayerHeadHashCache::load_skins(&items);
        // rust: NetworthDataSuppliers::update_skyblock_item_data(&items);
        // rust: });
    }

    /// java: private static CompletableFuture<JsonArray> updateItems()
    pub fn update_items() -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Vec<serde_json::Value>> {
        return CompletableFuture::supply_async(move || Ok((|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
                //   java: String response = Http.sendGetRequest("https://api.hypixel.net/v2/resources/skyblock/items");
                let mut response: String = todo!("fabric2seer: the initialiser of `response` (TODO above)");
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
