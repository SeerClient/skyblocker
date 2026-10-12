//! Ported from de.hysky.skyblocker.utils.RomanNumerals by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::text::decimal_format_symbols::Locale;

// TODO(fabric2seer): static call Int2ObjectMaps.unmodifiable(..)
//   java: private static final Int2ObjectMap<String> ROMAN_NUMERALS = Int2ObjectMaps.unmodifiable(new Int2ObjectLinkedOpenHashMap<>(
//   java: 			new int[]{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1},
//   java: 			new String[]{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}
//   java: 	));
/// java: private static final Int2ObjectMap<String> ROMAN_NUMERALS = Int2ObjectMaps.unmodifiable(new Int2ObjectLinkedOpenHashMap<>( new int[]{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}, new String[]
pub static ROMAN_NUMERALS: LazyLock<std::collections::HashMap<i32, String>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `ROMAN_NUMERALS` - see the TODO above"));

/// java: public class RomanNumerals
#[derive(Clone)]
pub struct RomanNumerals {}

impl RomanNumerals {
    /// java: private RomanNumerals()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static int getDecimalValue(char romanChar)
    pub fn get_decimal_value(mut roman_char: char) -> i32 {
        return match roman_char {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
    }

    /// java: public static boolean isValidRomanNumeral(@Nullable String romanNumeral)
    pub fn is_valid_roman_numeral(mut roman_numeral: Option<String>) -> bool {
        if roman_numeral.is_none() || roman_numeral.as_ref().unwrap().is_empty() {
            return false;
        }
        for i in 0..compat::str_len(&roman_numeral.as_ref().unwrap()) {
            if RomanNumerals::get_decimal_value(compat::char_at(&roman_numeral.as_ref().unwrap(), i)) == 0 {
                return false;
            }
        }
        return true;
    }

    /// java: public static int romanToDecimal(@Nullable String romanNumeral)
    pub fn roman_to_decimal(mut roman_numeral: Option<String>) -> i32 {
        if roman_numeral.is_none() || roman_numeral.as_ref().unwrap().is_empty() {
            return 0;
        }
        roman_numeral = Some(compat::trim(&roman_numeral.as_ref().unwrap()).to_uppercase());
        let mut decimal: i32 = 0;
        let mut last_number: i32 = 0;
        {
            let mut i: i32 = (compat::str_len(&roman_numeral.as_ref().unwrap())).wrapping_sub(1);
            while i >= 0 {
                let mut ch: char = compat::char_at(&roman_numeral.as_ref().unwrap(), i);
                let mut number: i32 = RomanNumerals::get_decimal_value(ch);
                if number == 0 {
                    return 0;
                }
                decimal = (if number >= last_number { decimal.wrapping_add(number) } else { decimal.wrapping_sub(number) });
                last_number = number;
                i -= 1;
            }
        }
        return decimal;
    }

    /// java: public static String decimalToRoman(int decimal)
    pub fn decimal_to_roman(mut decimal: i32) -> String {
        if decimal <= 0 || decimal >= 4000 {
            return "".to_string();
        }
        let mut roman: String = String::new();
        for entry in ROMAN_NUMERALS.iter().map(|(k, v)| (k.clone(), v.clone())) {
            let mut value: i32 = entry.0;
            let mut numeral: String = entry.1.clone();
            while decimal >= value {
                { roman.push_str(&numeral); };
                decimal = decimal.wrapping_sub(value);
            }
        }
        return roman.clone();
    }

}
