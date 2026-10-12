//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ClientLevelMixin by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.ClientLevel)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject skyblocker$beforeBlockUpdate
///   java: @Inject(method = "setServerVerifiedBlockState", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/Level;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)Z"))
/// target: net.minecraft.world.level.Level#setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)Z  [no Rust counterpart]
/// within: net.minecraft.client.multiplayer.ClientLevel#setServerVerifiedBlockState
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
fn skyblocker_before_block_update(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): `oldState`: this argument is not visible to the injector (see the notes above the handler)
    //   java: oldState.set(getBlockState(pos));
}

/// @Inject skyblocker$afterBlockUpdate
///   java: @Inject(method = "setServerVerifiedBlockState", at = @At("RETURN"))
/// target: net.minecraft.client.multiplayer.ClientLevel#setServerVerifiedBlockState  [exists]
/// Rust: net/minecraft/client/multiplayer/client_level.rs:2604 fn set_server_verified_block_state(&self, pos: &BlockPos, block_state: &Obj, update_flag: i32)
fn skyblocker_after_block_update(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel>()) else { return; };
    let Some(pos) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::core::block_pos::BlockPos>(0)) else { return; };
    let state: i32 = ci.arg::<i32>(2).copied().unwrap_or_default();
    // TODO(fabric2seer): `.get(..)` on a LocalRef
    //   java: WorldEvents.BLOCK_STATE_UPDATE.invoker().onBlockStateUpdate(pos, oldState.get(), state);
}

/// @Inject skyblocker$allowSoundsFromEntity
///   java: @Inject(method = "playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.multiplayer.ClientLevel#playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V  [exists]
/// Rust: net/minecraft/client/multiplayer/client_level.rs:4880 fn play_seeded_sound_at_entity(&self, source_entity: std::sync::Arc<std::sync::Mutex<StoredEntity>>, sound: &crate::net::minecraft::sounds::sound_event::SoundEvent, source: crate::net::minecraft::sounds::sound_source::SoundSource, volume: f32, pitch: f32, seed: i64, sounds: &mut crate::net::minecraf
fn skyblocker_allow_sounds_from_entity(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel>()) else { return; };
    let Some(sound) = compat::detach_ref(ci.arg::<crate::net::minecraft::sounds::sound_event::SoundEvent>(1)) else { return; };
    if !((crate::de::hysky::skyblocker::events::play_sound_events::allow_sound_invoker())(sound.value())) {
        ci.cancel();
    }
}

/// @Inject skyblocker$allowSounds
///   java: @Inject(method = "playSound(DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FFZJ)V", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.multiplayer.ClientLevel#playSound  [exists]
/// Rust: net/minecraft/client/multiplayer/client_level.rs:4994 fn play_sound(&self, x: f64, y: f64, z: f64, sound: &crate::net::minecraft::sounds::sound_event::SoundEvent, source: crate::net::minecraft::sounds::sound_source::SoundSource, volume: f32, pitch: f32, distance_delay: bool, seed: i64, sounds: &mut crate::net::minecraft::client::sounds::sound_manager::
fn skyblocker_allow_sounds(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel>()) else { return; };
    let Some(sound) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::sounds::sound_event::SoundEvent>(3)) else { return; };
    if !((crate::de::hysky::skyblocker::events::play_sound_events::allow_sound_invoker())(sound.clone())) {
        ci.cancel();
    }
}

/// @Inject onAddEntity
///   java: @Inject(method = "addEntity", at = @At("TAIL"))
/// target: net.minecraft.client.multiplayer.ClientLevel#addEntity  [exists]
/// Rust: net/minecraft/client/multiplayer/client_level.rs:9816 fn add_entity(&self, entity: &Shared<StoredEntity>)
fn on_add_entity(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel>()) else { return; };
    // TODO(fabric2seer): `entity` is Rust argument 0: `&Shared<StoredEntity>` - generic or borrowed with a lifetime; read it with `unsafe { ci.arg_unchecked::<..>(0) }`
    crate::de::hysky::skyblocker::skyblock::garden::greenhouse_paste::GreenhousePaste::on_entity_change(&entity);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
    // report(hook::call_site(`net.minecraft.client.multiplayer.ClientLevel#setServerVerifiedBlockState`, `net.minecraft.world.level.Level#setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)Z`).priority(1000).inject(Shift::Before, skyblocker_before_block_update));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#setServerVerifiedBlockState", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_after_block_update)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_allow_sounds_from_entity)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#playSound", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_allow_sounds)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#addEntity", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_add_entity)));
}
