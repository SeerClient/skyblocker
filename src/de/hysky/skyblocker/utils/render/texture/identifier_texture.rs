//! Ported from de.hysky.skyblocker.utils.render.texture.IdentifierTexture by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::texture::fallbacked_texture::FallbackedTexture;
use mcrust::net::minecraft::client::renderer::texture::texture_atlas::TextureAtlas;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::server::packs::resources::resource::Resource;

/// java: class IdentifierTexture implements FallbackedTexture<Identifier>
#[derive(Clone)]
pub struct IdentifierTexture {
    /// java: protected final Identifier texture;
    pub texture: Identifier,
    /// java: protected final Identifier fallback;
    pub fallback: Identifier,
    /// java: private final @Nullable Identifier atlas;
    pub atlas: Option<Identifier>,
    /// java: private Identifier currentTexture;
    pub current_texture: Identifier,
}

impl IdentifierTexture {
    /// java: protected IdentifierTexture(Identifier texture, Identifier fallback, @Nullable Identifier atlas)
    pub fn new(mut texture: Identifier, mut fallback: Identifier, mut atlas: Option<Identifier>) -> compat::Shared<Self> {
        let __init_texture = texture.clone();
        let __init_fallback = fallback.clone();
        let __init_atlas = atlas.clone();
        let __init_current_texture = texture.clone();
        let __this = compat::shared(Self {
            texture: __init_texture,
            fallback: __init_fallback,
            atlas: __init_atlas,
            current_texture: __init_current_texture,
        });
        if compat::minecraft().is_game_load_finished() {
            IdentifierTexture::update(&__this);
        }
        __this
    }

    /// java: @Override public Identifier get()
    pub fn get(__this: &compat::Shared<Self>) -> Identifier {
        return compat::read(__this, |__t| __t.current_texture.clone());
    }

    /// java: void update()
    pub fn update(__this: &compat::Shared<Self>) {
        let minecraft = compat::minecraft();
        if compat::read(__this, |__t| __t.atlas.clone()).is_some() {
            // TODO(fabric2seer): `Minecraft.getAtlasManager` has no Rust method in the port (looked for `get_atlas_manager` in Minecraft and its supertypes)
            // rust: let mut texture_atlas: TextureAtlas = minecraft.get_atlas_manager().get_atlas_or_throw(&compat::read(__this, |__t| __t.atlas.clone()).unwrap());
            let mut texture_atlas: TextureAtlas = todo!("fabric2seer: the initialiser of `textureAtlas` (TODO above)");
            // java: textureAtlas.missingSprite() == textureAtlas.getSprite(this.texture) - the same sprite object
            if std::ptr::eq(texture_atlas.missing_sprite(), texture_atlas.get_sprite(&compat::read(__this, |__t| __t.texture.clone()))) {
                { let __v = compat::read(__this, |__t| __t.fallback.clone()); compat::write(__this, |__t| __t.current_texture = __v); }
            } else {
                { let __v = compat::read(__this, |__t| __t.texture.clone()); compat::write(__this, |__t| __t.current_texture = __v); }
            }
        } else {
            let mut resource: Option<Resource> = minecraft.get_resource_manager().get_resource(&compat::read(__this, |__t| __t.texture.clone()));
            if resource.is_none() {
                { let __v = compat::read(__this, |__t| __t.fallback.clone()); compat::write(__this, |__t| __t.current_texture = __v); }
            } else {
                { let __v = compat::read(__this, |__t| __t.texture.clone()); compat::write(__this, |__t| __t.current_texture = __v); }
            }
        }
    }

}

impl FallbackedTexture<mcrust::net::minecraft::resources::identifier::Identifier> for IdentifierTexture {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isUsingFallback()
    fn is_using_fallback(&self) -> bool {
        // java: return currentTexture == fallback; - `currentTexture` is only ever assigned `texture` or `fallback`, so the
        // reference test is "was it assigned the fallback"; two separate fields never share an address here, so the
        // port compares the identifiers' values (differs from Java only when `texture` equals `fallback`)
        return self.current_texture == self.fallback;
    }

}
