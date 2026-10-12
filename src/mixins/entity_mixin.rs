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
    this.set_attached(&ATTACH_SKYBLOCKER_CUSTOM_NAME, custom_name.clone());
}

/// @ModifyExpressionValue skyblocker$showInvisibleArmorStands
///   java: @ModifyExpressionValue(method = "isInvisibleTo", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/player/Player;isSpectator()Z"))
/// target: net.minecraft.world.entity.player.Player#isSpectator  [exists]
/// within: net.minecraft.world.entity.Entity#isInvisibleTo
/// Rust: net/minecraft/world/entity/player/player.rs:2620 fn is_spectator(&self)
///
/// rust: port gap - the port's `isInvisibleTo` (entity.rs:4961) is a free function handed `player_is_spectator`
/// and `is_invisible` as VALUES - there is no `isSpectator()` call inside it to select and no `this` to read
/// `isInvisible()` / `type` from (GAPS_batchD). The value is handed back unchanged.
fn skyblocker_show_invisible_armor_stands(v: bool) -> bool {
    let mut is_spectator = v;
    //   java: return isSpectator || (isInvisible() && Utils.isOnHypixel() && Debug.debugEnabled() && SkyblockerConfigManager.get().debug.showInvisibleArmorStands && type.equals(EntityTypes.ARMOR_STAND));
    return is_spectator;
}

/// @ModifyReturnValue modifyStartRidingReturnValue
///   java: @ModifyReturnValue(method = "startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z", at = @At("RETURN"))
/// target: net.minecraft.world.entity.Entity#startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z  [exists]
/// Rust: net/minecraft/world/entity/entity.rs:9850 fn start_riding(&mut self, level: &dyn CollisionGetter, entity_to_ride: &mut dyn Entity, force: bool, _send_event_and_triggers: bool)
fn modify_start_riding_return_value(ci: &mut CallbackInfo<'_>) {
    let original_return_value: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    // `type == EntityTypes.ENDERMAN`: the trait method's `this` is the concrete class, so the test is the receiver's
    // type (an `EnderMan` is the only class `ENDERMAN` builds).
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::entity::monster::ender_man::EnderMan>()) else { return; };
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
fn on_tick(ci: &mut CallbackInfo<'_>) {
    // `type == EntityTypes.ENDERMAN` - see `modify_start_riding_return_value`.
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::world::entity::monster::ender_man::EnderMan>()) else { return; };
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

/// The hook of `on_remove`. rust: port gap - `Entity#onRemoval` is a default method of the port's `Entity` TRAIT, so
/// the hook's `this` is the CONCRETE class of the call and a mod cannot view an arbitrary one as `&dyn Entity`
/// (GAPS_batchD). The classes Skyblocker's two readers can act on are tried in turn: the slayer bosses' bodies and
/// the garden's entities are all `ArmorStand`s, `EnderMan`s or one of the other mobs listed below.
fn on_remove_hook(ci: &mut CallbackInfo<'_>) {
    macro_rules! try_class {
        ($($class:ty),* $(,)?) => {
            $(if let Some(this) = compat::detach_ref(ci.this::<$class>()) {
                on_remove(this);
                return;
            })*
        };
    }
    try_class!(
        mcrust::net::minecraft::world::entity::decoration::armor_stand::ArmorStand,
        mcrust::net::minecraft::world::entity::monster::ender_man::EnderMan,
        mcrust::net::minecraft::world::entity::monster::zombie::zombie::Zombie,
        mcrust::net::minecraft::world::entity::monster::spider::spider::Spider,
        mcrust::net::minecraft::world::entity::animal::wolf::wolf::Wolf,
        mcrust::net::minecraft::world::entity::monster::blaze::Blaze,
    );
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
    report(hook::call_site("net.minecraft.world.entity.Entity#isInvisibleTo", "net.minecraft.world.entity.player.Player#isSpectator").priority(1000).modify_expression_value::<bool>(skyblocker_show_invisible_armor_stands));
    report(hook::register_injector("net.minecraft.world.entity.Entity#startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_start_riding_return_value)));
    report(hook::register_injector("net.minecraft.world.entity.Entity#tick", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_tick)));
    report(hook::register_injector("net.minecraft.world.entity.Entity#onRemoval", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_remove_hook)));
}
