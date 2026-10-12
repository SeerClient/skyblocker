//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.KeyMappingMixin by fabric2seer.
//! @Mixin(net.minecraft.client.KeyMapping)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject onKeyPressed
///   java: @Inject(method = "click", at = @At("HEAD"))
/// target: net.minecraft.client.KeyMapping#click  [exists]
/// Rust: net/minecraft/client/key_mapping.rs:590 fn click(key: &Key)
fn on_key_pressed(ci: &mut CallbackInfo<'_>) {
    let Some(key) = compat::detach_ref(ci.arg::<Key>(0)) else { return; };
    crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts::Shortcuts::on_key_pressed(&key);
}

/// @Inject setKeyPressed
///   java: @Inject(method = "set", at = @At("HEAD"))
/// target: net.minecraft.client.KeyMapping#set  [exists]
/// Rust: net/minecraft/client/key_mapping.rs:549 fn set(key: &Key, state: bool)
fn set_key_pressed(ci: &mut CallbackInfo<'_>) {
    let Some(key) = compat::detach_ref(ci.arg::<Key>(0)) else { return; };
    let pressed: bool = ci.arg::<bool>(1).copied().unwrap_or_default();
    crate::de::hysky::skyblocker::skyblock::shortcut::shortcuts::Shortcuts::set_key_pressed(key.clone(), pressed);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.KeyMapping#click", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(on_key_pressed)));
    report(hook::register_injector("net.minecraft.client.KeyMapping#set", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(set_key_pressed)));
}
