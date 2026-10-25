//! Ported from de.hysky.skyblocker.skyblock.tabhud.TabHud by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::net::minecraft::client::key_mapping;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::WidgetManager;
use crate::de::hysky::skyblocker::utils::utils::Utils;

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
        // java: toggleSecondary = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.toggleA", InputConstants.Type.KEYSYM, InputConstants.KEY_Z, SkyblockerMod.KEYBINDING_CATEGORY));
        *TOGGLE_SECONDARY.get_mut() = Some(key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.toggleA", mcrust::com::mojang::blaze3d::platform::input_constants::KEY_Z, compat::category("skyblocker:main"))));
        // java: defaultTgl = KeyMappingHelper.registerKeyMapping(new KeyMapping("key.skyblocker.defaultTgl", InputConstants.Type.KEYSYM, InputConstants.KEY_M, SkyblockerMod.KEYBINDING_CATEGORY));
        *DEFAULT_TGL.get_mut() = Some(key_binding_helper::register_key_binding(KeyMapping::new("key.skyblocker.defaultTgl", mcrust::com::mojang::blaze3d::platform::input_constants::KEY_M, compat::category("skyblocker:main"))));
        // java: HudElementRegistry.replaceElement(VanillaHudElements.PLAYER_LIST, hudElement -> {
        //           if (!Utils.isOnSkyblock() || TabHud.shouldRenderVanilla() || !WidgetManager.hasFancyTab()) return hudElement;
        //           return (_, _) -> {}; });
        hud_element_registry::replace_element(vanilla_hud_elements::PLAYER_LIST, Arc::new(|hud_element: Arc<HudElement>| -> Arc<HudElement> {
            if !Utils::is_on_skyblock() || TabHud::should_render_vanilla() || !WidgetManager::has_fancy_tab() {
                return hud_element;
            }
            // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the shared empty element
            // (`compat::empty_hud_element`). Was:
            //     let empty: Arc<HudElement> = Arc::new(|_: &mut hud_render_callback::HudRenderContext<'_>| {});
            //     empty
            compat::empty_hud_element()
        }));
    }

    /// java: public static boolean shouldRenderVanilla()
    pub fn should_render_vanilla() -> bool {
        // java: return defaultTgl.isDown() != SkyblockerConfigManager.get().uiAndVisuals.tabHud.showVanillaTabByDefault;
        return (key_mapping::is_down("key.skyblocker.defaultTgl") != SkyblockerConfigManager::get().ui_and_visuals.tab_hud.show_vanilla_tab_by_default);
    }

    /// java: public static float getScaleFactor()
    pub fn get_scale_factor() -> f32 {
        return ((SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_scale as f32) / 100.0f32);
    }

}
