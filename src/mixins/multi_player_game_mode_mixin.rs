//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MultiPlayerGameModeMixin by fabric2seer.
//! @Mixin(net.minecraft.client.multiplayer.MultiPlayerGameMode)

#[allow(unused_imports)]
use crate::prelude::*;

use mcrust::net::minecraft::client::multiplayer::multi_player_game_mode::MultiPlayerGameMode;
use mcrust::net::minecraft::world::interaction_hand::InteractionHand;

/// @Unique
/// java: playerEntity.swing(hand, false); - the `swing(InteractionHand, boolean)` overload, which sends no packet. The
/// lambda's player is held by the game's use path, so the swing is parked on the game mode and made on that player
/// when `useItem` returns (`MultiPlayerGameMode::swing_without_packets`).
fn swing_hand_without_packets(game_mode: &mut MultiPlayerGameMode, hand: InteractionHand) {
    // The playerEntity override for swingHand is the other method with just the hand parameter, this one isn't
    // overridden and doesn't lead to sending packets.
    game_mode.swing_without_packets(hand);
}

/// @Inject swingOnAbility
///   java: @Dynamic("Lambda inside of useItem")
///   java: @Inject(method = "lambda$useItem$0",
///       slice = @Slice(from = @At(value = "INVOKE", target = "ItemStack;use(..)")),
///       at = @At(value = "INVOKE_ASSIGN", target = "Lnet/minecraft/world/entity/player/Player;getItemInHand(Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack;"))
/// target: net.minecraft.client.multiplayer.MultiPlayerGameMode#lambda$useItem$0  [exists: the lambda's hook point
/// `MultiPlayerGameMode::lambda_use_item_0_after_use`, at the first `getItemInHand` after `itemStack.use`]
/// Arguments: `hand` (0), `playerEntity` (1), `@Local(name = "itemStack")` (2).
fn swing_on_ability(ci: &mut CallbackInfo<'_>) {
    let Some(hand) = ci.arg::<InteractionHand>(0).copied() else { return; };
    let Some(item_stack) = compat::detach_ref(ci.arg::<ItemStack>(2)) else { return; };
    let Some(this) = compat::detach(ci.this_mut::<MultiPlayerGameMode>()) else { return; };
    if crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager::get().ui_and_visuals.swing_on_abilities
        && crate::de::hysky::skyblocker::skyblock::swing_animation::SwingAnimation::has_ability(item_stack) {
        swing_hand_without_packets(this, hand);
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.multiplayer.MultiPlayerGameMode#lambda$useItem$0", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(swing_on_ability)));
}
