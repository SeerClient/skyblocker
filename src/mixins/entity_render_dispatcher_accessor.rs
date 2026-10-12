//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.EntityRenderDispatcherAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.EntityRenderDispatcher)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getEquipmentAssets -> net.minecraft.client.renderer.entity.EntityRenderDispatcher.equipmentAssets
// TODO(fabric2seer): `EquipmentAssetManager` - return `&obj.equipment_assets` with its Rust type
// pub fn get_equipment_assets(obj: &mcrust::net::minecraft::client::renderer::entity::entity_render_dispatcher::EntityRenderDispatcher) -> &T { &obj.equipment_assets }

/// @Accessor getBlockModelResolver -> net.minecraft.client.renderer.entity.EntityRenderDispatcher.blockModelResolver
// TODO(fabric2seer): `BlockModelResolver` - return `&obj.block_model_resolver` with its Rust type
// pub fn get_block_model_resolver(obj: &mcrust::net::minecraft::client::renderer::entity::entity_render_dispatcher::EntityRenderDispatcher) -> &T { &obj.block_model_resolver }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
