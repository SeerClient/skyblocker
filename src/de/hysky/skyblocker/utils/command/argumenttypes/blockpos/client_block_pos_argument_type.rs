//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.blockpos.ClientBlockPosArgumentType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_pos_argument::ClientPosArgument;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::default_client_pos_argument::DefaultClientPosArgument;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::looking_client_pos_argument::LookingClientPosArgument;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::com::mojang::brigadier::suggestion::suggestions::Suggestions;
use mcrust::com::mojang::brigadier::suggestion::suggestions_builder::SuggestionsBuilder;
use mcrust::net::minecraft::client::multiplayer::client_level::ClientLevel;
use mcrust::net::minecraft::commands::commands;
use mcrust::net::minecraft::commands::shared_suggestion_provider;
use mcrust::net::minecraft::commands::shared_suggestion_provider::SharedSuggestionProvider;
use mcrust::net::minecraft::commands::shared_suggestion_provider::TextCoordinates;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::level::level;
use mcrust::com::mojang::brigadier::immutable_string_reader::ImmutableStringReader as _;

/// java: public class ClientBlockPosArgumentType implements ArgumentType<ClientPosArgument>
#[derive(Clone)]
pub struct ClientBlockPosArgumentType {}

impl ClientBlockPosArgumentType {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static ClientBlockPosArgumentType blockPos()
    pub fn block_pos() -> ClientBlockPosArgumentType {
        return ClientBlockPosArgumentType::new();
    }

    /// java: public static BlockPos getLoadedBlockPos(CommandContext<FabricClientCommandSource> context, String name) throws CommandSyntaxException
    pub fn get_loaded_block_pos(context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, name: &str) -> BlockPos {
        // java: return getLoadedBlockPos(context, context.getSource().getLevel(), name); - Fabric's client source: `client.level`
        let level = mcrust::mcrust::modding::game::level().unwrap_or_else(|| panic!("NullPointerException: no level"));
        return ClientBlockPosArgumentType::get_loaded_block_pos_3(context, &level, name);
    }

    /// java: @SuppressWarnings("deprecation") public static BlockPos getLoadedBlockPos(CommandContext<FabricClientCommandSource> context, ClientLevel world, String name) throws CommandSyntaxException
    pub fn get_loaded_block_pos_3(context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, world: &ClientLevel, name: &str) -> BlockPos {
        let mut block_pos: BlockPos = ClientBlockPosArgumentType::get_block_pos(context, name);
        if !(world.has_chunk_at(&block_pos)) {
            panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::block_pos_argument::error_not_loaded().create());
        }
        if !(world.is_in_world_bounds(&block_pos)) {
            panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::block_pos_argument::error_out_of_world().create());
        }
        return block_pos;
    }

    /// java: public static BlockPos getBlockPos(CommandContext<FabricClientCommandSource> context, String name)
    pub fn get_block_pos(context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, name: &str) -> BlockPos {
        return ({ let __r = &context.get_argument::<compat::Shared<dyn ClientPosArgument>>(name).unwrap(); let __f = compat::dispatch(__r, |__t| __t.to_absolute_block_pos()); __f(__r, context.get_source()) });
    }

    /// java: public static BlockPos getValidBlockPos(CommandContext<FabricClientCommandSource> context, String name) throws CommandSyntaxException
    pub fn get_valid_block_pos(context: &CommandContext<mcrust::mcrust::modding::events::client_command_registration_callback::FabricClientCommandSource>, name: &str) -> BlockPos {
        let mut block_pos: BlockPos = ClientBlockPosArgumentType::get_block_pos(context, name);
        if !(level::is_in_spawnable_bounds(&block_pos)) {
            panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::block_pos_argument::error_out_of_bounds().create());
        } else {
            return block_pos;
        }
    }

    /// java: public ClientPosArgument parse(StringReader stringReader) throws CommandSyntaxException
    pub fn parse(&self, string_reader: &mut StringReader) -> compat::Shared<dyn ClientPosArgument> {
        return ((if string_reader.can_read() && string_reader.peek() == '^' { (LookingClientPosArgument::parse(string_reader) as compat::Shared<dyn ClientPosArgument>) } else { (DefaultClientPosArgument::parse(string_reader) as compat::Shared<dyn ClientPosArgument>) }) as compat::Shared<dyn ClientPosArgument>);
    }

    /// java: @Override public <S> CompletableFuture<Suggestions> listSuggestions(CommandContext<S> context, SuggestionsBuilder builder)
    pub fn list_suggestions<S: 'static>(&self, context: &CommandContext<S>, builder: &mut SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        // java: if (!(context.getSource() instanceof SharedSuggestionProvider commandSource)) return Suggestions.empty();
        // (Fabric's client source is a SharedSuggestionProvider; any other source answers no suggestions)
        let _ = context;
        let string: String = builder.get_remaining().to_string();
        // java: Collection<TextCoordinates> collection = !string.isEmpty() && string.charAt(0) == '^' ? Collections.singleton(TextCoordinates.DEFAULT_LOCAL) : commandSource.getRelevantCoordinates();
        // (`SharedSuggestionProvider.getRelevantCoordinates()`'s default: `Collections.singleton(TextCoordinates.DEFAULT_GLOBAL)`)
        let collection: Vec<TextCoordinates> = if string.starts_with('^') { vec![TextCoordinates::default_local()] } else { vec![TextCoordinates::default_global()] };
        return shared_suggestion_provider::suggest_coordinates(&string, &collection, builder, &commands::create_validator(|__a0: &mut StringReader| { self.parse(__a0); Ok(()) }));
    }

    /// java: @Override public Collection<String> getExamples()
    pub fn get_examples(&self) -> Vec<String> {
        return mcrust::net::minecraft::commands::arguments::coordinates::block_pos_argument::EXAMPLES.iter().map(|example| example.to_string()).collect();
    }

}

/// java: `implements ArgumentType<ClientPosArgument>` - the parsed value is the `ClientPosArgument` handle
impl std::fmt::Debug for ClientBlockPosArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ClientBlockPosArgumentType")
    }
}

impl mcrust::com::mojang::brigadier::arguments::argument_type::ArgumentType for ClientBlockPosArgumentType {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn std::any::Any + Send + Sync>, mcrust::com::mojang::brigadier::exceptions::command_syntax_exception::CommandSyntaxException> {
        // the Java parsers throw `CommandSyntaxException`; the port's parse helpers panic with it
        let start = reader.get_cursor();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ClientBlockPosArgumentType::parse(self, reader))) {
            Ok(argument) => Ok(Arc::new(argument)),
            Err(payload) => {
                reader.set_cursor(start);
                match payload.downcast::<mcrust::com::mojang::brigadier::exceptions::command_syntax_exception::CommandSyntaxException>() {
                    Ok(exception) => Err(*exception),
                    Err(payload) => std::panic::resume_unwind(payload),
                }
            }
        }
    }

    fn list_suggestions(&self, context: &dyn std::any::Any, builder: &mut SuggestionsBuilder) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Suggestions> {
        let _ = context;
        let string: String = builder.get_remaining().to_string();
        let collection: Vec<TextCoordinates> = if string.starts_with('^') { vec![TextCoordinates::default_local()] } else { vec![TextCoordinates::default_global()] };
        shared_suggestion_provider::suggest_coordinates(&string, &collection, builder, &commands::create_validator(|__a0: &mut StringReader| { ClientBlockPosArgumentType::parse(self, __a0); Ok(()) }))
    }

    fn get_examples(&self) -> Vec<String> {
        ClientBlockPosArgumentType::get_examples(self)
    }
}
