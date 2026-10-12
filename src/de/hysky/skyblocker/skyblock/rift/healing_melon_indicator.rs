//! Ported from de.hysky.skyblocker.skyblock.rift.HealingMelonIndicator by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::area::TheRift;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::render::title::title_container::TitleContainer;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::player::local_player::LocalPlayer;

/// java: private static final Title title = new Title("skyblocker.rift.healNow", ChatFormatting.DARK_RED);
pub static TITLE: compat::MainThread<compat::Shared<Title>> = compat::MainThread::new(|| Title::new("skyblocker.rift.healNow".to_string(), ChatFormatting::DarkRed));

/// java: public class HealingMelonIndicator
#[derive(Clone)]
pub struct HealingMelonIndicator {}

impl HealingMelonIndicator {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void updateHealth()
    pub fn update_health() {
        if !(SkyblockerConfigManager::get().slayers.vampire_slayer.enable_healing_melon_indicator) || !(Utils::is_on_skyblock()) || !(Utils::is_in_the_rift()) || !compat::read(&Utils::get_area(), |__t| compat::DynAny::dyn_any(__t).downcast_ref::<TheRift>().is_some_and(|__e| *__e == TheRift::Chateau)) {
            TitleContainer::remove_title(TITLE.get_mut());
            return;
        }
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.get_health()).unwrap() <= (SkyblockerConfigManager::get().slayers.vampire_slayer.healing_melon_health_threshold * 2.0f32) {
            TitleContainer::add_title_and_play_sound(TITLE.get_mut());
        } else {
            TitleContainer::remove_title(TITLE.get_mut());
        }
    }

}
