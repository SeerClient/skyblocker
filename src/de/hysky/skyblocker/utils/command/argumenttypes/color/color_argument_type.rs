//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.color.ColorArgumentType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::color::hex_color_argument_type::HexColorArgumentType;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::color::rgb_color_argument_type::RgbColorArgumentType;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;

/// java: public final class ColorArgumentType
#[derive(Clone)]
pub struct ColorArgumentType {}

impl ColorArgumentType {
    /// java: private ColorArgumentType()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static RgbColorArgumentType rgb()
    pub fn rgb() -> RgbColorArgumentType {
        return RgbColorArgumentType::new();
    }

    /// java: public static HexColorArgumentType hex()
    pub fn hex() -> HexColorArgumentType {
        return HexColorArgumentType::new();
    }

    /// java: public static int getIntFromHex(CommandContext<FabricClientCommandSource> context, String name)
    pub fn get_int_from_hex(context: &CommandContext<crate::unresolved::FabricClientCommandSource>, name: &str) -> i32 {
        return HexColorArgumentType::get_int(context, name);
    }

    /// java: public static int getIntFromRgb(CommandContext<FabricClientCommandSource> context, String name)
    pub fn get_int_from_rgb(context: &CommandContext<crate::unresolved::FabricClientCommandSource>, name: &str) -> i32 {
        return RgbColorArgumentType::get_int(context, name);
    }

}
