//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.SkinTextureDownloaderMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.texture.SkinTextureDownloader)

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): state `STRIP_DE_FACTO_TRANSPARENT_PIXELS` of type `Set<String>` - give it a Rust type (`static STRIP_DE_FACTO_TRANSPARENT_PIXELS: LazyLock<Mutex<T>>`)
pub static BRIGHTNESS_THRESHOLD: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(0.1f32));

/// @Unique
fn strip_de_facto_transparent_pixels(this: &mut mcrust::net::minecraft::client::renderer::texture::skin_texture_downloader::SkinTextureDownloader, image: &mut mcrust::com::mojang::blaze3d::platform::native_image::NativeImage) {
    // TODO(fabric2seer): `.getHeight(..)` on a NativeImage
    //   java: int height = image.getHeight();
    let mut height: i32 = todo!("fabric2seer: the initialiser of `height` (TODO above)");
    // TODO(fabric2seer): `.getWidth(..)` on a NativeImage
    //   java: int width = image.getWidth();
    let mut width: i32 = todo!("fabric2seer: the initialiser of `width` (TODO above)");
    for x in 0..width {
        for y in 0..height {
            // TODO(fabric2seer): `.getPixel(..)` on a NativeImage
            //   java: int color = image.getPixel(x, y);
            let mut color: i32 = todo!("fabric2seer: the initialiser of `color` (TODO above)");
            // TODO(fabric2seer): static `Color.RGBtoHSB` has no Rust counterpart in the port
            // rust: let mut hsb: Vec<f32> = Color::rg_bto_hsb((color.wrapping_shr((16) as u32) & 0xFF), (color.wrapping_shr((8) as u32) & 0xFF), (color & 0xFF), None);
            let mut hsb: Vec<f32> = todo!("fabric2seer: the initialiser of `hsb` (TODO above)");
            if hsb[(2) as usize] <= compat::get_state(&BRIGHTNESS_THRESHOLD) {
                // TODO(fabric2seer): `.setPixel(..)` on a NativeImage
                //   java: image.setPixel(x, y, ARGB.color(0x00, color & 0x00FFFFFF));
            }
        }
    }
}

/// @Inject skyblocker$determineSkinSource
///   java: @Inject(method = "processLegacySkin", at = @At("HEAD"))
/// target: net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin  [no Rust counterpart]
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
fn skyblocker_determine_skin_source(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.dont_strip_skin_alpha_values && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen>>() || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer_screen::ProfileViewerScreen>>() {
        // TODO(fabric2seer): `uri`: this argument is not visible to the injector (see the notes above the handler)
        //   java: String skinTextureHash = PlayerHeadHashCache.getSkinHashFromUrl(uri);
        let mut skin_texture_hash: String = todo!("fabric2seer: the initialiser of `skinTextureHash` (TODO above)");
        let mut skin_hash: i32 = compat::string_hash_code(&skin_texture_hash);
        // TODO(fabric2seer): `.set(..)` on a LocalBooleanRef
        //   java: isSkyblockSkinTexture.set(PlayerHeadHashCache.contains(skinHash));
        if compat::get_state(&STRIP_DE_FACTO_TRANSPARENT_PIXELS).contains(&skin_texture_hash) {
            // TODO(fabric2seer): `image`: this argument is not visible to the injector (see the notes above the handler)
            //   java: stripDeFactoTransparentPixels(image);
        }
    }
}

/// @WrapWithCondition skyblocker$dontStripAlphaValues
///   java: @WrapWithCondition(method = "processLegacySkin", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/renderer/texture/SkinTextureDownloader;setNoAlpha(Lcom/mojang/blaze3d/platform/NativeImage;IIII)V"))
/// target: net.minecraft.client.renderer.texture.SkinTextureDownloader#setNoAlpha  [exists]
/// within: net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin
/// Rust: net/minecraft/client/renderer/texture/skin_texture_downloader.rs:256 fn set_no_alpha(image: &mut NativeImage, x0: i32, y0: i32, x1: i32, y1: i32)
/// note: `within` net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn skyblocker_dont_strip_alpha_values(ci: &CallbackInfo<'_>) -> bool {
    let x1: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    let y1: i32 = ci.arg::<i32>(4).copied().unwrap_or_default();
    let x2: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
    let y2: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    let is_skyblock_skin_texture: i32 = ci.arg::<i32>(4).copied().unwrap_or_default();
    // TODO(fabric2seer): `.get(..)` on a LocalBooleanRef
    //   java: return !isSkyblockSkinTexture.get();
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
    // report(hook::register_injector(`net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin`, InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_determine_skin_source)));
    // TODO(fabric2seer): not registered - `within` net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
    // report(hook::call_site(`net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin`, `net.minecraft.client.renderer.texture.SkinTextureDownloader#setNoAlpha`).priority(1000).wrap_with_condition(skyblocker_dont_strip_alpha_values));
}
