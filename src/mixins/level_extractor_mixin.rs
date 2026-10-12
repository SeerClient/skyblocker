//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.LevelExtractorMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.extract.LevelExtractor)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapOperation skyblocker$addBlockBreakingProgressRenderState
///   java: @WrapOperation(method = "extractBlockDestroyAnimation", at = @At(value = "NEW", target = "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/client/renderer/state/level/BlockBreakingRenderState;"))
/// note: @At("NEW") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
fn skyblocker_add_block_breaking_progress_render_state(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().mining.block_break_prediction.enabled {
        // TODO(fabric2seer): `pos`: this argument is not visible to the injector (see the notes above the handler)
        //   java: int pingModifiedProgress = BlockBreakPrediction.getBlockBreakPrediction(pos, progress);
        let mut ping_modified_progress: i32 = todo!("fabric2seer: the initialiser of `pingModifiedProgress` (TODO above)");
        // TODO(fabric2seer): `pos`: this argument is not visible to the injector (see the notes above the handler)
        //   java: return new BlockBreakingRenderState(pos, state, pingModifiedProgress);
    } else {
        // TODO(fabric2seer): `original`: this argument is not visible to the injector (see the notes above the handler)
        //   java: return original.call(pos, state, progress);
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): @WrapOperation skyblocker$addBlockBreakingProgressRenderState: nothing to register - @At("NEW") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
}
