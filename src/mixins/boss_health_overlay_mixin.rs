//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.BossHealthOverlayMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.BossHealthOverlay)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject onRender
///   java: @Inject(method = "extractRenderState", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.gui.components.BossHealthOverlay#extractRenderState  [exists]
/// Rust: net/minecraft/client/gui/components/boss_health_overlay.rs:374 fn extract_render_state(&self, gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, gui_width: i32, gui_height: i32, now_ms: i64)
///
/// `graphics` is the port's `gl` / `gfx` pair (arguments 0 and 1) and `graphics.guiWidth()` is `gui_width`
/// (argument 2).
fn on_render(ci: &mut CallbackInfo<'_>) {
    let Some(_this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::gui::components::boss_health_overlay::BossHealthOverlay>()) else { return; };
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.display_bossbar && crate::de::hysky::skyblocker::skyblock::slayers::slayer_boss_bar::SlayerBossBar::should_render_boss_bar() {
        let boss_bar: mcrust::net::minecraft::client::gui::components::lerping_boss_event::LerpingBossEvent = crate::de::hysky::skyblocker::skyblock::slayers::slayer_boss_bar::SlayerBossBar::update_boss_bar();
        let (Some(gl), Some(gfx)) = (compat::hook_arg::<mcrust::lwjgl::opengl::Gl>(ci, 0), compat::hook_arg::<mcrust::net::minecraft::client::minecraft::Minecraft>(ci, 1)) else { return; };
        let gui_width: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
        // `this.minecraft.font.width(bossBar.getName())` - `Font.width(FormattedText)`.
        let text_width: i32 = compat::font().width_of(boss_bar.base.get_name());
        // `graphics.text(this.minecraft.font, bossBar.getName(), graphics.guiWidth() / 2 - textWidth / 2, 3, CommonColors.WHITE)`
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::draw_formatted_text(gl, gfx, gui_width / 2 - text_width / 2, 3, boss_bar.base.get_name(), mcrust::net::minecraft::util::common_colors::WHITE);
        // rust: port gap - `BossHealthOverlay::extract_bar` (boss_health_overlay.rs:431) is a private `fn`, so the mod
        // cannot make the `@Shadow` call (GAPS_M #1).
        //   java: this.extractBar(graphics, (graphics.guiWidth() / 2) - 91, 12, bossBar);
        ci.cancel();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.components.BossHealthOverlay#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(on_render)));
}
