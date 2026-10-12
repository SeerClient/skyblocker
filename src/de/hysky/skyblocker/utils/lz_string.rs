//! Ported from de.hysky.skyblocker.utils.LZString by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static char[] keyStrBase64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".toCharArray();
pub static KEY_STR_BASE64: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| Mutex::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect::<Vec<char>>()));
/// java: private static char[] keyStrUriSafe = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$".toCharArray();
pub static KEY_STR_URI_SAFE: LazyLock<Mutex<Vec<char>>> = LazyLock::new(|| Mutex::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$".chars().collect::<Vec<char>>()));
// TODO(fabric2seer): expression not parsed
//   java:
/// java: 
/// java: private static Map<char[], Map<Character, Integer>> baseReverseDic = new HashMap<char[], Map<Character, Integer>>();
pub static BASE_REVERSE_DIC: LazyLock<Mutex<std::collections::HashMap<Vec<char>, std::collections::HashMap<char, i32>>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
/// java: 
pub static MAP: LazyLock<Mutex<std::collections::HashMap<Vec<char>, std::collections::HashMap<char, i32>>>> = LazyLock::new(|| Mutex::new(Default::default()));

/// java: public class LZString
#[derive(Clone)]
pub struct LZString {}

impl LZString {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static char getBaseValue(char[] alphabet, Character character)
    pub fn get_base_value(mut alphabet: Vec<char>, mut character: char) -> char {
        let mut map: Option<std::collections::HashMap<char, i32>> = compat::lock(&BASE_REVERSE_DIC).get(&alphabet).cloned();
        if map.is_none() {
            map = Some(std::collections::HashMap::new());
            // Java puts the map and then fills it through the same reference: fill first, then store the filled map.
            for i in 0..(alphabet.len() as i32) {
                map.as_mut().unwrap().insert(alphabet[(i) as usize], i);
            }
            compat::lock(&BASE_REVERSE_DIC).insert(alphabet.clone(), map.clone().unwrap());
        }
        return char::from_u32((map.as_ref().unwrap().get(&character).cloned().unwrap()) as u32).unwrap_or('\0');
    }

    /// java: public static String compressToBase64(String input)
    pub fn compress_to_base64(input: &str) -> String {
        if false /* java: input == null: never null here */ {
            return "".to_string();
        }
        let mut res: String = LZString::_compress(input, 6, &(move |a: i32| -> char {
            return compat::lock(&KEY_STR_BASE64)[(a) as usize];
        }));
        match (compat::str_len(&res)).wrapping_rem(4) {
            0 => {
                return res.clone();
            }
            1 => {
                return format!("{}===", res);
            }
            2 => {
                return format!("{}==", res);
            }
            3 => {
                return format!("{}=", res);
            }
            _ => {
                return res.clone();
            }
        }
    }

    /// java: public static String decompressFromBase64(final String inputStr)
    pub fn decompress_from_base64(input_str: &str) -> Option<String> {
        if false /* java: input_str == null: never null here */ {
            return Some("".to_string());
        }
        if input_str == "" {
            return None;
        }
        return LZString::_decompress(compat::str_len(&input_str), 32, &(move |index: i32| -> char {
            return LZString::get_base_value(compat::get_state(&KEY_STR_BASE64), compat::char_at(&input_str, index));
        }));
    }

    /// java: public static String compressToUTF16(String input)
    pub fn compress_to_utf16(input: &str) -> String {
        if false /* java: input == null: never null here */ {
            return "".to_string();
        }
        return format!("{} ", LZString::_compress(input, 15, &(move |a: i32| -> char {
            return LZString::fc(a.wrapping_add(32));
        })));
    }

    /// java: public static String decompressFromUTF16(final String compressedStr)
    pub fn decompress_from_utf16(compressed_str: &str) -> Option<String> {
        if false /* java: compressed_str == null: never null here */ {
            return Some("".to_string());
        }
        if compressed_str.is_empty() {
            return None;
        }
        return LZString::_decompress(compat::str_len(&compressed_str), 16384, &(move |index: i32| -> char {
            return char::from_u32(((compat::char_at(&compressed_str, index) as u32 as i32).wrapping_sub(32)) as u32).unwrap_or('\0');
        }));
    }

    /// java: public static String compressToEncodedURIComponent(String input)
    pub fn compress_to_encoded_uri_component(input: &str) -> String {
        if false /* java: input == null: never null here */ {
            return "".to_string();
        }
        return LZString::_compress(input, 6, &(move |a: i32| -> char {
            return compat::lock(&KEY_STR_URI_SAFE)[(a) as usize];
        }));
    }

    /// java: public static String decompressFromEncodedURIComponent(String inputStr)
    pub fn decompress_from_encoded_uri_component(input_str: &str) -> Option<String> {
        if false /* java: input_str == null: never null here */ {
            return Some("".to_string());
        }
        if input_str.is_empty() {
            return None;
        }
        let mut url_encoded_input_str: String = input_str.replace(' ', &'+'.to_string());
        return LZString::_decompress(compat::str_len(&url_encoded_input_str), 32, &(move |index: i32| -> char {
            return LZString::get_base_value(compat::get_state(&KEY_STR_URI_SAFE), compat::char_at(&url_encoded_input_str, index));
        }));
    }

    /// java: public static String compress(String uncompressed)
    pub fn compress(uncompressed: &str) -> String {
        return LZString::_compress(uncompressed, 16, &(move |a: i32| -> char {
            return LZString::fc(a);
        }));
    }

    /// java: private static String _compress(String uncompressedStr, int bitsPerChar, CompressFunctionWrapper getCharFromInt)
    pub fn _compress(uncompressed_str: &str, mut bits_per_char: i32, get_char_from_int: &dyn Fn(i32) -> char) -> String {
        if false /* java: uncompressed_str == null: never null here */ {
            return "".to_string();
        }
        let mut i: i32;
        let mut value: i32;
        let mut context_dictionary: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        let mut context_dictionary_to_create: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut context_c: String = "".to_string();
        let mut context_wc: String = "".to_string();
        let mut context_w: String = "".to_string();
        let mut context_enlarge_in: i32 = 2;
        let mut context_dict_size: i32 = 3;
        let mut context_num_bits: i32 = 2;
        let mut context_data: String = String::with_capacity(((compat::str_len(&uncompressed_str)).wrapping_div(3)) as usize);
        let mut context_data_val: i32 = 0;
        let mut context_data_position: i32 = 0;
        let mut ii: i32;
        {
            ii = 0;
            while ii < compat::str_len(&uncompressed_str) {
                context_c = compat::char_at(&uncompressed_str, ii).to_string();
                if !(context_dictionary.contains_key(&context_c)) {
                    context_dictionary.insert(context_c.clone(), { let __v = context_dict_size; context_dict_size += 1; __v });
                    context_dictionary_to_create.insert(context_c.clone());
                }
                context_wc = format!("{}{}", context_w, context_c);
                if context_dictionary.contains_key(&context_wc) {
                    context_w = context_wc.clone();
                } else {
                    if context_dictionary_to_create.contains(&context_w) {
                        if (compat::char_at(&context_w, 0) as u32 as i32) < 256 {
                            {
                                i = 0;
                                while i < context_num_bits {
                                    context_data_val = context_data_val.wrapping_shl((1) as u32);
                                    if context_data_position == bits_per_char.wrapping_sub(1) {
                                        context_data_position = 0;
                                        { context_data.push((get_char_from_int(context_data_val))); };
                                        context_data_val = 0;
                                    } else {
                                        context_data_position += 1;
                                    }
                                    i += 1;
                                }
                            }
                            value = (compat::char_at(&context_w, 0)) as i32;
                            {
                                i = 0;
                                while i < 8 {
                                    context_data_val = (context_data_val.wrapping_shl((1) as u32) | (value & 1));
                                    if context_data_position == bits_per_char.wrapping_sub(1) {
                                        context_data_position = 0;
                                        { context_data.push((get_char_from_int(context_data_val))); };
                                        context_data_val = 0;
                                    } else {
                                        context_data_position += 1;
                                    }
                                    value = value.wrapping_shr((1) as u32);
                                    i += 1;
                                }
                            }
                        } else {
                            value = 1;
                            {
                                i = 0;
                                while i < context_num_bits {
                                    context_data_val = (context_data_val.wrapping_shl((1) as u32) | value);
                                    if context_data_position == bits_per_char.wrapping_sub(1) {
                                        context_data_position = 0;
                                        { context_data.push((get_char_from_int(context_data_val))); };
                                        context_data_val = 0;
                                    } else {
                                        context_data_position += 1;
                                    }
                                    value = 0;
                                    i += 1;
                                }
                            }
                            value = (compat::char_at(&context_w, 0)) as i32;
                            {
                                i = 0;
                                while i < 16 {
                                    context_data_val = (context_data_val.wrapping_shl((1) as u32) | (value & 1));
                                    if context_data_position == bits_per_char.wrapping_sub(1) {
                                        context_data_position = 0;
                                        { context_data.push((get_char_from_int(context_data_val))); };
                                        context_data_val = 0;
                                    } else {
                                        context_data_position += 1;
                                    }
                                    value = value.wrapping_shr((1) as u32);
                                    i += 1;
                                }
                            }
                        }
                        context_enlarge_in -= 1;
                        if context_enlarge_in == 0 {
                            context_enlarge_in = LZString::power_of2(context_num_bits);
                            context_num_bits += 1;
                        }
                        context_dictionary_to_create.remove(&context_w);
                    } else {
                        value = context_dictionary.get(&context_w).cloned().unwrap();
                        {
                            i = 0;
                            while i < context_num_bits {
                                context_data_val = (context_data_val.wrapping_shl((1) as u32) | (value & 1));
                                if context_data_position == bits_per_char.wrapping_sub(1) {
                                    context_data_position = 0;
                                    { context_data.push((get_char_from_int(context_data_val))); };
                                    context_data_val = 0;
                                } else {
                                    context_data_position += 1;
                                }
                                value = value.wrapping_shr((1) as u32);
                                i += 1;
                            }
                        }
                    }
                    context_enlarge_in -= 1;
                    if context_enlarge_in == 0 {
                        context_enlarge_in = LZString::power_of2(context_num_bits);
                        context_num_bits += 1;
                    }
                    context_dictionary.insert(context_wc.clone(), { let __v = context_dict_size; context_dict_size += 1; __v });
                    context_w = context_c.clone();
                }
                ii = ii.wrapping_add(1);
            }
        }
        if !context_w.is_empty() {
            if context_dictionary_to_create.contains(&context_w) {
                if (compat::char_at(&context_w, 0) as u32 as i32) < 256 {
                    {
                        i = 0;
                        while i < context_num_bits {
                            context_data_val = context_data_val.wrapping_shl((1) as u32);
                            if context_data_position == bits_per_char.wrapping_sub(1) {
                                context_data_position = 0;
                                { context_data.push((get_char_from_int(context_data_val))); };
                                context_data_val = 0;
                            } else {
                                context_data_position += 1;
                            }
                            i += 1;
                        }
                    }
                    value = (compat::char_at(&context_w, 0)) as i32;
                    {
                        i = 0;
                        while i < 8 {
                            context_data_val = (context_data_val.wrapping_shl((1) as u32) | (value & 1));
                            if context_data_position == bits_per_char.wrapping_sub(1) {
                                context_data_position = 0;
                                { context_data.push((get_char_from_int(context_data_val))); };
                                context_data_val = 0;
                            } else {
                                context_data_position += 1;
                            }
                            value = value.wrapping_shr((1) as u32);
                            i += 1;
                        }
                    }
                } else {
                    value = 1;
                    {
                        i = 0;
                        while i < context_num_bits {
                            context_data_val = (context_data_val.wrapping_shl((1) as u32) | value);
                            if context_data_position == bits_per_char.wrapping_sub(1) {
                                context_data_position = 0;
                                { context_data.push((get_char_from_int(context_data_val))); };
                                context_data_val = 0;
                            } else {
                                context_data_position += 1;
                            }
                            value = 0;
                            i += 1;
                        }
                    }
                    value = (compat::char_at(&context_w, 0)) as i32;
                    {
                        i = 0;
                        while i < 16 {
                            context_data_val = (context_data_val.wrapping_shl((1) as u32) | (value & 1));
                            if context_data_position == bits_per_char.wrapping_sub(1) {
                                context_data_position = 0;
                                { context_data.push((get_char_from_int(context_data_val))); };
                                context_data_val = 0;
                            } else {
                                context_data_position += 1;
                            }
                            value = value.wrapping_shr((1) as u32);
                            i += 1;
                        }
                    }
                }
                context_enlarge_in -= 1;
                if context_enlarge_in == 0 {
                    context_enlarge_in = LZString::power_of2(context_num_bits);
                    context_num_bits += 1;
                }
                context_dictionary_to_create.remove(&context_w);
            } else {
                value = context_dictionary.get(&context_w).cloned().unwrap();
                {
                    i = 0;
                    while i < context_num_bits {
                        context_data_val = (context_data_val.wrapping_shl((1) as u32) | (value & 1));
                        if context_data_position == bits_per_char.wrapping_sub(1) {
                            context_data_position = 0;
                            { context_data.push((get_char_from_int(context_data_val))); };
                            context_data_val = 0;
                        } else {
                            context_data_position += 1;
                        }
                        value = value.wrapping_shr((1) as u32);
                        i += 1;
                    }
                }
            }
            context_enlarge_in -= 1;
            if context_enlarge_in == 0 {
                context_enlarge_in = LZString::power_of2(context_num_bits);
                context_num_bits += 1;
            }
        }
        value = 2;
        {
            i = 0;
            while i < context_num_bits {
                context_data_val = (context_data_val.wrapping_shl((1) as u32) | (value & 1));
                if context_data_position == bits_per_char.wrapping_sub(1) {
                    context_data_position = 0;
                    { context_data.push((get_char_from_int(context_data_val))); };
                    context_data_val = 0;
                } else {
                    context_data_position += 1;
                }
                value = value.wrapping_shr((1) as u32);
                i += 1;
            }
        }
        loop {
            context_data_val = context_data_val.wrapping_shl((1) as u32);
            if context_data_position == bits_per_char.wrapping_sub(1) {
                { context_data.push((get_char_from_int(context_data_val))); };
                break;
            } else {
                context_data_position += 1;
            }
        }
        return context_data.clone();
    }

    /// java: public static String f(int i)
    pub fn f(mut i: i32) -> String {
        return char::from_u32((i) as u32).unwrap_or('\0').to_string();
    }

    /// java: public static char fc(int i)
    pub fn fc(mut i: i32) -> char {
        return char::from_u32((i) as u32).unwrap_or('\0');
    }

    /// java: public static String decompress(final String compressed)
    pub fn decompress(compressed: &str) -> Option<String> {
        if false /* java: compressed == null: never null here */ {
            return Some("".to_string());
        }
        if compressed.is_empty() {
            return None;
        }
        return LZString::_decompress(compat::str_len(&compressed), 32768, &(move |i: i32| -> char {
            return compat::char_at(&compressed, i);
        }));
    }

    /// java: private static String _decompress(int length, int resetValue, DecompressFunctionWrapper getNextValue)
    pub fn _decompress(mut length: i32, mut reset_value: i32, get_next_value: &dyn Fn(i32) -> char) -> Option<String> {
        let mut dictionary: Vec<String> = Vec::new();
        let mut next: i32;
        let mut enlarge_in: i32 = 4;
        let mut dict_size: i32 = 4;
        let mut num_bits: i32 = 3;
        let mut entry: String = "".to_string();
        let mut result: String = String::new();
        let mut w: String;
        let mut bits: i32;
        let mut resb: i32;
        let mut maxpower: i32;
        let mut power: i32;
        let mut c: Option<String> = None;
        let mut data: DecData = DecData::new();
        data.val = (get_next_value(0));
        data.position = reset_value;
        data.index = 1;
        for i in 0..3 {
            dictionary.insert((i) as usize, LZString::f(i));
        }
        bits = 0;
        maxpower = LZString::power_of2(2);
        power = 1;
        while power != maxpower {
            resb = ((data.val as u32 as i32) & data.position);
            data.position = data.position.wrapping_shr((1) as u32);
            if data.position == 0 {
                data.position = reset_value;
                data.val = (get_next_value({ let __v = data.index; data.index += 1; __v }));
            }
            bits = (bits | (if resb > 0 { 1i32 } else { 0i32 }).wrapping_mul(power));
            power = power.wrapping_shl((1) as u32);
        }
        match { next = bits; next } {
            0 => {
                bits = 0;
                maxpower = LZString::power_of2(8);
                power = 1;
                while power != maxpower {
                    resb = ((data.val as u32 as i32) & data.position);
                    data.position = data.position.wrapping_shr((1) as u32);
                    if data.position == 0 {
                        data.position = reset_value;
                        data.val = (get_next_value({ let __v = data.index; data.index += 1; __v }));
                    }
                    bits = (bits | (if resb > 0 { 1i32 } else { 0i32 }).wrapping_mul(power));
                    power = power.wrapping_shl((1) as u32);
                }
                c = Some(LZString::f(bits));
            }
            1 => {
                bits = 0;
                maxpower = LZString::power_of2(16);
                power = 1;
                while power != maxpower {
                    resb = ((data.val as u32 as i32) & data.position);
                    data.position = data.position.wrapping_shr((1) as u32);
                    if data.position == 0 {
                        data.position = reset_value;
                        data.val = (get_next_value({ let __v = data.index; data.index += 1; __v }));
                    }
                    bits = (bits | (if resb > 0 { 1i32 } else { 0i32 }).wrapping_mul(power));
                    power = power.wrapping_shl((1) as u32);
                }
                c = Some(LZString::f(bits));
            }
            2 => {
                return Some("".to_string());
            }
            _ => {}
        }
        dictionary.insert((3) as usize, c.clone().unwrap());
        w = c.clone().unwrap();
        { result.push_str(&w); };
        loop {
            if data.index > length {
                return Some("".to_string());
            }
            bits = 0;
            maxpower = LZString::power_of2(num_bits);
            power = 1;
            while power != maxpower {
                resb = ((data.val as u32 as i32) & data.position);
                data.position = data.position.wrapping_shr((1) as u32);
                if data.position == 0 {
                    data.position = reset_value;
                    data.val = (get_next_value({ let __v = data.index; data.index += 1; __v }));
                }
                bits = (bits | (if resb > 0 { 1i32 } else { 0i32 }).wrapping_mul(power));
                power = power.wrapping_shl((1) as u32);
            }
            let mut cc: i32;
            match { cc = bits; cc } {
                0 => {
                    bits = 0;
                    maxpower = LZString::power_of2(8);
                    power = 1;
                    while power != maxpower {
                        resb = ((data.val as u32 as i32) & data.position);
                        data.position = data.position.wrapping_shr((1) as u32);
                        if data.position == 0 {
                            data.position = reset_value;
                            data.val = (get_next_value({ let __v = data.index; data.index += 1; __v }));
                        }
                        bits = (bits | (if resb > 0 { 1i32 } else { 0i32 }).wrapping_mul(power));
                        power = power.wrapping_shl((1) as u32);
                    }
                    dictionary.insert(({ let __v = dict_size; dict_size += 1; __v }) as usize, LZString::f(bits));
                    cc = dict_size.wrapping_sub(1);
                    enlarge_in -= 1;
                }
                1 => {
                    bits = 0;
                    maxpower = LZString::power_of2(16);
                    power = 1;
                    while power != maxpower {
                        resb = ((data.val as u32 as i32) & data.position);
                        data.position = data.position.wrapping_shr((1) as u32);
                        if data.position == 0 {
                            data.position = reset_value;
                            data.val = (get_next_value({ let __v = data.index; data.index += 1; __v }));
                        }
                        bits = (bits | (if resb > 0 { 1i32 } else { 0i32 }).wrapping_mul(power));
                        power = power.wrapping_shl((1) as u32);
                    }
                    dictionary.insert(({ let __v = dict_size; dict_size += 1; __v }) as usize, LZString::f(bits));
                    cc = dict_size.wrapping_sub(1);
                    enlarge_in -= 1;
                }
                2 => {
                    return Some(result.clone());
                }
                _ => {}
            }
            if enlarge_in == 0 {
                enlarge_in = LZString::power_of2(num_bits);
                num_bits += 1;
            }
            if cc < (dictionary.len() as i32) && true /* java: dictionary[(cc) as usize] != null: never null here */ {
                entry = dictionary[(cc) as usize].clone();
            } else {
                if cc == dict_size {
                    entry = format!("{}{}", w, compat::char_at(&w, 0));
                } else {
                    return None;
                }
            }
            { result.push_str(&entry); };
            dictionary.insert(({ let __v = dict_size; dict_size += 1; __v }) as usize, format!("{}{}", w, compat::char_at(&entry, 0)));
            enlarge_in -= 1;
            w = entry.clone();
            if enlarge_in == 0 {
                enlarge_in = LZString::power_of2(num_bits);
                num_bits += 1;
            }
        }
    }

    /// java: private static int powerOf2(int power)
    pub fn power_of2(mut power: i32) -> i32 {
        return 1i32.wrapping_shl((power) as u32);
    }

}

/// java: private abstract static class CompressFunctionWrapper
#[derive(Clone)]
pub struct CompressFunctionWrapperBase {}

impl CompressFunctionWrapperBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn CompressFunctionWrapper>, __R)) -> (compat::Shared<dyn CompressFunctionWrapper>, __R) {
        let this = Self {
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of CompressFunctionWrapper (implemented by CompressFunctionWrapperBase and every subclass)
pub trait CompressFunctionWrapper: compat::DynAny {
    fn compress_function_wrapper_base(&self) -> &CompressFunctionWrapperBase;
    fn compress_function_wrapper_base_mut(&mut self) -> &mut CompressFunctionWrapperBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract char doFunc(int i); (dispatch: the body for this class)
    fn do_func(&self) -> fn(&compat::Shared<dyn CompressFunctionWrapper>, i32) -> char;

}

/// java: private abstract static class DecompressFunctionWrapper
#[derive(Clone)]
pub struct DecompressFunctionWrapperBase {}

impl DecompressFunctionWrapperBase {
    /// java: the implicit no-argument constructor (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn DecompressFunctionWrapper>, __R)) -> (compat::Shared<dyn DecompressFunctionWrapper>, __R) {
        let this = Self {
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of DecompressFunctionWrapper (implemented by DecompressFunctionWrapperBase and every subclass)
pub trait DecompressFunctionWrapper: compat::DynAny {
    fn decompress_function_wrapper_base(&self) -> &DecompressFunctionWrapperBase;
    fn decompress_function_wrapper_base_mut(&mut self) -> &mut DecompressFunctionWrapperBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract char doFunc(int i); (dispatch: the body for this class)
    fn do_func(&self) -> fn(&compat::Shared<dyn DecompressFunctionWrapper>, i32) -> char;

}

/// java: protected static class DecData
#[derive(Clone)]
pub struct DecData {
    /// java: public char val;
    pub val: char,
    /// java: public int position;
    pub position: i32,
    /// java: public int index;
    pub index: i32,
}

impl DecData {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            val: '\0',
            position: 0,
            index: 0,
        };
        this
    }

}
