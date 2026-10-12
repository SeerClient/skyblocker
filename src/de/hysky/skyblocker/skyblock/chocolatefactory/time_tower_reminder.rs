//! Ported from de.hysky.skyblocker.skyblock.chocolatefactory.TimeTowerReminder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::events::skyblock_events;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::brigadier::message::Message;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;

/// java: private static final String TIME_TOWER_FILE = "time_tower.txt";
pub const TIME_TOWER_FILE: &str = "time_tower.txt";
/// java: private static final Pattern TIME_TOWER_PATTERN = Pattern.compile("^TIME TOWER! Your Chocolate Factory production has increased by \\+[\\d.]+x for \\dh!$");
pub static TIME_TOWER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("^TIME TOWER! Your Chocolate Factory production has increased by \\+[\\d.]+x for \\dh!$"));
/// java: private static boolean scheduled = false;
pub static SCHEDULED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// java: public class TimeTowerReminder
#[derive(Clone)]
pub struct TimeTowerReminder {}

impl TimeTowerReminder {
    /// java: private TimeTowerReminder()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        skyblock_events::JOIN.register(Arc::new(move || { TimeTowerReminder::check_temp_file(); }));
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(TimeTowerReminder::checkIfTimeTower);
    }

    /// java: public static boolean checkIfTimeTower(Message message, boolean overlay)
    pub fn check_if_time_tower(message: &dyn Message, mut overlay: bool) -> bool {
        if !(TIME_TOWER_PATTERN.matcher(&message.get_string()).matches()) || compat::get_state(&SCHEDULED) {
            return true;
        }
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { TimeTowerReminder::send_message(); }), (60i32.wrapping_mul(60)).wrapping_mul(20)) });
        compat::set_state(&SCHEDULED, true);
        let mut temp_file: std::path::PathBuf = skyblocker_mod::CONFIG_DIR.get_mut().join(TIME_TOWER_FILE).clone();
        if !temp_file.exists() {
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): Path.createNewFile
                //   java: tempFile.createNewFile();
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Time Tower Reminder] Failed to create temp file for Time Tower Reminder!"));
                return true;
            }
        }
        {
            // TODO(fabric2seer): `new FileWriter(..)`
            //   java: FileWriter writer = new FileWriter(tempFile);
            let mut writer: crate::unresolved::FileWriter = todo!("fabric2seer: the initialiser of `writer` (TODO above)");
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.write(..)` on a FileWriter
                //   java: writer.write(String.valueOf(System.currentTimeMillis()));
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Time Tower Reminder] Failed to write to temp file for Time Tower Reminder!"));
            }
        }
        return true;
    }

    /// java: private static void sendMessage()
    pub fn send_message() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() || !(Utils::is_on_skyblock()) {
            return;
        }
        if SkyblockerConfigManager::get().helpers.chocolate_factory.enable_time_tower_reminder {
            mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append(component::translatable("skyblocker.config.helpers.chocolateFactory.sendTimeTowerReminderMessage").with_style_format(ChatFormatting::Red)).to_string()); });
        }
        let mut temp_file: std::path::PathBuf = skyblocker_mod::CONFIG_DIR.get_mut().join(TIME_TOWER_FILE).clone();
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            compat::set_state(&SCHEDULED, false);
            if temp_file.exists() {
                std::fs::remove_file(&temp_file.clone()).expect("IOException");
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker Time Tower Reminder] Failed to delete temp file for Time Tower Reminder!"));
        }
    }

    /// java: private static void checkTempFile()
    pub fn check_temp_file() {
        let mut temp_file: std::path::PathBuf = skyblocker_mod::CONFIG_DIR.get_mut().join(TIME_TOWER_FILE).clone();
        if !temp_file.exists() || compat::get_state(&SCHEDULED) {
            return;
        }
        let mut time: i64;
        {
            // TODO(fabric2seer): Files.lines
            //   java: Stream<String> file = Files.lines(..);
            let mut file: Box<dyn Iterator<Item = String>> = todo!("fabric2seer: the initialiser of `file` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                time = file.clone().next().unwrap().trim().parse::<i64>().expect("NumberFormatException");
            }));
            if let Err(e) = __result {
                logger().error(&format!("[Skyblocker Time Tower Reminder] Failed to read temp file for Time Tower Reminder!"));
                return;
            }
        }
        if (compat::millis()).wrapping_sub(time) >= ((60i32.wrapping_mul(60)).wrapping_mul(1000) as i64) {
            TimeTowerReminder::send_message();
        } else {
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { TimeTowerReminder::send_message(); }), ((60i32.wrapping_mul(60)).wrapping_mul(20)).wrapping_sub((((compat::millis()).wrapping_sub(time)).wrapping_div(50 as i64)) as i32)) });
            compat::set_state(&SCHEDULED, true);
        }
    }

}
