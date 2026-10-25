//! Ported from de.hysky.skyblocker.skyblock.chat.SkyblockXpMessages by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern SKYBLOCK_XP_PATTERN = Pattern.compile("§b\\+\\d+ SkyBlock XP §7\\([^()]+§7\\)§b \\(\\d+\\/\\d+\\)");
pub static SKYBLOCK_XP_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("§b\\+\\d+ SkyBlock XP §7\\([^()]+§7\\)§b \\(\\d+\\/\\d+\\)"));
/// java: private static final IntOpenHashSet RECENT_MESSAGES = new IntOpenHashSet();
pub static RECENT_MESSAGES: LazyLock<Mutex<std::collections::HashSet<i32>>> = LazyLock::new(|| Mutex::new(std::collections::HashSet::new()));

/// java: public class SkyblockXpMessages
#[derive(Clone)]
pub struct SkyblockXpMessages {}

impl SkyblockXpMessages {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(SkyblockXpMessages::onMessage);
        compat::register_allow_game(|text, overlay| SkyblockXpMessages::on_message(text, overlay));
    }

    /// java: private static boolean onMessage(Component text, boolean overlay)
    pub fn on_message(text: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_on_skyblock() && overlay && SkyblockerConfigManager::get().chat.skyblock_xp_messages {
            let mut message: String = text.get_string();
            let mut matcher: compat::regex::Matcher = SKYBLOCK_XP_PATTERN.matcher(&message);
            if matcher.find() {
                let mut xp_message: String = matcher.group(0).unwrap();
                let mut hash: i32 = compat::string_hash_code(&xp_message);
                if !(compat::lock(&RECENT_MESSAGES).contains(&hash)) {
                    mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&(constants::PREFIX.get_mut())().append_str(&xp_message).to_string()); });
                    compat::lock(&RECENT_MESSAGES).insert(hash);
                    ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); __f(__r, Arc::new(move || { compat::lock(&RECENT_MESSAGES).remove(&hash); }), 20i32.wrapping_mul(10)) });
                }
            }
        }
        return true;
    }

}
