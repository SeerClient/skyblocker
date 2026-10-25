//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.EntityRenderDispatcherAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.entity.EntityRenderDispatcher)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getEquipmentAssets -> net.minecraft.client.renderer.entity.EntityRenderDispatcher.equipmentAssets
/// The port's dispatcher is built from (and does not keep) the `EquipmentAssetManager`; the game renderer holds the one
/// instance it was given (`GameRenderer.equipment_assets`). `None` before the first resource load.
pub fn get_equipment_assets(minecraft: &Minecraft) -> Option<Arc<mcrust::net::minecraft::client::resources::model::equipment_asset_manager::EquipmentAssetManager>> {
    minecraft.game_renderer.equipment_assets.clone()
}

/// @Accessor getBlockModelResolver -> net.minecraft.client.renderer.entity.EntityRenderDispatcher.blockModelResolver
/// `new BlockModelResolver(modelManager)` over the current bake's memoised `BlockModelSet` - the resolver the
/// dispatcher's renderers are built with (one `BlockModelSet` per bake, see game_renderer.rs). `None` before the first
/// bake.
pub fn get_block_model_resolver() -> Option<mcrust::net::minecraft::client::renderer::block::block_model_resolver::BlockModelResolver> {
    mcrust::net::minecraft::client::renderer::game_renderer::block_model_set()
        .map(mcrust::net::minecraft::client::renderer::block::block_model_resolver::BlockModelResolver::new)
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
