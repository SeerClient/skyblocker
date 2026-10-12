//! Ported from de.hysky.skyblocker.utils.ApiUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::google::common::cache::loading_cache::LoadingCache;
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
pub static NAME_2_UUID_CACHE: compat::MainThread<LoadingCache<String, String, compat::Object>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `NAME_2_UUID_CACHE` - see the TODO above"));

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
        // TODO(fabric2seer): `LoadingCache.getUnchecked` has no Rust method in the port (looked for `get_unchecked` in LoadingCache and its supertypes)
        // rust: return NAME_2_UUID_CACHE.get_mut().get_unchecked(name);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private static String name2UuidInternal(String name, int retries)
    pub fn name2_uuid_internal(name: &str, mut retries: i32) -> String {
        let mut session: User = compat::minecraft().get_user();
        if compat::equals_ignore_case(&session.get_name(), name) {
            // TODO(fabric2seer): static `UndashedUuid.toString` has no Rust counterpart in the port
            // rust: return UndashedUuid::to_string(session.get_profile_id());
            return todo!("fabric2seer: see the TODO above");
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
