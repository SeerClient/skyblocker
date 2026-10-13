//! Ported from de.hysky.skyblocker.compatibility.modmenu.ModMenuEntry by fabric2seer. Entrypoint: modmenu.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;

/// ModMenu's `ConfigScreenFactory<?>`: `S create(Screen parent)`. The screens are Dandelion's / YACL's
/// (`compat::ScreenRef`: the parent handed in, the config screen answered).
pub type ConfigScreenFactory = Arc<dyn Fn(dandelion::compat::ScreenRef) -> dandelion::compat::ScreenRef>;

pub fn get_mod_config_screen_factory() -> ConfigScreenFactory {
    // java: return SkyblockerConfigManager::createGUI;
    return Arc::new(move |parent: dandelion::compat::ScreenRef| SkyblockerConfigManager::create_gui(Some(parent)));
}
