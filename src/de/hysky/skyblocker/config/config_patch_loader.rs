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
                //   java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE).then(ClientCommands.literal("updateConfigPatches").executes(context -> {
                //   java: 	Minecraft minecraft = context.getSource().getClient();
                //   java: 	fetchPatchList().thenAcceptAsync(result -> { if (minecraft.player != null) { minecraft.player.sendSystemMessage(Constants.PREFIX.get()
                //   java: 		.append(Component.translatable(result ? "skyblocker.config.patches.updated.success" : "skyblocker.config.patches.updated.failed"))); } }, minecraft);
                //   java: 	return Command.SINGLE_SUCCESS; })));
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut update_config_patches = client_command_registration_callback::literal("updateConfigPatches");
                update_config_patches.executes(Arc::new(|_: &mcrust::com::mojang::brigadier::context::command_context::CommandContext<client_command_registration_callback::FabricClientCommandSource>| {
                    if let Some(future) = ConfigPatchLoader::fetch_patch_list() {
                        future.then_accept_async(|result: &bool| {
                            if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
                                let key: &str = if *result { "skyblocker.config.patches.updated.success" } else { "skyblocker.config.patches.updated.failed" };
                                compat::batch_w_send_feedback(&(crate::de::hysky::skyblocker::utils::constants::PREFIX.get())().append(mcrust::net::minecraft::network::chat::component::translatable(key)));
                            }
                        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t)));
                    }
                    Ok(1)
                }));
                let mut root = client_command_registration_callback::literal(skyblocker_mod::NAMESPACE);
                root.then(&update_config_patches);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: private static CompletableFuture<Boolean> fetchPatchList()
    pub fn fetch_patch_list() -> Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<bool>> {
        return Some((CompletableFuture::supply_async(move || Ok((|| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // java: String response = Http.sendGetRequest("https://api.azureaaron.net/skyblocker/configpatches");
                let mut response: String = crate::de::hysky::skyblocker::utils::http::Http::send_get_request("https://api.azureaaron.net/skyblocker/configpatches");
                return serde_json::from_str::<serde_json::Value>(&response).expect("JsonSyntaxException");
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    //   java: LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Patch Loader] Failed to load config patches!", e);
                    logger().error(&format!("[Skyblocker Config Patch Loader] Failed to load config patches!\n{}", compat::panic_message(&e)));
                    return Default::default(); // java: return null
                }
            }
        })()), compat::executor)).then_apply_async(|json| {
            // java: List<ConfigPatch> patches = ConfigPatch.PATCH_LIST_CODEC.parse(JsonOps.INSTANCE, json)
            //       .setPartial(List.of())
            //       .resultOrPartial(error -> LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Patch Loader] Failed to parse config patches! Error: {}", error))
            //       .get();
            let patches: Vec<Arc<dyn dandelion::net::azureaaron::dandelion::api::patching::config_patch::ConfigPatch>> = {
                use mcrust::com::mojang::serialization::decoder::Decoder as _;
                let codec = dandelion::net::azureaaron::dandelion::api::patching::config_patch::PATCH_LIST_CODEC.get().clone();
                codec
                    .parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &dandelion::compat::json_to_port(&json))
                    .set_partial(Vec::new())
                    .result_or_partial_with(|error| logger().error(&format!("[Skyblocker Config Patch Loader] Failed to parse config patches! Error: {}", error)))
                    .unwrap_or_default()
            };
            logger().info("[Skyblocker Config Patch Loader] Successfully loaded config patches.");
            SkyblockerConfigManager::set_patches(&patches);
            return true;
        }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| crate::compat::execute(__t))).exceptionally(|e| {
            //   java: LOGGER.error(LogUtils.FATAL_MARKER, "[Skyblocker Config Patch Loader] Failed to load config patches!", e);
            logger().error(&format!("[Skyblocker Config Patch Loader] Failed to load config patches!\n{}", e));
            return false;
        }));
    }

}
