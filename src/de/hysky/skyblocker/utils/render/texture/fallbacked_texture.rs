//! Ported from de.hysky.skyblocker.utils.render.texture.FallbackedTexture by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::texture::identifier_texture::IdentifierTexture;
use crate::de::hysky::skyblocker::utils::render::texture::widget_sprites_fallback::Helper;
use crate::de::hysky::skyblocker::utils::render::texture::widget_sprites_fallback::WidgetSpritesFallback;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: List<Runnable> UPDATE_CALLBACKS = new ArrayList<>();
pub static UPDATE_CALLBACKS: compat::MainThread<Vec<Arc<dyn Fn()>>> = compat::MainThread::new(|| Vec::new());

/// java: public interface FallbackedTexture<T> extends Supplier<T>
pub trait FallbackedTexture<T> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: boolean isUsingFallback();
    fn is_using_fallback(&self) -> bool;

}

/// java: static FallbackedTexture<Identifier> of(Identifier id, Identifier fallback, @Nullable Identifier atlas)
pub fn of(id: &Identifier, fallback: &Identifier, mut atlas: Option<Identifier>) -> compat::Shared<dyn FallbackedTexture<Identifier>> {
    let mut identifier_texture: compat::Shared<IdentifierTexture> = IdentifierTexture::new(id.clone(), fallback.clone(), atlas.clone());
    // TODO(fabric2seer): a lambda / method reference where this call does not take one
    //   java: UPDATE_CALLBACKS.add(identifierTexture::update);
    return (identifier_texture as compat::Shared<dyn FallbackedTexture<compat::Object>>);
}

/// java: static FallbackedTexture<Identifier> ofGuiSprite(Identifier id, Identifier fallback)
pub fn of_gui_sprite(id: &Identifier, fallback: &Identifier) -> compat::Shared<dyn FallbackedTexture<Identifier>> {
    return of(id, fallback, Some(mcrust::net::minecraft::data::atlas_ids::gui()));
}

/// java: static FallbackedTexture<Identifier> ofTexture(Identifier id, Identifier fallback)
pub fn of_texture(id: &Identifier, fallback: &Identifier) -> compat::Shared<dyn FallbackedTexture<Identifier>> {
    return of(id, fallback, None);
}

/// java: static FallbackedTexture<WidgetSprites> ofWidgetSprites(WidgetSprites sprites, WidgetSprites fallback)
pub fn of_widget_sprites(sprites: &WidgetSprites, fallback: &WidgetSprites) -> compat::Shared<dyn FallbackedTexture<WidgetSprites>> {
    let mut textures: Vec<compat::Shared<IdentifierTexture>> = Vec::with_capacity((4) as usize);
    let mut helper: Helper = Helper::new(sprites.clone(), fallback.clone());
    // TODO(fabric2seer): `WidgetSprites.enabled` has no Rust method in the port (looked for `enabled` in WidgetSprites and JavaSrc)
    // rust: helper.populate(&mut textures, Arc::new(move |__a0| __a0.enabled()));
    // TODO(fabric2seer): `WidgetSprites.disabled` has no Rust method in the port (looked for `disabled` in WidgetSprites and JavaSrc)
    // rust: helper.populate(&mut textures, Arc::new(move |__a0| __a0.disabled()));
    // TODO(fabric2seer): `WidgetSprites.enabledFocused` has no Rust method in the port (looked for `enabled_focused` in WidgetSprites and JavaSrc)
    // rust: helper.populate(&mut textures, Arc::new(move |__a0| __a0.enabled_focused()));
    // TODO(fabric2seer): `WidgetSprites.disabledFocused` has no Rust method in the port (looked for `disabled_focused` in WidgetSprites and JavaSrc)
    // rust: helper.populate(&mut textures, Arc::new(move |__a0| __a0.disabled_focused()));
    let mut sprites_fallback: WidgetSpritesFallback = WidgetSpritesFallback::new(textures[(0) as usize].clone(), textures[(1) as usize].clone(), textures[(2) as usize].clone(), textures[(3) as usize].clone());
    // TODO(fabric2seer): a lambda / method reference where this call does not take one
    //   java: UPDATE_CALLBACKS.add(spritesFallback::update);
    return (compat::shared(sprites_fallback) as compat::Shared<dyn FallbackedTexture<compat::Object>>);
}

/// java: @Init static void init()
pub fn init() {
    let mut id: Identifier = crate::entry::skyblocker_mod::id("fallback_texture_reloader");
    // TODO(fabric2seer): Fabric API `ResourceLoader.get` has no SeerClient equivalent
    //   java: ResourceLoader.get(PackType.CLIENT_RESOURCES).registerReloadListener(id,
    //   java: 				(_, executor, preparationBarrier, reloadExecutor) -> CompletableFuture
    //   java: 						.supplyAsync(() -> Unit.INSTANCE, executor)
    //   java: 						.thenCompose(preparationBarrier::wait)
    //   java: 						.thenAcceptAsync(_ -> UPDATE_CALLBACKS.forEach(Runnable::run), reloadExecutor));
    // TODO(fabric2seer): Fabric API `ResourceLoader.get` has no SeerClient equivalent
    //   java: ResourceLoader.get(PackType.CLIENT_RESOURCES).addListenerOrdering(ResourceReloaderKeys.AFTER_VANILLA, id);
}
