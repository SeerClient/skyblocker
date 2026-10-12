//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.DyedItemColorMixin by fabric2seer.
//! @Mixin(net.minecraft.world.item.component.DyedItemColor)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$customDyeColor
///   java: @ModifyReturnValue(method = "getOrDefault", at = @At("RETURN"))
/// target: net.minecraft.world.item.component.DyedItemColor#getOrDefault  [exists]
/// Rust: net/minecraft/world/item/component/dyed_item_color.rs:60 fn get_or_default(color: Option<&DyedItemColor>, default_color: i32)
fn skyblocker_custom_dye_color(ci: &mut CallbackInfo<'_>) {
    let original_color: i32 = ci.return_value::<i32>().copied().unwrap_or_default();
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        // TODO(fabric2seer): unknown name `stack`
        //   java: String itemUuid = stack.getUuid();
        let mut item_uuid: String = todo!("fabric2seer: the initialiser of `itemUuid` (TODO above)");
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_animated_dyes.contains_key(&item_uuid) {
            ci.set_return_value(mcrust::net::minecraft::util::argb::opaque(crate::de::hysky::skyblocker::skyblock::item::custom::custom_armor_animated_dyes::CustomArmorAnimatedDyes::animate_color_transition(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_animated_dyes.get(&item_uuid).cloned().unwrap())));
            return;
        }
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_dye_colors.contains_key(&item_uuid) {
            ci.set_return_value(mcrust::net::minecraft::util::argb::opaque(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().general.custom_dye_colors.get(&item_uuid).copied().unwrap_or_default()));
            return;
        }
    }
    ci.set_return_value(original_color);
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.world.item.component.DyedItemColor#getOrDefault", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_custom_dye_color)));
}
