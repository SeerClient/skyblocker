//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.accessors.PlayerTabOverlayAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.PlayerTabOverlay)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getOrdering -> net.minecraft.client.gui.components.PlayerTabOverlay.PLAYER_COMPARATOR
/// (the port's `PLAYER_COMPARATOR` compares the four keys it reads off each `PlayerInfo`)
pub fn get_ordering() -> fn(&mcrust::net::minecraft::client::gui::components::player_tab_overlay::PlayerSortKey, &mcrust::net::minecraft::client::gui::components::player_tab_overlay::PlayerSortKey) -> std::cmp::Ordering {
    mcrust::net::minecraft::client::gui::components::player_tab_overlay::compare_players
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
