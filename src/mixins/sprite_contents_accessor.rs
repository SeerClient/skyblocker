//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.SpriteContentsAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.texture.SpriteContents)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getOriginalImage -> net.minecraft.client.renderer.texture.SpriteContents.originalImage
pub fn get_original_image(
    obj: &mcrust::net::minecraft::client::renderer::texture::sprite_contents::SpriteContents,
) -> std::sync::RwLockReadGuard<'_, mcrust::com::mojang::blaze3d::platform::native_image::NativeImage> {
    obj.original_image()
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register: accessors are plain functions)
}
