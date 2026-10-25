//! Ported from de.hysky.skyblocker.skyblock.HideStatusEffectOverlay by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public final class HideStatusEffectOverlay
#[derive(Clone)]
pub struct HideStatusEffectOverlay {}

impl HideStatusEffectOverlay {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // java: HudElementRegistry.replaceElement(VanillaHudElements.MOB_EFFECTS, hudElement -> { if (Utils.isOnSkyblock() && SkyblockerConfigManager.get().uiAndVisuals.hideStatusEffectOverlay) return (_, _) -> {}; return hudElement; });
        use mcrust::mcrust::modding::hud_element_registry::{self, vanilla_hud_elements, HudElement};
        hud_element_registry::replace_element(vanilla_hud_elements::MOB_EFFECTS, Arc::new(|hud_element: Arc<HudElement>| -> Arc<HudElement> {
            if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.hide_status_effect_overlay {
                // OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P02-mods-runtime): the shared empty element
                // (`compat::empty_hud_element`). Was:
                //     return Arc::new(|_: &mut mcrust::mcrust::modding::events::hud_render_callback::HudRenderContext<'_>| {});
                return crate::compat::empty_hud_element();
            }
            hud_element
        }));
    }

}
