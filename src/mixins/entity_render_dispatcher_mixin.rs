//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.EntityRenderDispatcherMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.EntityRenderDispatcher)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$shouldRender
///   java: @ModifyReturnValue(method = "shouldRender", at = @At("RETURN"))
/// target: net.minecraft.client.renderer.entity.EntityRenderDispatcher#shouldRender  [exists]
/// Rust: net/minecraft/client/renderer/entity/entity_render_dispatcher.rs:1162 fn should_render(&self, entity: &dyn crate::net::minecraft::world::entity::entity::Entity, should_render_at_distance: bool, frustum_is_visible: &dyn Fn(&crate::net::minecraft::world::phys::aabb::AABB) -> bool, level: Option<&crate::net::minecraft::client::multiplayer::client_level::ClientLevel>,)
fn skyblocker_should_render(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::renderer::entity::entity_render_dispatcher::EntityRenderDispatcher>()) else { return; };
    let original: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    // TODO(fabric2seer): unknown name `entity`
    //   java: if ((Utils.isInHub() || Utils.isInPark()) && SkyblockerConfigManager.get().slayers.wolfSlayer.hideSvenPupNametag && entity instanceof ArmorStand armorStand && armorStand.getCustomName() instanceof Component nameTag && nameTag.getString().contains("Sven Pup")) return false;
    // TODO(fabric2seer): unknown name `entity`
    //   java: return Utils.isInDungeons() && SkyblockerConfigManager.get().dungeons.hideSoulweaverSkulls && entity instanceof ArmorStand armorStand && entity.isInvisible() && armorStand.hasItemInSlot(EquipmentSlot.HEAD) ? !ItemUtils.getHeadTexture(armorStand.getItemBySlot(EquipmentSlot.HEAD)).equals(HeadTextures.SOULWEAVER_HAUNTED_SKULL) : original;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.entity.EntityRenderDispatcher#shouldRender", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_should_render)));
}
