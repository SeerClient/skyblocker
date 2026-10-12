//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.PlayerTabOverlayAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.PlayerTabOverlay)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getOrdering -> net.minecraft.client.gui.components.PlayerTabOverlay.PLAYER_COMPARATOR
// TODO(fabric2seer): `Comparator<PlayerInfo>` - return `&obj.player_comparator` with its Rust type
// pub fn get_ordering(obj: &mcrust::net::minecraft::client::gui::components::player_tab_overlay::PlayerTabOverlay) -> &T { &obj.player_comparator }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
