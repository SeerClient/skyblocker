//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ChestBlockEntityMixin by fabric2seer.
//! @Mixin(net.minecraft.world.level.block.entity.ChestBlockEntity)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$onSyncedBlockEvent
///   java: @Inject(at = @At("HEAD"), method = "triggerEvent(II)Z")
/// target: net.minecraft.world.level.block.entity.ChestBlockEntity#triggerEvent  [exists]
/// Rust: net/minecraft/world/level/block/entity/chest_block_entity.rs:249 fn trigger_event(&self, _game_time: i64, b0: i32, b1: i32)
fn skyblocker_on_synced_block_event(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::level::block::entity::chest_block_entity::ChestBlockEntity>()) else { return; };
    let r#type: i32 = ci.arg::<i32>(1).copied().unwrap_or_default();
    let data: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
    if r#type != 1 || data <= 0 {
        return;
    }
    // TODO(fabric2seer): unknown name `worldPosition`
    //   java: DungeonManager.onChestOpened(worldPosition);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.world.level.block.entity.ChestBlockEntity#triggerEvent", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_synced_block_event)));
}
