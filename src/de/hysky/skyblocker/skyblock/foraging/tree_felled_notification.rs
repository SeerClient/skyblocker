//! Ported from de.hysky.skyblocker.skyblock.foraging.TreeFelledNotification by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern TIMBER_PATTERN = Pattern.compile("TIMBER! You felled the entire \\w+ Tree!");
pub static TIMBER_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("TIMBER! You felled the entire \\w+ Tree!"));
/// java: private static final Title TITLE = new Title("skyblocker.foraging.treeFelled", ChatFormatting.AQUA);
pub static TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new("skyblocker.foraging.treeFelled".to_string(), ChatFormatting::Aqua));

/// java: public class TreeFelledNotification
#[derive(Clone)]
pub struct TreeFelledNotification {}

impl TreeFelledNotification {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(TreeFelledNotification::onMessage);
    }

    /// java: private static boolean onMessage(Component message, boolean overlay)
    pub fn on_message(message: &MutableComponent, mut overlay: bool) -> bool {
        if Utils::is_on_skyblock() && Utils::is_in_foraging_island() && SkyblockerConfigManager::get().foraging.enable_tree_felled_notification {
            let mut stringified: String = message.get_string();
            let mut matcher: compat::regex::Matcher = TIMBER_PATTERN.matcher(&stringified);
            if matcher.matches() {
                TitleContainer::add_title_2(TITLE.get_mut(), 3i32.wrapping_mul(20));
            }
        }
        return true;
    }

}
