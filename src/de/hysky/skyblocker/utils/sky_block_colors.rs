//! Ported from de.hysky.skyblocker.utils.SkyBlockColors by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::text_color::TextColor;

/// java: public static final TextColor DARK_RED = TextColor.fromRgb(0xD13228);
pub static DARK_RED: compat::MainThread<TextColor> = compat::MainThread::new(|| TextColor::from_rgb(0xD13228));
/// java: public static final TextColor GOLD = TextColor.fromRgb(0xFF9000);
pub static GOLD: compat::MainThread<TextColor> = compat::MainThread::new(|| TextColor::from_rgb(0xFF9000));
/// java: public static final TextColor DARK_BLUE = TextColor.fromRgb(0x353FCE);
pub static DARK_BLUE: compat::MainThread<TextColor> = compat::MainThread::new(|| TextColor::from_rgb(0x353FCE));
/// java: public static final TextColor BLUE = TextColor.fromRgb(0x459BFF);
pub static BLUE: compat::MainThread<TextColor> = compat::MainThread::new(|| TextColor::from_rgb(0x459BFF));
/// java: public static final TextColor DARK_PURPLE = TextColor.fromRgb(0xA335EE);
pub static DARK_PURPLE: compat::MainThread<TextColor> = compat::MainThread::new(|| TextColor::from_rgb(0xA335EE));
/// java: public static final TextColor LIGHT_GRAY = TextColor.fromRgb(0xA8BFD2);
pub static LIGHT_GRAY: compat::MainThread<TextColor> = compat::MainThread::new(|| TextColor::from_rgb(0xA8BFD2));
/// java: public static final TextColor GRAY = TextColor.fromRgb(0x707592);
pub static GRAY: compat::MainThread<TextColor> = compat::MainThread::new(|| TextColor::from_rgb(0x707592));
/// java: public static final TextColor YELLOW = TextColor.fromRgb(0xFFDE2F);
pub static YELLOW: compat::MainThread<TextColor> = compat::MainThread::new(|| TextColor::from_rgb(0xFFDE2F));

/// java: public final class SkyBlockColors
#[derive(Clone)]
pub struct SkyBlockColors {}

impl SkyBlockColors {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int fromVanilla(int colour)
    pub fn from_vanilla(mut colour: i32) -> i32 {
        return match colour {
            0xAA0000 => DARK_RED.get_mut().get_value(),
            0xFFAA00 => GOLD.get_mut().get_value(),
            0x0000AA => DARK_BLUE.get_mut().get_value(),
            0x5555FF => BLUE.get_mut().get_value(),
            0xAA00AA => DARK_PURPLE.get_mut().get_value(),
            0xAAAAAA => LIGHT_GRAY.get_mut().get_value(),
            0x555555 => GRAY.get_mut().get_value(),
            0xFFFF55 => YELLOW.get_mut().get_value(),
            _ => colour,
        };
    }

}
