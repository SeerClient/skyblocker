//! Ported from de.hysky.skyblocker.skyblock.shortcut.ShortcutKeyBinding by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::com::mojang::blaze3d::platform::input_constants::Key;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: static final Codec<ShortcutKeyBinding> CODEC = Codec.STRING.xmap(
///   s -> Arrays.stream(s.split(" \\+ ")).map(InputConstants::getKey).toList(),
///   l -> l.stream().map(InputConstants.Key::getName).collect(Collectors.joining(" + "))
/// ).xmap(ShortcutKeyBinding::new, ShortcutKeyBinding::getBoundKeys);
/// (`LazyLock`: `JsonData` loads and saves on a worker thread)
pub static CODEC: LazyLock<CodecRef<ShortcutKeyBinding>> = LazyLock::new(|| {
    let keys: CodecRef<Vec<Key>> = codec::xmap(
        codec::string_codec(),
        |s: String| {
            compat::regex::split(&s, " \\+ ")
                .iter()
                // java: InputConstants.getKey(String) throws IllegalArgumentException for an unknown key name
                .map(|name| input_constants::get_key(name).unwrap_or_else(|| panic!("IllegalArgumentException: Unknown key name: {name}")))
                .collect::<Vec<Key>>()
        },
        |l: &Vec<Key>| l.iter().map(|key| key.get_name().to_string()).collect::<Vec<String>>().join(" + "),
    );
    codec::xmap(keys, |keys: Vec<Key>| ShortcutKeyBinding::new(keys), |binding: &ShortcutKeyBinding| binding.get_bound_keys())
});

/// java: public static final Comparator<ShortcutKeyBinding> COMPARATOR = Comparator.comparing(ShortcutKeyBinding::getBoundKeysTranslationKey, Comparators.lexicographical(String::compareTo));
pub fn comparator(a: &ShortcutKeyBinding, b: &ShortcutKeyBinding) -> std::cmp::Ordering {
    let (a, b) = (a.get_bound_keys_translation_key(), b.get_bound_keys_translation_key());
    // Guava `Comparators.lexicographical`: element by element, then the shorter list first
    for (x, y) in a.iter().zip(b.iter()) {
        let c = compat::compare_to(x, y);
        if c != 0 {
            return c.cmp(&0);
        }
    }
    a.len().cmp(&b.len())
}

/// java: public class ShortcutKeyBinding implements Comparable<ShortcutKeyBinding>
///
/// A value (`@GenEquals` / `@GenHashCode` compare the bound keys), so it is the key of
/// `ShortcutsRecord.keyBindings` as it is in Java. Where Java SHARES one object between two holders
/// (`KeybindShortcutEntry.keyBinding` and its `KeybindWidget`), the holders share a `compat::Shared`.
#[derive(Clone, Debug)]
pub struct ShortcutKeyBinding {
    /// java: private final List<InputConstants.Key> boundKeys;
    pub bound_keys: Vec<Key>,
}

impl PartialEq for ShortcutKeyBinding {
    /// java: @GenEquals - the class's one field
    fn eq(&self, other: &ShortcutKeyBinding) -> bool {
        self.bound_keys == other.bound_keys
    }
}

impl Eq for ShortcutKeyBinding {}

impl std::hash::Hash for ShortcutKeyBinding {
    /// java: @GenHashCode - the class's one field
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bound_keys.hash(state);
    }
}

impl PartialOrd for ShortcutKeyBinding {
    fn partial_cmp(&self, other: &ShortcutKeyBinding) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortcutKeyBinding {
    /// java: @Override public int compareTo(ShortcutKeyBinding shortcutKeyBinding)
    fn cmp(&self, other: &ShortcutKeyBinding) -> std::cmp::Ordering {
        comparator(self, other)
    }
}

impl ShortcutKeyBinding {
    /// java: ShortcutKeyBinding(List<InputConstants.Key> boundKeys)
    pub fn new(bound_keys: Vec<Key>) -> ShortcutKeyBinding {
        // java: this.boundKeys = new ArrayList<>(boundKeys);
        ShortcutKeyBinding { bound_keys }
    }

    /// java: ShortcutKeyBinding copy()
    pub fn copy(&self) -> ShortcutKeyBinding {
        return ShortcutKeyBinding::new(self.bound_keys.clone());
    }

    /// java: boolean isUnbound()
    pub fn is_unbound(&self) -> bool {
        return self.bound_keys.is_empty() || self.bound_keys == vec![input_constants::unknown()];
    }

    /// java: List<InputConstants.Key> getBoundKeys()
    pub fn get_bound_keys(&self) -> Vec<Key> {
        return self.bound_keys.clone();
    }

    /// java: void clearBoundKeys()
    pub fn clear_bound_keys(&mut self) {
        self.bound_keys.clear();
    }

    /// java: void addBoundKey(InputConstants.Key boundKey)
    pub fn add_bound_key(&mut self, bound_key: Key) {
        if !self.bound_keys.contains(&bound_key) {
            self.bound_keys.push(bound_key);
        }
    }

    /// java: Component getBoundKeysText()
    pub fn get_bound_keys_text(&self) -> MutableComponent {
        // java: append(key.getDisplayName()).append(" + ") for each key, then getSiblings().removeLast()
        let mut bound_keys_text: MutableComponent = component::empty();
        for (i, bound_key) in self.bound_keys.iter().enumerate() {
            if i > 0 {
                bound_keys_text = bound_keys_text.append_str(" + ");
            }
            bound_keys_text = bound_keys_text.append(compat::key_display_name(bound_key));
        }
        return bound_keys_text;
    }

    /// java: List<String> getBoundKeysTranslationKey()
    pub fn get_bound_keys_translation_key(&self) -> Vec<String> {
        return self.bound_keys.iter().map(|key| key.get_name().to_string()).collect();
    }

    /// java: @Override public int compareTo(ShortcutKeyBinding shortcutKeyBinding)
    pub fn compare_to(&self, shortcut_key_binding: &ShortcutKeyBinding) -> i32 {
        return compat::ord_int(comparator(self, shortcut_key_binding));
    }
}
