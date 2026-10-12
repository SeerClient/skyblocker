//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.OptionsMixin by fabric2seer.
//! @Mixin(net.minecraft.client.Options)

#[allow(unused_imports)]
use crate::prelude::*;

/// `CompoundTag` is the port's `Tag` (its compound arm carries `get` / `contains` / `put`).
type CompoundTag = mcrust::net::minecraft::nbt::tag::Tag;

/// @Unique
fn skyblocker_update(key: &str, nbt: &mut CompoundTag) {
    let element: Option<mcrust::net::minecraft::nbt::tag::Tag> = nbt.get(&format!("key_key.{}", key)).cloned();
    if let Some(element) = element && !nbt.contains(&format!("key_key.skyblocker.{}", key)) {
        nbt.put(&format!("key_key.skyblocker.{}", key), element);
    }
}

/// @Unique
fn skyblocker_migrate(old_key: &str, new_key: &str, nbt: &mut CompoundTag) {
    let mut new_entry: String = format!("key_key.skyblocker.{}", new_key);
    if nbt.contains(&new_entry) {
        return;
    }
    let element: Option<mcrust::net::minecraft::nbt::tag::Tag> = nbt.get(&format!("key_key.skyblocker.{}", old_key)).cloned();
    if let Some(element) = element {
        nbt.put(&new_entry, element);
    }
}

/// @Unique
fn migrate_to_loadout_keybinds(nbt: &mut CompoundTag) {
    let mut used_loadout_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    // Copy hotbar keys to loadout keys
    for i in 1..=9 {
        let mut number_string: String = (if i < 10 { format!("0{}", i.to_string()) } else { i.to_string() });
        let mut loadout_key: String = format!("key_key.skyblocker.loadout.{}", number_string);
        let mut hotbar_key: String = format!("key_key.hotbar.{}", i);
        if !nbt.contains(&loadout_key) && nbt.contains(&hotbar_key) {
            let hotbar_value: mcrust::net::minecraft::nbt::tag::Tag = nbt.get(&hotbar_key).cloned().expect("contains(hotbarKey)");
            nbt.put(&loadout_key, hotbar_value);
            // java: nbt.get(hotbarKey).asString().get() - `Optional.get()` throws on a non-string tag.
            used_loadout_keys.insert(nbt.get(&hotbar_key).and_then(|tag| tag.as_string()).map(str::to_string).expect("NoSuchElementException: No value present"));
        }
    }
    // Reset keys for loadouts 10, 11, and 12 if they conflict with a hotbar key
    let mut new_loadout_keys: std::collections::HashMap<String, String> = std::collections::HashMap::from([("key_key.skyblocker.loadout.10".to_string(), "key.keyboard.0".to_string()), ("key_key.skyblocker.loadout.11".to_string(), "key.keyboard.minus".to_string()), ("key_key.skyblocker.loadout.12".to_string(), "key.keyboard.equal".to_string())]);
    for entry in new_loadout_keys.iter().map(|(k, v)| (k.clone(), v.clone())) {
        // If the options does not have this loadout key and it is already taken then reset it
        if !nbt.contains(&entry.0) && used_loadout_keys.contains(&entry.1) {
            nbt.put_string(&entry.0, "key.keyboard.unknown");
        }
    }
}

/// @ModifyReturnValue updateSkyblockerKeybinds
///   java: @ModifyReturnValue(method = "dataFix", at = @At("TAIL"))
/// target: net.minecraft.client.Options#dataFix  [no Rust counterpart]
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
/// rust: port gap - `Options.dataFix` has no Rust body (GAPS_batchD); the handler is not registered.
fn update_skyblocker_keybinds(ci: &mut CallbackInfo<'_>) {
    let Some(mut nbt) = ci.take_return_value::<CompoundTag>() else { return; };
    skyblocker_update("wikiLookup.fandom", &mut nbt);
    skyblocker_migrate("wikiLookup.fandom", "wikiLookup.independent", &mut nbt);
    skyblocker_update("hotbarSlotLock", &mut nbt);
    skyblocker_update("itemPriceLookup", &mut nbt);
    skyblocker_update("itemProtection", &mut nbt);
    migrate_to_loadout_keybinds(&mut nbt);
    ci.set_return_value(nbt);
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
    // report(hook::register_injector(`net.minecraft.client.Options#dataFix`, InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(update_skyblocker_keybinds)));
}
