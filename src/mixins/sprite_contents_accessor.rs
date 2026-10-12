//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.SpriteContentsAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.texture.SpriteContents)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getOriginalImage -> net.minecraft.client.renderer.texture.SpriteContents.originalImage
// TODO(fabric2seer): `NativeImage` - return `&obj.original_image` with its Rust type
// pub fn get_original_image(obj: &mcrust::net::minecraft::client::renderer::texture::sprite_contents::SpriteContents) -> &T { &obj.original_image }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
