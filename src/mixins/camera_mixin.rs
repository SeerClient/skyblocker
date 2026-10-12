//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.CameraMixin by fabric2seer.
//! @Mixin(net.minecraft.client.Camera)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue skyblocker$onCameraUpdate
///   java: @ModifyReturnValue(method = "position", at = @At("RETURN"))
/// target: net.minecraft.client.Camera#position  [exists]
/// Rust: net/minecraft/client/camera.rs:1227 fn position(&self)
fn skyblocker_on_camera_update(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::camera::Camera>()) else { return; };
    let Some(original) = ci.take_return_value::<mcrust::net::minecraft::world::phys::vec3::Vec3>() else { return; };
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.smooth_aote.predictive {
        let pos: Option<mcrust::net::minecraft::world::phys::vec3::Vec3> = crate::de::hysky::skyblocker::skyblock::teleport::predictive_smooth_aote::PredictiveSmoothAOTE::get_interpolated_pos();
        if let Some(pos) = pos {
            ci.set_return_value(pos);
            return;
        }
    } else {
        let pos: Option<mcrust::net::minecraft::world::phys::vec3::Vec3> = crate::de::hysky::skyblocker::skyblock::teleport::responsive_smooth_aote::ResponsiveSmoothAOTE::get_interpolated_pos(original.clone());
        if let Some(pos) = pos {
            ci.set_return_value(pos);
            return;
        }
    }
    ci.set_return_value(original);
    return;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.Camera#position", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_on_camera_update)));
}
