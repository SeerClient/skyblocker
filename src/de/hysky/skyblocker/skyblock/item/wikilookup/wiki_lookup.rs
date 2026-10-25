//! Ported from de.hysky.skyblocker.skyblock.item.wikilookup.WikiLookup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::wikilookup::wiki_lookup_manager::WikiLookupManager;
use mcrust::com::mojang::datafixers::util::either::Either;
use mcrust::net::minecraft::world::entity::player::player::Player;
use mcrust::net::minecraft::world::inventory::slot::Slot;

/// java: Map<String, String> CHARACTER_ENCODER = Map.ofEntries( Map.entry(" ", "_"), Map.entry("'", encodeChar("'")), // Special case for only a visitor that has '?' Map.entry("?", encodeChar("?")) );
pub static CHARACTER_ENCODER: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| std::collections::HashMap::from([(" ".to_string(), "_".to_string()), ("'".to_string(), encode_char("'")), ("?".to_string(), encode_char("?"))]));
/// java: Function<String, String> REPLACING_FUNCTION = CHARACTER_ENCODER.entrySet().stream() .map(entryToReplace -> (Function<String, String>) s -> s.replace(entryToReplace.getKey(), entryToReplace.getValue())
pub static REPLACING_FUNCTION: compat::MainThread<Arc<dyn Fn(String) -> String>> = compat::MainThread::new(|| { let __fs: Vec<Arc<dyn Fn(String) -> String>> = CHARACTER_ENCODER.iter().map(|(k, v)| { let (k, v) = (k.clone(), v.clone()); Arc::new(move |s: String| s.replace(&k, &v)) as Arc<dyn Fn(String) -> String> }).collect(); Arc::new(move |s: String| __fs.iter().fold(s, |acc, f| f(acc))) as Arc<dyn Fn(String) -> String> });

/// java: public interface WikiLookup
pub trait WikiLookup: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void open(ItemStack itemStack, Player player); (dispatch: the body for this class)
    fn open(&self) -> fn(&compat::Shared<dyn WikiLookup>, &mcrust::net::minecraft::world::item::item_stack::ItemStack, &dyn Player);

    /// java: default boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either) (dispatch: the body for this class)
    fn can_search(&self) -> fn(&compat::Shared<dyn WikiLookup>, Option<String>, &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        <dyn WikiLookup>::can_search_body
    }

}

impl dyn WikiLookup {
    /// java: default boolean canSearch(@Nullable String title, Either<Slot, ItemStack> either)
    pub fn can_search_body(__this: &compat::Shared<dyn WikiLookup>, mut title: Option<String>, either: &Either<Slot, mcrust::net::minecraft::world::item::item_stack::ItemStack>) -> bool {
        let mut item_stack: mcrust::net::minecraft::world::item::item_stack::ItemStack = WikiLookupManager::map_either_to_item_stack(either);
        return !item_stack.is_empty();
    }

}

/// java: private static String encodeChar(String character)
pub fn encode_char(character: &str) -> String {
    // java: return URLEncoder.encode(character, StandardCharsets.UTF_8);
    // (`URLEncoder.encode`: `a-z A-Z 0-9 . - * _` are kept, a space is `+`, every other byte of the UTF-8 form is
    // `%XY` with upper-case hex)
    let mut encoded = String::new();
    for byte in character.as_bytes() {
        match *byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'.' | b'-' | b'*' | b'_' => encoded.push(*byte as char),
            b' ' => encoded.push('+'),
            other => encoded.push_str(&format!("%{:02X}", other)),
        }
    }
    return encoded;
}
