//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.BatMixin by fabric2seer.
//! @Mixin(net.minecraft.world.entity.ambient.Bat)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Override
fn on_client_removal(this: &mut mcrust::net::minecraft::world::entity::ambient::bat::Bat) {
    // java: super.onClientRemoval(); - the override is registered at the TAIL of `Entity#onClientRemoval` (the port's
    // `Bat` inherits the trait's default body, which therefore runs first).
    if this.base.base.base.get_health() <= (0 as f32) {
        crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager::DungeonManager::on_bat_removed(&*this);
    }
}

/// `public void onClientRemoval()` declared in the mixin class overrides `Bat`'s inherited `Entity.onClientRemoval`:
/// the method's TAIL, for a `this` that is a `Bat` (the hook's `this` is the concrete entity of the call).
fn on_client_removal_override(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::entity::ambient::bat::Bat>()) else { return; };
    on_client_removal(this);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.world.entity.Entity#onClientRemoval", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_client_removal_override)));
}
