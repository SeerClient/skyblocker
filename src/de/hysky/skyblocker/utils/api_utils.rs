//! Ported from de.hysky.skyblocker.utils.ApiUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::google::common::cache::cache_loader::CacheLoader;
use mcrust::com::google::common::cache::loading_cache::{CacheBuilder, LoadingCache};
use mcrust::net::minecraft::client::user::User;

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
            // java: try (ApiResponse response = Http.sendName2UuidRequest(name)) {..} catch (Exception e) - the request and
            // the parse both inside the try
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<String> {
                let response: crate::de::hysky::skyblocker::utils::http::ApiResponse = crate::de::hysky::skyblocker::utils::http::Http::send_name2_uuid_request(name);
                if response.ok() {
                    // java: JsonParser.parseString(response.content()).getAsJsonObject().get("id").getAsString()
                    let json: serde_json::Value = serde_json::from_str(&response.content()).unwrap_or_else(|e| panic!("JsonSyntaxException: {e}"));
                    let object = json.as_object().unwrap_or_else(|| panic!("IllegalStateException: Not a JSON Object: {json}"));
                    return Some(compat::json::get_as_string(object.get("id").expect("NullPointerException")));
                } else if response.ratelimited() && retries < 3 {
                    std::thread::sleep(std::time::Duration::from_millis(800));
                    retries += 1;
                    return Some(ApiUtils::name2_uuid_internal(name, retries));
                }
                None
            }));
            match result {
                Ok(Some(uuid)) => return uuid,
                Ok(None) => {}
                Err(e) => {
                    logger().error(&format!("[Skyblocker] Name to uuid lookup failed! Name: {} {}", name, compat::panic_message(&e)));
                }
            }
        }
        return "".to_string();
    }

}
