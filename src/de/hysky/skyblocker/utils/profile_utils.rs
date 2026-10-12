//! Ported from de.hysky.skyblocker.utils.ProfileUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::api_utils::ApiUtils;
use crate::entry::skyblocker_mod;
use mcrust::com::google::common::cache::loading_cache::LoadingCache;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

// TODO(fabric2seer): static call CacheBuilder.newBuilder(..)
//   java: private static final LoadingCache<String, JsonObject> UUID_TO_PROFILES_CACHE = CacheBuilder.newBuilder()
//   java: 			.expireAfterWrite(Duration.ofMinutes(5L))
//   java: 			.build(new CacheLoader<>() {
//   java: 				@Override
//   java: 				public JsonObject load(String uuid) throws Exception {
//   java: 					return fetchProfilesInternal(uuid);
//   java: 				}
//   java: 			});
/// java: private static final LoadingCache<String, JsonObject> UUID_TO_PROFILES_CACHE = CacheBuilder.newBuilder() .expireAfterWrite(Duration.ofMinutes(5L)) .build(new CacheLoader<>() { @Override public JsonObj
pub static UUID_TO_PROFILES_CACHE: compat::MainThread<LoadingCache<String, serde_json::Map<String, serde_json::Value>, compat::Object>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `UUID_TO_PROFILES_CACHE` - see the TODO above"));

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
        return Some(CompletableFuture::supply_async(move || Ok((|| {
            let mut uuid: String = ApiUtils::name2_uuid(name);
            if !uuid.is_empty() {
                // TODO(fabric2seer): `LoadingCache.getUnchecked` has no Rust method in the port (looked for `get_unchecked` in LoadingCache and its supertypes)
                // rust: let mut profile: serde_json::Map<String, serde_json::Value> = UUID_TO_PROFILES_CACHE.get_mut().get_unchecked(&uuid);
                let mut profile: serde_json::Map<String, serde_json::Value> = todo!("fabric2seer: the initialiser of `profile` (TODO above)");
                let mut player: serde_json::Map<String, serde_json::Value> = compat::json::get_as_json_object(&(compat::json::get_as_json_object_of(&(compat::json::get_as_json_array_of(&profile, "profiles")).clone().into_iter().map(|__a0| compat::json::get_as_json_object(&__a0)).filter(|profile_obj| compat::json::get_as_boolean(&(compat::json::get_as_json_primitive_of(&profile_obj, "selected")))).next().expect("Optional.orElseThrow"), "members")).get(&uuid).cloned().unwrap());
                return player;
            }
            return Default::default(); // java: return null
        })()), compat::executor));
    }

    /// java: public static CompletableFuture<@Nullable JsonObject> fetchFullProfile(String name)
    pub fn fetch_full_profile(name: &str) -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<serde_json::Map<String, serde_json::Value>>>> {
        return Some(CompletableFuture::supply_async(move || Ok((|| {
            let mut uuid: String = ApiUtils::name2_uuid(name);
            // TODO(fabric2seer): `LoadingCache.getUnchecked` has no Rust method in the port (looked for `get_unchecked` in LoadingCache and its supertypes)
            // rust: return (if !uuid.is_empty() { Some(UUID_TO_PROFILES_CACHE.get_mut().get_unchecked(&uuid)) } else { None });
        })()), compat::executor));
    }

    /// java: public static CompletableFuture<@Nullable JsonObject> fetchFullProfileByUuid(String uuid)
    pub fn fetch_full_profile_by_uuid(uuid: &str) -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<serde_json::Map<String, serde_json::Value>>>> {
        return Some(CompletableFuture::supply_async(move || Ok((|| {
            // TODO(fabric2seer): `LoadingCache.getUnchecked` has no Rust method in the port (looked for `get_unchecked` in LoadingCache and its supertypes)
            // rust: return (if !uuid.is_empty() { Some(UUID_TO_PROFILES_CACHE.get_mut().get_unchecked(uuid)) } else { None });
        })()), compat::executor));
    }

    /// java: private static @Nullable JsonObject fetchProfilesInternal(String uuid)
    pub fn fetch_profiles_internal(uuid: &str) -> Option<serde_json::Map<String, serde_json::Value>> {
        {
            // TODO(fabric2seer): `Http.sendHypixelRequest` is not a static method of Http
            //   java: Http.ApiResponse response = Http.sendHypixelRequest(..);
            let mut response: crate::unresolved::ApiResponse = todo!("fabric2seer: the initialiser of `response` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): the condition: `.ok(..)` on a ApiResponse
                //   java: !response.ok()
                if todo!("fabric2seer: the condition (TODO above)") {
                    // TODO(fabric2seer): `.statusCode(..)` on a ApiResponse
                    //   java: throw new IllegalStateException(String.format("Failed to get profile for player: %s!, Status Code: %d, Response: %s", uuid, response.statusCode(), response.content()));
                }
                // TODO(fabric2seer): `.fromJson(..)` on a Gson
                //   java: return SkyblockerMod.GSON.fromJson(response.content(), JsonObject.class);
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
