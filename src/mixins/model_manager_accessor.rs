//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.ModelManagerAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.resources.model.ModelManager)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getBakedItemStackModels -> net.minecraft.client.resources.model.ModelManager.bakedItemStackModels
// TODO(fabric2seer): `Map<Identifier, ItemModel>` - return `&obj.baked_item_stack_models` with its Rust type
// pub fn get_baked_item_stack_models(obj: &mcrust::net::minecraft::client::resources::model::model_manager::ModelManager) -> &T { &obj.baked_item_stack_models }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
