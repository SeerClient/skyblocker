//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.EntityMixin by fabric2seer.
//! @Mixin(net.minecraft.world.entity.Entity)

#[allow(unused_imports)]
use crate::prelude::*;

use crate::de::hysky::skyblocker::injected::skyblocker_entity::SkyblockerEntity;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @Unique private @Nullable Component skyblocker$customName;
/// One value per Entity OBJECT (a Mixin field): an attachment.
pub static ATTACH_SKYBLOCKER_CUSTOM_NAME: AttachmentType<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> = AttachmentType::new("skyblocker:entity_mixin_skyblocker_custom_name");

/// @Unique
fn skyblocker_get_custom_name(this: &mut mcrust::net::minecraft::world::entity::entity::Entity) -> /* TODO: Component */ () {
    return this.get_attached(&ATTACH_SKYBLOCKER_CUSTOM_NAME).unwrap();
}

/// @Unique
fn skyblocker_set_custom_name(this: &mut mcrust::net::minecraft::world::entity::entity::Entity, custom_name: &mut mcrust::net::minecraft::network::chat::component::Component) {
    this.set_attached(&ATTACH_SKYBLOCKER_CUSTOM_NAME, Some(custom_name.clone()));
}

/// @ModifyExpressionValue skyblocker$showInvisibleArmorStands
///   java: @ModifyExpressionValue(method = "isInvisibleTo", at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/player/Player;isSpectator()Z"))
/// target: net.minecraft.world.entity.player.Player#isSpectator  [exists]
/// within: net.minecraft.world.entity.Entity#isInvisibleTo
/// Rust: net/minecraft/world/entity/player/player.rs:2620 fn is_spectator(&self)
fn skyblocker_show_invisible_armor_stands(v: bool) -> bool {
    let mut is_spectator = v;
    // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
    //   java: return isSpectator || (isInvisible() && Utils.isOnHypixel() && Debug.debugEnabled() && SkyblockerConfigManager.get().debug.showInvisibleArmorStands && type.equals(EntityTypes.ARMOR_STAND));
    todo!("fabric2seer: the value of this function is in a TODO above")
}

/// @ModifyReturnValue modifyStartRidingReturnValue
///   java: @ModifyReturnValue(method = "startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z", at = @At("RETURN"))
/// target: net.minecraft.world.entity.Entity#startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z  [exists]
/// Rust: net/minecraft/world/entity/entity.rs:9850 fn start_riding(&mut self, level: &dyn CollisionGetter, entity_to_ride: &mut dyn Entity, force: bool, _send_event_and_triggers: bool)
fn modify_start_riding_return_value(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::entity::entity::Entity>()) else { return; };
    let original_return_value: bool = ci.return_value::<bool>().copied().unwrap_or_default();
    // TODO(fabric2seer): static `EntityTypes.ENDERMAN` has no Rust counterpart in the port's EntityTypes file
    // TODO(fabric2seer): the condition: unknown name `entity`
    //   java: originalReturnValue && SkyblockerConfigManager.get().slayers.endermanSlayer.lazerTimer &&
    //   java: 				type == EntityTypes.ENDERMAN &&
    //   java: 				entity.getType() == EntityTypes.ARMOR_STAND &&
    //   java: 				SlayerManager.isSelectedBoss(uuid) &&
    //   java: 				!LazerTimer.isActive()
    if todo!("fabric2seer: the condition (TODO above)") {
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
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::entity::entity::Entity>()) else { return; };
    // TODO(fabric2seer): static `EntityTypes.ENDERMAN` has no Rust counterpart in the port's EntityTypes file
    if this.r#type == crate::unresolved::EntityTypes::ENDERMAN && crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.enderman_slayer.lazer_timer && crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_selected_boss(&this.uuid) && crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::lazer_timer::LazerTimer::is_active() {
        crate::de::hysky::skyblocker::skyblock::slayers::boss::voidgloom::lazer_timer::LazerTimer::tick();
    }
}

/// @Inject onRemove
///   java: @Inject(method = "onRemoval", at = @At("TAIL"))
/// target: net.minecraft.world.entity.Entity#onRemoval  [exists]
/// Rust: net/minecraft/world/entity/entity.rs:10516 fn on_removal(&mut self, _reason: RemovalReason)
fn on_remove(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::world::entity::entity::Entity>()) else { return; };
    let Some(reason) = compat::detach_ref(ci.arg::<RemovalReason>(0)) else { return; };
    if crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_selected_boss(&this.uuid) {
        if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().slayers.slain_time && crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_in_slayer_quest() {
            let mut slayer_quest: compat::Shared<crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerQuest> = crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::get_slayer_quest().unwrap();
            if true /* java: slayer_quest != null: never null here */ && compat::read(&slayer_quest, |__t| __t.boss_spawned) && crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::SlayerManager::is_fighting_owned_slayer() {
                slayer_quest.boss_death_time = std::time::SystemTime::now();
            }
        }
        crate::de::hysky::skyblocker::skyblock::slayers::slayer_manager::BossFight::remove();
    }
    let mut entity: Box<dyn mcrust::net::minecraft::world::entity::entity::Entity> = this;
    crate::de::hysky::skyblocker::skyblock::garden::greenhouse_paste::GreenhousePaste::on_entity_change(&entity);
}

/// EntityMixin makes every Entity implement SkyblockerEntity (an interface added by a Mixin): an
/// extension trait of the mod, implemented for the port's type.
impl SkyblockerEntity for mcrust::net::minecraft::world::entity::entity::Entity {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn skyblocker_get_custom_name(&self) -> Option<MutableComponent> {
        return self.get_attached(&ATTACH_SKYBLOCKER_CUSTOM_NAME);
    }

    fn skyblocker_set_custom_name(&self, custom_name: &MutableComponent) {
        self.set_attached(&ATTACH_SKYBLOCKER_CUSTOM_NAME, Some(custom_name.clone()));
    }

}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.world.entity.Entity#isInvisibleTo", "net.minecraft.world.entity.player.Player#isSpectator").priority(1000).modify_expression_value::<bool>(skyblocker_show_invisible_armor_stands));
    report(hook::register_injector("net.minecraft.world.entity.Entity#startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(modify_start_riding_return_value)));
    report(hook::register_injector("net.minecraft.world.entity.Entity#tick", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_tick)));
    report(hook::register_injector("net.minecraft.world.entity.Entity#onRemoval", InjectOptions::at(At::Tail).kind(InjectorKind::Inject).priority(1000), Arc::new(on_remove)));
}
