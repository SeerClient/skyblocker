//! Ported from de.hysky.skyblocker.utils.TextTransformer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::network::chat::text_color::TextColor;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern REDUNDANT_COLOR = Pattern.compile("&(?:#[\\da-f]{6}|[\\da-f])(\\s*)&(#[\\da-f]{6}|[\\da-f])");
pub static REDUNDANT_COLOR: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("&(?:#[\\da-f]{6}|[\\da-f])(\\s*)&(#[\\da-f]{6}|[\\da-f])"));
/// java: private static final Pattern TRAILING_WHITESPACE = Pattern.compile("(\\s*)&(#[\\da-f]{6}|[\\da-fk-or])(\\s+)");
pub static TRAILING_WHITESPACE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(\\s*)&(#[\\da-f]{6}|[\\da-fk-or])(\\s+)"));
/// java: private static final Pattern ALL_WHITESPACE = Pattern.compile("\\s+");
pub static ALL_WHITESPACE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\s+"));
//// java: private static final CharList FORMAT_CODES = CharList.of('4', 'c', '6', 'e', '2', 'a', 'b', '3', '1', '9', 'd', '5', 'f', '7', '8', '0', 'r', 'k', 'l', 'm', 'n', 'o');
pub static FORMAT_CODES: LazyLock<Vec<char>> = LazyLock::new(|| vec!['4', 'c', '6', 'e', '2', 'a', 'b', '3', '1', '9', 'd', '5', 'f', '7', '8', '0', 'r', 'k', 'l', 'm', 'n', 'o']);
/// java: private static final Map<Integer, Character> HEX_TO_CODES = Arrays.stream(ChatFormatting.values()).filter(c -> TextColor.fromLegacyFormat(c) != null).collect(Collectors.toUnmodifiableMap(cf -> TextColor.fromLegacyFormat(cf).getValue(), cf -> cf.code));
pub static HEX_TO_CODES: LazyLock<std::collections::HashMap<i32, char>> = LazyLock::new(|| {
    chat_formatting::VALUES
        .iter()
        .filter_map(|cf| TextColor::from_legacy_format(*cf).map(|color| (color.get_value(), cf.code())))
        .collect()
});
/// java: private static final List<Map.Entry<Predicate<Style>, Character>> PREDICATE_FORMAT_LIST = List.of(
///   Map.entry(Style::isBold, 'l'), Map.entry(Style::isItalic, 'o'), Map.entry(Style::isObfuscated, 'k'),
///   Map.entry(Style::isUnderlined, 'n'), Map.entry(Style::isStrikethrough, 'm'));
pub static PREDICATE_FORMAT_LIST: LazyLock<Vec<(fn(&Style) -> bool, char)>> = LazyLock::new(|| {
    vec![
        (Style::is_bold as fn(&Style) -> bool, 'l'),
        (Style::is_italic as fn(&Style) -> bool, 'o'),
        (Style::is_obfuscated as fn(&Style) -> bool, 'k'),
        (Style::is_underlined as fn(&Style) -> bool, 'n'),
        (Style::is_strikethrough as fn(&Style) -> bool, 'm'),
    ]
});

/// java: public class TextTransformer
#[derive(Clone)]
pub struct TextTransformer {}

impl TextTransformer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static String getCodes(Style style, Style previous)
    pub fn get_codes(style: &Style, previous: &Style) -> String {
        // java: final var codes = new StringJoiner("&", "&", "").setEmptyValue("");
        let mut codes: Vec<String> = Vec::new();
        let mut has_reset: bool = false;
        let mut needs_reset: bool = (previous.get_color().is_some() && style.get_color().is_none()) || PREDICATE_FORMAT_LIST.iter().any(|x| (x.0)(previous) && !(x.0)(style));
        if needs_reset && style.get_color().is_none() {
            codes.push("r".to_string());
            has_reset = true;
        } else if style.get_color().is_some() && (needs_reset || previous.get_color() != style.get_color()) {
            let color: TextColor = style.get_color().unwrap();
            let code: Option<char> = HEX_TO_CODES.get(&color.get_value()).copied();
            match code {
                // Adds non-standard hex colors as &#RRGGBB
                // java: codes.add(style.getColor().formatValue()); - formatValue() is "#%06X"
                None => codes.push(format!("#{:06X}", color.get_value())),
                Some(code) => codes.push(code.to_string()),
            }
            has_reset = true;
        }
        let mut already_reset: bool = has_reset;
        // java: PREDICATE_FORMAT_LIST.stream().filter(x -> x.getKey().test(style) && (alreadyReset || !x.getKey().test(previous)))
        //           .map(Map.Entry::getValue).forEach(c -> codes.add("" + c));
        for x in PREDICATE_FORMAT_LIST.iter() {
            if (x.0)(style) && (already_reset || !(x.0)(previous)) {
                codes.push(x.1.to_string());
            }
        }
        // java: codes.toString() - "" when empty, else "&" + the codes joined by "&"
        return if codes.is_empty() { String::new() } else { format!("&{}", codes.join("&")) };
    }

    /// java: private static String removeRedundantStyles(String message)
    pub fn remove_redundant_styles(message: &str) -> String {
        let mut result: String = message.to_string();
        let mut redundant: compat::regex::Matcher = REDUNDANT_COLOR.matcher(message);
        while redundant.find() {
            result = redundant.replace_all("$1&$2");
            redundant.reset_with(&result);
        }
        redundant = TRAILING_WHITESPACE.matcher(&result);
        while redundant.find() {
            result = redundant.replace_all("$1$3&$2");
            redundant.reset_with(&result);
        }
        return result;
    }

    /// java: public static String toLegacy(Component component)
    pub fn to_legacy(component: &MutableComponent) -> String {
        let mut result = String::new();
        // java: final var previous = new AtomicReference<>(Style.EMPTY);
        let mut previous: Style = Style::EMPTY;
        component.visit_styled(&mut |style: &Style, string: &str| {
            // don't bother styling whitespace
            if ALL_WHITESPACE.matcher(string).matches() {
                result.push_str(string);
            } else {
                result.push_str(&TextTransformer::get_codes(style, &previous));
                result.push_str(&string.replace('§', "&"));
            }
            previous = style.clone();
            // java: return Optional.empty(); - keep visiting
            true
        }, &Style::EMPTY);
        return TextTransformer::remove_redundant_styles(&result);
    }

    /// java: public static MutableComponent fromLegacy(String legacy)
    pub fn from_legacy(legacy: &str) -> MutableComponent {
        return TextTransformer::from_legacy_3(legacy, '§', true);
    }

    /// java: public static MutableComponent fromLegacy(String legacy, char legacyPrefix, boolean override)
    pub fn from_legacy_3(legacy: &str, mut legacy_prefix: char, mut r#override: bool) -> MutableComponent {
        let mut new_text: MutableComponent = component::empty();
        let mut builder: String = String::new();
        let mut formatting: Option<ChatFormatting> = None;
        // java: Boolean bold = override ? false : null; (and the other four)
        let mut bold: Option<bool> = if r#override { Some(false) } else { None };
        let mut italic: Option<bool> = if r#override { Some(false) } else { None };
        let mut underline: Option<bool> = if r#override { Some(false) } else { None };
        let mut strikethrough: Option<bool> = if r#override { Some(false) } else { None };
        let mut obfuscated: Option<bool> = if r#override { Some(false) } else { None };
        let chars: Vec<char> = legacy.chars().collect();
        let length: usize = chars.len();
        for i in 0..length {
            //If we've encountered a new formatting code then append the text from the previous "sequence" and reset state
            if i != 0 && chars[i - 1] == legacy_prefix && FORMAT_CODES.contains(&compat::char_lower(chars[i])) && !builder.is_empty() {
                new_text = new_text.append(component::literal(&builder).set_style(Style::EMPTY.with_color_format(formatting).with_bold(bold).with_italic(italic).with_underlined(underline).with_strikethrough(strikethrough).with_obfuscated(obfuscated)));
                //Erase all characters in the builder so we can reuse it, also clear formatting
                builder.clear();
                formatting = None;
                bold = if r#override { Some(false) } else { None };
                italic = if r#override { Some(false) } else { None };
                underline = if r#override { Some(false) } else { None };
                strikethrough = if r#override { Some(false) } else { None };
                obfuscated = if r#override { Some(false) } else { None };
            }
            if i != 0 && chars[i - 1] == legacy_prefix {
                let mut fmt: Option<ChatFormatting> = chat_formatting::get_by_code(chars[i]);
                match fmt {
                    Some(ChatFormatting::Bold) => {
                        bold = Some(true);
                    }
                    Some(ChatFormatting::Italic) => {
                        italic = Some(true);
                    }
                    Some(ChatFormatting::Underline) => {
                        underline = Some(true);
                    }
                    Some(ChatFormatting::Strikethrough) => {
                        strikethrough = Some(true);
                    }
                    Some(ChatFormatting::Obfuscated) => {
                        obfuscated = Some(true);
                    }
                    _ => {
                        formatting = fmt;
                    }
                }
                continue;
            }
            //This character isn't the start of a formatting sequence or this character isn't part of a formatting sequence
            if chars[i] != legacy_prefix && (i == 0 || chars[i - 1] != legacy_prefix) {
                builder.push(chars[i]);
            }
            // We've read the last character so append the last text with all the formatting
            if i == length - 1 {
                new_text = new_text.append(component::literal(&builder).set_style(Style::EMPTY.with_color_format(formatting).with_bold(bold).with_italic(italic).with_underlined(underline).with_strikethrough(strikethrough).with_obfuscated(obfuscated)));
            }
        }
        return new_text;
    }

}
