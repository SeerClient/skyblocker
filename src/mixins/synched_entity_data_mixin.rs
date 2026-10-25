//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.SynchedEntityDataMixin by fabric2seer.
//! @Mixin(net.minecraft.network.syncher.SynchedEntityData)

#[allow(unused_imports)]
use crate::prelude::*;

use mcrust::net::minecraft::network::syncher::synched_entity_data::{self, DataItem, DataValue};

/// @Inject skyblocker$onWriteUpdatedEntries
///   java: @Inject(method = "assignValues", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/syncher/SynchedEntityData;assignValue(Lnet/minecraft/network/syncher/SynchedEntityData$DataItem;Lnet/minecraft/network/syncher/SynchedEntityData$DataValue;)V"))
/// target: net.minecraft.network.syncher.SynchedEntityData#assignValue  [exists]
/// within: net.minecraft.network.syncher.SynchedEntityData#assignValues (the port's per-item `assign_value_at_for`)
/// Rust: net/minecraft/network/syncher/synched_entity_data.rs fn assign_value(data_item: &mut DataItem, item: &DataValue)
fn skyblocker_on_write_updated_entries(ci: &mut CallbackInfo<'_>) {
    // `@Local(name = "dataItem")` / `@Local(name = "item")` are the call's two arguments.
    let Some(data_item) = compat::detach_ref(ci.arg::<DataItem>(0)) else { return; };
    let Some(new_value) = compat::detach_ref(ci.arg::<DataValue>(1)) else { return; };
    // `@Shadow private SyncedDataHolder entity` - the holder `assignValues` is running for.
    let Some(entity) = synched_entity_data::assigning_holder() else { return; };
    crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::beacon_highlighter::BeaconHighlighter::on_throw_beacon(&entity, data_item, new_value);
}

/// @Inject skyblocker$ignoreInvalidDataExceptions
///   java: @Inject(method = "assignValue", at = @At(value = "NEW", target = "Ljava/lang/IllegalStateException;"), cancellable = true)
///
/// `assignValue`'s one `new IllegalStateException` is its serializer-mismatch branch (:122-133), which the port's
/// `assign_value` answers as `Err` (the throw) without assigning. Cancelling at that `NEW` returns before the throw and
/// before the assignment, so the port's form is: at RETURN, an `Err` becomes `Ok(())` - the same "no assignment, no
/// exception" outcome.
fn skyblocker_ignore_invalid_data_exceptions(ci: &mut CallbackInfo<'_>) {
    let threw = ci.return_value::<Result<(), String>>().is_some_and(|r| r.is_err());
    if threw && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() {
        // ci.cancel();
        ci.set_return_value::<Result<(), String>>(Ok(()));
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.network.syncher.SynchedEntityData#assignValues", "net.minecraft.network.syncher.SynchedEntityData#assignValue").priority(1000).inject(Shift::Before, skyblocker_on_write_updated_entries));
    report(hook::register_injector("net.minecraft.network.syncher.SynchedEntityData#assignValue", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(skyblocker_ignore_invalid_data_exceptions)));
}
