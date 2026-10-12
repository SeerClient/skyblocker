//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MultiPlayerGameModeMixin by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.MultiPlayerGameMode)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Unique
fn swing_hand_without_packets(player_entity: &mut dyn mcrust::net::minecraft::world::entity::player::player::Player, hand: mcrust::net::minecraft::world::interaction_hand::InteractionHand) {
    // The playerEntity override for swingHand is the other method with just the hand parameter, this one isn't
    // overridden and doesn't lead to sending packets.
    player_entity.swing_with(hand, false);
}

/// @Inject swingOnAbility
///   java: @Dynamic("Lambda inside of useItem")
///   java: @Inject(method = "lambda$useItem$0",
/// target: net.minecraft.world.entity.LivingEntity#getItemInHand  [not a method of that class in JavaSrc]
/// within: net.minecraft.client.multiplayer.MultiPlayerGameMode#lambda$useItem$0
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// note: slice = @Slice(..) ignored
/// note: `getItemInHand` is inherited: hooked on net.minecraft.world.entity.LivingEntity (it fires for every subclass - test the object's type in the handler if net.minecraft.world.entity.player.Player matters)
/// note: `within` net.minecraft.client.multiplayer.MultiPlayerGameMode#lambda$useItem$0: not a method of that class in JavaSrc (a synthetic lambda of the vanilla method: the port has no method of that name - hook the method that contains the lambda) - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
///
/// rust: port gap - `lambda$useItem$0` is a synthetic lambda with no port method to be `within`, and its locals
/// (`playerEntity`, `hand`, `itemStack`) are not visible to a call-site injector on `getItemInHand` (GAPS_batchD); the
/// handler's body is the Java's over those three, and it is not registered.
fn swing_on_ability(player_entity: &mut dyn mcrust::net::minecraft::world::entity::player::player::Player, hand: mcrust::net::minecraft::world::interaction_hand::InteractionHand, item_stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack) {
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.swing_on_abilities
        && crate::de::hysky::skyblocker::skyblock::swing_animation::SwingAnimation::has_ability(item_stack) {
        swing_hand_without_packets(player_entity, hand);
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - `within` net.minecraft.client.multiplayer.MultiPlayerGameMode#lambda$useItem$0: not a method of that class in JavaSrc (a synthetic lambda of the vanilla method: the port has no method of that name - hook the method that contains the lambda) - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
    // report(hook::call_site(`net.minecraft.client.multiplayer.MultiPlayerGameMode#lambda$useItem$0`, `net.minecraft.world.entity.LivingEntity#getItemInHand`).priority(1000).inject(Shift::After, swing_on_ability));
}
