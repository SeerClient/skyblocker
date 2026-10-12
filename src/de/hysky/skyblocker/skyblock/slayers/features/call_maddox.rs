//! Ported from de.hysky.skyblocker.skyblock.slayers.features.CallMaddox by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;
use mcrust::net::minecraft::network::chat::click_event::ClickEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class CallMaddox
#[derive(Clone)]
pub struct CallMaddox {}

impl CallMaddox {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static void sendMessage()
    pub fn send_message() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_none() {
            return;
        }
        let mut text: MutableComponent = (constants::PREFIX.get_mut())().append(component::translatable("skyblocker.slayer.callMaddox.message")).with_style_fn(|style| (style.with_click_event(Some(ClickEvent::new_run_command("/call slayer".to_string())))).with_color_format(Some(ChatFormatting::Aqua)));
        mcrust::mcrust::modding::game::with_player(|player| { script_host::send_feedback(&text.to_string()); });
    }

    /// java: public static void onSlayerFailed()
    pub fn on_slayer_failed() {
        if !(SkyblockerConfigManager::get().slayers.call_maddox.send_message_on_fail) {
            return;
        }
        if Utils::is_in_the_rift() {
            return;
        }
        CallMaddox::send_message();
    }

    /// java: public static void onBossKilled()
    pub fn on_boss_killed() {
        if !(SkyblockerConfigManager::get().slayers.call_maddox.send_message_on_kill) {
            return;
        }
        if Utils::is_in_the_rift() {
            return;
        }
        CallMaddox::send_message();
    }

}
