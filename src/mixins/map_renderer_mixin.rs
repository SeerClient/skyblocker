//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MapRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.MapRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyExpressionValue preventDecorationInDungeons
///   java: @ModifyExpressionValue(method = "extractDecorationRenderState", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/saveddata/maps/MapDecoration;renderOnFrame()Z"))
/// target: net.minecraft.world.level.saveddata.maps.MapDecoration#renderOnFrame  [exists]
/// within: net.minecraft.client.renderer.MapRenderer#extractDecorationRenderState
/// Rust: net/minecraft/world/level/saveddata/maps/map_decoration.rs:128 fn render_on_frame(&self)
fn prevent_decoration_in_dungeons(v: bool) -> bool {
    let mut always_rendered = v;
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_in_dungeons() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.dungeon_map.enable_map {
        // TODO(fabric2seer): `decoration`: this argument is not visible to the injector (see the notes above the handler)
        //   java: MapDecorationType decorationType = decoration.type().value();
        let mut decoration_type: mcrust::net::minecraft::world::level::saveddata::maps::map_decoration_type::MapDecorationType = todo!("fabric2seer: the initialiser of `decorationType` (TODO above)");
        let mut fancy_map: bool = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.dungeon_map.fancy_map;
        // TODO(fabric2seer): static `MapDecorationTypes.FRAME` has no Rust counterpart in the port's MapDecorationTypes file
        // rust: let mut should_show_self_marker: bool = (decoration_type == crate::unresolved::MapDecorationTypes::FRAME.value() && !fancy_map || !(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.dungeon_map.show_self_head));
        let mut should_show_self_marker: bool = todo!("fabric2seer: the initialiser of `shouldShowSelfMarker` (TODO above)");
        // TODO(fabric2seer): static `MapDecorationTypes.BLUE_MARKER` has no Rust counterpart in the port's MapDecorationTypes file
        // rust: let mut should_show_other_player_markers: bool = (decoration_type == crate::unresolved::MapDecorationTypes::BLUE_MARKER.value() && fancy_map);
        let mut should_show_other_player_markers: bool = todo!("fabric2seer: the initialiser of `shouldShowOtherPlayerMarkers` (TODO above)");
        return (should_show_self_marker || should_show_other_player_markers);
    }
    return always_rendered;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.renderer.MapRenderer#extractDecorationRenderState", "net.minecraft.world.level.saveddata.maps.MapDecoration#renderOnFrame").priority(1000).modify_expression_value::<bool>(prevent_decoration_in_dungeons));
}
