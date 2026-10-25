//! Ported from de.hysky.skyblocker.utils.discord.DiscordRPCManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::misc_config::Info;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::sky_block_icons;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::java::text::decimal_format::DecimalFormat;
use crate::meteordevelopment::discordipc::discord_ipc::DiscordIPC;
use crate::meteordevelopment::discordipc::rich_presence::RichPresence;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

/// java: public static final DecimalFormat DECIMAL_FORMAT = new DecimalFormat("###,###.##");
pub static DECIMAL_FORMAT: LazyLock<DecimalFormat> = LazyLock::new(|| DecimalFormat::new("###,###.##", mcrust::java::text::decimal_format_symbols::get_instance(mcrust::java::text::decimal_format_symbols::Locale::Us)));
// (`new DecimalFormat(pattern)` takes the default locale's symbols; the port models no default locale and the mod's
// other formatters use US - the same CLDR grouping and decimal symbols as ROOT)
/// java: public static final Logger LOGGER = LoggerFactory.getLogger("Skyblocker Discord RPC");
pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("Skyblocker Discord RPC"));
/// java: public static CompletableFuture<Void> updateTask;
pub static UPDATE_TASK: compat::MainThread<Option<mcrust::java::util::concurrent::completable_future::CompletableFuture<()>>> = compat::MainThread::new(|| None);
/// java: public static long startTimeStamp;
pub static START_TIME_STAMP: LazyLock<Mutex<i64>> = LazyLock::new(|| Mutex::new(0));
/// java: public static int cycleCount;
pub static CYCLE_COUNT: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

/// java: public class DiscordRPCManager
#[derive(Clone)]
pub struct DiscordRPCManager {}

impl DiscordRPCManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        skyblock_events::LEAVE.register(Arc::new(move || { DiscordRPCManager::init_and_update_presence(); }));
        skyblock_events::JOIN.register(Arc::new(move || {
            compat::set_state(&START_TIME_STAMP, compat::millis());
            DiscordRPCManager::init_and_update_presence_1(true);
        }));
    }

    /// java: public static void updateDataAndPresence()
    pub fn update_data_and_presence() {
        if SkyblockerConfigManager::get().misc.rich_presence.custom_message.is_empty() {
            SkyblockerConfigManager::update(Arc::new(move |mut config| { config.misc.rich_presence.custom_message = "Playing Skyblock".to_string(); }));
        }
        if SkyblockerConfigManager::get().misc.rich_presence.cycle_mode {
            compat::set_state(&CYCLE_COUNT, ((compat::get_state(&CYCLE_COUNT)).wrapping_add(1)).wrapping_rem(3));
        }
        DiscordRPCManager::init_and_update_presence();
    }

    /// java: private static void initAndUpdatePresence()
    pub fn init_and_update_presence() {
        DiscordRPCManager::init_and_update_presence_1(false);
    }

    /// java: private static void initAndUpdatePresence(boolean initialization)
    pub fn init_and_update_presence_1(mut initialization: bool) {
        if UPDATE_TASK.get_mut().is_none() || UPDATE_TASK.get_mut().as_ref().unwrap().is_done() {
            *UPDATE_TASK.get_mut() = Some(CompletableFuture::<()>::run_async(move || (|| {
                if SkyblockerConfigManager::get().misc.rich_presence.enable_rich_presence && Utils::is_on_skyblock() {
                    if !DiscordIPC::is_connected() {
                        if DiscordIPC::start(934607927837356052, None) {
                            LOGGER.info("[Skyblocker] Discord RPC connected successfully");
                        } else {
                            if initialization {
                                LOGGER.error("[Skyblocker] Discord RPC failed to connect");
                            }
                            return;
                        }
                    }
                    DiscordIPC::set_activity(&DiscordRPCManager::build_presence());
                } else if DiscordIPC::is_connected() {
                    DiscordIPC::stop();
                    LOGGER.info("[Skyblocker] Discord RPC disconnected");
                } else if initialization {
                    LOGGER.info("[Skyblocker] Discord RPC is currently disabled, will not connect");
                }
            })(), compat::executor));
        }
    }

    /// java: public static RichPresence buildPresence()
    pub fn build_presence() -> RichPresence {
        let mut presence = RichPresence::new();
        presence.set_large_image(Some("skyblocker-default".to_string()), None);
        presence.set_start(compat::get_state(&START_TIME_STAMP));
        presence.set_details(Some(SkyblockerConfigManager::get().misc.rich_presence.custom_message.clone()));
        presence.set_state(DiscordRPCManager::get_info());
        return presence;
    }

    /// java: @SuppressWarnings("deprecation") private static String islandArea()
    pub fn island_area() -> String {
        return (Utils::get_island_area().replace(sky_block_icons::AREA, &'⏣'.to_string())).replace(sky_block_icons::RIFT_AREA, &'ф'.to_string());
    }

    /// java: public static String getInfo() - `null` when no arm matches
    pub fn get_info() -> Option<String> {
        let mut info: Option<String> = None;
        if !(SkyblockerConfigManager::get().misc.rich_presence.cycle_mode) {
            match SkyblockerConfigManager::get().misc.rich_presence.info {
                Info::Bits => {
                    info = Some(format!("Bits: {}", DECIMAL_FORMAT.format((Utils::get_bits()) as f64)));
                }
                Info::Purse => {
                    info = Some(format!("Purse: {}", DECIMAL_FORMAT.format(Utils::get_purse())));
                }
                Info::Location => {
                    info = Some(DiscordRPCManager::island_area());
                }
                _ => {}
            }
        } else if SkyblockerConfigManager::get().misc.rich_presence.cycle_mode {
            match compat::get_state(&CYCLE_COUNT) {
                0 => {
                    info = Some(format!("Bits: {}", DECIMAL_FORMAT.format((Utils::get_bits()) as f64)));
                }
                1 => {
                    info = Some(format!("Purse: {}", DECIMAL_FORMAT.format(Utils::get_purse())));
                }
                2 => {
                    info = Some(DiscordRPCManager::island_area());
                }
                _ => {}
            }
        }
        return info;
    }

}
