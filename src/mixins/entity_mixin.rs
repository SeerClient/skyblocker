//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.EntityMixin by fabric2seer.
//! @Mixin(net.minecraft.world.entity.Entity)

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::injected::skyblocker_entity::SkyblockerEntity;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::entity::Entity;

/// java: @Unique private @Nullable Component skyblocker$customName;
/// One value per Entity OBJECT (a Mixin field): an attachment.
pub static ATTACH_SKYBLOCKER_CUSTOM_NAME: AttachmentType<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> = AttachmentType::new("skyblocker:entity_mixin_skyblocker_custom_name");

/// @Unique
fn skyblocker_get_custom_name<E: Entity + ?Sized>(this: &E) -> Option<MutableComponent> {
    return this.get_attached(&ATTACH_SKYBLOCKER_CUSTOM_NAME);
}

/// @Unique
fn skyblocker_set_custom_name<E: Entity + ?Sized>(this: &E, custom_name: &MutableComponent) {
    ANY_SKYBLOCKER_CUSTOM_NAME.store(true, std::sync::atomic::Ordering::Relaxed);
    this.set_attached(&ATTACH_SKYBLOCKER_CUSTOM_NAME, custom_name.clone());
}

/// PERF (port-only): set (never cleared) by the first `skyblocker$setCustomName` - the attachment's only writer. Until
/// then no entity carries a custom name, so `EntityRendererMixin.skyblocker$applyCustomName` answers `original` for
/// every `getNameTag` (its pre-filter, `custom_name_may_apply`).
pub static ANY_SKYBLOCKER_CUSTOM_NAME: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// @ModifyExpressionValue skyblocker$showInvisibleArmorStands
///   java: @ModifyExpressionValue(method = "isInvisibleTo", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/player/Player;isSpectator()Z"))
/// within: net.minecraft.world.entity.Entity#isInvisibleTo
///
/// The port evaluates `player.isSpectator()` before `isInvisibleTo`'s body (the free `entity::is_invisible_to` is
/// handed it as a value), and every reader of `isInvisibleTo` passes the entity and the game's answer through the
/// hooked `Entity#isInvisibleTo(Player)` (`mcrust::modding::entities::is_invisible_to`). A `true` from
/// `isSpectator()` makes `isInvisibleTo` answer `false` (:2762-2763), so the injector forces `false` exactly where the
/// mixin's expression turns `true`: `isInvisible() && Utils.isOnHypixel() && Debug.debugEnabled()
/// && showInvisibleArmorStands && type.equals(EntityTypes.ARMOR_STAND)`.
fn skyblocker_show_invisible_armor_stands(ci: &mut CallbackInfo<'_>) {
    //   java: return isSpectator || (isInvisible() && Utils.isOnHypixel() && Debug.debugEnabled() && SkyblockerConfigManager.get().debug.showInvisibleArmorStands && type.equals(EntityTypes.ARMOR_STAND));
    // PERF (port-only, same result): every operand is a side-effect-free read, so the global ones go first (they are
    // also the injector's pre-filter, `show_invisible_armor_stands_may_apply`); the downcast and `isInvisible()` only
    // run when they all hold.
    if !show_invisible_armor_stands_may_apply() {
        return;
    }
    // `this` - argument 0, the entity's `Entity::as_any()`; `type == ARMOR_STAND` is the class `ARMOR_STAND` builds.
    let Some(entity) = compat::hook_arg::<Option<&'static dyn std::any::Any>>(ci, 0).copied().flatten() else { return; };
    let Some(armor_stand) = entity.downcast_ref::<mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand>() else { return; };
    if armor_stand.base().is_invisible() {
        ci.set_return_value(false);
    }
}

/// PERF (port-only, same result): the global operands of the expression above - the injector's pre-filter
/// (`InjectOptions::when`). Without debug mode (the normal case) `isInvisibleTo` (~1,900 calls a frame) runs as if
/// unhooked.
fn show_invisible_armor_stands_may_apply() -> bool {
    crate::de::hysky::skyblocker::debug::debug::Debug::debug_enabled()
        && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().debug.show_invisible_armor_stands
        && crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel()
}

/// @ModifyReturnValue modifyStartRidingReturnValue
///   java: @ModifyReturnValue(method = "startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z", at = @At("RETURN"))
/// target: net.minecraft.world.entity.Entity#startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z  [exists]
/// Rust: net/minecraft/world/entity/entity.rs:9850 fn start_riding(&mut self, level: &dyn CollisionGetter, entity_to_ride: &mut dyn Entity, force: bool, _send_event_and_triggers: bool)
fn modify_start_riding_return_value(ci: &mut CallbackInfo<'_>) {
    let original_return_value: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    // `type == EntityTypes.ENDERMAN`: the trait method's `this` is the concrete class, so the test is the receiver's
    // type (an `EnderMan` is the only class `ENDERMAN` builds).
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::entity::monster::enderman::Enderman>()) else { return; };
    // `Entity entity` is argument 1 (`entity_to_ride`), a `&mut dyn Entity`.
    let Some(entity) = compat::hook_arg::<dyn Entity>(ci, 1) else { return; };
    if original_return_value && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.enderman_slayer.lazer_timer &&
            compat::is_entity_type(entity, "armor_stand") &&
            crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_selected_boss(&compat::uuid_of(this.base().get_uuid())) &&
            !crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::lazer_timer::LazerTimer::is_active() {
        crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::lazer_timer::LazerTimer::activate();
    }
    ci.set_return_value(original_return_value);
    return;
}

/// @Inject onTick
///   java: @Inject(method = "tick", at = @At("TAIL"))
/// target: net.minecraft.world.entity.Entity#tick  [exists]
/// Rust: net/minecraft/world/entity/entity.rs:9030 fn tick(&mut self, level: &dyn CollisionGetter, _is_client_side: bool)
/// PERF (port-only, same result): `on_tick` does nothing unless the laser timer is active and its option is on;
/// neither is changed by `Entity.tick`'s own body (only by this callback and by `startRiding`), so the guard
/// keeps the injector off every other entity tick. Since perf round 1 the hook also runs for living entities
/// (`LivingEntity::entity_super_tick`), as Java's `super.tick()` does.
fn lazer_timer_may_apply() -> bool {
    crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::lazer_timer::LazerTimer::is_active()
        && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.enderman_slayer.lazer_timer
}

fn on_tick(ci: &mut CallbackInfo<'_>) {
    // `type == EntityTypes.ENDERMAN` - see `modify_start_riding_return_value`.
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::entity::monster::enderman::Enderman>()) else { return; };
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.enderman_slayer.lazer_timer && crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_selected_boss(&compat::uuid_of(this.base().get_uuid())) && crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::lazer_timer::LazerTimer::is_active() {
        crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::lazer_timer::LazerTimer::tick();
    }
}

/// @Inject onRemove
///   java: @Inject(method = "onRemoval", at = @At("TAIL"))
/// target: net.minecraft.world.entity.Entity#onRemoval  [exists]
/// Rust: net/minecraft/world/entity/entity.rs:10516 fn on_removal(&mut self, _reason: RemovalReason)
fn on_remove(this: &dyn Entity) {
    if crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_selected_boss(&compat::uuid_of(this.base().get_uuid())) {
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.slain_time && crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_in_slayer_quest() {
            let slayer_quest: Option<compat::Shared<crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerQuest>> = crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::get_slayer_quest();
            if let Some(slayer_quest) = slayer_quest && compat::read(&slayer_quest, |__t| __t.boss_spawned) && crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_fighting_owned_slayer() {
                compat::write(&slayer_quest, |__t| __t.boss_death_time = Some(std::time::SystemTime::now()));
            }
        }
        crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::BossFight::remove();
    }
    // literal voodoo magic
    let entity: &dyn Entity = this;
    crate::de::hysky::skyblocker::skyblock::garden::greenhouse_paste::GreenhousePaste::on_entity_change(entity);
}

/// The hook of `on_remove`: `Entity#onRemoval`'s receiver-carrying slot (`entity::on_removal_base`, reached from
/// `setRemoved:3972` for every entity) hands `this`'s `Entity` half as argument 0. `EntityBase` is itself an
/// `Entity`, and both readers ask it only for its UUID and position.
fn on_remove_hook(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::world::entity::entity::EntityBase>(0)) else { return; };
    on_remove(this);
}

/// EntityMixin makes every Entity implement SkyblockerEntity (an interface added by a Mixin): an
/// extension trait of the mod, implemented for the port's entity classes (`Entity` is a trait here).
impl<E: Entity + 'static> SkyblockerEntity for E {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn skyblocker_get_custom_name(&self) -> Option<MutableComponent> {
        return skyblocker_get_custom_name(self);
    }

    fn skyblocker_set_custom_name(&self, custom_name: &MutableComponent) {
        skyblocker_set_custom_name(self, custom_name);
    }
}

/// The same, for an entity seen as the trait object the level hands out.
impl SkyblockerEntity for dyn Entity {
    fn as_any(&self) -> &dyn std::any::Any {
        // `Entity::as_any` is a per-class slot; a class that never gave itself one answers no class.
        Entity::as_any(self).unwrap_or(&())
    }

    fn skyblocker_get_custom_name(&self) -> Option<MutableComponent> {
        return skyblocker_get_custom_name(self);
    }

    fn skyblocker_set_custom_name(&self, custom_name: &MutableComponent) {
        skyblocker_set_custom_name(self, custom_name);
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.world.entity.Entity#isInvisibleTo(Lnet/minecraft/world/entity/player/Player;)Z", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000).when(show_invisible_armor_stands_may_apply), Arc::new(skyblocker_show_invisible_armor_stands)));
    report(hook::register_injector("net.minecraft.world.entity.Entity#startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_start_riding_return_value)));
    report(hook::register_injector("net.minecraft.world.entity.Entity#tick", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000).when(lazer_timer_may_apply), Arc::new(on_tick)));
    report(hook::register_injector("net.minecraft.world.entity.Entity#onRemoval", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_remove_hook)));
}
