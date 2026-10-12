//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.BossHealthOverlayMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.BossHealthOverlay)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject onRender
///   java: @Inject(method = "extractRenderState", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.gui.components.BossHealthOverlay#extractRenderState  [exists]
/// Rust: net/minecraft/client/gui/components/boss_health_overlay.rs:374 fn extract_render_state(&self, gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, gui_width: i32, gui_height: i32, now_ms: i64)
fn on_render(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::gui::components::boss_health_overlay::BossHealthOverlay>()) else { return; };
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.display_bossbar && crate::de::hysky::skyblocker::skyblock::slayers::slayer_boss_bar::SlayerBossBar::should_render_boss_bar() {
        let mut boss_bar: mcrust::net::minecraft::client::gui::components::lerping_boss_event::LerpingBossEvent = crate::de::hysky::skyblocker::skyblock::slayers::slayer_boss_bar::SlayerBossBar::update_boss_bar();
        // TODO(fabric2seer): field `font` of a Minecraft
        //   java: int textWidth = this.minecraft.font.width(bossBar.getName());
        let mut text_width: i32 = todo!("fabric2seer: the initialiser of `textWidth` (TODO above)");
        // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
        //   java: graphics.text(this.minecraft.font, bossBar.getName(), graphics.guiWidth() / 2 - textWidth / 2, 3, CommonColors.WHITE);
        // TODO(fabric2seer): `graphics`: this argument is not visible to the injector (see the notes above the handler)
        //   java: this.extractBar(graphics, (graphics.guiWidth() / 2) - 91, 12, bossBar);
        ci.cancel();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.components.BossHealthOverlay#extractRenderState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(on_render)));
}
