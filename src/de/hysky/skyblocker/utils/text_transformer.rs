//! Ported from de.hysky.skyblocker.utils.TextTransformer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::chat_formatting;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Pattern REDUNDANT_COLOR = Pattern.compile("&(?:#[\\da-f]{6}|[\\da-f])(\\s*)&(#[\\da-f]{6}|[\\da-f])");
pub static REDUNDANT_COLOR: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("&(?:#[\\da-f]{6}|[\\da-f])(\\s*)&(#[\\da-f]{6}|[\\da-f])"));
/// java: private static final Pattern TRAILING_WHITESPACE = Pattern.compile("(\\s*)&(#[\\da-f]{6}|[\\da-fk-or])(\\s+)");
pub static TRAILING_WHITESPACE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("(\\s*)&(#[\\da-f]{6}|[\\da-fk-or])(\\s+)"));
/// java: private static final Pattern ALL_WHITESPACE = Pattern.compile("\\s+");
pub static ALL_WHITESPACE: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("\\s+"));
// TODO(fabric2seer): static call CharList.of(..)
//   java: private static final CharList FORMAT_CODES = CharList.of('4', 'c', '6', 'e', '2', 'a', 'b', '3', '1', '9', 'd', '5', 'f', '7', '8', '0', 'r', 'k', 'l', 'm', 'n', 'o');
/// java: private static final CharList FORMAT_CODES = CharList.of('4', 'c', '6', 'e', '2', 'a', 'b', '3', '1', '9', 'd', '5', 'f', '7', '8', '0', 'r', 'k', 'l', 'm', 'n', 'o');
pub static FORMAT_CODES: LazyLock<Vec<char>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `FORMAT_CODES` - see the TODO above"));
// TODO(fabric2seer): static `ChatFormatting.values` has no Rust counterpart in the port
//   java: private static final Map<Integer, Character> HEX_TO_CODES = Arrays.stream(ChatFormatting.values()).filter(c -> TextColor.fromLegacyFormat(c) != null).collect(Collectors.toUnmodifiableMap(cf -> TextColor.fromLegacyFormat(cf).getValue(), cf -> cf.code));
/// java: private static final Map<Integer, Character> HEX_TO_CODES = Arrays.stream(ChatFormatting.values()).filter(c -> TextColor.fromLegacyFormat(c) != null).collect(Collectors.toUnmodifiableMap(cf -> TextCol
pub static HEX_TO_CODES: LazyLock<std::collections::HashMap<i32, char>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `HEX_TO_CODES` - see the TODO above"));
// TODO(fabric2seer): `.isBold(..)` on a value of unknown type
//   java: private static final List<Map.Entry<Predicate<Style>, Character>> PREDICATE_FORMAT_LIST = List.of(
//   java: 			Map.entry(Style::isBold, 'l'),
//   java: 			Map.entry(Style::isItalic, 'o'),
//   java: 			Map.entry(Style::isObfuscated, 'k'),
//   java: 			Map.entry(Style::isUnderlined, 'n'),
//   java: 			Map.entry(Style::isStrikethrough, 'm')
//   java: 	);
/// java: private static final List<Map.Entry<Predicate<Style>, Character>> PREDICATE_FORMAT_LIST = List.of( Map.entry(Style::isBold, 'l'), Map.entry(Style::isItalic, 'o'), Map.entry(Style::isObfuscated, 'k'), 
pub static PREDICATE_FORMAT_LIST: compat::MainThread<Vec<(Arc<dyn Fn(Style) -> bool>, char)>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `PREDICATE_FORMAT_LIST` - see the TODO above"));

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
        // TODO(fabric2seer): `new StringJoiner(..)`
        //   java: final var codes = new StringJoiner("&", "&", "").setEmptyValue("");
        let mut has_reset: bool = false;
        let mut needs_reset: bool = (previous.get_color().is_some() && style.get_color().is_none() || PREDICATE_FORMAT_LIST.get_mut().iter().cloned().any(|x| ((x.0)(previous.clone()) && !((x.0)(style.clone())))));
        if needs_reset && style.get_color().is_none() {
            // TODO(fabric2seer): `.add(..)` on a value of unknown type
            //   java: codes.add("r");
            has_reset = true;
        } else if style.get_color().is_some() && needs_reset || !(previous.get_color().unwrap() == style.get_color().unwrap()) {
            let mut code: char = HEX_TO_CODES.get(&style.get_color().unwrap().get_value()).cloned().unwrap();
            if false /* java: code == null: never null here */ {
                // TODO(fabric2seer): `.add(..)` on a value of unknown type
                //   java: codes.add(style.getColor().formatValue());
            } else {
                // TODO(fabric2seer): `.add(..)` on a value of unknown type
                //   java: codes.add("" + code);
            }
            has_reset = true;
        }
        let mut already_reset: bool = has_reset;
        // TODO(fabric2seer): static field Map.Entry
        //   java: PREDICATE_FORMAT_LIST.stream()
        //   java: 				.filter(x -> x.getKey().test(style) && (alreadyReset || !x.getKey().test(previous)))
        //   java: 				.map(Map.Entry::getValue)
        //   java: 				.forEach(c -> codes.add("" + c));
        return compat::jstring(&codes);
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
        // TODO(fabric2seer): `new AtomicReference(..)`
        //   java: final var previous = new AtomicReference<>(Style.EMPTY);
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`net.minecraft.network.chat.FormattedText.StyledContentConsumer<T>`)
        // rust: component.visit_styled(|style, string| {
        // rust: if ALL_WHITESPACE.matcher(&string).matches() {
        // rust: { result.push_str(&string); };
        // rust: } else {
        // rust: // TODO(fabric2seer): `.get(..)` on a value of unknown type
        // rust: //   java: result.append(getCodes(style, previous.get()));
        // rust: { result.push_str(&string.replace('§', &'&'.to_string())); };
        // rust: }
        // rust: // TODO(fabric2seer): `.set(..)` on a value of unknown type
        // rust: //   java: previous.set(style);
        // rust: return None;
        // rust: }, &mcrust::net::minecraft::network::chat::style::Style::EMPTY);
        return TextTransformer::remove_redundant_styles(&result.clone());
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
        let mut bold: bool = (if r#override { Some(false) } else { None }).unwrap();
        let mut italic: bool = (if r#override { Some(false) } else { None }).unwrap();
        let mut underline: bool = (if r#override { Some(false) } else { None }).unwrap();
        let mut strikethrough: bool = (if r#override { Some(false) } else { None }).unwrap();
        let mut obfuscated: bool = (if r#override { Some(false) } else { None }).unwrap();
        for i in 0..compat::str_len(&legacy) {
            if i != 0 && compat::char_at(&legacy, i.wrapping_sub(1)) == legacy_prefix && FORMAT_CODES.iter().any(|e| *e == compat::char_lower(compat::char_at(&legacy, i))) && !builder.is_empty() {
                new_text.append(component::literal(&builder.clone()).set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_format(formatting.clone()).with_bold(Some(bold)).with_italic(Some(italic)).with_underlined(Some(underline)).with_strikethrough(Some(strikethrough)).with_obfuscated(Some(obfuscated))));
                // TODO(fabric2seer): StringBuilder.delete
                //   java: builder.delete(0, builder.length());
                formatting = None;
                bold = (if r#override { Some(false) } else { None }).unwrap();
                italic = (if r#override { Some(false) } else { None }).unwrap();
                underline = (if r#override { Some(false) } else { None }).unwrap();
                strikethrough = (if r#override { Some(false) } else { None }).unwrap();
                obfuscated = (if r#override { Some(false) } else { None }).unwrap();
            }
            if i != 0 && compat::char_at(&legacy, i.wrapping_sub(1)) == legacy_prefix {
                let mut fmt: Option<ChatFormatting> = chat_formatting::get_by_code(compat::char_at(&legacy, i));
                match fmt {
                    Some(ChatFormatting::Bold) => {
                        bold = true;
                    }
                    Some(ChatFormatting::Italic) => {
                        italic = true;
                    }
                    Some(ChatFormatting::Underline) => {
                        underline = true;
                    }
                    Some(ChatFormatting::Strikethrough) => {
                        strikethrough = true;
                    }
                    Some(ChatFormatting::Obfuscated) => {
                        obfuscated = true;
                    }
                    _ => {
                        formatting = fmt.clone();
                    }
                }
                continue;
            }
            if compat::char_at(&legacy, i) != legacy_prefix && i == 0 || compat::char_at(&legacy, i.wrapping_sub(1)) != legacy_prefix {
                { builder.push(compat::char_at(&legacy, i)); };
            }
            if i == (compat::str_len(&legacy)).wrapping_sub(1) {
                new_text.append(component::literal(&builder.clone()).set_style(mcrust::net::minecraft::network::chat::style::Style::EMPTY.with_color_format(formatting.clone()).with_bold(Some(bold)).with_italic(Some(italic)).with_underlined(Some(underline)).with_strikethrough(Some(strikethrough)).with_obfuscated(Some(obfuscated))));
            }
        }
        return new_text;
    }

}
