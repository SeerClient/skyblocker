//! Ported from de.hysky.skyblocker.config.ConfigPatchLoader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::entry::skyblocker_mod;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: private static final int UPDATE_INTERVAL = 1800 * 20;
pub const UPDATE_INTERVAL: i32 = 1800i32.wrapping_mul(20);

/// java: public class ConfigPatchLoader
#[derive(Clone)]
pub struct ConfigPatchLoader {}

impl ConfigPatchLoader {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { ConfigPatchLoader::fetch_patch_list(); }), UPDATE_INTERVAL) });
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: private static CompletableFuture<Boolean> fetchPatchList()
    pub fn fetch_patch_list() -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<bool>> {
        return Some((CompletableFuture::supply_async(move || Ok((|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `Http.sendGetRequest` is not a static method of Http
                //   java: String response = Http.sendGetRequest("https://api.azureaaron.net/skyblocker/configpatches");
                let mut response: String = todo!("fabric2seer: the initialiser of `response` (TODO above)");
                return serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException");
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    // TODO(fabric2seer): a log call with a non-literal format
                    //   java: LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Patch Loader] Failed to load config patches!", e);
                    return Default::default(); // java: return null
                }
            }
        })()), compat::executor)).then_apply_async(|json| {
            // TODO(fabric2seer): static field ConfigPatch.PATCH_LIST_CODEC
            //   java: List<ConfigPatch> patches = ConfigPatch.PATCH_LIST_CODEC.parse(JsonOps.INSTANCE, json)
            //   java: 					.setPartial(List.of())
            //   java: 					.resultOrPartial(error -> LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Patch Loader] Failed to parse config patches! Error: {}", error))
            //   java: 					.get();
            let mut patches: Vec<crate::unresolved::ConfigPatch> = todo!("fabric2seer: the initialiser of `patches` (TODO above)");
            logger().info("[Skyblocker Config Patch Loader] Successfully loaded config patches.");
            SkyblockerConfigManager::set_patches(&patches);
            return true;
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t))).exceptionally(|e| {
            // TODO(fabric2seer): a log call with a non-literal format
            //   java: LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Patch Loader] Failed to load config patches!", e);
            return false;
        }));
    }

}
