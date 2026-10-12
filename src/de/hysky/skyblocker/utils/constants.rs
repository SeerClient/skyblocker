//! Ported from de.hysky.skyblocker.utils.Constants by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::ok_lab_color::OkLabColor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @SuppressWarnings("UnnecessaryUnicodeEscape") String LEVEL_EMBLEMS = "\u2E15\u273F\u2741\u2E19\u03B1\u270E\u2615\u2616\u2663\u213B\u2694\u27B6\u26A1\u2604\u269A\u2693\u2620\u269B\u2666\u2660\u2764\u27
pub const LEVEL_EMBLEMS: &str = "⸕✿❁⸙α✎☕☖♣℻⚔➶⚡☄⚚⚓☠⚛♦♠❤✧⎊፠☬⚝⧉ꈔ㋖⸎⚠ꕁ〠ツ⥈☢☣✾⚜ௐ੭❂ᛃ〣ჶф♪♫Ӄ⛁⛃ᛝ☘ꖘꀾᱪΣ৫☃♔🐸⛂࿇ꤥ✨ꕪ▒☀✩✬✭✯✶✳✴✷✸✹✺ዞ";
/// java: Pattern PLAYER_NAME = Pattern.compile("(?:\\[[0-9]+\\] )?(?:[" + Constants.LEVEL_EMBLEMS + "] )?(?:\\[[A-Z+]+\\] )?([A-Za-z0-9_]+)");
pub static PLAYER_NAME: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile(&format!("(?:\\[[0-9]+\\] )?(?:[{}] )?(?:\\[[A-Z+]+\\] )?([A-Za-z0-9_]+)", LEVEL_EMBLEMS)));
/// java: Supplier<MutableComponent> PREFIX = () -> { .. };
pub static PREFIX: compat::MainThread<Arc<dyn Fn() -> MutableComponent>> = compat::MainThread::new(|| {
    Arc::new(|| -> MutableComponent {
        use crate::de::hysky::skyblocker::utils::fun_utils::FunUtils;
        use mcrust::net::minecraft::chat_formatting::ChatFormatting;
        if FunUtils::should_enable_fun() {
            return component::empty().append(component::literal("[").with_style_format(ChatFormatting::Gray)).append(FunUtils::get_random_fun_name()).append(component::literal("] ").with_style_format(ChatFormatting::Gray));
        } else if FunUtils::should_enable_christmas_fun() {
            return component::empty().append(component::literal("[").with_style_format(ChatFormatting::Gray)).append(create_skyblocker_gradient("Jollyblocker")).append(component::literal("] ").with_style_format(ChatFormatting::Gray));
        } else if FunUtils::should_enable_spooky_fun() {
            return component::empty().append(component::literal("[").with_style_format(ChatFormatting::Gray)).append(create_skyblocker_gradient("S-Scaryblocker")).append(component::literal("] ").with_style_format(ChatFormatting::Gray));
        }
        return component::empty().append(component::literal("[").with_style_format(ChatFormatting::Gray)).append(create_skyblocker_gradient("Skyblocker")).append(component::literal("] ").with_style_format(ChatFormatting::Gray));
    })
});
/// java: List<String> SEYMOUR_IDS = List.of("VELVET_TOP_HAT", "CASHMERE_JACKET", "SATIN_TROUSERS", "OXFORD_SHOES");
pub static SEYMOUR_IDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["VELVET_TOP_HAT".to_string(), "CASHMERE_JACKET".to_string(), "SATIN_TROUSERS".to_string(), "OXFORD_SHOES".to_string()]);
/// java: List<String> CRYSTAL_HEXES = List.of("1F0030", "46085E", "54146E", "5D1C78", "63237D", "6A2C82", "7E4196", "8E51A6", "9C64B3", "A875BD", "B88BC9", "C6A3D4", "D9C1E3", "E5D1ED", "EFE1F5", "FCF3FF");
pub static CRYSTAL_HEXES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["1F0030".to_string(), "46085E".to_string(), "54146E".to_string(), "5D1C78".to_string(), "63237D".to_string(), "6A2C82".to_string(), "7E4196".to_string(), "8E51A6".to_string(), "9C64B3".to_string(), "A875BD".to_string(), "B88BC9".to_string(), "C6A3D4".to_string(), "D9C1E3".to_string(), "E5D1ED".to_string(), "EFE1F5".to_string(), "FCF3FF".to_string()]);
/// java: List<String> FAIRY_HEXES = List.of("330066", "4C0099", "660033", "660066", "6600CC", "7F00FF", "99004C", "990099", "9933FF", "B266FF", "CC0066", "CC00CC", "CC99FF", "E5CCFF", "FF007F", "FF00FF", "FF33
pub static FAIRY_HEXES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["330066".to_string(), "4C0099".to_string(), "660033".to_string(), "660066".to_string(), "6600CC".to_string(), "7F00FF".to_string(), "99004C".to_string(), "990099".to_string(), "9933FF".to_string(), "B266FF".to_string(), "CC0066".to_string(), "CC00CC".to_string(), "CC99FF".to_string(), "E5CCFF".to_string(), "FF007F".to_string(), "FF00FF".to_string(), "FF3399".to_string(), "FF33FF".to_string(), "FF66B2".to_string(), "FF66FF".to_string(), "FF99CC".to_string(), "FF99FF".to_string(), "FFCCE5".to_string(), "FFCCFF".to_string()]);
/// java: List<String> OG_FAIRY_HEXES = List.of("FF99FF", "FFCCFF", "E5CCFF", "CC99FF", "CC00CC", "FF00FF", "FF33FF", "FF66FF", "B266FF", "9933FF", "7F00FF", "660066", "6600CC", "4C0099", "330066", "990099", "6
pub static OG_FAIRY_HEXES: LazyLock<Vec<String>> = LazyLock::new(|| vec!["FF99FF".to_string(), "FFCCFF".to_string(), "E5CCFF".to_string(), "CC99FF".to_string(), "CC00CC".to_string(), "FF00FF".to_string(), "FF33FF".to_string(), "FF66FF".to_string(), "B266FF".to_string(), "9933FF".to_string(), "7F00FF".to_string(), "660066".to_string(), "6600CC".to_string(), "4C0099".to_string(), "330066".to_string(), "990099".to_string(), "660033".to_string(), "99004C".to_string(), "CC0066".to_string(), "660033".to_string(), "99004C".to_string(), "FFCCE5".to_string(), "660033".to_string(), "FFCCE5".to_string(), "FF99CC".to_string(), "FFCCE5".to_string(), "FF99CC".to_string(), "FF66B2".to_string()]);
/// java: List<String> SPOOK = List.of("000000", "070008", "0E000F", "150017", "1B001F", "220027", "29002E", "300036", "37003E", "3E0046", "45004D", "4C0055", "52005D", "590065", "60006C", "670074", "6E007C", "
pub static SPOOK: LazyLock<Vec<String>> = LazyLock::new(|| vec!["000000".to_string(), "070008".to_string(), "0E000F".to_string(), "150017".to_string(), "1B001F".to_string(), "220027".to_string(), "29002E".to_string(), "300036".to_string(), "37003E".to_string(), "3E0046".to_string(), "45004D".to_string(), "4C0055".to_string(), "52005D".to_string(), "590065".to_string(), "60006C".to_string(), "670074".to_string(), "6E007C".to_string(), "750084".to_string(), "7C008B".to_string(), "830093".to_string(), "89009B".to_string(), "9000A3".to_string(), "9700AA".to_string(), "993399".to_string(), "9E00B2".to_string()]);
/// java: Map<String, String> MISC_GLITCHED_HEXES = Map.of( "FFDC51", "SHARK_SCALE", "F7DA33", "FROZEN_BLAZE", "606060", "BAT_PERSON");
pub static MISC_GLITCHED_HEXES: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("FFDC51".to_string(), "SHARK_SCALE".to_string()), ("F7DA33".to_string(), "FROZEN_BLAZE".to_string()), ("606060".to_string(), "BAT_PERSON".to_string())]));
/// java: Map<String, String> WITHER_CHESTPLATE_HEXES = Map.of( "E7413C", "POWER_WITHER_CHESTPLATE", "45413C", "TANK_WITHER_CHESTPLATE", "4A14B7", "SPEED_WITHER_CHESTPLATE", "1793C4", "WISE_WITHER_CHESTPLATE", 
pub static WITHER_CHESTPLATE_HEXES: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("E7413C".to_string(), "POWER_WITHER_CHESTPLATE".to_string()), ("45413C".to_string(), "TANK_WITHER_CHESTPLATE".to_string()), ("4A14B7".to_string(), "SPEED_WITHER_CHESTPLATE".to_string()), ("1793C4".to_string(), "WISE_WITHER_CHESTPLATE".to_string()), ("000000".to_string(), "WITHER_CHESTPLATE".to_string())]));
/// java: Map<String, String> WITHER_LEGGINGS_HEXES = Map.of( "E75C3C", "POWER_WITHER_LEGGINGS", "65605A", "TANK_WITHER_LEGGINGS", "5D2FB9", "SPEED_WITHER_LEGGINGS", "17A8C4", "WISE_WITHER_LEGGINGS", "000000", 
pub static WITHER_LEGGINGS_HEXES: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("E75C3C".to_string(), "POWER_WITHER_LEGGINGS".to_string()), ("65605A".to_string(), "TANK_WITHER_LEGGINGS".to_string()), ("5D2FB9".to_string(), "SPEED_WITHER_LEGGINGS".to_string()), ("17A8C4".to_string(), "WISE_WITHER_LEGGINGS".to_string()), ("000000".to_string(), "WITHER_LEGGINGS".to_string())]));
/// java: Map<String, String> WITHER_BOOTS_HEXES = Map.of( "E76E3C", "POWER_WITHER_BOOTS", "88837E", "TANK_WITHER_BOOTS", "8969C8", "SPEED_WITHER_BOOTS", "1CD4E4", "WISE_WITHER_BOOTS", "000000", "WITHER_BOOTS")
pub static WITHER_BOOTS_HEXES: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([("E76E3C".to_string(), "POWER_WITHER_BOOTS".to_string()), ("88837E".to_string(), "TANK_WITHER_BOOTS".to_string()), ("8969C8".to_string(), "SPEED_WITHER_BOOTS".to_string()), ("1CD4E4".to_string(), "WISE_WITHER_BOOTS".to_string()), ("000000".to_string(), "WITHER_BOOTS".to_string())]));
/// java: List<String> GLITCHED = List.of("FFDC51", "F7DA33", "606060", "E7413C", "45413C", "4A14B7", "1793C4", "000000", "E75C3C", "65605A", "5D2FB9", "17A8C4", "E76E3C", "88837E", "8969C8", "1CD4E4");
pub static GLITCHED: LazyLock<Vec<String>> = LazyLock::new(|| vec!["FFDC51".to_string(), "F7DA33".to_string(), "606060".to_string(), "E7413C".to_string(), "45413C".to_string(), "4A14B7".to_string(), "1793C4".to_string(), "000000".to_string(), "E75C3C".to_string(), "65605A".to_string(), "5D2FB9".to_string(), "17A8C4".to_string(), "E76E3C".to_string(), "88837E".to_string(), "8969C8".to_string(), "1CD4E4".to_string()]);
/// java: List<String> RANCHERS = List.of("CC5500", "000000", "0");
pub static RANCHERS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["CC5500".to_string(), "000000".to_string(), "0".to_string()]);
/// java: List<String> REAPER = List.of("1B1B1B", "FF0000");
pub static REAPER: LazyLock<Vec<String>> = LazyLock::new(|| vec!["1B1B1B".to_string(), "FF0000".to_string()]);
/// java: List<String> ADAPTIVE_CHEST = List.of("3ABE78", "82E3D8", "BFBCB2", "D579FF", "FF4242", "FFC234");
pub static ADAPTIVE_CHEST: LazyLock<Vec<String>> = LazyLock::new(|| vec!["3ABE78".to_string(), "82E3D8".to_string(), "BFBCB2".to_string(), "D579FF".to_string(), "FF4242".to_string(), "FFC234".to_string()]);
/// java: List<String> ADAPTIVE = List.of("169F57", "2AB5A5", "6E00A0", "BB0000", "BFBCB2", "FFF7E6");
pub static ADAPTIVE: LazyLock<Vec<String>> = LazyLock::new(|| vec!["169F57".to_string(), "2AB5A5".to_string(), "6E00A0".to_string(), "BB0000".to_string(), "BFBCB2".to_string(), "FFF7E6".to_string()]);

/// java: public interface Constants
pub trait Constants: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// java: static Component createSkyblockerGradient(String string)
pub fn create_skyblocker_gradient(string: &str) -> MutableComponent {
    let mut component: MutableComponent = component::empty();
    for i in 0..compat::str_len(&string) {
        component = component.append((component::literal(&compat::substring2(&string, i, i.wrapping_add(1)))).with_color(OkLabColor::interpolate(0x00FF4C, 0x14D0FF, (((i) as f32) / ((compat::str_len(&string)).wrapping_sub(1) as f32)))));
    }
    return component;
}
