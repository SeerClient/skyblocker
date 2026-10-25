//! Ported from net.azureaaron.renderchest.impl.RenderChest (Render Chest 1.0.3) - the library's client entrypoint.
//!
//! Render Chest is its own Fabric mod in Java (bundled by Skyblocker's `include`); Fabric applies its mixins with the
//! game's. Ported into Skyblocker's crate, its mixins are registered with Skyblocker's (`register_mixins`, called from
//! the pre-launch entrypoint as hm-api's are).

use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public static final String MOD_ID = "render-chest";
pub const MOD_ID: &str = "render-chest";

/// java: public final class RenderChest implements ClientModInitializer
pub struct RenderChest;

impl RenderChest {
    /// java: @Override public void onInitializeClient() {}
    pub fn on_initialize_client() {}

    /// java: public static Identifier id(String path)
    pub fn id(path: &str) -> Identifier {
        Identifier::from_namespace_and_path(MOD_ID, path)
    }

    /// render-chest.mixins.json's `client` list, as far as it is ported (see `mixins`).
    pub fn register_mixins() {
        crate::net::azureaaron::renderchest::mixins::entity_renderer_mixin::register();
    }
}
