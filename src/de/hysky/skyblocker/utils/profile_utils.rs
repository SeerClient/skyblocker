//! Ported from de.hysky.skyblocker.utils.ProfileUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::api_utils::ApiUtils;
use crate::entry::skyblocker_mod;
use mcrust::com::google::common::cache::cache_loader::CacheLoader;
use mcrust::com::google::common::cache::loading_cache::{CacheBuilder, LoadingCache};
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

//   java: private static final LoadingCache<String, JsonObject> UUID_TO_PROFILES_CACHE = CacheBuilder.newBuilder()
//   java: 			.expireAfterWrite(Duration.ofMinutes(5L))
//   java: 			.build(new CacheLoader<>() {
//   java: 				@Override
//   java: 				public JsonObject load(String uuid) throws Exception {
//   java: 					return fetchProfilesInternal(uuid);
//   java: 				}
//   java: 			});
/// java: private static final LoadingCache<String, JsonObject> UUID_TO_PROFILES_CACHE = CacheBuilder.newBuilder() .expireAfterWrite(Duration.ofMinutes(5L)) .build(new CacheLoader<>() { @Override public JsonObj
pub static UUID_TO_PROFILES_CACHE: LazyLock<LoadingCache<String, serde_json::Map<String, serde_json::Value>, UuidToProfilesLoader>> = LazyLock::new(|| {
    CacheBuilder::new_builder().expire_after_write(std::time::Duration::from_secs(5 * 60)).build(UuidToProfilesLoader)
});

/// java: the anonymous `new CacheLoader<>() { public JsonObject load(String uuid) }` of UUID_TO_PROFILES_CACHE
pub struct UuidToProfilesLoader;

impl CacheLoader<String, serde_json::Map<String, serde_json::Value>> for UuidToProfilesLoader {
    fn load(&self, uuid: &String) -> serde_json::Map<String, serde_json::Value> {
        // a null answer is Guava's InvalidCacheLoadException
        return ProfileUtils::fetch_profiles_internal(uuid)
            .unwrap_or_else(|| panic!("InvalidCacheLoadException: CacheLoader returned null for key {uuid}."));
    }
}

/// `supplyAsync`'s task: a throw completes the future exceptionally.
fn supply<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|e| compat::panic_message(&e))
}

/// java: public class ProfileUtils
#[derive(Clone)]
pub struct ProfileUtils {}

impl ProfileUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static CompletableFuture<@Nullable JsonObject> fetchProfileMember(String name)
    pub fn fetch_profile_member(name: &str) -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<serde_json::Map<String, serde_json::Value>>>> {
        let name: String = name.to_string();
        return Some(CompletableFuture::supply_async(move || supply(|| {
            let mut uuid: String = ApiUtils::name2_uuid(&name);
            if !uuid.is_empty() {
                let mut profile: serde_json::Map<String, serde_json::Value> = UUID_TO_PROFILES_CACHE.get_unchecked(&uuid);
                let mut player: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_array_of(&profile, "profiles")).clone().into_iter().map(|__a0| compat::json::get_as_json_object(&__a0)).filter(|profile_obj| compat::json::get_as_boolean(&(compat::json::get_as_json_primitive_of(&profile_obj, "selected")))).next().expect("Optional.orElseThrow"), "members")).get(&uuid).cloned().unwrap());
                return Some(player);
            }
            return None; // java: return null
        }), compat::executor));
    }

    /// java: public static CompletableFuture<@Nullable JsonObject> fetchFullProfile(String name)
    pub fn fetch_full_profile(name: &str) -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<serde_json::Map<String, serde_json::Value>>>> {
        let name: String = name.to_string();
        return Some(CompletableFuture::supply_async(move || supply(|| {
            let mut uuid: String = ApiUtils::name2_uuid(&name);
            return (if !uuid.is_empty() { Some(UUID_TO_PROFILES_CACHE.get_unchecked(&uuid)) } else { None });
        }), compat::executor));
    }

    /// java: public static CompletableFuture<@Nullable JsonObject> fetchFullProfileByUuid(String uuid)
    pub fn fetch_full_profile_by_uuid(uuid: &str) -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<serde_json::Map<String, serde_json::Value>>>> {
        let uuid: String = uuid.to_string();
        return Some(CompletableFuture::supply_async(move || supply(|| {
            return (if !uuid.is_empty() { Some(UUID_TO_PROFILES_CACHE.get_unchecked(&uuid)) } else { None });
        }), compat::executor));
    }

    /// java: private static @Nullable JsonObject fetchProfilesInternal(String uuid)
    pub fn fetch_profiles_internal(uuid: &str) -> Option<serde_json::Map<String, serde_json::Value>> {
        {
            // java: try (Http.ApiResponse response = Http.sendHypixelRequest("skyblock/profiles", "?uuid=" + uuid)) {..} catch (Exception e)
            // - the request is inside the try: its failure is caught too
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let response: crate::de::hysky::skyblocker::utils::http::ApiResponse = crate::de::hysky::skyblocker::utils::http::Http::send_hypixel_request("skyblock/profiles", &format!("?uuid={}", uuid));
                if !response.ok() {
                    panic!("IllegalStateException: {}", format!("Failed to get profile for player: {}!, Status Code: {}, Response: {}", uuid, response.status_code(), response.content()));
                }
                // java: SkyblockerMod.GSON.fromJson(response.content(), JsonObject.class) - a JSON null is Java's null, a non-object throws
                let __json: serde_json::Value = serde_json::from_str(&response.content()).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                return (if __json.is_null() { None } else { Some(compat::json::get_as_json_object(&__json)) });
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    logger().error(&format!("[Skyblocker Profile Utils] Failed to get Player Profile Data for player {}, is the API Down/Limited?", uuid));
                }
            }
        }
        return None;
    }

}
