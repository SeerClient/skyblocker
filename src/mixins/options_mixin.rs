//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.OptionsMixin by fabric2seer.
//! @Mixin(net.minecraft.client.Options)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Unique
fn skyblocker_update(this: &mut mcrust::net::minecraft::client::options::Options, key: &str, nbt: &mut mcrust::net::minecraft::nbt::compound_tag::CompoundTag) {
    // TODO(fabric2seer): `.get(..)` on a CompoundTag
    //   java: Tag element = nbt.get("key_key." + key);
    let mut element: mcrust::net::minecraft::nbt::tag::Tag = todo!("fabric2seer: the initialiser of `element` (TODO above)");
    // TODO(fabric2seer): the condition: `.contains(..)` on a CompoundTag
    //   java: element != null && !nbt.contains("key_key.skyblocker." + key)
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `.put(..)` on a CompoundTag
        //   java: nbt.put("key_key.skyblocker." + key, element);
    }
}

/// @Unique
fn skyblocker_migrate(this: &mut mcrust::net::minecraft::client::options::Options, old_key: &str, new_key: &str, nbt: &mut mcrust::net::minecraft::nbt::compound_tag::CompoundTag) {
    let mut new_entry: String = format!("key_key.skyblocker.{}", new_key);
    // TODO(fabric2seer): the condition: `.contains(..)` on a CompoundTag
    //   java: nbt.contains(..)
    if todo!("fabric2seer: the condition (TODO above)") {
        return;
    }
    // TODO(fabric2seer): `.get(..)` on a CompoundTag
    //   java: Tag element = nbt.get("key_key.skyblocker." + oldKey);
    let mut element: mcrust::net::minecraft::nbt::tag::Tag = todo!("fabric2seer: the initialiser of `element` (TODO above)");
    if true /* java: element != null: never null here */ {
        // TODO(fabric2seer): `.put(..)` on a CompoundTag
        //   java: nbt.put(newEntry, element);
    }
}

/// @Unique
fn migrate_to_loadout_keybinds(this: &mut mcrust::net::minecraft::client::options::Options, nbt: &mut mcrust::net::minecraft::nbt::compound_tag::CompoundTag) {
    let mut used_loadout_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    for i in 1..=9 {
        let mut number_string: String = (if i < 10 { format!("0{}", i.to_string()) } else { i.to_string() });
        let mut loadout_key: String = format!("key_key.skyblocker.loadout.{}", number_string);
        let mut hotbar_key: String = format!("key_key.hotbar.{}", i);
        // TODO(fabric2seer): the condition: `.contains(..)` on a CompoundTag
        //   java: !nbt.contains(loadoutKey) && nbt.contains(hotbarKey)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.put(..)` on a CompoundTag
            //   java: nbt.put(loadoutKey, nbt.get(hotbarKey));
            // TODO(fabric2seer): `.get(..)` on a CompoundTag
            //   java: usedLoadoutKeys.add(nbt.get(hotbarKey).asString().get());
        }
    }
    let mut new_loadout_keys: std::collections::HashMap<String, String> = std::collections::HashMap::from([("key_key.skyblocker.loadout.10".to_string(), "key.keyboard.0".to_string()), ("key_key.skyblocker.loadout.11".to_string(), "key.keyboard.minus".to_string()), ("key_key.skyblocker.loadout.12".to_string(), "key.keyboard.equal".to_string())]);
    for entry in new_loadout_keys.iter().map(|(k, v)| (k.clone(), v.clone())) {
        // TODO(fabric2seer): the condition: `.contains(..)` on a CompoundTag
        //   java: !nbt.contains(entry.getKey()) && usedLoadoutKeys.contains(entry.getValue())
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.putString(..)` on a CompoundTag
            //   java: nbt.putString(entry.getKey(), "key.keyboard.unknown");
        }
    }
}

/// @ModifyReturnValue updateSkyblockerKeybinds
///   java: @ModifyReturnValue(method = "dataFix", at = @At("TAIL"))
/// target: net.minecraft.client.Options#dataFix  [no Rust counterpart]
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
fn update_skyblocker_keybinds(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): read the return value typed: ci.return_value::<T>() (CompoundTag)
    skyblocker_update("wikiLookup.fandom", nbt);
    skyblocker_migrate("wikiLookup.fandom", "wikiLookup.independent", nbt);
    skyblocker_update("hotbarSlotLock", nbt);
    skyblocker_update("itemPriceLookup", nbt);
    skyblocker_update("itemProtection", nbt);
    migrate_to_loadout_keybinds(nbt);
    ci.set_return_value(nbt.clone());
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
    // report(hook::register_injector(`net.minecraft.client.Options#dataFix`, InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(update_skyblocker_keybinds)));
}
