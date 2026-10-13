//! Ported from de.hysky.skyblocker.skyblock.ViewstashAutocomplete by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::command_utils::no_op;
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::command::Command;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::com::mojang::brigadier::tree::command_node::CommandNode;
use mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource;
use mcrust::net::minecraft::commands::shared_suggestion_provider;

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
    pub fn get_command_node() -> ClientCommandNode {
        //   java: return literal("viewstash")
        //   java: 					.requires(_ -> Utils.isOnSkyblock())
        //   java: 					.executes(CommandUtils.noOp)
        //   java: 					.then(argument("stash", StringArgumentType.word())
        //   java: 									.suggests((_, builder) -> SharedSuggestionProvider.suggest(Stream.of("material", "item"), builder))
        //   java: 									.executes(CommandUtils.noOp)
        //   java: 					).build();
        let mut stash = client_command_registration_callback::argument("stash", Arc::new(StringArgumentType::word()));
        stash.suggests(Some(Arc::new(|_: &CommandContext<FabricClientCommandSource>, builder: &mut SuggestionsBuilder| Ok(shared_suggestion_provider::suggest(&["material".to_string(), "item".to_string()], builder)))));
        stash.executes(no_op());
        let mut node = client_command_registration_callback::literal("viewstash");
        node.requires(Arc::new(|_: &FabricClientCommandSource| crate::de::hysky::skyblocker::utils::utils::Utils::is_on_skyblock()));
        node.executes(no_op());
        node.then(&stash);
        return node.build();
    }

}

/// `LiteralCommandNode<FabricClientCommandSource>`: the port's `ArgumentBuilder.build()` answers the node as a
/// `CommandNode` (its `NodeKind::Literal`), the type `ClientboundCommandsPacketMixin` hands back.
pub type ClientCommandNode = Arc<CommandNode<FabricClientCommandSource>>;

