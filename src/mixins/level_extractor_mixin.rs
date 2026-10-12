//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.LevelExtractorMixin by fabric2seer.
//! @Mixin(net.minecraft.client.renderer.extract.LevelExtractor)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapOperation skyblocker$addBlockBreakingProgressRenderState
///   java: @WrapOperation(method = "extractBlockDestroyAnimation", at = @At(value = "NEW", target = "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/client/renderer/state/level/BlockBreakingRenderState;"))
/// target: net.minecraft.client.renderer.state.level.BlockBreakingRenderState#<init>  [exists]
/// within: net.minecraft.client.renderer.extract.LevelExtractor#extractBlockDestroyAnimation
/// Rust: net/minecraft/client/renderer/state/level/block_breaking_render_state.rs:63 fn new(block_pos: BlockPos, block_state: Arc<BlockState>, progress: i32) -> BlockBreakingRenderState
///
/// The record's canonical constructor is a hooked `fn` in the port, so `@At("NEW")` here IS a call site
/// (`level_extractor.rs:2285`, the only `new BlockBreakingRenderState` in the method).
fn skyblocker_add_block_breaking_progress_render_state(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().mining.block_break_prediction.enabled {
        let (Some(pos), Some(state)) = (ci.arg::<mcrust::net::minecraft::core::block_pos::BlockPos>(0).cloned(), ci.arg::<std::sync::Arc<mcrust::net::minecraft::world::level::block::state::block_state::BlockState>>(1).cloned()) else {
            original.call(ci);
            return;
        };
        let progress: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
        let ping_modified_progress: i32 = crate::de::hysky::skyblocker::skyblock::dwarven::block_break_prediction::BlockBreakPrediction::get_block_break_prediction(&pos, progress);
        //   java: return new BlockBreakingRenderState(pos, state, pingModifiedProgress);
        // The record's fields directly (calling the hooked constructor from its own wrapper would wrap it again).
        ci.set_return_value(mcrust::net::minecraft::client::renderer::state::level::block_breaking_render_state::BlockBreakingRenderState { block_pos: pos, block_state: state, progress: ping_modified_progress });
    }
    //if the setting is not enabled do not modify anything
    else {
        original.call(ci);
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.client.renderer.extract.LevelExtractor#extractBlockDestroyAnimation", "net.minecraft.client.renderer.state.level.BlockBreakingRenderState#<init>").priority(1000).wrap_operation(skyblocker_add_block_breaking_progress_render_state));
}
