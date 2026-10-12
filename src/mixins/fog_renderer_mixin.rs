//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.FogRendererMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.fog.FogRenderer)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject applyFogModifyDistance
///   java: @Inject(method = "setupFog(Lnet/minecraft/client/Camera;ILnet/minecraft/client/DeltaTracker;FLnet/minecraft/client/multiplayer/ClientLevel;)Lnet/minecraft/client/renderer/fog/FogData;", at = @At(value = "FIELD", target = "Lnet/minecraft/client/renderer/fog/FogData;renderDistanceEnd:F", opcode = Opcodes.PUTFIELD, shift = At.Shift.AFTER))
/// note: @At("FIELD") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
fn apply_fog_modify_distance(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && crate::de::hysky::skyblocker::utils::utils::Utils::is_in_crimson() && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().crimson_isle.extend_nether_fog {
        // TODO(fabric2seer): `fog`: this argument is not visible to the injector (see the notes above the handler)
        //   java: fog.environmentalStart = Float.MAX_VALUE;
        // TODO(fabric2seer): `fog`: this argument is not visible to the injector (see the notes above the handler)
        //   java: fog.environmentalEnd = Float.MAX_VALUE;
        // TODO(fabric2seer): `fog`: this argument is not visible to the injector (see the notes above the handler)
        //   java: fog.renderDistanceStart = Float.MAX_VALUE;
        // TODO(fabric2seer): `fog`: this argument is not visible to the injector (see the notes above the handler)
        //   java: fog.renderDistanceEnd = Float.MAX_VALUE;
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): @Inject applyFogModifyDistance: nothing to register - @At("FIELD") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
}
