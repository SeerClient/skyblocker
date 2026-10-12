//! Ported from de.hysky.skyblocker.utils.ApiUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::google::common::cache::cache_loader::CacheLoader;
use mcrust::com::google::common::cache::loading_cache::{CacheBuilder, LoadingCache};
use mcrust::net::minecraft::client::user::User;

// TODO(fabric2seer): static call CacheBuilder.newBuilder(..)
//   java: private static final LoadingCache<String, String> NAME_2_UUID_CACHE = CacheBuilder.newBuilder()
//   java: 			.expireAfterWrite(Duration.ofMinutes(20L))
//   java: 			.build(new CacheLoader<>() {
//   java: 				@Override
//   java: 				public String load(String key) throws Exception {
//   java: 					return name2UuidInternal(key, 0);
//   java: 				}
//   java: 			});
/// java: private static final LoadingCache<String, String> NAME_2_UUID_CACHE = CacheBuilder.newBuilder() .expireAfterWrite(Duration.ofMinutes(20L)) .build(new CacheLoader<>() { @Override public String load(Str
pub static NAME_2_UUID_CACHE: LazyLock<LoadingCache<String, String, Name2UuidLoader>> = LazyLock::new(|| {
    CacheBuilder::new_builder().expire_after_write(std::time::Duration::from_secs(20 * 60)).build(Name2UuidLoader)
});

/// java: the anonymous `new CacheLoader<>() { public String load(String key) }` of NAME_2_UUID_CACHE
pub struct Name2UuidLoader;

impl CacheLoader<String, String> for Name2UuidLoader {
    fn load(&self, key: &String) -> String {
        return ApiUtils::name2_uuid_internal(key, 0);
    }
}

/// java: public class ApiUtils
#[derive(Clone)]
pub struct ApiUtils {}

impl ApiUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static String name2Uuid(String name)
    pub fn name2_uuid(name: &str) -> String {
        return NAME_2_UUID_CACHE.get_unchecked(&name.to_string());
    }

    /// java: private static String name2UuidInternal(String name, int retries)
    pub fn name2_uuid_internal(name: &str, mut retries: i32) -> String {
        let session = compat::minecraft().get_user();
        if compat::equals_ignore_case(&session.get_name(), name) {
            // java: UndashedUuid.toString(session.getProfileId()) - the uuid's string without its dashes
            return session.get_profile_id().replace('-', "");
        }
        {
            // TODO(fabric2seer): `Http.sendName2UuidRequest` is not a static method of Http
            //   java: ApiResponse response = Http.sendName2UuidRequest(..);
            let mut response: crate::unresolved::ApiResponse = todo!("fabric2seer: the initialiser of `response` (TODO above)");
            // java: try {..} catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
            {
                // TODO(fabric2seer): `.ratelimited(..)` on a ApiResponse
                //   java: if (response.ok()) {
                //   java: 				return JsonParser.parseString(response.content()).getAsJsonObject().get("id").getAsString();
                //   java: 			} else if (response.ratelimited() && retries < 3) {
                //   java: 				Thread.sleep(800);
                //   java:
                //   java: 				return name2UuidInternal(name, ++retries);
                //   java: 			}
            }
            // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
            //   java: {
            //   java: 			LOGGER.error("[Skyblocker] Name to uuid lookup failed! Name: {}", name, e);
            //   java: 		}
        }
        return "".to_string();
    }

}
