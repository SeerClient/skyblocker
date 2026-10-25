//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.FogRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.fog.FogRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject applyFogModifyDistance
///   java: @Inject(method = "setupFog(Lnet/minecraft/client/Camera;ILnet/minecraft/client/DeltaTracker;FLnet/minecraft/client/multiplayer/ClientLevel;)Lnet/minecraft/client/renderer/fog/FogData;", at = @At(value = "FIELD", target = "Lnet/minecraft/client/renderer/fog/FogData;renderDistanceEnd:F", opcode = Opcodes.PUTFIELD, shift = At.Shift.AFTER))
///
/// The `PUTFIELD renderDistanceEnd` (shift AFTER) is `setupFog:200`, whose next statement is `return fog`: so this is
/// a RETURN modification of the answered `FogData`.
/// The game's per-frame fog comes from `setup_fog_with_fluid_in_camera` (the port's `setupFog` for a caller that has
/// already read the camera's fluid), which is hooked under the same name, so this applies to the frame's fog.
fn apply_fog_modify_distance(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::utils::utils::Utils::is_in_crimson() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().crimson_isle.extend_nether_fog {
        let Some(fog) = ci.return_value_mut::<mcrust::net::minecraft::client::renderer::fog::fog_data::FogData>() else { return; };
        fog.environmental_start = f32::MAX;
        fog.environmental_end = f32::MAX;
        fog.render_distance_start = f32::MAX;
        fog.render_distance_end = f32::MAX;
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.renderer.fog.FogRenderer#setupFog", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(apply_fog_modify_distance)));
}
