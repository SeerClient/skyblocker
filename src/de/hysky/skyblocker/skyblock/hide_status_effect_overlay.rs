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
        // TODO(fabric2seer): HudElementRegistry.replaceElement: no SeerClient equivalent yet
        //   java: HudElementRegistry.replaceElement(VanillaHudElements.MOB_EFFECTS, hudElement -> {
        //   java: 			if (Utils.isOnSkyblock() && SkyblockerConfigManager.get().uiAndVisuals.hideStatusEffectOverlay) return (_, _) -> {};
        //   java: 			return hudElement;
        //   java: 		});
    }

}
