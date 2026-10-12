//! Ported from de.hysky.skyblocker.skyblock.ViewstashAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::brigadier::tree::literal_command_node::LiteralCommandNode;

/// java: public class ViewstashAutocomplete
#[derive(Clone)]
pub struct ViewstashAutocomplete {}

impl ViewstashAutocomplete {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static LiteralCommandNode<FabricClientCommandSource> getCommandNode()
    pub fn get_command_node() -> LiteralCommandNode {
        // TODO(fabric2seer): a command builder outside dispatcher.register(..)
        //   java: return literal("viewstash")
        //   java: 					.requires(_ -> Utils.isOnSkyblock())
        //   java: 					.executes(CommandUtils.noOp)
        //   java: 					.then(argument("stash", StringArgumentType.word())
        //   java: 									.suggests((_, builder) -> SharedSuggestionProvider.suggest(Stream.of("material", "item"), builder))
        //   java: 									.executes(CommandUtils.noOp)
        //   java: 					).build();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
