//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.GameRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.GameRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$onGameRendererClose
///   java: @Inject(method = "close", at = @At("TAIL"))
/// target: net.minecraft.client.renderer.GameRenderer#close  [no Rust counterpart]
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
fn skyblocker_on_game_renderer_close(ci: &mut CallbackInfo<'_>) {
    crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper::close();
    crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_map_texture::DungeonMapTexture::close();
}

/// @ModifyReturnValue onGetNightVisionStrength
///   java: @ModifyReturnValue(method = "nightVisionScale", at = @At("RETURN"))
/// target: net.minecraft.client.renderer.GameRenderer#nightVisionScale  [exists]
/// Rust: net/minecraft/client/renderer/game_renderer.rs:5211 fn night_vision_scale(night_vision: Option<&crate::net::minecraft::world::effect::mob_effect_instance::MobEffectInstance>, a: f32)
fn on_get_night_vision_strength(ci: &mut CallbackInfo<'_>) {
    let original: f32 = ci.return_value::<f32>().copied().unwrap_or_default();
    if original == 1.0f32 && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() {
        let mut strength = crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.night_vision_strength;
        if (strength as f32) == 0.0f32 {
            ci.set_return_value(0.0f32);
            return;
        }
        ci.set_return_value(((strength as f32) / 100.0f32).clamp((0 as f32), (1 as f32)));
        return;
    }
    ci.set_return_value(original);
    return;
}

/// @Inject skyblocker$onRenderGui
///   java: @Inject(method = "render", at = @At(value = "CONSTANT", args = "stringValue=gui"))
/// note: @At("CONSTANT") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
fn skyblocker_on_render_gui(ci: &mut CallbackInfo<'_>) {
    crate::de::hysky::skyblocker::utils::render::gui_helper::GuiHelper::update_screen_blit_texture();
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target)
    // report(hook::register_injector(`net.minecraft.client.renderer.GameRenderer#close`, InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_game_renderer_close)));
    report(hook::register_injector("net.minecraft.client.renderer.GameRenderer#nightVisionScale", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(on_get_night_vision_strength)));
    // java: @At(CONSTANT "gui") in render - the `profiler.push("gui")` right before `this.guiRenderer.render()`
    // (GameRenderer:446-447, its only caller); the port runs that call from the client loop, so it is GuiRenderer#render's HEAD.
    report(hook::register_injector("net.minecraft.client.gui.render.GuiRenderer#render", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_render_gui)));
}
