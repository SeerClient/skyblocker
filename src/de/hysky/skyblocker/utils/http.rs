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
