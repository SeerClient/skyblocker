//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.InventoryMixin by fabric2seer.
//! @Mixin(net.minecraft.world.entity.player.Inventory)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$onHotbarScroll
///   java: @Inject(method = "setSelectedSlot", at = @At("TAIL"))
/// target: net.minecraft.world.entity.player.Inventory#setSelectedSlot  [exists]
/// Rust: net/minecraft/world/entity/player/inventory.rs:313 fn set_selected_slot(&mut self, selected: i32)
fn skyblocker_on_hotbar_scroll(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::entity::player::inventory::Inventory>()) else { return; };
    crate::de::hysky::skyblocker::skyblock::crimson::kuudra::arrow_poison_warning::ArrowPoisonWarning::try_warn(this.selected);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.world.entity.player.Inventory#setSelectedSlot", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_on_hotbar_scroll)));
}
