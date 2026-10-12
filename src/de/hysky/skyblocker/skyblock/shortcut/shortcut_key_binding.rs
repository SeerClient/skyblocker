//! Ported from de.hysky.skyblocker.skyblock.shortcut.ShortcutKeyBinding by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::platform::input_constants::Key;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

// TODO(fabric2seer): `.split(..)` on a A
//   java: static final Codec<ShortcutKeyBinding> CODEC = Codec.STRING.xmap(
//   java: 			s -> Arrays.stream(s.split(" \\+ ")).map(InputConstants::getKey).toList(),
//   java: 			l -> l.stream().map(InputConstants.Key::getName).collect(Collectors.joining(" + "))
//   java: 	).xmap(ShortcutKeyBinding::new, ShortcutKeyBinding::getBoundKeys);
/// java: static final Codec<ShortcutKeyBinding> CODEC = Codec.STRING.xmap( s -> Arrays.stream(s.split(" \\+ ")).map(InputConstants::getKey).toList(), l -> l.stream().map(InputConstants.Key::getName).collect(Co
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<ShortcutKeyBinding>>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
// TODO(fabric2seer): static call Comparators.lexicographical(..)
//   java: public static final Comparator<ShortcutKeyBinding> COMPARATOR = Comparator.comparing(ShortcutKeyBinding::getBoundKeysTranslationKey, Comparators.lexicographical(String::compareTo));
/// java: public static final Comparator<ShortcutKeyBinding> COMPARATOR = Comparator.comparing(ShortcutKeyBinding::getBoundKeysTranslationKey, Comparators.lexicographical(String::compareTo));
pub static COMPARATOR: compat::MainThread<Arc<dyn Fn(&compat::Shared<ShortcutKeyBinding>, &compat::Shared<ShortcutKeyBinding>) -> std::cmp::Ordering>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `COMPARATOR` - see the TODO above"));

/// java: public class ShortcutKeyBinding implements Comparable<ShortcutKeyBinding>
#[derive(Clone)]
pub struct ShortcutKeyBinding {
    /// java: private final List<InputConstants.Key> boundKeys;
    pub bound_keys: Vec<Key>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl ShortcutKeyBinding {
    /// java: ShortcutKeyBinding(List<InputConstants.Key> boundKeys)
    pub fn new(mut bound_keys: Vec<Key>) -> compat::Shared<Self> {
        let __init_bound_keys = bound_keys.iter().cloned().collect::<Vec<_>>();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            bound_keys: __init_bound_keys,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: ShortcutKeyBinding copy()
    pub fn copy(__this: &compat::Shared<Self>) -> compat::Shared<ShortcutKeyBinding> {
        return ShortcutKeyBinding::new(compat::read(__this, |__t| __t.bound_keys.clone()));
    }

    /// java: boolean isUnbound()
    pub fn is_unbound(__this: &compat::Shared<Self>) -> bool {
        return (__this.lock().borrow().bound_keys.is_empty() || __this.lock().borrow().bound_keys == &vec![mcrust::com::mojang::blaze3d::platform::input_constants::unknown()]);
    }

    /// java: List<InputConstants.Key> getBoundKeys()
    pub fn get_bound_keys(__this: &compat::Shared<Self>) -> Vec<Key> {
        return compat::read(__this, |__t| __t.bound_keys.clone());
    }

    /// java: void clearBoundKeys()
    pub fn clear_bound_keys(__this: &compat::Shared<Self>) {
        __this.lock().borrow_mut().bound_keys.clear();
    }

    /// java: void addBoundKey(InputConstants.Key boundKey)
    pub fn add_bound_key(__this: &compat::Shared<Self>, mut bound_key: Key) {
        if !(__this.lock().borrow().bound_keys.iter().any(|e| *e == bound_key.clone())) {
            let __h0 = bound_key.clone();
            __this.lock().borrow_mut().bound_keys.push(__h0);
        }
    }

    /// java: Component getBoundKeysText()
    pub fn get_bound_keys_text(__this: &compat::Shared<Self>) -> MutableComponent {
        let mut bound_keys_text: MutableComponent = component::empty();
        for bound_key in compat::read(__this, |__t| __t.bound_keys.clone()) {
            // TODO(fabric2seer): `get_display_name` takes 2 argument(s) in the port (`key_name: &dyn Fn(i32, i32) -> Option<String>, has: &dyn Fn(&str) -> bool`), Java passes 0
            // rust: bound_keys_text.append(bound_key.get_display_name()).append_str(" + ");
        }
        if !(bound_keys_text.get_siblings().is_empty()) {
            bound_keys_text.get_siblings().pop().unwrap();
        }
        return bound_keys_text;
    }

    /// java: List<String> getBoundKeysTranslationKey()
    pub fn get_bound_keys_translation_key(__this: &compat::Shared<Self>) -> Vec<String> {
        // TODO(fabric2seer): `.getName(..)` on a value of unknown type
        //   java: return boundKeys.stream().map(InputConstants.Key::getName).toList();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override @GenEquals public native boolean equals(Object o);
    pub fn equals(&self, o: &compat::Object) -> bool {
        todo!("native method: its body is generated at build time by an annotation processor (@GenEquals)")
    }

    /// java: @Override @GenHashCode public native int hashCode();
    pub fn hash_code(&self) -> i32 {
        todo!("native method: its body is generated at build time by an annotation processor (@GenHashCode)")
    }

    /// java: @Override public int compareTo(ShortcutKeyBinding shortcutKeyBinding)
    pub fn compare_to(__this: &compat::Shared<Self>, shortcut_key_binding: &compat::Shared<ShortcutKeyBinding>) -> i32 {
        return (COMPARATOR.get_mut())(__this.clone(), shortcut_key_binding.clone());
    }

}
