//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.DataComponentHolderMixin by fabric2seer.
//! @Mixin(net.minecraft.core.component.DataComponentHolder)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$customComponents
///   java: @SuppressWarnings("unchecked")
///   java: @ModifyReturnValue(method = "get", at = @At("RETURN"))
/// target: net.minecraft.core.component.DataComponentHolder#get  [exists]
/// Rust: net/minecraft/core/component/data_component_holder.rs:51 fn get_from_components(&self, ty: &Arc<DataComponentType>)
///
/// rust: port gap - `ItemStack`'s `get` is its own `DataComponentGetter::get` body (item_stack.rs:2627), which
/// neither is hooked nor reaches the hooked `DataComponentHolder::get_from_components` (GAPS_batchD), so for the
/// one class this handler acts on (`this instanceof ItemStack`) it does not fire.
fn skyblocker_custom_components(ci: &mut CallbackInfo<'_>) {
    use mcrust::com::mojang::serialization::Obj;
    use mcrust::net::minecraft::core::component::data_components;
    // `(Object) this instanceof ItemStack stack`: the trait method's `this` is the implementing class.
    let Some(stack) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::item::item_stack::ItemStack>()) else { return; };
    let Some(data_component_type) = compat::detach_ref(ci.arg::<std::sync::Arc<mcrust::net::minecraft::core::component::data_component_type::DataComponentType>>(0)) else { return; };
    let Some(original) = ci.take_return_value::<Option<Obj>>() else { return; };
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let mut item_uuid: String = stack.get_uuid();
        if std::sync::Arc::ptr_eq(data_component_type, data_components::trim()) {
            let trim_key = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_armor_trims.get(&item_uuid).cloned();
            if let Some(trim_key) = trim_key {
                // `TRIMS_CACHE.getOrDefault(trimKey, (ArmorTrim) original)`
                let trim: Option<Obj> = match crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_trims::TRIMS_CACHE.get().get(&trim_key) {
                    Some(trim) => Some(std::sync::Arc::new(trim.clone()) as Obj),
                    None => original,
                };
                ci.set_return_value(trim);
                return;
            }
        } else if std::sync::Arc::ptr_eq(data_component_type, data_components::profile()) && stack.is_item(&mcrust::net::minecraft::world::item::items::items().get("player_head")) {
            // Normal head textures
            let tex = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_helmet_textures.get(&item_uuid).cloned();
            if let Some(tex) = tex {
                ci.set_return_value(Some(std::sync::Arc::new(crate::de::hysky::skyblocker::skyblock::item::custom::custom_helmet_textures::CustomHelmetTextures::get_profile(tex)) as Obj));
                return;
            }
            // Animated heads
            let animated_tex_id = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_animated_helmet_textures.get(&item_uuid).cloned();
            let frame = match &animated_tex_id {
                Some(animated_tex_id) => crate::de::hysky::skyblocker::skyblock::item::custom::custom_animated_helmet_textures::CustomAnimatedHelmetTextures::animate_head_texture(animated_tex_id),
                None => None,
            };
            if let Some(frame) = frame {
                ci.set_return_value(Some(std::sync::Arc::new(frame) as Obj));
                return;
            }
        } else if std::sync::Arc::ptr_eq(data_component_type, data_components::enchantment_glint_override()) {
            let glint: Option<bool> = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_glint.get(&item_uuid).copied(); // sorry fastutil :(
            if let Some(glint) = glint {
                ci.set_return_value(Some(std::sync::Arc::new(glint) as Obj));
                return;
            }
        } else if std::sync::Arc::ptr_eq(data_component_type, data_components::item_model()) {
            let id = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_item_model.get(&item_uuid).cloned();
            if let Some(id) = id {
                ci.set_return_value(Some(std::sync::Arc::new(id) as Obj));
                return;
            }
        }
    }
    ci.set_return_value(original);
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.core.component.DataComponentHolder#get", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_custom_components)));
}
