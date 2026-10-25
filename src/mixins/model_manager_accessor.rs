//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.ModelManagerAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.resources.model.ModelManager)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getBakedItemStackModels -> net.minecraft.client.resources.model.ModelManager.bakedItemStackModels
/// The model manager's item half is the published bake's `ItemModelSet` (model_manager.rs); `None` before the first bake.
pub fn get_baked_item_stack_models() -> Option<
    std::collections::BTreeMap<
        mcrust::net::minecraft::resources::identifier::Identifier,
        Arc<dyn mcrust::net::minecraft::client::renderer::item::item_model::ItemModel>,
    >,
> {
    mcrust::net::minecraft::client::renderer::game_renderer::baked().map(|models| models.item_models.baked_item_stack_models.clone())
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
