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
use mcrust::java::util::concurrent::completable_future::CompletableFuture;

// TODO(fabric2seer): `new` takes 2 argument(s) in the port (`pattern: &str, symbols: DecimalFormatSymbols`), Java passes 1
//   java: public static final DecimalFormat DECIMAL_FORMAT = new DecimalFormat("###,###.##");
/// java: public static final DecimalFormat DECIMAL_FORMAT = new DecimalFormat("###,###.##");
pub static DECIMAL_FORMAT: compat::MainThread<DecimalFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `DECIMAL_FORMAT` - see the TODO above"));
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
            SkyblockerConfigManager::update(Arc::new(move |config| { config.misc.rich_presence.custom_message = "Playing Skyblock".to_string(); }));
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
                // TODO(fabric2seer): static call DiscordIPC.isConnected(..)
                //   java: if (SkyblockerConfigManager.get().misc.richPresence.enableRichPresence && Utils.isOnSkyblock()) {
                //   java: 					if (!DiscordIPC.isConnected()) {
                //   java: 						if (DiscordIPC.start(934607927837356052L, null)) {
                //   java: 							LOGGER.info("[Skyblocker] Discord RPC connected successfully");
                //   java: 						} else {
                //   java: 							if (initialization) {
                //   java: 								LOGGER.error("[Skyblocker] Discord RPC failed to connect");
                //   java: 							}
                //   java: 							return;
                //   java: 						}
                //   java: 					}
                //   java: 					DiscordIPC.setActivity(buildPresence());
                //   java: 				} else if (DiscordIPC.isConnected()) {
                //   java: 					DiscordIPC.stop();
                //   java: 					LOGGER.info("[Skyblocker] Discord RPC disconnected");
                //   java: 				} else if (initialization) {
                //   java: 					LOGGER.info("[Skyblocker] Discord RPC is currently disabled, will not connect");
                //   java: 				}
            })(), compat::executor));
        }
    }

    /// java: public static RichPresence buildPresence()
    pub fn build_presence() -> crate::unresolved::RichPresence {
        // TODO(fabric2seer): `new RichPresence(..)`
        //   java: RichPresence presence = new RichPresence();
        let mut presence: crate::unresolved::RichPresence = todo!("fabric2seer: the initialiser of `presence` (TODO above)");
        // TODO(fabric2seer): `.setLargeImage(..)` on a RichPresence
        //   java: presence.setLargeImage("skyblocker-default", null);
        // TODO(fabric2seer): `.setStart(..)` on a RichPresence
        //   java: presence.setStart(startTimeStamp);
        // TODO(fabric2seer): `.setDetails(..)` on a RichPresence
        //   java: presence.setDetails(SkyblockerConfigManager.get().misc.richPresence.customMessage);
        // TODO(fabric2seer): `.setState(..)` on a RichPresence
        //   java: presence.setState(getInfo());
        return presence;
    }

    /// java: @SuppressWarnings("deprecation") private static String islandArea()
    pub fn island_area() -> String {
        return (Utils::get_island_area().replace(sky_block_icons::AREA, &'⏣'.to_string())).replace(sky_block_icons::RIFT_AREA, &'ф'.to_string());
    }

    /// java: public static String getInfo()
    pub fn get_info() -> String {
        let mut info: Option<String> = None;
        if !(SkyblockerConfigManager::get().misc.rich_presence.cycle_mode) {
            match SkyblockerConfigManager::get().misc.rich_presence.info {
                Info::Bits => {
                    info = Some(format!("Bits: {}", DECIMAL_FORMAT.get_mut().format((Utils::get_bits()) as f64)));
                }
                Info::Purse => {
                    info = Some(format!("Purse: {}", DECIMAL_FORMAT.get_mut().format(Utils::get_purse())));
                }
                Info::Location => {
                    info = Some(DiscordRPCManager::island_area());
                }
                _ => {}
            }
        } else if SkyblockerConfigManager::get().misc.rich_presence.cycle_mode {
            match compat::get_state(&CYCLE_COUNT) {
                0 => {
                    info = Some(format!("Bits: {}", DECIMAL_FORMAT.get_mut().format((Utils::get_bits()) as f64)));
                }
                1 => {
                    info = Some(format!("Purse: {}", DECIMAL_FORMAT.get_mut().format(Utils::get_purse())));
                }
                2 => {
                    info = Some(DiscordRPCManager::island_area());
                }
                _ => {}
            }
        }
        return info.unwrap();
    }

}
