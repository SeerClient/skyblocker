//! Ported from de.hysky.skyblocker.utils.render.texture.WidgetSpritesFallback by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use crate::de::hysky::skyblocker::utils::render::texture::identifier_texture::IdentifierTexture;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: class WidgetSpritesFallback implements FallbackedTexture<WidgetSprites>
#[derive(Clone)]
pub struct WidgetSpritesFallback {
    /// java: private final IdentifierTexture enabled;
    pub enabled: compat::Shared<IdentifierTexture>,
    /// java: private final IdentifierTexture disabled;
    pub disabled: compat::Shared<IdentifierTexture>,
    /// java: private final IdentifierTexture enabledFocused;
    pub enabled_focused: compat::Shared<IdentifierTexture>,
    /// java: private final IdentifierTexture disabledFocused;
    pub disabled_focused: compat::Shared<IdentifierTexture>,
    /// java: private WidgetSprites sprites;
    pub sprites: WidgetSprites,
}

impl WidgetSpritesFallback {
    /// java: WidgetSpritesFallback(IdentifierTexture enabled, IdentifierTexture disabled, IdentifierTexture enabledFocused, IdentifierTexture disabledFocused)
    pub fn new(mut enabled: compat::Shared<IdentifierTexture>, mut disabled: compat::Shared<IdentifierTexture>, mut enabled_focused: compat::Shared<IdentifierTexture>, mut disabled_focused: compat::Shared<IdentifierTexture>) -> Self {
        let __init_enabled = enabled.clone();
        let __init_disabled = disabled.clone();
        let __init_enabled_focused = enabled_focused.clone();
        let __init_disabled_focused = disabled_focused.clone();
        let __init_sprites = WidgetSprites::new(IdentifierTexture::get(&enabled), IdentifierTexture::get(&disabled), IdentifierTexture::get(&enabled_focused), IdentifierTexture::get(&disabled_focused));
        let mut this = Self {
            enabled: __init_enabled,
            disabled: __init_disabled,
            enabled_focused: __init_enabled_focused,
            disabled_focused: __init_disabled_focused,
            sprites: __init_sprites,
        };
        this
    }

    /// java: @Override public WidgetSprites get()
    pub fn get(&self) -> WidgetSprites {
        return self.sprites.clone();
    }

    /// java: void update()
    pub fn update(&mut self) {
        IdentifierTexture::update(&self.enabled);
        IdentifierTexture::update(&self.disabled);
        IdentifierTexture::update(&self.enabled_focused);
        IdentifierTexture::update(&self.disabled_focused);
        self.sprites = WidgetSprites::new(IdentifierTexture::get(&self.enabled), IdentifierTexture::get(&self.disabled), IdentifierTexture::get(&self.enabled_focused), IdentifierTexture::get(&self.disabled_focused));
    }

}

impl FallbackedTexture<mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites> for WidgetSpritesFallback {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isUsingFallback()
    fn is_using_fallback(&self) -> bool {
        return (self.enabled.lock().borrow().is_using_fallback() || self.disabled.lock().borrow().is_using_fallback() || self.enabled_focused.lock().borrow().is_using_fallback() || self.disabled_focused.lock().borrow().is_using_fallback());
    }

    /// java: @Override public WidgetSprites get() (the inherent `get` is the same body)
    fn get(&self) -> mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites {
        return self.sprites.clone();
    }

}

/// java: record Helper(WidgetSprites sprites, WidgetSprites fallback)
#[derive(Clone, Debug, PartialEq)]
pub struct Helper {
    /// java: WidgetSprites sprites
    pub sprites: WidgetSprites,
    /// java: WidgetSprites fallback
    pub fallback: WidgetSprites,
}

impl Helper {
    /// java: the canonical record constructor
    pub fn new(mut sprites: WidgetSprites, mut fallback: WidgetSprites) -> Self {
        Self { sprites, fallback }
    }

    /// java: the record accessor `sprites()`
    pub fn sprites(&self) -> WidgetSprites {
        self.sprites.clone()
    }

    /// java: the record accessor `fallback()`
    pub fn fallback(&self) -> WidgetSprites {
        self.fallback.clone()
    }

    /// java: void populate(List<IdentifierTexture> textures, Function<WidgetSprites, Identifier> identifierFunction)
    pub fn populate(&self, textures: &mut Vec<compat::Shared<IdentifierTexture>>, identifier_function: Arc<dyn Fn(WidgetSprites) -> Identifier>) {
        let mut identifier: Identifier = (identifier_function)(self.sprites.clone());
        let mut fallback_identifier: Identifier = (identifier_function)(self.fallback.clone());
        for i in 0..(textures.len() as i32) {
            let mut identifier_texture: compat::Shared<IdentifierTexture> = textures[(i) as usize].clone();
            if compat::read(&identifier_texture, |__t| __t.texture.clone()) == identifier && compat::read(&identifier_texture, |__t| __t.fallback.clone()) == fallback_identifier {
                textures.push(identifier_texture.clone());
                return;
            }
        }
        textures.push(IdentifierTexture::new(identifier.clone(), fallback_identifier.clone(), Some(mcrust::net::minecraft::data::atlas_ids::gui())));
    }

}
