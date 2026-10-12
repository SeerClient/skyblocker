//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.SynchedEntityDataMixin by fabric2seer.
//! @Mixin(net.minecraft.network.syncher.SynchedEntityData)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$onWriteUpdatedEntries
///   java: @Inject(method = "assignValues", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/syncher/SynchedEntityData;assignValue(Lnet/minecraft/network/syncher/SynchedEntityData$DataItem;Lnet/minecraft/network/syncher/SynchedEntityData$DataValue;)V"))
/// target: net.minecraft.network.syncher.SynchedEntityData#assignValue  [exists]
/// within: net.minecraft.network.syncher.SynchedEntityData#assignValues
/// Rust: net/minecraft/network/syncher/synched_entity_data.rs:314 fn assign_value(data_item: &mut DataItem, item: &DataValue)
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
fn skyblocker_on_write_updated_entries(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
    //   java: BeaconHighlighter.onThrowBeacon(entity, dataItem, newValue);
}

/// @Inject skyblocker$ignoreInvalidDataExceptions
///   java: @Inject(method = "assignValue", at = @At(value = "NEW", target = "Ljava/lang/IllegalStateException;"), cancellable = true)
/// note: @At("NEW") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
fn skyblocker_ignore_invalid_data_exceptions(ci: &mut CallbackInfo<'_>) {
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() {
        ci.cancel();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.network.syncher.SynchedEntityData#assignValues", "net.minecraft.network.syncher.SynchedEntityData#assignValue").priority(1000).inject(Shift::Before, skyblocker_on_write_updated_entries));
    // TODO(fabric2seer): @Inject skyblocker$ignoreInvalidDataExceptions: nothing to register - @At("NEW") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
}
