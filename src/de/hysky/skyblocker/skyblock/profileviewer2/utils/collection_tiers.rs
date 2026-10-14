//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.utils.CollectionTiers by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile::ApiProfile;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::java::util::uuid::Uuid;

/// java: private static Map<String, IntList> tiers = Map.of();
pub static TIERS: LazyLock<Mutex<std::collections::HashMap<String, Vec<i32>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::from([])));
/// java: private static Map<String, List<String>> categories = Map.of();
pub static CATEGORIES: LazyLock<Mutex<std::collections::HashMap<String, Vec<String>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::from([])));

/// java: public class CollectionTiers
#[derive(Clone)]
pub struct CollectionTiers {}

impl CollectionTiers {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        if !(*profile_viewer::ENABLED) {
            return;
        }
        (CompletableFuture::supply_async(move || Ok((|| CollectionTiers::load_collections_data())()), compat::executor)).then_accept_async(|__a0| CollectionTiers::parse_collections_data(__a0.clone()), std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t)));
    }

    /// java: public static boolean unlockedTier(ApiProfile profile, String collection, int tier)
    pub fn unlocked_tier(profile: &compat::Shared<ApiProfile>, collection: &str, mut tier: i32) -> bool {
        return (CollectionTiers::get_max_unlocked_tier(profile, collection) >= tier);
    }

    /// java: public static int getMaxUnlockedTier(ApiProfile profile, String collection)
    pub fn get_max_unlocked_tier(profile: &compat::Shared<ApiProfile>, collection: &str) -> i32 {
        // TODO(fabric2seer): static field Map.Entry
        //   java: long collected = profile.members.entrySet().stream()
        //   java: 				.map(Map.Entry::getValue)
        //   java: 				.mapToLong(member -> member.collection.getOrDefault(collection, 0L))
        //   java: 				.sum();
        let mut collected: i64 = { let __members: Vec<compat::Shared<crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember>> = profile.lock().borrow().members.values().cloned().collect(); __members.iter().fold(0i64, |sum, m| sum.wrapping_add(m.lock().borrow().collection.get(collection).cloned().unwrap_or(0))) };
        return CollectionTiers::get_max_unlocked_tier_string_long(collection, collected);
    }

    /// java: private static int getMaxUnlockedTier(String collection, long collected)
    pub fn get_max_unlocked_tier_string_long(collection: &str, mut collected: i64) -> i32 {
        let mut collection_tiers: Vec<i32> = compat::lock(&TIERS).get(collection).cloned().unwrap_or_else(|| vec![]);
        if !collection_tiers.is_empty() {
            {
                let mut i: i32 = (collection_tiers.len() as i32).wrapping_sub(1);
                while i >= 0 {
                    let mut amount_required: i32 = collection_tiers[(i) as usize];
                    if (amount_required as i64) <= collected {
                        return i.wrapping_add(1);
                    }
                    i -= 1;
                }
            }
        }
        return 0;
    }

    /// java: public static int getMaxTier(String collection)
    pub fn get_max_tier(collection: &str) -> i32 {
        return ((compat::lock(&TIERS).get(collection).cloned().unwrap_or_else(|| vec![])).len() as i32);
    }

    /// java: public static Map<String, List<String>> getCollectionCategoryContents()
    pub fn get_collection_category_contents() -> std::collections::HashMap<String, Vec<String>> {
        return compat::get_state(&CATEGORIES);
    }

    /// java: public static Report getCollectionReport(ApiProfile profile, UUID member, String collection)
    pub fn get_collection_report(profile: &compat::Shared<ApiProfile>, member: &Uuid, collection: &str) -> Report {
        let mut personal: i64 = profile.lock().borrow().members.get(member).cloned().unwrap().lock().borrow().collection.get(collection).cloned().unwrap_or_else(|| 0);
        // TODO(fabric2seer): static field Map.Entry
        //   java: long coop = profile.members.entrySet().stream()
        //   java: 				.filter(entry -> !entry.getKey().equals(member))
        //   java: 				.map(Map.Entry::getValue)
        //   java: 				.mapToLong(profileMember -> profileMember.collection.getOrDefault(collection, 0L))
        //   java: 				.sum();
        let mut coop: i64 = { let __members: Vec<compat::Shared<crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember>> = profile.lock().borrow().members.iter().filter(|(k, _)| *k != member).map(|(_, v)| v.clone()).collect(); __members.iter().fold(0i64, |sum, m| sum.wrapping_add(m.lock().borrow().collection.get(collection).cloned().unwrap_or(0))) };
        let mut total: i64 = personal.wrapping_add(coop);
        let mut tier: i32 = CollectionTiers::get_max_unlocked_tier_string_long(collection, total);
        return Report::new(personal, coop, total, tier);
    }

    /// java: private static @Nullable JsonElement loadCollectionsData()
    pub fn load_collections_data() -> Option<serde_json::Value> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
            //   java: String response = Http.sendGetRequest("https://api.hypixel.net/v2/resources/skyblock/collections");
            let mut response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://api.hypixel.net/v2/resources/skyblock/collections");
            return Some(serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException"));
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to load collections data."));
                return None;
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static void parseCollectionsData(@Nullable JsonElement data)
    pub fn parse_collections_data(mut data: Option<serde_json::Value>) {
        let Some(mut data) = data.clone() else {
            return;
        };
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut parsed_tiers: std::collections::HashMap<String, Vec<i32>> = std::collections::HashMap::new();
            let mut parsed_categories: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
            let mut collections: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&compat::json::get_as_json_object(&data), "collections");
            for type_entry in collections.iter().map(|(k, v)| (k.clone(), v.clone())) {
                let mut type_name: String = type_entry.0.clone();
                let mut items: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object_of(&compat::json::get_as_json_object(&type_entry.1), "items");
                for items_entry in items.iter().map(|(k, v)| (k.clone(), v.clone())) {
                    let mut item_name: String = items_entry.0.clone();
                    let mut item: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&items_entry.1);
                    let mut item_tiers: Vec<i32> = compat::sorted_by((compat::json::get_as_json_array_of(&item, "tiers")).clone().into_iter().map(|__a0| compat::json::get_as_json_object(&__a0)), |o1, o2| (compat::ord_int(compat::json::get_as_int(&o1.get("tier").cloned().unwrap()).cmp(&compat::json::get_as_int(&o2.get("tier").cloned().unwrap())))).cmp(&0)).map(|tier| compat::json::get_as_int(&tier.get("amountRequired").cloned().unwrap())).map(|__a0| __a0).collect::<Vec<_>>();
                    parsed_tiers.insert(item_name.clone(), item_tiers.to_vec());
                }
                // TODO(fabric2seer): static field Map.Entry
                //   java: List<String> collectionItems = items.entrySet().stream()
                //   java: 						.sorted((o1, o2) -> {
                //   java: 							String name1 = o1.getValue().getAsJsonObject().get("name").getAsString();
                //   java: 							String name2 = o2.getValue().getAsJsonObject().get("name").getAsString();
                //   java:
                //   java: 							return String.CASE_INSENSITIVE_ORDER.compare(name1, name2);
                //   java: 						})
                //   java: 						.map(Map.Entry::getKey)
                //   java: 						.toList();
                let mut collection_items: Vec<String> = todo!("fabric2seer: the initialiser of `collectionItems` (TODO above)");
                parsed_categories.insert(type_name.clone(), collection_items.clone());
            }
            compat::set_state(&TIERS, parsed_tiers.clone());
            compat::set_state(&CATEGORIES, parsed_categories.clone());
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Profile Viewer] Failed to parse collections data."));
        }
    }

}

/// java: public record Report(long personal, long coop, long total, int tier)
#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    /// java: long personal
    pub personal: i64,
    /// java: long coop
    pub coop: i64,
    /// java: long total
    pub total: i64,
    /// java: int tier
    pub tier: i32,
}

impl Report {
    /// java: the canonical record constructor
    pub fn new(mut personal: i64, mut coop: i64, mut total: i64, mut tier: i32) -> Self {
        Self { personal, coop, total, tier }
    }

    /// java: the record accessor `personal()`
    pub fn personal(&self) -> i64 {
        self.personal
    }

    /// java: the record accessor `coop()`
    pub fn coop(&self) -> i64 {
        self.coop
    }

    /// java: the record accessor `total()`
    pub fn total(&self) -> i64 {
        self.total
    }

    /// java: the record accessor `tier()`
    pub fn tier(&self) -> i32 {
        self.tier
    }

}
