//! Ported from de.hysky.skyblocker.compatibility.modmenu.ModMenuEntry by fabric2seer. Entrypoint: modmenu.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;


pub fn get_mod_config_screen_factory() -> crate::unresolved::ConfigScreenFactory {
    // java: return SkyblockerConfigManager::createGUI;
    return Arc::new(move |__a0: mcrust::net::minecraft::client::gui::screens::screen::Screen| SkyblockerConfigManager::create_gui(Some(__a0)));
}
