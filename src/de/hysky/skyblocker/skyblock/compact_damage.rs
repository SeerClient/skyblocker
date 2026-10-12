//! Ported from de.hysky.skyblocker.skyblock.CompactDamage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern DAMAGE_PATTERN = Pattern.compile("([✧✯]?)[\\d,]+[✧✯]?([❤+⚔☄♞]?)");
pub static DAMAGE_PATTERN: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("([✧✯]?)[\\d,]+[✧✯]?([❤+⚔☄♞]?)"));
/// java: private static final int[] guesses = new int[]{ 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 12, 12, 12, 13, 13, 13, 13, 14,
pub static GUESSES: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 12, 12, 12, 13, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 16, 16, 16, 17, 17, 17, 18, 18, 18, 19, 19, 19]);
/// java: private static final long[] powersOfTen = new long[]{ 1L, 10L, 100L, 1000L, 10000L, 100000L, 1000000L, 10000000L, 100000000L, 1000000000L, 10000000000L, 100000000000L, 1000000000000L, 10000000000000L,
pub static POWERS_OF_TEN: LazyLock<Vec<i64>> = LazyLock::new(|| vec![1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000, 10000000000, 100000000000, 1000000000000, 10000000000000, 100000000000000, 1000000000000000, 10000000000000000, 100000000000000000, 1000000000000000000]);

/// java: public class CompactDamage
#[derive(Clone)]
pub struct CompactDamage {}

impl CompactDamage {
    /// java: private CompactDamage()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void compactDamage(ArmorStand entity)
    pub fn compact_damage(entity: &ArmorStand) {
        if !(SkyblockerConfigManager::get().ui_and_visuals.compact_damage.enabled) {
            return;
        }
        if !entity.is_invisible_field() || !entity.base.base.has_custom_name() || !entity.base.base.is_custom_name_visible() {
            return;
        }
        let Some(mut custom_name) = entity.base.base.get_custom_name() else {
            return;
        };
        let Some(mut prettier_custom_name) = CompactDamage::compact_damage_component(&custom_name) else {
            return;
        };
        // TODO(fabric2seer): `ArmorStand.skyblocker$setCustomName` has no Rust method in the port (looked for `skyblocker_set_custom_name` in ArmorStand and JavaSrc)
        // rust: entity.skyblocker_set_custom_name(&prettier_custom_name);
    }

    /// java: @VisibleForTesting public static @Nullable Component compactDamage(Component customName)
    pub fn compact_damage_component(custom_name: &MutableComponent) -> Option<MutableComponent> {
        let config = &SkyblockerConfigManager::get().ui_and_visuals.compact_damage;
        let mut custom_name_string: String = custom_name.get_string();
        let mut matcher: compat::regex::Matcher = DAMAGE_PATTERN.matcher(&custom_name_string);
        if !matcher.matches() {
            return None;
        }
        let siblings = custom_name.get_siblings();
        if siblings.is_empty() {
            return None;
        }
        let mut is_crit: bool = !(matcher.group(1).unwrap().is_empty());
        let mut dmg: String = compat::regex::replace_all(&custom_name_string, "\\D", "");
        // TODO(fabric2seer): the condition: static call NumberUtils.isParsable(..)
        //   java: !NumberUtils.isParsable(dmg)
        if todo!("fabric2seer: the condition (TODO above)") {
            return None;
        }
        let mut text_color: Option<TextColor> = siblings[0].get_style().get_color();
        let mut prettier_custom_name: MutableComponent = component::empty();
        let mut prettified_dmg: String = CompactDamage::prettify_damage_number(dmg.trim().parse::<i64>().expect("NumberFormatException"), config.max_precision);
        if is_crit {
            let mut dmg_symbol: String = matcher.group(1).unwrap();
            prettified_dmg = format!("{}{}{}", dmg_symbol, prettified_dmg, dmg_symbol);
            let mut length: i32 = compat::str_len(&prettified_dmg);
            for i in 0..length {
                // TODO(fabric2seer): `.getRGB(..)` on a Color
                //   java: prettierCustomName.append(Component.literal(prettifiedDmg.substring(i, i + 1)).withColor(
                //   java: 						OkLabColor.interpolate(
                //   java: 								config.critDamageGradientStart.getRGB() & 0x00_FF_FF_FF,
                //   java: 								config.critDamageGradientEnd.getRGB() & 0x00_FF_FF_FF,
                //   java: 								i / (length - 1.0f)
                //   java: 						)
                //   java: 				));
            }
            prettier_custom_name.set_style(custom_name.get_style().clone());
        } else {
            let mut color: i32;
            if text_color.is_none() || std::ptr::eq(text_color.as_ref().unwrap(), &TextColor::from_legacy_format(ChatFormatting::Gray).unwrap()) {
                // TODO(fabric2seer): `.getRGB(..)` on a Color
                //   java: color = config.normalDamageColor.getRGB() & 0x00_FF_FF_FF;
            } else {
                color = text_color.as_ref().unwrap().get_value();
            }
            prettier_custom_name = component::literal(&prettified_dmg).set_style(custom_name.get_style().clone()).with_color(color);
        }
        if !(matcher.group(2).unwrap().is_empty()) {
            prettier_custom_name.append(component::literal(matcher.group(2).as_deref().unwrap()).set_style((siblings[siblings.len() - 1]).get_style().clone()));
        }
        return Some(prettier_custom_name);
    }

    /// java: @VisibleForTesting static String prettifyDamageNumber(final long damage, final int maxPrecision)
    pub fn prettify_damage_number(mut damage: i64, mut max_precision: i32) -> String {
        let mut target_damage: i64 = damage;
        let mut target_precision: i32 = max_precision;
        let mut used_precision: i32 = CompactDamage::base_ten_digits(target_damage);
        if used_precision > target_precision {
            let mut power_to_round_to: f64 = (POWERS_OF_TEN[(used_precision.wrapping_sub(max_precision)) as usize]) as f64;
            target_damage = ((((compat::java_round_f64((((target_damage) as f64) / power_to_round_to)) as f64) * power_to_round_to)) as i64);
        } else if target_precision > used_precision {
            target_precision = used_precision;
        }
        if target_damage < 1000 {
            return target_damage.to_string();
        }
        if target_damage < 1000000 {
            return format!("{}k", CompactDamage::format_to_precision(((target_damage as f64) / 1000.0), target_precision));
        }
        if target_damage < 1000000000 {
            return format!("{}M", CompactDamage::format_to_precision(((target_damage as f64) / 1000000.0), target_precision));
        }
        if target_damage < 1000000000000 {
            return format!("{}B", CompactDamage::format_to_precision(((target_damage as f64) / 1000000000.0), target_precision));
        }
        if target_damage < 1000000000000000 {
            return format!("{}T", CompactDamage::format_to_precision(((target_damage as f64) / 1000000000000.0), target_precision));
        }
        return format!("{}Q", CompactDamage::format_to_precision(((target_damage as f64) / 1000000000000000.0), target_precision));
    }

    /// java: @VisibleForTesting static String formatToPrecision(double number, int precision)
    pub fn format_to_precision(mut number: f64, mut precision: i32) -> String {
        let mut used_precision: i32 = CompactDamage::base_ten_digits(((((number) as i32)) as i64) as i64);
        let mut remaining_precision: i32 = precision.wrapping_sub(used_precision);
        if remaining_precision <= 0 {
            let mut power_to_round_to: i64 = POWERS_OF_TEN[(used_precision.wrapping_sub(precision)) as usize];
            return (compat::java_round_f64((number / (power_to_round_to as f64)))).wrapping_mul(power_to_round_to).to_string();
        }
        // TODO(fabric2seer): String.format with a non-literal format
        //   java: return ("%." + remainingPrecision + "f").formatted(number);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static int baseTwoDigits(long x)
    pub fn base_two_digits(mut x: i64) -> i32 {
        // TODO(fabric2seer): Long.numberOfLeadingZeros
        //   java: return 64 - Long.numberOfLeadingZeros(x);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @VisibleForTesting static int baseTenDigits(long x)
    pub fn base_ten_digits(mut x: i64) -> i32 {
        let mut guess: i32 = GUESSES[(CompactDamage::base_two_digits(x)) as usize];
        return guess.wrapping_add(if x >= POWERS_OF_TEN[(guess) as usize] { 1 } else { 0 });
    }

}
