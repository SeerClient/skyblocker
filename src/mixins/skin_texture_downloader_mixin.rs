//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.SkinTextureDownloaderMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.texture.SkinTextureDownloader)

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Unique private static final Set<String> STRIP_DE_FACTO_TRANSPARENT_PIXELS = Set.of(..)
pub static STRIP_DE_FACTO_TRANSPARENT_PIXELS: LazyLock<std::collections::HashSet<String>> = LazyLock::new(|| {
    use crate::de::hysky::skyblocker::skyblock::item::head_textures;
    use crate::de::hysky::skyblocker::skyblock::item::player_head_hash_cache::PlayerHeadHashCache;
    std::collections::HashSet::from([
        PlayerHeadHashCache::get_skin_hash_from_base64(head_textures::TITANIUM_NECKLACE),
        PlayerHeadHashCache::get_skin_hash_from_base64(head_textures::TITANIUM_CLOAK),
        PlayerHeadHashCache::get_skin_hash_from_base64(head_textures::TITANIUM_BELT),
    ])
});
pub static BRIGHTNESS_THRESHOLD: LazyLock<Mutex<f32>> = LazyLock::new(|| Mutex::new(0.1f32));
/// `@Share("isSkyblockSkinTexture") LocalBooleanRef` - one value per run of `processLegacySkin`, shared by the two
/// injectors.
static IS_SKYBLOCK_SKIN_TEXTURE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// @Unique
fn strip_de_facto_transparent_pixels(image: &mut mcrust::com::mojang::blaze3d::platform::native_image::NativeImage) {
    let mut height: i32 = image.get_height();
    let mut width: i32 = image.get_width();
    for x in 0..width {
        for y in 0..height {
            let mut color: i32 = image.get_pixel(x, y);
            let hsb: [f32; 3] = mcrust::java::awt::color::Color::rgb_to_hsb((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF, None);
            //Work around "fake" transparent pixels - Thanks Hypixel I totally appreciate this!
            if hsb[2] <= compat::get_state(&BRIGHTNESS_THRESHOLD) {
                image.set_pixel(x, y, mcrust::net::minecraft::util::argb::color_alpha_rgb(0x00, color & 0x00FFFFFF));
            }
        }
    }
}

/// @Inject skyblocker$determineSkinSource
///   java: @Inject(method = "processLegacySkin", at = @At("HEAD"))
/// target: net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin  [no Rust counterpart]
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
/// rust: port gap - `SkinTextureDownloader.processLegacySkin` has no Rust body (GAPS_batchD); the handler is the
/// Java's over its two arguments (`image`, `uri`) and is not registered.
fn skyblocker_determine_skin_source(image: &mut mcrust::com::mojang::blaze3d::platform::native_image::NativeImage, uri: &str) {
    IS_SKYBLOCK_SKIN_TEXTURE.store(false, std::sync::atomic::Ordering::Relaxed);
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.dont_strip_skin_alpha_values && (crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::profileviewer::profile_viewer_screen::ProfileViewerScreen>>() || mcrust::mcrust::modding::game::current_screen_is::<compat::ScreenAdapter<crate::de::hysky::skyblocker::skyblock::profileviewer2::profile_viewer_screen::ProfileViewerScreen>>()) {
        let mut skin_texture_hash: String = crate::de::hysky::skyblocker::skyblock::item::player_head_hash_cache::PlayerHeadHashCache::get_skin_hash_from_url(uri);
        let mut skin_hash: i32 = compat::string_hash_code(&skin_texture_hash);
        IS_SKYBLOCK_SKIN_TEXTURE.store(crate::de::hysky::skyblocker::skyblock::item::player_head_hash_cache::PlayerHeadHashCache::contains(skin_hash), std::sync::atomic::Ordering::Relaxed);
        //Hypixel had the grand idea of using black pixels in place of actual transparent pixels on the titanium equipment so here we go!
        if STRIP_DE_FACTO_TRANSPARENT_PIXELS.contains(&skin_texture_hash) {
            strip_de_facto_transparent_pixels(image);
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
    return !IS_SKYBLOCK_SKIN_TEXTURE.load(std::sync::atomic::Ordering::Relaxed);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
    // report(hook::register_injector(`net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin`, InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_determine_skin_source)));
    // TODO(fabric2seer): not registered - `within` net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
    // report(hook::call_site(`net.minecraft.client.renderer.texture.SkinTextureDownloader#processLegacySkin`, `net.minecraft.client.renderer.texture.SkinTextureDownloader#setNoAlpha`).priority(1000).wrap_with_condition(skyblocker_dont_strip_alpha_values));
}
