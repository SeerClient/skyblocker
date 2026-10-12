//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ClientboundCommandsPacketMixin by fabric2seer.
//! @Mixin(net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver)

#[allow(unused_imports)]
use crate::prelude::*;

/// The client's command node type: `ClientPacketListener.handleCommands` resolves the packet with the
/// `CommandNodeBuilder` over `ClientSuggestionProvider` (`client_packet_listener.rs:3782`).
type ClientCommandNode = std::sync::Arc<
    mcrust::com::mojang::brigadier::tree::command_node::CommandNode<
        std::sync::Arc<mcrust::net::minecraft::client::multiplayer::client_suggestion_provider::ClientSuggestionProvider>,
    >,
>;

/// @ModifyExpressionValue modifyCommandSuggestions
///   java: @ModifyExpressionValue(method = "resolve", at = @At(value = "INVOKE", target = "Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket$NodeResolver;resolve(I)Lcom/mojang/brigadier/tree/CommandNode;", ordinal = 1))
/// target: net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver#resolve  [exists]
/// within: net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver#resolve
/// Rust: net/minecraft/network/protocol/game/clientbound_commands_packet.rs:559 fn resolve(&mut self, index: i32)
fn modify_command_suggestions(v: ClientCommandNode) -> ClientCommandNode {
    let mut original = v;
    if crate::de::hysky::skyblocker::utils::utils::Utils::is_on_hypixel()
        && let mcrust::com::mojang::brigadier::tree::command_node::NodeKind::Literal(literal_command_node) = original.kind()
    {
        // TODO(batchD): cross-package - every replacement node below is a static of another package typed
        // `Option<LiteralCommandNode>` (the literal's name only, no children or suggestions) or a
        // `CommandNode<FabricClientCommandSource>`; Java hands back the mod's whole `CommandNode`, which here must be
        // a `ClientCommandNode` (`Arc<CommandNode<Arc<ClientSuggestionProvider>>>`). Until those statics carry that
        // type, each case answers `original` (Java's `default`).
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
        let _literal: &str = literal_command_node.get_literal();
        return original;
    }
    return original;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // `ordinal = 1` is the SECOND `resolve` call INSTRUCTION of `resolve:309-340` - the children loop's `:332`, for
    // every child - while SeerClient's `ordinal(n)` counts calls as they run (the first `:321` redirect call shifts
    // it). So no ordinal is given: every nested `resolve` result passes through the handler, which also sees a
    // redirect target (`:321`); the handler leaves any node it does not replace unchanged.
    report(hook::call_site("net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver#resolve", "net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeResolver#resolve").priority(1000).modify_expression_value::<ClientCommandNode>(modify_command_suggestions));
}
