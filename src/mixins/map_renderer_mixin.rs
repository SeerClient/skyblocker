//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MapRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.MapRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// `MapDecorationTypes.X.value()`: the registered value behind the built-in key `minecraft:<name>`.
fn map_decoration_type_value(name: &str) -> Option<mcrust::net::minecraft::world::level::saveddata::maps::map_decoration_type::MapDecorationType> {
    use mcrust::net::minecraft::core::registry::Registry as _;
    let value = mcrust::net::minecraft::core::registries::built_in_registries::map_decoration_type().get_value_by_identifier(&mcrust::net::minecraft::resources::identifier::Identifier::with_default_namespace(name))?;
    mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::level::saveddata::maps::map_decoration_type::MapDecorationType>(&value).cloned()
}

/// @ModifyExpressionValue preventDecorationInDungeons
///   java: @ModifyExpressionValue(method = "extractDecorationRenderState", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/saveddata/maps/MapDecoration;renderOnFrame()Z"))
/// target: net.minecraft.world.level.saveddata.maps.MapDecoration#renderOnFrame  [exists]
/// within: net.minecraft.client.renderer.MapRenderer#extractDecorationRenderState
/// Rust: net/minecraft/world/level/saveddata/maps/map_decoration.rs:128 fn render_on_frame(&self)
///
/// `@Local(name = "decoration") MapDecoration decoration` is the call's RECEIVER (`decoration.renderOnFrame()`), so
/// this runs after the call (`Shift::After`) and reads it as `ci.this`.
fn prevent_decoration_in_dungeons(ci: &mut CallbackInfo<'_>) {
    let Some(always_rendered) = ci.return_value::<bool>().copied() else { return; };
    // Allow alwaysRendered if
    // 1. not in dungeons and map is disabled OR
    // 2. the decoration type is frame (self player) and either fancy map or show self head are off OR
    // 3. the decoration type is blue marker (other player) and the fancy map is off
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_in_dungeons() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.dungeon_map.enable_map {
        let Some(decoration) = ci.this::<mcrust::net::minecraft::world::level::saveddata::maps::map_decoration::MapDecoration>() else { return; };
        //   java: MapDecorationType decorationType = decoration.type().value();
        let decoration_type: Option<mcrust::net::minecraft::world::level::saveddata::maps::map_decoration_type::MapDecorationType> = decoration.value();
        let fancy_map: bool = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.dungeon_map.fancy_map;
        let should_show_self_marker: bool = decoration_type.is_some() && decoration_type == map_decoration_type_value("frame") && (!fancy_map || !(crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().dungeons.dungeon_map.show_self_head));
        let should_show_other_player_markers: bool = decoration_type.is_some() && decoration_type == map_decoration_type_value("blue_marker") && fancy_map;
        ci.set_return_value(should_show_self_marker || should_show_other_player_markers);
        return;
    }
    ci.set_return_value(always_rendered);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // @ModifyExpressionValue: runs after the call (it reads the call's receiver and replaces its value).
    report(hook::call_site("net.minecraft.client.renderer.MapRenderer#extractDecorationRenderState", "net.minecraft.world.level.saveddata.maps.MapDecoration#renderOnFrame").priority(1000).inject(Shift::After, prevent_decoration_in_dungeons));
}
