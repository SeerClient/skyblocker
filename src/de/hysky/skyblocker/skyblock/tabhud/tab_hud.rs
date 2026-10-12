//! Ported from de.hysky.skyblocker.skyblock.tabhud.TabHud by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::net::minecraft::client::key_mapping;

/// java: public static KeyMapping toggleSecondary;
pub static TOGGLE_SECONDARY: compat::MainThread<Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>> = compat::MainThread::new(|| None);
/// java: public static KeyMapping defaultTgl;
pub static DEFAULT_TGL: compat::MainThread<Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>> = compat::MainThread::new(|| None);

/// java: public class TabHud
#[derive(Clone)]
pub struct TabHud {}

impl TabHud {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.toggleA", 90, Category::Misc));
        key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.defaultTgl", 77, Category::Misc));
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.PLAYER_LIST, hudElement -> {
        //   java: 			if (!Utils.isOnSkyblock() || TabHud.shouldRenderVanilla() || !WidgetManager.hasFancyTab()) return hudElement;
        //   java: 			return (_, _) -> {};
        //   java: 		});
    }

    /// java: public static boolean shouldRenderVanilla()
    pub fn should_render_vanilla() -> bool {
        // TODO(fabric2seer): `is_down` takes 1 argument(s) in the port (`name: &str`), Java passes 0
        // rust: return (key_mapping::is_down() != SkyblockerConfigManager::get().ui_and_visuals.tab_hud.show_vanilla_tab_by_default);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public static float getScaleFactor()
    pub fn get_scale_factor() -> f32 {
        return ((SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_scale as f32) / 100.0f32);
    }

}
