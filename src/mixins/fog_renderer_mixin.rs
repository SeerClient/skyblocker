//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.FogRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.fog.FogRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject applyFogModifyDistance
///   java: @Inject(method = "setupFog(Lnet/minecraft/client/Camera;ILnet/minecraft/client/DeltaTracker;FLnet/minecraft/client/multiplayer/ClientLevel;)Lnet/minecraft/client/renderer/fog/FogData;", at = @At(value = "FIELD", target = "Lnet/minecraft/client/renderer/fog/FogData;renderDistanceEnd:F", opcode = Opcodes.PUTFIELD, shift = At.Shift.AFTER))
/// note: @At("FIELD") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
///
/// The `PUTFIELD renderDistanceEnd` (shift AFTER) is `setupFog:200`, whose next statement is `return fog`: so this is
/// a RETURN modification of the answered `FogData`.
/// rust: port gap - the game's per-frame fog comes from the unhooked `setup_fog_with_fluid_in_camera` (a port
/// optimisation beside `setupFog`), so only callers of `setupFog` itself see this (GAPS_M #20).
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
