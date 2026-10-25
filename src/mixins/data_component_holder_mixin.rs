//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.DataComponentHolderMixin by fabric2seer.
//! @Mixin(net.minecraft.core.component.DataComponentHolder)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$customComponents
///   java: @SuppressWarnings("unchecked")
///   java: @ModifyReturnValue(method = "get", at = @At("RETURN"))
/// target: net.minecraft.core.component.DataComponentHolder#get  [exists]
/// Rust: net/minecraft/core/component/data_component_holder.rs fn get_from_components(&self, ty: &Arc<DataComponentType>),
/// and `ItemStack`'s inherited copy of it (item_stack.rs, `impl DataComponentGetter for ItemStack`'s `get`), both hooked
/// under this name - the handler acts on the `ItemStack` one (`this instanceof ItemStack`).
fn skyblocker_custom_components(ci: &mut CallbackInfo<'_>) {
    use mcrust::com::mojang::serialization::Obj;
    use mcrust::net::minecraft::core::component::data_components;
    // PERF (port-only, same result): off Skyblock every arm answers `original` (already the return value), so
    // leave on the one relaxed atomic load before the receiver/argument downcasts (~870 calls a frame).
    if !crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        return;
    }
    // `(Object) this instanceof ItemStack stack`: the trait method's `this` is the implementing class.
    let Some(stack) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let Some(data_component_type) = compat::detach_ref(ci.arg::<std::sync::Arc<mcrust::net::minecraft::core::component::data_component_type::DataComponentType>>(0)) else { return; };
    // PERF (port-only, same result): every arm below answers `original`
    // unless the type is one of these four and we are on Skyblock, so the
    // other ~99% of calls return before `getUuid()` (a CUSTOM_DATA read and
    // a String) and any config read. The value is left in place untouched,
    // which is what `set_return_value(original)` would leave.
    if !crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()
        || !(std::sync::Arc::ptr_eq(data_component_type, data_components::trim())
            || std::sync::Arc::ptr_eq(data_component_type, data_components::profile())
            || std::sync::Arc::ptr_eq(data_component_type, data_components::enchantment_glint_override())
            || std::sync::Arc::ptr_eq(data_component_type, data_components::item_model()))
    {
        return;
    }
    let Some(original) = ci.take_return_value::<Option<Obj>>() else { return; };
    // `getUuid()` below reads the stack's own CUSTOM_DATA through this same
    // `get`; a nested call answers the original instead of recursing.
    thread_local! { static INSIDE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    if INSIDE.with(|f| f.replace(true)) {
        ci.set_return_value(original);
        return;
    }
    struct Reset;
    impl Drop for Reset { fn drop(&mut self) { INSIDE.with(|f| f.set(false)); } }
    let _reset = Reset;
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        // PERF (port-only, same result): each arm below answers `original` unless its config map holds the
        // stack's uuid, so with that map (or both, for PROFILE) empty - the usual case - the uuid (an attachment
        // read and a String clone, per call) is never needed.
        let config = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get();
        let no_customisation: bool = if std::sync::Arc::ptr_eq(data_component_type, data_components::trim()) {
            config.general.custom_armor_trims.is_empty()
        } else if std::sync::Arc::ptr_eq(data_component_type, data_components::profile()) {
            config.general.custom_helmet_textures.is_empty() && config.general.custom_animated_helmet_textures.is_empty()
        } else if std::sync::Arc::ptr_eq(data_component_type, data_components::enchantment_glint_override()) {
            config.general.custom_glint.is_empty()
        } else {
            config.general.custom_item_model.is_empty()
        };
        if no_customisation {
            ci.set_return_value(original);
            return;
        }
        // PERF (port-only, same result): the uuid borrowed (`with_uuid`), not a `String` clone per call - it is only
        // looked up in the maps below. The arms' values are cloned out under the borrow and used after it.
        enum Found {
            Trim(crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_trims::ArmorTrimId),
            Head(Option<String>, Option<String>),
            Glint(bool),
            Model(mcrust::net::minecraft::resources::identifier::Identifier),
            Nothing,
        }
        let is_trim = std::sync::Arc::ptr_eq(data_component_type, data_components::trim());
        let is_profile = !is_trim && std::sync::Arc::ptr_eq(data_component_type, data_components::profile());
        let is_glint = !is_trim && !is_profile && std::sync::Arc::ptr_eq(data_component_type, data_components::enchantment_glint_override());
        // `stack.is(Items.PLAYER_HEAD)` before the uuid: the PROFILE arm's own `&&` order reads the uuid only for a head.
        let is_head = is_profile && stack.is_item(&mcrust::net::minecraft::world::item::items::items().get("player_head"));
        let found: Found = if is_profile && !is_head {
            Found::Nothing
        } else {
            crate::mixins::item_stack_mixin::with_uuid(stack, |item_uuid: &str| {
                let general = &config.general;
                if is_trim {
                    general.custom_armor_trims.get(item_uuid).cloned().map_or(Found::Nothing, Found::Trim)
                } else if is_head {
                    let tex = general.custom_helmet_textures.get(item_uuid).cloned();
                    // (the animated map is read only when the plain one has no entry, as below)
                    let animated = if tex.is_none() { general.custom_animated_helmet_textures.get(item_uuid).cloned() } else { None };
                    Found::Head(tex, animated)
                } else if is_glint {
                    general.custom_glint.get(item_uuid).copied().map_or(Found::Nothing, Found::Glint)
                } else {
                    general.custom_item_model.get(item_uuid).cloned().map_or(Found::Nothing, Found::Model)
                }
            })
        };
        match found {
            Found::Trim(trim_key) => {
                // `TRIMS_CACHE.getOrDefault(trimKey, (ArmorTrim) original)`
                let trim: Option<Obj> = match crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_trims::TRIMS_CACHE.get().get(&trim_key) {
                    Some(trim) => Some(std::sync::Arc::new(trim.clone()) as Obj),
                    None => original,
                };
                ci.set_return_value(trim);
                return;
            }
            Found::Head(Some(tex), _) => {
                ci.set_return_value(Some(std::sync::Arc::new(crate::de::hysky::skyblocker::skyblock::item::custom::custom_helmet_textures::CustomHelmetTextures::get_profile(tex)) as Obj));
                return;
            }
            Found::Head(None, Some(animated_tex_id)) => {
                if let Some(frame) = crate::de::hysky::skyblocker::skyblock::item::custom::custom_animated_helmet_textures::CustomAnimatedHelmetTextures::animate_head_texture(&animated_tex_id) {
                    ci.set_return_value(Some(std::sync::Arc::new(frame) as Obj));
                    return;
                }
            }
            Found::Glint(glint) => {
                ci.set_return_value(Some(std::sync::Arc::new(glint) as Obj));
                return;
            }
            Found::Model(id) => {
                ci.set_return_value(Some(std::sync::Arc::new(id) as Obj));
                return;
            }
            Found::Head(None, None) | Found::Nothing => {}
        }
        ci.set_return_value(original);
        return;
    }
    ci.set_return_value(original);
    return;
}

/// PERF (port-only, same result): the injector's pre-filter (`InjectOptions::when`). Off Skyblock every arm answers
/// `original`; on it, every arm still answers `original` while all of the customisation maps it consults are empty
/// (`no_customisation` above, for each of the four types). So the call needs the handler only when on Skyblock AND
/// at least one map holds an entry - otherwise `get` (thousands of calls a frame) runs as if unhooked.
fn custom_components_may_apply() -> bool {
    if !crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        return false;
    }
    // `SkyblockerConfigManager::get()` may build its view (a config clone) on first use; a `get` reached from inside
    // that is answered "may apply", i.e. handed to the handler exactly as before this filter existed.
    thread_local! { static IN_FILTER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    if IN_FILTER.with(|f| f.replace(true)) {
        return true;
    }
    struct Reset;
    impl Drop for Reset { fn drop(&mut self) { IN_FILTER.with(|f| f.set(false)); } }
    let _reset = Reset;
    let general = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general;
    !(general.custom_armor_trims.is_empty()
        && general.custom_helmet_textures.is_empty()
        && general.custom_animated_helmet_textures.is_empty()
        && general.custom_glint.is_empty()
        && general.custom_item_model.is_empty())
}

/// PERF (port-only, same result): the injector's ARGUMENT-AWARE pre-filter (`InjectOptions::when_args`). With a
/// customisation configured `custom_components_may_apply` passes on every call, yet the handler answers `original`
/// (already the return value) for every call that is not an `ItemStack`'s read of TRIM / PROFILE /
/// ENCHANTMENT_GLINT_OVERRIDE / ITEM_MODEL whose map is non-empty (its `no_customisation`). Rendering ~40 armoured
/// players reads EQUIPPABLE, DYED_COLOR, CUSTOM_DATA, ... ~1,100 times a frame; those now skip the handler and the
/// rest of `dispatch`. Reads only the receiver, the type argument and the config - none written by `get`.
fn custom_components_may_apply_to(ci: &CallbackInfo<'_>) -> bool {
    use mcrust::net::minecraft::core::component::data_components;
    if ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>().is_none() {
        return false;
    }
    let Some(ty) = ci.arg::<std::sync::Arc<mcrust::net::minecraft::core::component::data_component_type::DataComponentType>>(0) else {
        // Not readable: hand the call to the handler, as before this filter existed.
        return true;
    };
    let is_trim = std::sync::Arc::ptr_eq(ty, data_components::trim());
    let is_profile = !is_trim && std::sync::Arc::ptr_eq(ty, data_components::profile());
    let is_glint = !is_trim && !is_profile && std::sync::Arc::ptr_eq(ty, data_components::enchantment_glint_override());
    let is_model = !is_trim && !is_profile && !is_glint && std::sync::Arc::ptr_eq(ty, data_components::item_model());
    if !(is_trim || is_profile || is_glint || is_model) {
        return false;
    }
    // Perf round 1 (P01-hooks-core): the game skips every call whose type is not one of the injector's KEYS
    // (`custom_component_keys`) before this guard runs - so every type this guard accepts must be one of them.
    debug_assert!(
        custom_component_keys().contains(&(std::sync::Arc::as_ptr(ty) as usize)),
        "custom_components_may_apply_to accepts a component type custom_component_keys does not declare"
    );
    // As `custom_components_may_apply`: a `get` reached while the config view is being built is handed on.
    thread_local! { static IN_FILTER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    if IN_FILTER.with(|f| f.replace(true)) {
        return true;
    }
    struct Reset;
    impl Drop for Reset { fn drop(&mut self) { IN_FILTER.with(|f| f.set(false)); } }
    let _reset = Reset;
    let general = &crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general;
    if is_trim {
        !general.custom_armor_trims.is_empty()
    } else if is_profile {
        !(general.custom_helmet_textures.is_empty() && general.custom_animated_helmet_textures.is_empty())
    } else if is_glint {
        !general.custom_glint.is_empty()
    } else {
        !general.custom_item_model.is_empty()
    }
}

/// OPTIMIZED BUT NOT FULLY VANILLA (perf round 1, P01-hooks-core; port-only, same result): the injector's KEYS
/// (`InjectOptions::when_keys`) - the four component types `custom_components_may_apply_to` can accept, each as the
/// address `Arc::ptr_eq` compares (`Arc::as_ptr(type) as usize`, which is what the hooked `get`s pass as their
/// `key`). The handler answers `original` - a no-op - for every other type, so the game runs `get`'s body for those
/// without leaving it (`hook::key_selected`): no `enterN`, no guards, no `dispatch`. `when` and `when_args` still
/// decide among these four. Asked once per published injector set, on its first engaged call.
fn custom_component_keys() -> Vec<usize> {
    use mcrust::net::minecraft::core::component::data_components;
    vec![
        std::sync::Arc::as_ptr(data_components::trim()) as usize,
        std::sync::Arc::as_ptr(data_components::profile()) as usize,
        std::sync::Arc::as_ptr(data_components::enchantment_glint_override()) as usize,
        std::sync::Arc::as_ptr(data_components::item_model()) as usize,
    ]
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // Perf round 1 (P01-hooks-core): `.when_keys(custom_component_keys)` added; was the same chain without it.
    report(hook::register_injector("net.minecraft.core.component.DataComponentHolder#get", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000).when(custom_components_may_apply).when_args(custom_components_may_apply_to).when_keys(custom_component_keys), Arc::new(skyblocker_custom_components)));
}
