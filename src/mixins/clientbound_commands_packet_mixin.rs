//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ClientboundCommandsPacketMixin by fabric2seer.
//! @Mixin(net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyExpressionValue modifyCommandSuggestions
///   java: @ModifyExpressionValue(method = "resolve", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket$NodeResolver;resolve(I)Lcom/mojang/brigadier/tree/CommandNode;", ordinal = 1))
/// target: net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver#resolve  [exists]
/// within: net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver#resolve
/// Rust: net/minecraft/network/protocol/game/clientbound_commands_packet.rs:559 fn resolve(&mut self, index: i32)
/// note: ordinal 1: the call is under an `if` in the Java of the method, so the run-time count (per run of the method) may differ from Mixin's bytecode ordinal - check it
fn modify_command_suggestions(v: mcrust::com::mojang::brigadier::tree::command_node::CommandNode<Box<dyn mcrust::net::minecraft::commands::shared_suggestion_provider::SharedSuggestionProvider>>) -> mcrust::com::mojang::brigadier::tree::command_node::CommandNode<Box<dyn mcrust::net::minecraft::commands::shared_suggestion_provider::SharedSuggestionProvider>> {
    let mut original = v;
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel() && let Some(literal_command_node) = original.as_any().downcast_ref::<mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode>() {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (literalCommandNode.getLiteral()) {
        //   java: 				case String s when s.equals("setmaxspeed") -> SpeedPresets.getCommandNode();
        //   java: 				case String s when s.equals("warp") && WarpAutocomplete.commandNode != null -> WarpAutocomplete.commandNode;
        //   java: 				case String s when s.equals("getfromsacks") && SackItemAutocomplete.longCommandNode != null -> SackItemAutocomplete.longCommandNode;
        //   java: 				case String s when s.equals("gfs") && SackItemAutocomplete.shortCommandNode != null -> SackItemAutocomplete.shortCommandNode;
        //   java: 				case String s when s.equals("viewstash") -> ViewstashAutocomplete.getCommandNode();
        //   java: 				case String s when s.equals("joininstance") && JoinInstanceAutocomplete.joinInstanceCommand != null -> JoinInstanceAutocomplete.joinInstanceCommand;
        //   java: 				case String s when s.equals("joindungeon") && JoinInstanceAutocomplete.dungeonCommand != null -> JoinInstanceAutocomplete.dungeonCommand;
        //   java: 				case String s when s.equals("joinkuudra") && JoinInstanceAutocomplete.kuudraCommand != null -> JoinInstanceAutocomplete.kuudraCommand;
        //   java: 				case String s when s.equals("rngmeter") && RngMeterAutocomplete.longCommand != null -> RngMeterAutocomplete.longCommand;
        //   java: 				case String s when s.equals("rng") && RngMeterAutocomplete.shortCommand != null -> RngMeterAutocomplete.shortCommand;
        //   java: 				case String s when s.equals("call") && CallAutocomplete.commandNode != null -> CallAutocomplete.commandNode;
        //   java: 				case String s when s.equals("ahs") && AuctionBazaarAutocomplete.ahsNode != null -> AuctionBazaarAutocomplete.ahsNode;
        //   java: 				case String s when s.equals("ahsearch") && AuctionBazaarAutocomplete.ahsearchNode != null -> AuctionBazaarAutocomplete.ahsearchNode;
        //   java: 				case String s when s.equals("bz") && AuctionBazaarAutocomplete.bzNode != null -> AuctionBazaarAutocomplete.bzNode;
        //   java: 				case String s when s.equals("bazaar") && AuctionBazaarAutocomplete.bazaarNode != null -> AuctionBazaarAutocomplete.bazaarNode;
        //   java: 				case String s when s.equals("chapter") && ChaptersAutocomplete.singularCommandNode != null -> ChaptersAutocomplete.singularCommandNode;
        //   java: 				case String s when s.equals("chapters") && ChaptersAutocomplete.pluralCommandNode != null -> ChaptersAutocomplete.pluralCommandNode;
        //   java: 				default -> original;
        //   java: 			};
    }
    return original;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver#resolve", "net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver#resolve").ordinal(1).priority(1000).modify_expression_value::<mcrust::com::mojang::brigadier::tree::command_node::CommandNode<Box<dyn mcrust::net::minecraft::commands::shared_suggestion_provider::SharedSuggestionProvider>>>(modify_command_suggestions));
}
