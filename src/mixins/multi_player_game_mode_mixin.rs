//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MultiPlayerGameModeMixin by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.MultiPlayerGameMode)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Unique
fn swing_hand_without_packets(this: &mut mcrust::net::minecraft::client::multiplayer::multi_player_game_mode::MultiPlayerGameMode, player_entity: &mut mcrust::net::minecraft::world::entity::player::player::Player, hand: &mut mcrust::net::minecraft::world::interaction_hand::InteractionHand) {
    // TODO(fabric2seer): `.swing(..)` on a Player
    //   java: playerEntity.swing(hand, false);
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
fn swing_on_ability(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): the condition: `itemStack`: this argument is not visible to the injector (see the notes above the handler)
    //   java: SkyblockerConfigManager.get().uiAndVisuals.swingOnAbilities
    //   java: 				&& SwingAnimation.hasAbility(itemStack)
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `playerEntity`: this argument is not visible to the injector (see the notes above the handler)
        //   java: swingHandWithoutPackets(playerEntity, hand);
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - `within` net.minecraft.client.multiplayer.MultiPlayerGameMode#lambda$useItem$0: not a method of that class in JavaSrc (a synthetic lambda of the vanilla method: the port has no method of that name - hook the method that contains the lambda) - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
    // report(hook::call_site(`net.minecraft.client.multiplayer.MultiPlayerGameMode#lambda$useItem$0`, `net.minecraft.world.entity.LivingEntity#getItemInHand`).priority(1000).inject(Shift::After, swing_on_ability));
}
