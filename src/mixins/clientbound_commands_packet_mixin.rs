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
        //   java: return switch (literalCommandNode.getLiteral()) { .. default -> original; };
        use crate::de::hysky::skyblocker::skyblock as sb;
        let literal: String = literal_command_node.get_literal().to_string();
        let replacement: Option<ClientCommandNode> = match literal.as_str() {
            //   java: case String s when s.equals("setmaxspeed") -> SpeedPresets.getCommandNode();
            "setmaxspeed" => Some(sb::speedpreset::speed_presets::SpeedPresets::get_command_node()),
            "warp" => compat::get_state(&sb::warp_autocomplete::COMMAND_NODE),
            "getfromsacks" => compat::get_state(&sb::sack_item_autocomplete::LONG_COMMAND_NODE),
            "gfs" => compat::get_state(&sb::sack_item_autocomplete::SHORT_COMMAND_NODE),
            "viewstash" => Some(sb::viewstash_autocomplete::ViewstashAutocomplete::get_command_node()),
            "joininstance" => compat::get_state(&sb::join_instance_autocomplete::JOIN_INSTANCE_COMMAND),
            "joindungeon" => compat::get_state(&sb::join_instance_autocomplete::DUNGEON_COMMAND),
            "joinkuudra" => compat::get_state(&sb::join_instance_autocomplete::KUUDRA_COMMAND),
            "rngmeter" => compat::get_state(&sb::rng_meter_autocomplete::LONG_COMMAND),
            "rng" => compat::get_state(&sb::rng_meter_autocomplete::SHORT_COMMAND),
            //   java: case String s when s.equals("call") && CallAutocomplete.commandNode != null -> CallAutocomplete.commandNode;
            "call" => compat::get_state(&sb::call_autocomplete::COMMAND_NODE),
            "ahs" => compat::get_state(&sb::auction_bazaar_autocomplete::AHS_NODE),
            "ahsearch" => compat::get_state(&sb::auction_bazaar_autocomplete::AHSEARCH_NODE),
            "bz" => compat::get_state(&sb::auction_bazaar_autocomplete::BZ_NODE),
            "bazaar" => compat::get_state(&sb::auction_bazaar_autocomplete::BAZAAR_NODE),
            "chapter" => compat::get_state(&sb::chapters_autocomplete::SINGULAR_COMMAND_NODE),
            "chapters" => compat::get_state(&sb::chapters_autocomplete::PLURAL_COMMAND_NODE),
            _ => None,
        };
        // `case .. && X != null -> X; default -> original`
        return replacement.unwrap_or(original);
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
