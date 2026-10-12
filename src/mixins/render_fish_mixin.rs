//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.RenderFishMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.FishingHookRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$render
///   java: @ModifyReturnValue(method = "shouldRender", at = @At("RETURN"))
/// target: net.minecraft.client.renderer.entity.FishingHookRenderer#shouldRender  [exists]
/// Rust: net/minecraft/client/renderer/entity/fishing_hook_renderer.rs:438 fn should_render(&self, should_render_at_distance: bool, entity: &dyn crate::net::minecraft::world::entity::entity::Entity, frustum_is_visible: &dyn Fn( &crate::net::minecraft::world::phys::aabb::AABB, ) -> bool, level: Option<&crate::net::minecraft::client::multiplayer::client_level::ClientLevel>,)
fn skyblocker_render(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::renderer::entity::fishing_hook_renderer::FishingHookRenderer>()) else { return; };
    let original: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    // TODO(fabric2seer): unknown name `fishingBobberEntity`
    //   java: return Utils.isOnSkyblock() && SkyblockerConfigManager.get().helpers.fishing.hideOtherPlayersRods
    //   java: 				? original && Objects.equals(Minecraft.getInstance().player, fishingBobberEntity.getPlayerOwner())
    //   java: 				: original;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.entity.FishingHookRenderer#shouldRender", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_render)));
}
