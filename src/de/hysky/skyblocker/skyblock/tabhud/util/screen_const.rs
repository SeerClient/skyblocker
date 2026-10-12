//! Ported from de.hysky.skyblocker.skyblock.tabhud.util.ScreenConst by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;

/// java: public static final int WIDGET_PAD = 5;
pub const WIDGET_PAD: i32 = 5;
/// java: public static final int WIDGET_PAD_HALF = 3;
pub const WIDGET_PAD_HALF: i32 = 3;
/// java: private static final int SCREEN_PAD_BASE = 20;
pub const SCREEN_PAD_BASE: i32 = 20;

/// java: public class ScreenConst
#[derive(Clone)]
pub struct ScreenConst {}

impl ScreenConst {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int getScreenPad()
    pub fn get_screen_pad() -> i32 {
        return ((((1.0f32 / (((SkyblockerConfigManager::get().ui_and_visuals.tab_hud.tab_hud_scale) as f32) / 100.0f32)) * (SCREEN_PAD_BASE as f32))) as i32);
    }

}
