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
    // `FishingHook fishingBobberEntity` is `shouldRender`'s argument 1 (`entity`, a `&dyn Entity`).
    let Some(fishing_bobber_entity) = compat::hook_arg::<dyn mcrust::net::minecraft::world::entity::entity::Entity>(ci, 1) else { return; };
    let _ = this;
    let result = if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().helpers.fishing.hide_other_players_rods {
        // `Objects.equals(Minecraft.getInstance().player, fishingBobberEntity.getPlayerOwner())`
        let owner = mcrust::net::minecraft::world::entity::entity::Entity::as_any(fishing_bobber_entity)
            .and_then(|a| a.downcast_ref::<mcrust::net::minecraft::world::entity::projectile::fishing_hook::FishingHook>())
            .and_then(|hook| hook.get_player_owner());
        original && owner.is_some_and(mcrust::mcrust::modding::game::is_local_player)
    } else {
        original
    };
    ci.set_return_value(result);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.entity.FishingHookRenderer#shouldRender", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_render)));
}
