//! Ported from de.hysky.skyblocker.utils.Http by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static final String NAME_2_UUID = "https://api.minecraftservices.com/minecraft/profile/lookup/name/";
pub const NAME_2_UUID: &str = "https://api.minecraftservices.com/minecraft/profile/lookup/name/";
/// java: private static final String HYPIXEL_PROXY = "https://hysky.de/api/hypixel/v2/";
pub const HYPIXEL_PROXY: &str = "https://hysky.de/api/hypixel/v2/";
/// java: private static final Duration REQUEST_TIMEOUT = Duration.ofSeconds(60);
pub static REQUEST_TIMEOUT: compat::MainThread<std::time::Duration> = compat::MainThread::new(|| std::time::Duration::from_secs((60) as u64));
// TODO(fabric2seer): expression not parsed
//   java:
/// java: 
pub static USER_AGENT: LazyLock<String> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `USER_AGENT` - see the TODO above"));

/// java: public class Http
#[derive(Clone)]
pub struct Http {}

impl Http {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: public record ApiResponse(String content, int statusCode, String[] cacheStatuses, int age) implements AutoCloseable
#[derive(Clone, Debug, PartialEq)]
pub struct ApiResponse {
    /// java: String content
    pub content: String,
    /// java: int statusCode
    pub status_code: i32,
    /// java: String[] cacheStatuses
    pub cache_statuses: Vec<String>,
    /// java: int age
    pub age: i32,
}

impl ApiResponse {
    /// java: the canonical record constructor
    pub fn new(content: String, status_code: i32, cache_statuses: Vec<String>, age: i32) -> Self {
        Self { content, status_code, cache_statuses, age }
    }

    /// java: the record accessor `content()`
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// java: the record accessor `statusCode()`
    pub fn status_code(&self) -> i32 {
        self.status_code
    }

    /// java: the record accessor `cacheStatuses()`
    pub fn cache_statuses(&self) -> Vec<String> {
        self.cache_statuses.clone()
    }

    /// java: the record accessor `age()`
    pub fn age(&self) -> i32 {
        self.age
    }

    /// java: public boolean ok()
    pub fn ok(&self) -> bool {
        return self.status_code == 200;
    }

    /// java: public boolean ratelimited()
    pub fn ratelimited(&self) -> bool {
        return self.status_code == 429;
    }

    /// java: public boolean cached()
    pub fn cached(&self) -> bool {
        return self.cache_statuses[0] == "HIT" || self.cache_statuses[1] == "HIT";
    }

    /// java: public void close() - "Maybe one day we'll have some resources to free"
    pub fn close(&self) {}
}
