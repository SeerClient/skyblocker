//! Ported from de.hysky.skyblocker.SkyblockerMod by fabric2seer. Entrypoint: client.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::config::backup::config_backup_manager::ConfigBackupManager;
use crate::de::hysky::skyblocker::config::config_null_fields_fix::ConfigNullFieldsFix;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::backpack_preview::BackpackPreview;
use crate::de::hysky::skyblocker::skyblock::tabhud::util::player_list_manager::PlayerListManager;
use crate::de::hysky::skyblocker::utils::discord::discord_rpc_manager::DiscordRPCManager;
use crate::de::hysky::skyblocker::utils::scheduler::message_scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public static final String NAMESPACE = "skyblocker";
pub const NAMESPACE: &str = "skyblocker";
/// java: public static final ModContainer SKYBLOCKER_MOD = FabricLoader.getInstance().getModContainer(NAMESPACE).orElseThrow();
pub static SKYBLOCKER_MOD: compat::MainThread<std::sync::Arc<mcrust::mcrust::modding::loader::ModContainer>> = compat::MainThread::new(|| mcrust::mcrust::modding::loader::get_mod(NAMESPACE).unwrap());
/// java: public static final String VERSION = SKYBLOCKER_MOD.getMetadata().getVersion().getFriendlyString();
pub static VERSION: LazyLock<String> = LazyLock::new(|| SKYBLOCKER_MOD.get_mut().metadata.version.to_string());
/// java: public static final Path CONFIG_DIR = FabricLoader.getInstance().getConfigDir().resolve(NAMESPACE);
pub static CONFIG_DIR: compat::MainThread<std::path::PathBuf> = compat::MainThread::new(|| mcrust::mcrust::modding::config::config_dir().join(NAMESPACE));
// TODO(fabric2seer): `new GsonBuilder(..)`
//   java: public static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
/// java: public static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
pub static GSON: compat::MainThread<crate::unresolved::Gson> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `GSON` - see the TODO above"));
// TODO(fabric2seer): `new GsonBuilder(..)`
//   java: public static final Gson GSON_COMPACT = new GsonBuilder().create();
/// java: public static final Gson GSON_COMPACT = new GsonBuilder().create();
pub static GSON_COMPACT: compat::MainThread<crate::unresolved::Gson> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `GSON_COMPACT` - see the TODO above"));
// TODO(fabric2seer): static `KeyMapping.Category` has no Rust counterpart in the port's KeyMapping file
//   java: public static final KeyMapping.Category KEYBINDING_CATEGORY = KeyMapping.Category.register(id("main"));
/// java: public static final KeyMapping.Category KEYBINDING_CATEGORY = KeyMapping.Category.register(id("main"));
pub static KEYBINDING_CATEGORY: compat::MainThread<mcrust::net::minecraft::client::key_mapping::Category> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `KEYBINDING_CATEGORY` - see the TODO above"));
// TODO(fabric2seer): Executors.newVirtualThreadPerTaskExecutor: use std::thread / the port's executors
//   java: public static final ExecutorService VIRTUAL_THREAD_EXECUTOR = Executors.newVirtualThreadPerTaskExecutor();
/// java: public static final ExecutorService VIRTUAL_THREAD_EXECUTOR = Executors.newVirtualThreadPerTaskExecutor();
pub static VIRTUAL_THREAD_EXECUTOR: compat::MainThread<crate::unresolved::ExecutorService> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `VIRTUAL_THREAD_EXECUTOR` - see the TODO above"));

pub fn id(path: &str) -> Identifier {
    return Identifier::from_namespace_and_path(NAMESPACE, path);
}

pub fn on_initialize_client() {
    Utils::is_on_skyblock();
    client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(__r) });
        ({ let __r = message_scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Scheduler>)) });
    }));
    SkyblockerConfigManager::init();
    ConfigNullFieldsFix::init();
    ConfigBackupManager::init();
    crate::entry::skyblocker_mod::init();
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { Utils::update(); }), 20) });
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { DiscordRPCManager::update_data_and_presence(); }), 200) });
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { BackpackPreview::tick(); }), 50) });
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { PlayerListManager::try_update_list(); }), 2) });
}

pub fn tick(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(__r) });
    ({ let __r = message_scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.tick()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Scheduler>)) });
}

pub fn init() {
}
