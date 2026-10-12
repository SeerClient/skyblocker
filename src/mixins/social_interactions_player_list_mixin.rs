//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.SocialInteractionsPlayerListMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.social.SocialInteractionsPlayerList)

#[allow(unused_imports)]
use crate::prelude::*;

/// `String.matches("[A-Za-z0-9_]+")`, compiled once.
static VALID_PLAYER_NAME: LazyLock<compat::regex::Pattern> = LazyLock::new(|| compat::regex::Pattern::compile("[A-Za-z0-9_]+"));

/// @WrapWithCondition skyblocker$hideInvalidPlayers
///   java: @WrapWithCondition(method = "addOnlinePlayers", at = @At(value = "INVOKE", target = "Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"))
/// target: java.util.Map#put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;  [class not in JavaSrc]
/// within: net.minecraft.client.gui.screens.social.SocialInteractionsPlayerList#addOnlinePlayers
/// Rust: net/minecraft/client/gui/screens/social/social_interactions_player_list.rs:146 fn add_online_players(&self, ctx: &PopulationContext<'_>, players_to_add: &[Uuid], output: &mut Vec<PlayerEntry>)
///
/// `output.put(id, player)` is not a call in the port (`retain` + `push` on a `Vec`, :150-153), so the condition is
/// applied at RETURN to what the method put: `output` is the fresh map `updatePlayerList:50` hands in (empty before
/// the call), so dropping every entry the condition refuses leaves exactly the entries the wrapped `put`s would
/// have made.
fn skyblocker_hide_invalid_players(ci: &mut CallbackInfo<'_>) {
    let Some(output) = compat::hook_arg_mut::<Vec<mcrust::net::minecraft::client::gui::screens::social::player_entry::PlayerEntry>>(ci, 2) else { return; };
    output.retain(|entry| hide_invalid_players_condition(entry));
}

/// The `@WrapWithCondition` body: `true` makes the `put`.
fn hide_invalid_players_condition(entry: &mcrust::net::minecraft::client::gui::screens::social::player_entry::PlayerEntry) -> bool {
    return !(crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock() && !VALID_PLAYER_NAME.matches_all(entry.get_player_name()));
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // java: @WrapWithCondition on `Map.put` inside addOnlinePlayers - see the handler.
    report(hook::register_injector("net.minecraft.client.gui.screens.social.SocialInteractionsPlayerList#addOnlinePlayers", InjectOptions::at(At::Return).kind(InjectorKind::Inject).priority(1000), Arc::new(skyblocker_hide_invalid_players)));
}
