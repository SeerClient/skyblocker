//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ItemInHandRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.ItemInHandRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$cancelComponentUpdateAnimation
///   java: @ModifyReturnValue(method = "shouldInstantlyReplaceVisibleItem", at = @At("RETURN"))
/// target: net.minecraft.client.renderer.ItemInHandRenderer#shouldInstantlyReplaceVisibleItem  [exists]
/// Rust: net/minecraft/client/renderer/item_in_hand_renderer.rs:1740 fn should_instantly_replace_visible_item(&self, resolver: &ItemModelResolver, currently_visible_item: &ItemStack, expected_item: &ItemStack)
fn skyblocker_cancel_component_update_animation(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::renderer::item_in_hand_renderer::ItemInHandRenderer>()) else { return; };
    let original: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    // `ItemStack from, ItemStack to` are `shouldInstantlyReplaceVisibleItem`'s arguments 1 and 2 (argument 0 is the
    // port's `ItemModelResolver`, Java's `this.itemModelResolver`).
    let _ = this;
    let Some(from) = ci.arg::<ItemStack>(1).cloned() else { return; };
    let Some(to) = ci.arg::<ItemStack>(2).cloned() else { return; };
    // java: return Utils.isOnSkyblock() && from.getItem() == to.getItem() ? original || ...cancelComponentUpdateAnimation : original;
    let result = if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && Arc::ptr_eq(&from.get_item(), &to.get_item()) {
        original || crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.cancel_component_update_animation
    } else {
        original
    };
    ci.set_return_value(result);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.ItemInHandRenderer#shouldInstantlyReplaceVisibleItem", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_cancel_component_update_animation)));
}
