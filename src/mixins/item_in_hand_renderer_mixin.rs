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
    // TODO(fabric2seer): unknown name `from`
    //   java: return Utils.isOnSkyblock() && from.getItem() == to.getItem() ? original || SkyblockerConfigManager.get().uiAndVisuals.cancelComponentUpdateAnimation : original;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.ItemInHandRenderer#shouldInstantlyReplaceVisibleItem", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_cancel_component_update_animation)));
}
