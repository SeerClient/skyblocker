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
/// `@Share("old") LocalRef<BlockState> oldState` - one value per run of `setServerVerifiedBlockState`, shared by
/// the two injectors (both on the render thread, one run at a time).
static OLD_STATE: LazyLock<Mutex<Option<std::sync::Arc<mcrust::net::minecraft::world::level::block::state::block_state::BlockState>>>> = LazyLock::new(|| Mutex::new(None));

/// rust: port gap - `Level.setBlock` is reached through the unhooked `ClientLevel::super_set_block` (GAPS_batchD),
/// so the call site cannot be selected; this runs at the method's HEAD instead, where the block is the same one
/// `:203`'s `setBlock` would replace (`:202` changes only the prediction handler). The state is therefore recorded
/// also on the run whose `:202` answers true and skips the `setBlock`, where Java's `oldState` stays null.
fn skyblocker_before_block_update(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel>()) else { return; };
    let Some(pos) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::core::block_pos::BlockPos>(0)) else { return; };
    compat::set_state(&OLD_STATE, Some(this.get_block_state_typed(pos)));
}

/// @Inject skyblocker$afterBlockUpdate
///   java: @Inject(method = "setServerVerifiedBlockState", at = @At("RETURN"))
/// target: net.minecraft.client.multiplayer.ClientLevel#setServerVerifiedBlockState  [exists]
/// Rust: net/minecraft/client/multiplayer/client_level.rs:2604 fn set_server_verified_block_state(&self, pos: &BlockPos, block_state: &Obj, update_flag: i32)
fn skyblocker_after_block_update(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel>()) else { return; };
    let Some(pos) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::core::block_pos::BlockPos>(0)) else { return; };
    // `@Local(name = "blockState") BlockState state` is argument 1, the erased state `Obj`.
    let Some(state) = ci.arg::<mcrust::com::mojang::serialization::Obj>(1).and_then(|state| mcrust::com::mojang::serialization::cast::<mcrust::net::minecraft::world::level::block::block::ErasedBlockState>(state)).map(|state| std::sync::Arc::clone(&state.0)) else { return; };
    let Some(old_state) = compat::lock(&OLD_STATE).take() else { return; };
    (crate::de::hysky::skyblocker::events::world_events::block_state_update_invoker())(pos.clone(), old_state, state);
}

/// @Inject skyblocker$allowSoundsFromEntity
///   java: @Inject(method = "playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V", at = @At("HEAD"), cancellable = true)
/// target: net.minecraft.client.multiplayer.ClientLevel#playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V  [exists]
/// Rust: net/minecraft/client/multiplayer/client_level.rs:4880 fn play_seeded_sound_at_entity(&self, source_entity: std::sync::Arc<std::sync::Mutex<StoredEntity>>, sound: &crate::net::minecraft::sounds::sound_event::SoundEvent, source: crate::net::minecraft::sounds::sound_source::SoundSource, volume: f32, pitch: f32, seed: i64, sounds: &mut crate::net::minecraf
fn skyblocker_allow_sounds_from_entity(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel>()) else { return; };
    // `Holder<SoundEvent> sound`: the port's `playSeededSound` is handed the resolved event (argument 1).
    let Some(sound) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::sounds::sound_event::SoundEvent>(1)) else { return; };
    if !((crate::de::hysky::skyblocker::events::play_sound_events::allow_sound_invoker())(sound.clone())) {
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
    let Some(entity) = compat::hook_arg::<mcrust::net::minecraft::world::level::entity::uniquely_identifyable::Shared<mcrust::net::minecraft::client::multiplayer::client_level::StoredEntity>>(ci, 0) else { return; };
    // At TAIL `addEntity` has let the entity's lock go; it is read through the game's tracked lock.
    compat::with_entity_handle(entity, |entity| {
        crate::de::hysky::skyblocker::skyblock::garden::greenhouse_paste::GreenhousePaste::on_entity_change(&*entity);
    });
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // java: @At(INVOKE Level.setBlock) - see `skyblocker_before_block_update` (HEAD of the method).
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#setServerVerifiedBlockState", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_before_block_update)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#setServerVerifiedBlockState", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_after_block_update)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_allow_sounds_from_entity)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#playSound", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_allow_sounds)));
    report(hook::register_injector("net.minecraft.client.multiplayer.ClientLevel#addEntity", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_add_entity)));
}
